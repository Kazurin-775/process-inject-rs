pub mod inject;
pub mod memory;
pub mod process;
pub mod sys;

pub type Error = sys::Error;
pub type Result<T> = std::result::Result<T, Error>;
