use super::Process;

pub struct ProcessOpenOptions {
}

impl ProcessOpenOptions {
    pub fn new() -> ProcessOpenOptions {
        unimplemented!()
    }

    pub fn open(self) -> crate::Result<Process> {
        unimplemented!()
    }

    pub fn enable_mem_alloc(self) -> Self {
        unimplemented!()
    }

    pub fn enable_mem_read(self) -> Self {
        unimplemented!()
    }

    pub fn enable_mem_write(self) -> Self {
        unimplemented!()
    }

    pub fn enable_execute(self) -> Self {
        unimplemented!()
    }

    pub fn enable_all_access(self) -> Self {
        unimplemented!()
    }
}
