use std::os::windows::ffi::OsStrExt;
use std::path::Path;

use winapi::um::libloaderapi::{GetModuleHandleW, GetProcAddress};
use wstr::{wstr, wstr_impl};

pub fn encode_path_string(path: &Path) -> Vec<u16> {
    path.as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

pub unsafe fn get_load_library_fn() -> usize {
    let kernel32 = GetModuleHandleW(wstr!("kernel32").as_ptr());
    assert!(!kernel32.is_null());
    let load_library_w = GetProcAddress(kernel32, "LoadLibraryW\0".as_ptr() as *const _);
    assert!(!load_library_w.is_null());
    load_library_w as usize
}
