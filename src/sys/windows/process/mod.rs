mod enumerator;
mod open_options;
mod process;

pub use enumerator::{ProcessEnumerator, ProcessRecord};
pub use open_options::ProcessOpenOptions;
pub use process::Process;
use winapi::shared::minwindef::DWORD;

#[derive(Clone, Copy)]
pub struct Pid(DWORD);

impl From<DWORD> for Pid {
    fn from(pid: DWORD) -> Self {
        Pid(pid)
    }
}
