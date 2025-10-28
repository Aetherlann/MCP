use std::path::PathBuf;
use serde::{Serialize, Deserialize};

/// Terminal size in cells and pixels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PtySize {
    pub rows: u16,
    pub cols: u16,
    pub pixel_width: u16,
    pub pixel_height: u16,
}

impl PtySize {
    pub fn new(rows: u16, cols: u16) -> Self {
        Self {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        }
    }

    pub fn with_pixels(rows: u16, cols: u16, pixel_width: u16, pixel_height: u16) -> Self {
        Self {
            rows,
            cols,
            pixel_width,
            pixel_height,
        }
    }
}

/// Events from the PTY
#[derive(Debug)]
pub enum PtyEvent {
    /// Data received from the shell
    Data(Vec<u8>),
    /// PTY process exited with code
    Exit(Option<i32>),
}

/// PTY trait for cross-platform abstraction
#[async_trait::async_trait]
pub trait Pty: Send + Sync {
    /// Spawn a new PTY with the given command and environment
    async fn spawn(
        &mut self,
        command: &str,
        args: &[String],
        env: &[(String, String)],
        cwd: Option<PathBuf>,
        size: PtySize,
    ) -> anyhow::Result<()>;

    /// Write data to the PTY
    async fn write(&mut self, data: &[u8]) -> anyhow::Result<()>;

    /// Resize the PTY
    async fn resize(&mut self, size: PtySize) -> anyhow::Result<()>;

    /// Read next event from the PTY
    async fn read_event(&mut self) -> anyhow::Result<PtyEvent>;

    /// Kill the PTY process
    async fn kill(&mut self) -> anyhow::Result<()>;
}
