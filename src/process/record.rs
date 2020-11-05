use super::Pid;
use crate::sys::process as imp;

pub struct ProcessRecord {
    inner: imp::ProcessRecord,
}

impl From<imp::ProcessRecord> for ProcessRecord {
    fn from(inner: imp::ProcessRecord) -> Self {
        ProcessRecord { inner }
    }
}

impl From<ProcessRecord> for imp::ProcessRecord {
    fn from(outer: ProcessRecord) -> Self {
        outer.inner
    }
}

impl ProcessRecord {
    pub fn pid(&self) -> Pid {
        self.inner.pid().into()
    }

    pub fn executable(&self) -> String {
        self.inner.executable()
    }
}
