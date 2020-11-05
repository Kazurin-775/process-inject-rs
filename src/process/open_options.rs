use super::{Pid, Process};
use crate::sys::process as imp;

pub struct ProcessOpenOptions {
    inner: imp::ProcessOpenOptions,
}

impl From<imp::ProcessOpenOptions> for ProcessOpenOptions {
    fn from(inner: imp::ProcessOpenOptions) -> Self {
        ProcessOpenOptions { inner }
    }
}

impl From<ProcessOpenOptions> for imp::ProcessOpenOptions {
    fn from(outer: ProcessOpenOptions) -> Self {
        outer.inner
    }
}

impl ProcessOpenOptions {
    pub fn new(pid: Pid) -> ProcessOpenOptions {
        ProcessOpenOptions {
            inner: crate::sys::process::ProcessOpenOptions::new(pid.into()),
        }
    }

    pub fn open(self) -> crate::Result<Process> {
        self.inner.open().map(|inner| inner.into())
    }

    pub fn enable_mem_alloc(mut self) -> Self {
        self.inner.enable_mem_alloc();
        self
    }

    pub fn enable_mem_read(mut self) -> Self {
        self.inner.enable_mem_read();
        self
    }

    pub fn enable_mem_write(mut self) -> Self {
        self.inner.enable_mem_write();
        self
    }

    pub fn enable_execute(mut self) -> Self {
        self.inner.enable_execute();
        self
    }

    pub fn enable_all_access(mut self) -> Self {
        self.inner.enable_all_access();
        self
    }
}
