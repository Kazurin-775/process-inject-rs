use std::{convert::TryFrom, ffi::OsString, os::windows::prelude::OsStringExt};
use win32_error::Win32Error;
use winapi::shared::minwindef::FALSE;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW,
    PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};
use winapi::um::winbase::lstrlenW;
use winapi::um::winnt::HANDLE;

use crate::{Error, Result};

use super::Pid;

pub struct ProcessEnumerator {
    toolhelp32: HANDLE,
    is_first: bool,
}

pub struct ProcessRecord {
    inner: PROCESSENTRY32W,
}

impl ProcessEnumerator {
    pub fn new() -> Result<ProcessEnumerator> {
        let toolhelp32 = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
        if toolhelp32 != INVALID_HANDLE_VALUE {
            Ok(ProcessEnumerator {
                toolhelp32,
                is_first: true,
            })
        } else {
            Err(Error::Toolhelp32Error(Win32Error::new()))
        }
    }
}

impl Iterator for ProcessEnumerator {
    type Item = Result<ProcessRecord>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut entry: PROCESSENTRY32W = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;
        let result = if self.is_first {
            unsafe { Process32FirstW(self.toolhelp32, &mut entry) }
        } else {
            unsafe { Process32NextW(self.toolhelp32, &mut entry) }
        };
        self.is_first = false;
        if result != FALSE {
            Some(Ok(ProcessRecord { inner: entry }))
        } else {
            None
        }
    }
}

impl ProcessRecord {
    pub fn pid(&self) -> Pid {
        Pid(self.inner.th32ProcessID)
    }

    pub fn executable(&self) -> String {
        let wstr_ptr = self.inner.szExeFile.as_ptr();
        let num_chars = unsafe { lstrlenW(wstr_ptr) };
        let num_chars = usize::try_from(num_chars).expect("invalid number returned by lstrlenW");
        OsString::from_wide(unsafe { std::slice::from_raw_parts(wstr_ptr, num_chars) })
            .into_string()
            .expect("the executable name contains invalid Unicode characters")
    }
}
