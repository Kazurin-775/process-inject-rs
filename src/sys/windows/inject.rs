use std::{os::windows::prelude::OsStrExt, path::Path};

use winapi::um::libloaderapi::{GetModuleHandleW, GetProcAddress};

use super::process::Process;
use crate::Result;

unsafe fn get_load_library() -> usize {
    // fix a bug in `wstr` crate
    use wstr::wstr_impl;

    let kernel32 = GetModuleHandleW(wstr::wstr!("kernel32.dll\0").as_ptr());
    assert!(!kernel32.is_null());

    let load_library = GetProcAddress(kernel32, b"LoadLibraryW\0".as_ptr() as *mut i8);
    assert!(!load_library.is_null());

    load_library as usize
}

pub unsafe fn inject_shared_library<P>(process: &mut Process, path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref().as_os_str();
    let path_encoded: Vec<_> = path.encode_wide().chain(std::iter::once(0)).collect();

    let addr = process.alloc_memory(path_encoded.len() * std::mem::size_of::<u16>())?;
    // ensure memory deallocation with a closure
    let result: Result<()> = (|| {
        process.write_memory(addr, crate::memory::transmute_to_bytes(&path_encoded))?;
        process.call_function(get_load_library(), addr)?;
        Ok(())
    })();

    result.and(process.dealloc_memory(addr))
}
