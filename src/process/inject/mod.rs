#[cfg(windows)]
mod windows;

#[cfg(windows)]
use windows as os;

use std::path::Path;

use anyhow::Context;

use super::MemoryAccess;
use crate::Process;

pub unsafe fn inject_shared_library(
    process: &mut Process,
    lib_path: &Path,
) -> Result<(), anyhow::Error> {
    let lib_path = os::encode_path_string(lib_path);
    let data = crate::memory::transmute_to_bytes(&lib_path);
    let address = process
        .allocate_memory(data.len(), MemoryAccess::ReadWrite)
        .context("failed to allocate memory in the target process")?;
    process
        .write_memory(address, data)
        .context("failed to write to memory in the target process")?;
    let join_handle = process
        .create_thread(os::get_load_library_fn(), address)
        .context("failed to create thread in the target process")?;
    join_handle
        .join()
        .context("failed to join the remote thread")?;
    process
        .deallocate_memory(address)
        .context("failed to deallocate memory")?;
    Ok(())
}
