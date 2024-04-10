#[cfg(unix)]
pub mod unix;
#[cfg(not(unix))]
pub mod unix {}
#[cfg(windows)]
pub mod windows;
#[cfg(not(windows))]
pub mod windows {}
