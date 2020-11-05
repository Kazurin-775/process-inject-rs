pub mod inject;
pub mod process;

pub enum Error {
}

pub type Result<T> = std::result::Result<T, Error>;
