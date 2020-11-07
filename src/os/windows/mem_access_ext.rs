use winapi::shared::minwindef::DWORD;
use winapi::um::winnt::*;

use crate::process::MemAccess;

pub trait MemAccessExt {
    fn into_page_protect_loose(self) -> DWORD;
}

impl MemAccessExt for MemAccess {
    fn into_page_protect_loose(self) -> DWORD {
        match self {
            MemAccess::None => PAGE_NOACCESS,
            MemAccess::X => PAGE_EXECUTE,
            MemAccess::W => PAGE_READWRITE,
            MemAccess::WX => PAGE_EXECUTE_READWRITE,
            MemAccess::R => PAGE_READONLY,
            MemAccess::RX => PAGE_EXECUTE_READ,
            MemAccess::RW => PAGE_EXECUTE_READWRITE,
            MemAccess::RWX => PAGE_EXECUTE_READWRITE,
        }
    }
}
