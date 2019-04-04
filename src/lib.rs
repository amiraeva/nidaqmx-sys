#[cfg_attr(not(windows), path = "unix.rs")]
#[cfg_attr(windows, path = "windows.rs")]
mod os;

pub use os::*;
