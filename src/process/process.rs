use crate::sys::process as imp;

use super::{MemAccess, Pid, ProcessOpenOptions};

pub struct Process {
    inner: imp::Process,
}

impl From<imp::Process> for Process {
    fn from(inner: imp::Process) -> Self {
        Process { inner }
    }
}

impl From<Process> for imp::Process {
    fn from(outer: Process) -> Self {
        outer.inner
    }
}

impl AsRef<imp::Process> for Process {
    fn as_ref(&self) -> &imp::Process {
        &self.inner
    }
}

impl AsMut<imp::Process> for Process {
    fn as_mut(&mut self) -> &mut imp::Process {
        &mut self.inner
    }
}

impl Process {
    pub fn open(pid: Pid) -> crate::Result<Process> {
        ProcessOpenOptions::new(pid).enable_all_access().open()
    }

    pub unsafe fn alloc_memory(&mut self, size: usize, access: MemAccess) -> crate::Result<usize> {
        self.inner.alloc_memory(size, access)
    }

    pub unsafe fn dealloc_memory(&mut self, addr: usize) -> crate::Result<()> {
        self.inner.dealloc_memory(addr)
    }

    pub unsafe fn read_memory(&mut self, addr: usize, data: &mut [u8]) -> crate::Result<()> {
        self.inner.read_memory(addr, data)
    }

    pub unsafe fn write_memory(&mut self, addr: usize, data: &[u8]) -> crate::Result<()> {
        self.inner.write_memory(addr, data)
    }

    pub unsafe fn call_function(&mut self, addr: usize, arg: usize) -> crate::Result<()> {
        self.inner.call_function(addr, arg)
    }
}
