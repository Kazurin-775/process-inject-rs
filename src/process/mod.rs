mod enumerator;
mod mem_access;
mod open_options;
mod process;
mod record;

use crate::sys::process as imp;
pub use enumerator::ProcessEnumerator;
pub use mem_access::MemAccess;
pub use open_options::ProcessOpenOptions;
pub use process::Process;
pub use record::ProcessRecord;

#[derive(Clone, Copy)]
pub struct Pid {
    inner: imp::Pid,
}

impl From<imp::Pid> for Pid {
    fn from(inner: imp::Pid) -> Self {
        Pid { inner }
    }
}

impl From<Pid> for imp::Pid {
    fn from(outer: Pid) -> Self {
        outer.inner
    }
}
