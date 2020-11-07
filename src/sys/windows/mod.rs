pub mod inject;
pub mod process;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Win32Error(win32_error::Win32Error),

    #[error("CreateToolhelp32Snapshot failed: {0}")]
    Toolhelp32Error(win32_error::Win32Error),

    #[error("CreateRemoteThread failed: {0}")]
    CreateThreadError(win32_error::Win32Error),

    #[error("failed to wait for remote thread: {0}")]
    WaitForThreadError(win32_error::Win32Error),
}
