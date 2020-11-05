pub struct Process {
}

impl Process {
    pub unsafe fn alloc_memory(&mut self, size: usize) -> crate::Result<usize> {
        unimplemented!()
    }

    pub unsafe fn dealloc_memory(&mut self, addr: usize) -> crate::Result<()> {
        unimplemented!()
    }

    pub unsafe fn read_memory(&mut self, addr: usize, data: &mut [u8]) -> crate::Result<()> {
        unimplemented!()
    }

    pub unsafe fn write_memory(&mut self, addr: usize, data: &[u8]) -> crate::Result<()> {
        unimplemented!()
    }

    pub unsafe fn call_function(&mut self, addr: usize, arg: usize) -> crate::Result<()> {
        unimplemented!()
    }
}
