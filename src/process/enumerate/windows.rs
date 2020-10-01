use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use anyhow::Context;
use win32_error::Win32Error;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};
use winapi::um::winnt::HANDLE;
use winapi::{shared::minwindef::FALSE, um::winbase::lstrlenW};

use super::ProcessRecord;

pub struct Enumerator {
    toolhelp32: HANDLE,
    is_first: bool,
}
pub type Error = anyhow::Error;

pub unsafe fn enumerator_new() -> Result<Enumerator, Error> {
    let toolhelp32 = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
    if toolhelp32 != INVALID_HANDLE_VALUE {
        Ok(Enumerator {
            toolhelp32,
            is_first: true,
        })
    } else {
        Err(Win32Error::new()).context("failed to create Win32 toolhelp snapshot")
    }
}

pub unsafe fn enumerator_next(enumerator: &mut Enumerator) -> Option<ProcessRecord> {
    let mut entry: PROCESSENTRY32W = std::mem::MaybeUninit::uninit().assume_init();
    entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;
    let result = if enumerator.is_first {
        Process32FirstW(enumerator.toolhelp32, &mut entry)
    } else {
        Process32NextW(enumerator.toolhelp32, &mut entry)
    };
    enumerator.is_first = false;
    if result != FALSE {
        let command = OsString::from_wide(std::slice::from_raw_parts(
            entry.szExeFile.as_ptr(),
            lstrlenW(entry.szExeFile.as_ptr()) as usize,
        ))
        .into_string()
        .expect("the executable path of some process contains invalid Unicode characters");
        Some(ProcessRecord {
            pid: entry.th32ProcessID,
            command,
        })
    } else {
        None
    }
}
