// **************************************************************************
// Copyright (c) 2018 Roland Ruckerbauer All Rights Reserved.
//
// This file is part of hidapi-rs, based on hidapi-rs by Osspial
// **************************************************************************

use cfg_if::cfg_if;

mod ffi;
use ffi::{ApiDevice, ApiEnumerator};

cfg_if! {
    if #[cfg(unix)]{
        cfg_if!{
            if #[cfg(feature = "linux-rust-hidraw")] {
                mod linux_hidraw;

                pub use self::linux_hidraw::HidrawBackend;
            } else if #[cfg(any(
                feature = "linux-static-hidraw",
                feature = "linux-static-libusb",
                feature = "linux-shared-hidraw",
                feature = "linux-shared-libusb"
            ))] {
                mod hidapi;

                pub use self::hidapi::HidapiBackend;
                pub use self::hidapi::libc;
            }else {
                compile_error!("No backend selected!");
            }
        }
    } else if #[cfg(windows)]{
        extern crate winapi;

        mod windows;

        pub use winapi::um::setupapi;
        pub use self::windows::WindowsBackend as Backend;
    } else {
        compile_error!("No backend found!");
    }
}

use crate::error::{ErrorEnum, ApiResult};
use std::io::{Read, Write};

pub trait ApiBackend
where
    Self: Sized,
    Self::Device: ApiDevice + Read + Write,
    Self::DeviceInfo: ApiDeviceInfo,
    Self::DeviceInfoIter: ApiEnumerator<Self::DeviceInfo>,
{
    type Device;
    type DeviceInfo;
    type DeviceInfoIter;

    fn create<T>() -> ApiResult<T> where T: ApiBackend;
    fn open_device(&self, vid: u16, pid: u16) -> ApiResult<Self::Device>;
    fn open_device_with_serial(&self, vid: u16, pid: u16, serial: &str) -> ApiResult<Self::Device>;
    fn enumerate(&mut self) -> ApiResult<Self::DeviceInfoIter>;
}

/// A common trait to be shared between the user layer and the ffi layer.
/// ffi layer will construct a new ApiDeviceInfo and pass it to the mid layer
/// to be shared between the two. Reason being that if something changes with
/// the device, the ffi layer will have direct access to it, and will notify the
/// user layer of any changes such as a disconnected/unpaired event.
pub trait ApiDeviceInfo {
    fn path(&self) -> Option<String>;
    fn vendor_id(&self) -> u16;
    fn product_id(&self) -> u16;
    fn serial_number(&self) -> Option<String>;
    fn release_number(&self) -> u16;
    fn manufacturer_string(&self) -> Option<String>;
    fn product_string(&self) -> Option<String>;
    fn usage_page(&self) -> Option<u16>;
    fn usage(&self) -> u16;
    fn interface_number(&self) -> i32;
}

/// TODO: Create an API Initializer. Like the following:
/// 
/// ```rust
/// struct HidApi{
///     pub fn new() -> Option<HidApi>{
///         let _backend: ApiBackend;
///         cfg_if!{
///             if #[cfg(windows)]{
///                 backend = create_backend<WindowsBackend>().unwrap();
///                 // ..Whatever else needs to be intialized
///             }
///         }
///         // ..Whatever else needs to be intialized
///         Ok(Self{
///             _backend,
///             // ..Whatever else needs to be initialzied
///         })
///     }
/// }
/// ```
fn create_backend<T>() -> ApiResult<T> where T: ApiBackend {
    return T::create();
}
