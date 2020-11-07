use std::mem::ManuallyDrop;

use win32_error::Win32Error;
use winapi::um::{
    handleapi::CloseHandle,
    memoryapi::{ReadProcessMemory, VirtualAllocEx, VirtualFreeEx, WriteProcessMemory},
    processthreadsapi::CreateRemoteThread,
    synchapi::WaitForSingleObject,
    winbase::*,
    winnt::*,
};

use crate::os::windows::{MemAccessExt, ProcessExt};
use crate::process::MemAccess;
use crate::{Error, Result};

pub struct Process {
    handle: HANDLE,
}

impl ProcessExt for Process {
    fn to_raw_handle(&self) -> HANDLE {
        self.handle
    }

    fn into_raw_handle(self) -> HANDLE {
        let guard = ManuallyDrop::new(self);
        guard.handle
    }

    unsafe fn from_raw_handle(handle: HANDLE) -> Self {
        Process { handle }
    }
}

impl Process {
    pub unsafe fn alloc_memory(&mut self, size: usize, access: MemAccess) -> Result<usize> {
        // TODO: memory protection control
        let ptr = VirtualAllocEx(
            self.handle,
            std::ptr::null_mut(),
            size,
            MEM_RESERVE | MEM_COMMIT,
            access.into_page_protect_loose(),
        );
        if !ptr.is_null() {
            Ok(ptr as usize)
        } else {
            Err(Error::Win32Error(Win32Error::new()))
        }
    }

    pub unsafe fn dealloc_memory(&mut self, addr: usize) -> Result<()> {
        let result = VirtualFreeEx(self.handle, addr as *mut _, 0, MEM_RELEASE);
        if result != 0 {
            Ok(())
        } else {
            Err(Error::Win32Error(Win32Error::new()))
        }
    }

    pub unsafe fn read_memory_raw<T>(
        &mut self,
        addr: usize,
        data: *mut T,
        num_bytes: usize,
    ) -> Result<()> {
        let mut bytes_read = 0;
        let result = ReadProcessMemory(
            self.handle,
            addr as *const _,
            data as *mut _,
            num_bytes,
            &mut bytes_read,
        );
        if result == 0 {
            return Err(Error::Win32Error(Win32Error::new()));
        }
        assert_eq!(bytes_read, num_bytes);
        Ok(())
    }

    pub unsafe fn write_memory_raw<T>(
        &mut self,
        addr: usize,
        data: *const T,
        num_bytes: usize,
    ) -> Result<()> {
        let mut bytes_written = 0;
        let result = WriteProcessMemory(
            self.handle,
            addr as *mut _,
            data as *const _,
            num_bytes,
            &mut bytes_written,
        );
        if result == 0 {
            return Err(Error::Win32Error(Win32Error::new()));
        }
        assert_eq!(bytes_written, num_bytes);
        Ok(())
    }

    pub unsafe fn call_function(&mut self, addr: usize, arg: usize) -> Result<()> {
        let handle = CreateRemoteThread(
            self.handle,
            std::ptr::null_mut(),
            0,
            Some(std::mem::transmute(addr)),
            arg as *mut _,
            0,
            std::ptr::null_mut(),
        );
        if handle.is_null() {
            return Err(Error::CreateThreadError(Win32Error::new()));
        }

        let result = WaitForSingleObject(handle, INFINITE);
        if result == WAIT_OBJECT_0 {
            Ok(())
        } else if result == WAIT_FAILED {
            Err(Error::WaitForThreadError(Win32Error::new()))
        } else {
            panic!(format!(
                "unexpected return value from WaitForSingleObject: {:#X}",
                result
            ))
        }
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.handle);
            // the return value is silently ignored
        }
    }
}
