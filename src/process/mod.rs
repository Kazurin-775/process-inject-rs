pub mod open_options;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
use windows as os;

use os::{Error, Handle, Pid};

pub use open_options::ProcessOpenOptions;

pub type Result<T> = std::result::Result<T, Error>;

pub struct Process {
    handle: Handle,
}

impl Process {
    pub fn open(pid: Pid) -> Result<Process> {
        ProcessOpenOptions::new(pid)
            .enable_all_access()
            .open()
    }

    pub fn from_raw(handle: Handle) -> Process {
        Process { handle }
    }

    pub fn to_raw_handle(&self) -> Handle {
        self.handle
    }

    pub fn into_raw(self) -> Handle {
        let boxed = std::mem::ManuallyDrop::new(self);
        boxed.handle
    }

    pub unsafe fn read_memory(&self, address: usize, buffer: &mut [u8]) -> Result<()> {
        os::read_process_memory(self, address, buffer)
    }

    pub unsafe fn write_memory(&mut self, address: usize, buffer: &[u8]) -> Result<()> {
        os::write_process_memory(self, address, buffer)
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        unsafe {
            os::close_process(self);
        }
    }
}
