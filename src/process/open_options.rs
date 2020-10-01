use super::Process;

pub struct ProcessOpenOptions {
    pid: super::Pid,
    mem_alloc: bool,
    mem_read: bool,
    mem_write: bool,
    execute: bool,
}

impl ProcessOpenOptions {
    pub fn new(pid: super::Pid) -> ProcessOpenOptions {
        ProcessOpenOptions {
            pid,
            mem_alloc: false,
            mem_read: false,
            mem_write: false,
            execute: false,
        }
    }

    pub fn enable_mem_allocate(mut self) -> Self {
        self.mem_alloc = true;
        self
    }

    pub fn enable_mem_read(mut self) -> Self {
        self.mem_read = true;
        self
    }

    pub fn enable_mem_readwrite(mut self) -> Self {
        self.mem_read = true;
        self.mem_write = true;
        self
    }

    pub fn enable_execute(mut self) -> Self {
        self.execute = true;
        self
    }

    pub fn enable_all_access(mut self) -> Self {
        self.mem_alloc = true;
        self.mem_read = true;
        self.mem_write = true;
        self.execute = true;
        self
    }

    pub fn open(self) -> super::Result<Process> {
        unsafe {
            super::os::open_process(
                self.pid,
                self.mem_alloc,
                self.mem_read,
                self.mem_write,
                self.execute,
            )
        }
    }
}
