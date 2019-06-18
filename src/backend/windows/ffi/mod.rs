/// This module is responsible for managing the platform-specific (or os-specific)
/// implementations of the lower-layer of the api. See lib.rs for details on the
/// api's architecture.
/// 
/// The lower level is responsible for handling all the unsafe ffi code. This layer
/// is pretty much a direct rust translation of the C interface by signal11, with
/// some tweaks. This layer is going to have unsafe implementations which will
/// construct the safer representation of the unsafe data. Originally, we were
/// allowing types such as libc::c_void to pass into the user layer of the api.
/// This is not okay in rustland. Rust's main idea is to be safe while working in
/// the systems' dungeons (lower level spaces, c-world I guess?). Another part of
/// rust is better concurrency with green threads. The old hidapi-rs (0.5.x) was
/// not at all thread-safe. Attempting to use any struct between threads would
/// cause the compiler to abort with error messages stating that the types involved
/// are not thread-safe. This also required the user to abstract hidapi-rs even
/// further. An entire abstraction over the api to allow safe threading, which is
/// rather inconvenient, led to this 1.0 rewrite and this particular api architecture.
/// 
/// - Alex Couch, 18 June, 2019

use winapi::um::{setupapi, handleapi, fileapi, winbase, winnt};
use crate::backend::{ApiDeviceInfo, ApiResult};
use crate::backend::windows::{WindowsDeviceInfo, WindowsDevice, WindowsEnumerator};
use std::io::{Write, Read};
use std::ffi::{CStr, CString};

impl Write for WindowsDevice {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        unimplemented!()
    }

    fn flush(&mut self) -> Result<()> {
        unimplemented!()
    }
}

impl Read for WindowsDevice{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        unimplemented!()
    }
}