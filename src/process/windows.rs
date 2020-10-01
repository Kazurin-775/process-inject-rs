use win32_error::Win32Error;
use winapi::shared::minwindef::{DWORD, FALSE};
use winapi::um::winnt::{HANDLE, PROCESS_CREATE_THREAD, PROCESS_VM_READ, PROCESS_VM_WRITE};
use winapi::um::{
    handleapi::CloseHandle, memoryapi::ReadProcessMemory, memoryapi::WriteProcessMemory,
    processthreadsapi::CreateRemoteThread, processthreadsapi::OpenProcess,
};

use super::Process;

pub type Pid = DWORD;
pub type Handle = HANDLE;
pub type Error = Win32Error;

pub unsafe fn open_process(
    pid: Pid,
    mem_read: bool,
    mem_write: bool,
    execute: bool,
) -> Result<Process, Error> {
    let mut access = 0;
    if mem_read {
        access |= PROCESS_VM_READ;
    }
    if mem_write {
        access |= PROCESS_VM_WRITE;
    }
    if execute {
        access |= PROCESS_CREATE_THREAD;
    }
    let handle = OpenProcess(access, FALSE, pid);
    if !handle.is_null() {
        Ok(Process::from_raw(handle))
    } else {
        Err(Win32Error::new())
    }
}

pub unsafe fn read_process_memory(
    process: &Process,
    address: usize,
    buffer: &mut [u8],
) -> Result<(), Error> {
    let mut bytes_read = 0;
    let result = ReadProcessMemory(
        process.to_raw_handle(),
        address as *const _,
        buffer.as_mut_ptr() as *mut _,
        buffer.len(),
        &mut bytes_read,
    );
    if result == 0 {
        return Err(Win32Error::new());
    }
    assert_eq!(bytes_read, buffer.len());
    Ok(())
}

pub unsafe fn write_process_memory(
    process: &mut Process,
    address: usize,
    buffer: &[u8],
) -> Result<(), Error> {
    let mut bytes_written = 0;
    let result = WriteProcessMemory(
        process.to_raw_handle(),
        address as *mut _,
        buffer.as_ptr() as *const _,
        buffer.len(),
        &mut bytes_written,
    );
    if result == 0 {
        return Err(Win32Error::new());
    }
    assert_eq!(bytes_written, buffer.len());
    Ok(())
}

pub unsafe fn create_thread(
    process: &mut Process,
    function: usize,
    argument: usize,
) -> Result<(), Error> {
    let handle = CreateRemoteThread(
        process.to_raw_handle(),
        std::ptr::null_mut(),
        0,
        Some(std::mem::transmute(function)),
        argument as *mut _,
        0,
        std::ptr::null_mut(),
    );
    if !handle.is_null() {
        CloseHandle(handle);
        Ok(())
    } else {
        Err(Win32Error::new())
    }
}

pub unsafe fn close_process(process: &mut Process) {
    CloseHandle(process.to_raw_handle());
}
