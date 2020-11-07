#[repr(u8)]
pub enum MemAccess {
    None = 0,
    X = 1,
    W = 2,
    WX = 3,
    R = 4,
    RX = 5,
    RW = 6,
    RWX = 7,
}

impl MemAccess {
    pub fn to_rwx_tuple(self) -> (bool, bool, bool) {
        let octet = self as u8;
        (octet & 4 != 0, octet & 2 != 0, octet & 1 != 0)
    }

    pub fn from_rwx_tuple(read: bool, write: bool, execute: bool) -> Self {
        match (read, write, execute) {
            (false, false, false) => Self::None,
            (false, false, true) => Self::X,
            (false, true, false) => Self::W,
            (false, true, true) => Self::WX,
            (true, false, false) => Self::R,
            (true, false, true) => Self::RX,
            (true, true, false) => Self::RW,
            (true, true, true) => Self::RWX,
        }
    }
}

impl Default for MemAccess {
    fn default() -> Self {
        MemAccess::RWX
    }
}
