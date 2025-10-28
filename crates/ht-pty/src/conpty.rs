//! Windows ConPTY implementation

use std::path::PathBuf;
use std::os::windows::io::AsRawHandle;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use windows::Win32::Foundation::*;
use windows::Win32::System::Console::*;
use windows::Win32::System::Threading::*;
use windows::Win32::Storage::FileSystem::*;
use windows::Win32::System::Pipes::*;
use windows::core::*;

use crate::pty::{Pty, PtySize, PtyEvent};

pub struct ConPty {
    pseudo_console: HPCON,
    process_handle: HANDLE,
    input_pipe: tokio::fs::File,
    output_pipe: tokio::fs::File,
}

impl ConPty {
    pub fn new() -> Self {
        Self {
            pseudo_console: HPCON::default(),
            process_handle: HANDLE::default(),
            input_pipe: unsafe { std::mem::zeroed() }, // Will be initialized in spawn
            output_pipe: unsafe { std::mem::zeroed() },
        }
    }

    fn create_pipes() -> anyhow::Result<(HANDLE, HANDLE, HANDLE, HANDLE)> {
        unsafe {
            let mut in_read = HANDLE::default();
            let mut in_write = HANDLE::default();
            let mut out_read = HANDLE::default();
            let mut out_write = HANDLE::default();

            CreatePipe(&mut in_read, &mut in_write, None, 0)?;
            CreatePipe(&mut out_read, &mut out_write, None, 0)?;

            Ok((in_read, in_write, out_read, out_write))
        }
    }
}

#[async_trait::async_trait]
impl Pty for ConPty {
    async fn spawn(
        &mut self,
        command: &str,
        args: &[String],
        env: &[(String, String)],
        cwd: Option<PathBuf>,
        size: PtySize,
    ) -> anyhow::Result<()> {
        unsafe {
            // Create pipes
            let (in_read, in_write, out_read, out_write) = Self::create_pipes()?;

            // Create pseudo console
            let coord = COORD {
                X: size.cols as i16,
                Y: size.rows as i16,
            };

            CreatePseudoConsole(coord, in_read, out_write, 0, &mut self.pseudo_console)?;

            // Set up process to inherit console
            let mut size: usize = 0;
            InitializeProcThreadAttributeList(LPPROC_THREAD_ATTRIBUTE_LIST::default(), 1, 0, &mut size)?;

            let mut attr_list = vec![0u8; size];
            let attr_list_ptr = LPPROC_THREAD_ATTRIBUTE_LIST(attr_list.as_mut_ptr() as *mut _);

            InitializeProcThreadAttributeList(attr_list_ptr, 1, 0, &mut size)?;

            UpdateProcThreadAttribute(
                attr_list_ptr,
                0,
                PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE as usize,
                Some(self.pseudo_console.0 as *const _),
                std::mem::size_of::<HPCON>(),
                None,
                None,
            )?;

            // Build command line
            let mut cmdline = format!("{}", command);
            for arg in args {
                cmdline.push(' ');
                cmdline.push_str(arg);
            }
            let mut cmdline_w: Vec<u16> = cmdline.encode_utf16().chain(std::iter::once(0)).collect();

            // Create process
            let mut startup_info = STARTUPINFOEXW {
                StartupInfo: STARTUPINFOW {
                    cb: std::mem::size_of::<STARTUPINFOEXW>() as u32,
                    ..Default::default()
                },
                lpAttributeList: attr_list_ptr,
            };

            let mut process_info = PROCESS_INFORMATION::default();

            let cwd_w = cwd.map(|p| {
                p.to_string_lossy()
                    .encode_utf16()
                    .chain(std::iter::once(0))
                    .collect::<Vec<u16>>()
            });

            CreateProcessW(
                None,
                PWSTR(cmdline_w.as_mut_ptr()),
                None,
                None,
                false,
                EXTENDED_STARTUPINFO_PRESENT,
                None,
                cwd_w.as_ref().map(|c| PCWSTR(c.as_ptr())),
                &startup_info.StartupInfo,
                &mut process_info,
            )?;

            self.process_handle = process_info.hProcess;
            CloseHandle(process_info.hThread)?;

            // Clean up
            DeleteProcThreadAttributeList(attr_list_ptr);
            CloseHandle(in_read)?;
            CloseHandle(out_write)?;

            // Convert to tokio files
            self.input_pipe = tokio::fs::File::from_std(std::fs::File::from(
                std::os::windows::io::OwnedHandle::from_raw_handle(in_write.0 as _)
            ));
            self.output_pipe = tokio::fs::File::from_std(std::fs::File::from(
                std::os::windows::io::OwnedHandle::from_raw_handle(out_read.0 as _)
            ));

            tracing::info!("ConPTY spawned: {} {:?}", command, args);
            Ok(())
        }
    }

    async fn write(&mut self, data: &[u8]) -> anyhow::Result<()> {
        self.input_pipe.write_all(data).await?;
        Ok(())
    }

    async fn resize(&mut self, size: PtySize) -> anyhow::Result<()> {
        unsafe {
            let coord = COORD {
                X: size.cols as i16,
                Y: size.rows as i16,
            };
            ResizePseudoConsole(self.pseudo_console, coord)?;
            Ok(())
        }
    }

    async fn read_event(&mut self) -> anyhow::Result<PtyEvent> {
        let mut buffer = vec![0u8; 8192];

        tokio::select! {
            result = self.output_pipe.read(&mut buffer) => {
                match result {
                    Ok(0) => Ok(PtyEvent::Exit(None)),
                    Ok(n) => {
                        buffer.truncate(n);
                        Ok(PtyEvent::Data(buffer))
                    }
                    Err(e) => Err(e.into()),
                }
            }
            _ = Self::wait_for_exit(self.process_handle) => {
                let exit_code = Self::get_exit_code(self.process_handle)?;
                Ok(PtyEvent::Exit(Some(exit_code)))
            }
        }
    }

    async fn kill(&mut self) -> anyhow::Result<()> {
        unsafe {
            TerminateProcess(self.process_handle, 1)?;
            Ok(())
        }
    }
}

impl ConPty {
    async fn wait_for_exit(handle: HANDLE) {
        tokio::task::spawn_blocking(move || unsafe {
            WaitForSingleObject(handle, INFINITE);
        }).await.ok();
    }

    fn get_exit_code(handle: HANDLE) -> anyhow::Result<i32> {
        unsafe {
            let mut exit_code = 0u32;
            GetExitCodeProcess(handle, &mut exit_code)?;
            Ok(exit_code as i32)
        }
    }
}

impl Drop for ConPty {
    fn drop(&mut self) {
        unsafe {
            if !self.pseudo_console.is_invalid() {
                ClosePseudoConsole(self.pseudo_console);
            }
            if !self.process_handle.is_invalid() {
                let _ = CloseHandle(self.process_handle);
            }
        }
    }
}
