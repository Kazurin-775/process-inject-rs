pub mod enumerate;
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

#[repr(u8)]
pub enum MemoryAccess {
    None = 0,
    ExecuteOnly = 1,
    WriteOnly = 2,
    WriteExecute = 3,
    ReadOnly = 4,
    ReadExecute = 5,
    ReadWrite = 6,
    ReadWriteExecute = 7,
}

impl Process {
    pub fn open(pid: Pid) -> Result<Process> {
        ProcessOpenOptions::new(pid).enable_all_access().open()
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

    pub unsafe fn allocate_memory(&mut self, size: usize, access: MemoryAccess) -> Result<usize> {
        os::allocate_process_memory(self, size, access)
    }

    pub unsafe fn deallocate_memory(&mut self, address: usize) -> Result<()> {
        os::deallocate_process_memory(self, address)
    }

    pub unsafe fn read_memory(&self, address: usize, buffer: &mut [u8]) -> Result<()> {
        os::read_process_memory(self, address, buffer)
    }

    pub unsafe fn write_memory(&mut self, address: usize, buffer: &[u8]) -> Result<()> {
        os::write_process_memory(self, address, buffer)
    }

    pub unsafe fn create_thread(&mut self, function: usize, argument: usize) -> Result<()> {
        os::create_thread(self, function, argument)
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        unsafe {
            os::close_process(self);
        }
    }
}

impl MemoryAccess {
    pub fn to_rwx_tuple(self) -> (bool, bool, bool) {
        let oct = self as u8;
        (oct & 4 != 0, oct & 2 != 0, oct & 1 != 0)
    }

    pub fn from_rwx_tuple(read: bool, write: bool, execute: bool) -> Self {
        match (read, write, execute) {
            (false, false, false) => Self::None,
            (false, false, true) => Self::ExecuteOnly,
            (false, true, false) => Self::WriteOnly,
            (false, true, true) => Self::WriteExecute,
            (true, false, false) => Self::ReadOnly,
            (true, false, true) => Self::ReadExecute,
            (true, true, false) => Self::ReadWrite,
            (true, true, true) => Self::ReadWriteExecute,
        }
    }
}
