use crate::sys::process as imp;

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

impl Process {
    pub unsafe fn alloc_memory(&mut self, size: usize) -> crate::Result<usize> {
        self.inner.alloc_memory(size)
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
