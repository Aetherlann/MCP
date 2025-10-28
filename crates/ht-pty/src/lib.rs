pub mod pty;

pub use pty::{Pty, PtySize, PtyEvent};

#[cfg(windows)]
pub mod conpty;

#[cfg(unix)]
pub mod openpty;
