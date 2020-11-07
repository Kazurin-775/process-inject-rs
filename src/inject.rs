use std::path::Path;

use crate::process::Process;
use crate::Result;

pub unsafe fn inject_shared_library<P>(process: &mut Process, path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    crate::sys::inject::inject_shared_library(process.as_mut(), path)
}
