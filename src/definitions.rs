use std::ffi::c_void;

use windows::{
    core::PCSTR,
    Win32::System::Ioctl::{FILE_DEVICE_UNKNOWN, FILE_SPECIAL_ACCESS, METHOD_BUFFERED},
};

const fn ctl_code(device_type: u32, function: u32, method: u32, access: u32) -> u32 {
    (device_type << 16) | (access << 14) | (function << 2) | (method)
}

pub const IO_READ: u32 = ctl_code(
    FILE_DEVICE_UNKNOWN,
    0x1,
    METHOD_BUFFERED,
    FILE_SPECIAL_ACCESS,
);

pub const IO_WRITE: u32 = ctl_code(
    FILE_DEVICE_UNKNOWN,
    0x2,
    METHOD_BUFFERED,
    FILE_SPECIAL_ACCESS,
);

pub const DEV_NAME: PCSTR = PCSTR::from_raw(c"\\\\.\\IOkmm".as_ptr() as _);

#[repr(C)]
pub struct IoMessage {
    pub pid: usize,
    pub size: usize,
    pub buffer: *mut c_void,
    pub address: *mut c_void,
}
