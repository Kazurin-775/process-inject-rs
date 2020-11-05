use super::ProcessRecord;

pub struct ProcessEnumerator {
}

impl ProcessEnumerator {
    pub fn new() -> crate::Result<ProcessEnumerator> {
        unimplemented!()
    }
}

impl Iterator for ProcessEnumerator {
    type Item = crate::Result<ProcessRecord>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}
