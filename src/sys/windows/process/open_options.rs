use win32_error::Win32Error;
use winapi::shared::minwindef::*;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winnt::*;

use crate::{Error, Result};

use super::{Pid, Process};

pub struct ProcessOpenOptions {
    pid: Pid,
    all_access: bool,
    mem_alloc: bool,
    mem_read: bool,
    mem_write: bool,
    execute: bool,
}

impl ProcessOpenOptions {
    pub fn new(pid: Pid) -> ProcessOpenOptions {
        ProcessOpenOptions {
            pid,
            all_access: false,
            mem_alloc: false,
            mem_read: false,
            mem_write: false,
            execute: false,
        }
    }

    pub fn open(self) -> Result<Process> {
        let mut access = 0;
        if self.all_access {
            access = PROCESS_ALL_ACCESS;
        } else {
            if self.mem_alloc {
                access |= PROCESS_VM_OPERATION;
            }
            if self.mem_read {
                access |= PROCESS_VM_READ;
            }
            if self.mem_write {
                access |= PROCESS_VM_WRITE;
            }
            if self.execute {
                access |= PROCESS_CREATE_THREAD;
            }
        }
        let handle = unsafe { OpenProcess(access, FALSE, self.pid.0) };
        if !handle.is_null() {
            Ok(unsafe { Process::from_raw_handle(handle) })
        } else {
            Err(Error::Win32Error(Win32Error::new()))
        }
    }

    pub fn enable_mem_alloc(&mut self) {
        self.mem_alloc = true;
    }

    pub fn enable_mem_read(&mut self) {
        self.mem_read = true;
    }

    pub fn enable_mem_write(&mut self) {
        self.mem_write = true;
    }

    pub fn enable_execute(&mut self) {
        self.execute = true;
    }

    pub fn enable_all_access(&mut self) {
        self.all_access = true;
    }
}
