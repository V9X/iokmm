#![cfg(windows)]

use std::{
    ffi::c_void,
    mem::size_of,
    ptr::{addr_of, addr_of_mut},
};

use definitions::{IoMessage, DEV_NAME, IO_READ, IO_WRITE};
use windows::{
    core::Error,
    Win32::{
        Foundation::{CloseHandle, GENERIC_READ, GENERIC_WRITE, HANDLE},
        Storage::FileSystem::{
            CreateFileA, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
        },
        System::IO::DeviceIoControl,
    },
};
mod definitions;

pub struct Devio {
    driver_handle: HANDLE,
}

impl Devio {
    /// Connects to kernel driver
    pub fn new() -> Result<Self, Error> {
        let driver_handle = unsafe {
            CreateFileA(
                DEV_NAME,
                GENERIC_READ.0 | GENERIC_WRITE.0,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                None,
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                None,
            )?
        };

        Ok(Self { driver_handle })
    }

    /// Reads data from memory
    /// - pid - process ID
    /// - address - Address from which to read
    /// - size - length of the generated vector
    pub fn read<T>(&self, pid: usize, address: *mut T, size: usize) -> Result<Vec<T>, Error> {
        pub fn inner(
            handle: HANDLE,
            pid: usize,
            address: *mut c_void,
            buffer: *mut c_void,
            size: usize,
            byte_size_t: usize,
        ) -> Result<usize, Error> {
            let bytes_size = size * byte_size_t;
            let mut bytes_copied: usize = 0;

            let msg = IoMessage {
                pid,
                size: bytes_size,
                buffer,
                address,
            };

            unsafe {
                DeviceIoControl(
                    handle,
                    IO_READ,
                    Some(addr_of!(msg) as _),
                    size_of::<IoMessage>() as _,
                    Some(addr_of_mut!(bytes_copied) as _),
                    size_of::<usize>() as _,
                    None,
                    None,
                )?;
            }

            // Not sure if this is even possible but it's better to check than to continue with buffer overflow
            assert!(bytes_copied <= bytes_size);

            Ok(bytes_copied / byte_size_t)
        }

        let mut buffer: Vec<T> = Vec::with_capacity(size);

        let len = inner(
            self.driver_handle,
            pid,
            address as _,
            buffer.as_mut_ptr() as _,
            size,
            size_of::<T>(),
        )?;

        unsafe { buffer.set_len(len) };

        Ok(buffer)
    }

    /// Writes data to memory
    /// - pid - process ID
    /// - address - Address from which to overwrite
    /// - bujffer - Buffer with which to overwrite
    pub fn write<T>(&self, pid: usize, address: *mut c_void, buffer: &[T]) -> Result<usize, Error> {
        fn inner(
            handle: HANDLE,
            pid: usize,
            address: *mut c_void,
            buffer: *mut c_void,
            size: usize,
            byte_size_t: usize,
        ) -> Result<usize, Error> {
            let bytes_size = size * byte_size_t;
            let mut bytes_copied: usize = 0;

            let msg = IoMessage {
                pid,
                size: bytes_size,
                buffer,
                address,
            };

            unsafe {
                DeviceIoControl(
                    handle,
                    IO_WRITE,
                    Some(addr_of!(msg) as _),
                    size_of::<IoMessage>() as _,
                    Some(addr_of_mut!(bytes_copied) as _),
                    size_of::<usize>() as _,
                    None,
                    None,
                )?;
            }

            Ok(bytes_copied)
        }

        inner(
            self.driver_handle,
            pid,
            address as _,
            buffer.as_ptr() as _,
            buffer.len(),
            size_of::<T>(),
        )
    }
}

impl Drop for Devio {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.driver_handle).ok() };
    }
}
