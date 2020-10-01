#[cfg(windows)]
mod windows;

#[cfg(windows)]
use windows as os;

use std::path::Path;

use super::MemoryAccess;
use crate::Process;

pub unsafe fn inject_shared_library(process: &mut Process, lib_path: &Path) -> super::Result<()> {
    let lib_path = os::encode_path_string(lib_path);
    let data = crate::memory::transmute_to_bytes(&lib_path);
    let address = process.allocate_memory(data.len(), MemoryAccess::ReadWrite)?;
    process.write_memory(address, data)?;
    process.create_thread(os::get_load_library_fn(), address)?;
    // TODO: free memory
    Ok(())
}
