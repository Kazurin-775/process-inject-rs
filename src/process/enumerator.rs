use super::ProcessRecord;
use crate::sys::process as imp;

pub struct ProcessEnumerator {
    inner: imp::ProcessEnumerator,
}

impl From<imp::ProcessEnumerator> for ProcessEnumerator {
    fn from(inner: imp::ProcessEnumerator) -> Self {
        ProcessEnumerator { inner }
    }
}

impl From<ProcessEnumerator> for imp::ProcessEnumerator {
    fn from(outer: ProcessEnumerator) -> Self {
        outer.inner
    }
}

impl ProcessEnumerator {
    pub fn new() -> crate::Result<ProcessEnumerator> {
        Ok(ProcessEnumerator {
            inner: imp::ProcessEnumerator::new()?,
        })
    }
}

impl Iterator for ProcessEnumerator {
    type Item = crate::Result<ProcessRecord>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|result| result.map(|record| record.into()))
    }
}
