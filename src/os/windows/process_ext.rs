use winapi::um::winnt::HANDLE;

use crate::process::Process;
use crate::sys::process as imp;

pub trait ProcessExt {
    fn to_raw_handle(&self) -> HANDLE;
    fn into_raw_handle(self) -> HANDLE;
    unsafe fn from_raw_handle(handle: HANDLE) -> Self;
}

impl ProcessExt for Process {
    fn to_raw_handle(&self) -> HANDLE {
        self.as_ref().to_raw_handle()
    }

    fn into_raw_handle(self) -> HANDLE {
        imp::Process::from(self).into_raw_handle()
    }

    unsafe fn from_raw_handle(handle: HANDLE) -> Self {
        imp::Process::from_raw_handle(handle).into()
    }
}
