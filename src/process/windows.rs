use win32_error::Win32Error;
use winapi::shared::minwindef::{DWORD, FALSE};
use winapi::um::winnt::{
    HANDLE, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_EXECUTE, PAGE_EXECUTE_READ,
    PAGE_EXECUTE_READWRITE, PAGE_NOACCESS, PAGE_READONLY, PAGE_READWRITE, PROCESS_CREATE_THREAD,
    PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE,
};
use winapi::um::{
    handleapi::CloseHandle, memoryapi::ReadProcessMemory, memoryapi::VirtualAllocEx,
    memoryapi::VirtualFreeEx, memoryapi::WriteProcessMemory, processthreadsapi::CreateRemoteThread,
    processthreadsapi::OpenProcess,
};

use super::{MemoryAccess, Process};

pub type Pid = DWORD;
pub type Handle = HANDLE;
pub type Error = Win32Error;

pub unsafe fn open_process(
    pid: Pid,
    mem_alloc: bool,
    mem_read: bool,
    mem_write: bool,
    execute: bool,
) -> Result<Process, Error> {
    let mut access = 0;
    if mem_alloc {
        access |= PROCESS_VM_OPERATION;
    }
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

pub unsafe fn allocate_process_memory(
    process: &mut Process,
    size: usize,
    access: MemoryAccess,
) -> Result<usize, Error> {
    let access = match access {
        MemoryAccess::None => PAGE_NOACCESS,
        MemoryAccess::ExecuteOnly => PAGE_EXECUTE,
        MemoryAccess::WriteOnly => PAGE_READWRITE,
        MemoryAccess::WriteExecute => PAGE_EXECUTE_READWRITE,
        MemoryAccess::ReadOnly => PAGE_READONLY,
        MemoryAccess::ReadExecute => PAGE_EXECUTE_READ,
        MemoryAccess::ReadWrite => PAGE_READWRITE,
        MemoryAccess::ReadWriteExecute => PAGE_EXECUTE_READWRITE,
    };
    let result = VirtualAllocEx(
        process.to_raw_handle(),
        std::ptr::null_mut(),
        size,
        MEM_RESERVE | MEM_COMMIT,
        access,
    );
    if !result.is_null() {
        Ok(result as usize)
    } else {
        Err(Win32Error::new())
    }
}

pub unsafe fn deallocate_process_memory(
    process: &mut Process,
    address: usize,
) -> Result<(), Error> {
    let result = VirtualFreeEx(process.to_raw_handle(), address as *mut _, 0, MEM_RELEASE);
    if result != 0 {
        Ok(())
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
