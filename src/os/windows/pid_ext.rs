use winapi::shared::minwindef::DWORD;

use crate::process::Pid;
use crate::sys::process as imp;

pub trait PidExt {
    fn into_os_pid(self) -> DWORD;
    fn from_os_pid(pid: DWORD) -> Self;
}

impl PidExt for Pid {
    fn into_os_pid(self) -> DWORD {
        imp::Pid::from(self).into()
    }

    fn from_os_pid(pid: DWORD) -> Self {
        Pid::from(imp::Pid::from(pid))
    }
}
