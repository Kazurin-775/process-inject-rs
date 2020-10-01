#[cfg(windows)]
mod windows;
#[cfg(windows)]
use windows as os;

pub use os::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub struct ProcessEnumerator {
    inner: os::Enumerator,
}

pub struct ProcessRecord {
    pub pid: super::Pid,
    pub command: String,
}

impl ProcessEnumerator {
    pub fn create() -> Result<ProcessEnumerator> {
        Ok(ProcessEnumerator {
            inner: unsafe { os::enumerator_new()? },
        })
    }
}

impl Iterator for ProcessEnumerator {
    type Item = ProcessRecord;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe { os::enumerator_next(&mut self.inner) }
    }
}

#[cfg(windows)]
#[test]
fn test_enumeration() {
    let e = ProcessEnumerator::create().unwrap();
    for record in e {
        if record.command.contains("cargo.exe") {
            // Test pass
            return;
        }
    }
    panic!("expected to find a process named cargo.exe");
}
