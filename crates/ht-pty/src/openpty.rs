//! POSIX PTY implementation using forkpty

use std::path::PathBuf;
use std::os::unix::io::{AsRawFd, FromRawFd};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use nix::pty::{Winsize, openpty};
use nix::unistd::{ForkResult, fork, setsid, dup2, execve, close};
use nix::sys::wait::{waitpid, WaitStatus};
use std::ffi::CString;

use crate::pty::{Pty, PtySize, PtyEvent};

pub struct OpenPty {
    master_fd: Option<tokio::fs::File>,
    child_pid: Option<nix::unistd::Pid>,
}

impl OpenPty {
    pub fn new() -> Self {
        Self {
            master_fd: None,
            child_pid: None,
        }
    }

    fn setup_environment(env: &[(String, String)]) -> anyhow::Result<Vec<CString>> {
        let mut env_vars = Vec::new();

        for (key, value) in env {
            let env_str = format!("{}={}", key, value);
            env_vars.push(CString::new(env_str)?);
        }

        Ok(env_vars)
    }
}

#[async_trait::async_trait]
impl Pty for OpenPty {
    async fn spawn(
        &mut self,
        command: &str,
        args: &[String],
        env: &[(String, String)],
        cwd: Option<PathBuf>,
        size: PtySize,
    ) -> anyhow::Result<()> {
        // Create PTY
        let winsize = Winsize {
            ws_row: size.rows,
            ws_col: size.cols,
            ws_xpixel: size.pixel_width,
            ws_ypixel: size.pixel_height,
        };

        let pty_result = openpty(Some(&winsize), None)?;

        // Fork process
        match unsafe { fork()? } {
            ForkResult::Parent { child } => {
                // Parent process
                close(pty_result.slave)?;

                self.child_pid = Some(child);
                self.master_fd = Some(unsafe {
                    tokio::fs::File::from_std(std::fs::File::from_raw_fd(pty_result.master))
                });

                tracing::info!("PTY spawned: {} {:?} (pid: {})", command, args, child);
                Ok(())
            }
            ForkResult::Child => {
                // Child process
                close(pty_result.master).expect("Failed to close master fd");

                // Create new session
                setsid().expect("Failed to create new session");

                // Redirect stdio to slave
                dup2(pty_result.slave, 0).expect("Failed to dup stdin");
                dup2(pty_result.slave, 1).expect("Failed to dup stdout");
                dup2(pty_result.slave, 2).expect("Failed to dup stderr");
                close(pty_result.slave).expect("Failed to close slave fd");

                // Change directory
                if let Some(cwd) = cwd {
                    std::env::set_current_dir(cwd).expect("Failed to change directory");
                }

                // Set environment
                for (key, value) in env {
                    std::env::set_var(key, value);
                }

                // Build arguments
                let command_cstr = CString::new(command).expect("Invalid command");
                let mut args_cstr = vec![command_cstr.clone()];
                for arg in args {
                    args_cstr.push(CString::new(arg.as_str()).expect("Invalid argument"));
                }

                // Prepare environment
                let env_cstr = Self::setup_environment(env).expect("Failed to setup environment");

                // Execute command
                execve(&command_cstr, &args_cstr, &env_cstr).expect("Failed to exec");

                unreachable!()
            }
        }
    }

    async fn write(&mut self, data: &[u8]) -> anyhow::Result<()> {
        if let Some(master) = &mut self.master_fd {
            master.write_all(data).await?;
        }
        Ok(())
    }

    async fn resize(&mut self, size: PtySize) -> anyhow::Result<()> {
        if let Some(master) = &self.master_fd {
            let winsize = Winsize {
                ws_row: size.rows,
                ws_col: size.cols,
                ws_xpixel: size.pixel_width,
                ws_ypixel: size.pixel_height,
            };

            nix::ioctl_write_ptr_bad!(tiocswinsz, libc::TIOCSWINSZ, Winsize);
            unsafe {
                tiocswinsz(master.as_raw_fd(), &winsize)?;
            }
        }
        Ok(())
    }

    async fn read_event(&mut self) -> anyhow::Result<PtyEvent> {
        if let Some(master) = &mut self.master_fd {
            let mut buffer = vec![0u8; 8192];

            tokio::select! {
                result = master.read(&mut buffer) => {
                    match result {
                        Ok(0) => Ok(PtyEvent::Exit(None)),
                        Ok(n) => {
                            buffer.truncate(n);
                            Ok(PtyEvent::Data(buffer))
                        }
                        Err(e) => Err(e.into()),
                    }
                }
                exit_code = Self::wait_for_child(self.child_pid) => {
                    Ok(PtyEvent::Exit(exit_code))
                }
            }
        } else {
            anyhow::bail!("PTY not initialized")
        }
    }

    async fn kill(&mut self) -> anyhow::Result<()> {
        if let Some(pid) = self.child_pid {
            nix::sys::signal::kill(pid, nix::sys::signal::SIGTERM)?;
        }
        Ok(())
    }
}

impl OpenPty {
    async fn wait_for_child(pid: Option<nix::unistd::Pid>) -> Option<i32> {
        if let Some(pid) = pid {
            tokio::task::spawn_blocking(move || {
                match waitpid(pid, None) {
                    Ok(WaitStatus::Exited(_, code)) => Some(code),
                    _ => None,
                }
            }).await.ok().flatten()
        } else {
            None
        }
    }
}

impl Drop for OpenPty {
    fn drop(&mut self) {
        if let Some(pid) = self.child_pid {
            let _ = nix::sys::signal::kill(pid, nix::sys::signal::SIGKILL);
        }
    }
}
