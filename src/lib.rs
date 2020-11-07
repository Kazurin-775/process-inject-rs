pub mod inject;
pub mod memory;
pub mod os;
pub mod process;
mod sys;

pub type Error = sys::Error;
pub type Result<T> = std::result::Result<T, Error>;
