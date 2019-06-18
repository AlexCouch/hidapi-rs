use crate::backend::{ApiBackend, ApiResult, ApiDeviceInfo};
use crate::backend::ffi::ApiDevice;
use crate::error;
use std::io::{Write, Read, Result};

mod ffi;
mod mid;

pub struct WindowsBackend;
pub struct WindowsEnumerator;
pub struct WindowsDevice;
pub struct WindowsDeviceInfo{
    path: Option<String>,
    vendor_id: u16,
    product_id: u16,
    serial_number: Option<String>,
    release_number: u16,
    manufacturer_string: Option<String>,
    product_string: Option<String>,
    usage_page: Option<u16>,
    usage: u16,
    interface_number: i32
}

impl ApiBackend for WindowsBackend{
    type Device = WindowsDevice;
    type DeviceInfo = WindowsDeviceInfo;
    type DeviceInfoIter = WindowsEnumerator;

    //@FIXME: Create a better way of writing the create function!
    fn create<T>() -> error::ApiResult<T> {
        unimplemented!()
    }
    fn open_device(&self, vid: u16, pid: u16) -> ApiResult<Self::Device> {
        unimplemented!()
    }
    fn open_device_with_serial(&self, vid: u16, pid: u16, serial: &str) -> ApiResult<Self::Device> {
        unimplemented!()        
    }
    /// TODO: Abstract the device enumerator into the common module. Define a common trait for device enumerators.
    fn enumerate(&mut self) -> ApiResult<Self::DeviceInfoIter> {
        unimplemented!()
    }
}

impl ApiDeviceInfo for WindowsDeviceInfo{
    fn path(&self) -> Option<String>{
        self.path
    }
    fn vendor_id(&self) -> u16{
        self.vendor_id
    }
    fn product_id(&self) -> u16{
        self.product_id
    }
    fn serial_number(&self) -> Option<String>{
        self.serial_number
    }
    fn release_number(&self) -> u16{
        self.release_number
    }
    fn manufacturer_string(&self) -> Option<String>{
        self.manufacturer_string
    }
    fn product_string(&self) -> Option<String>{
        self.product_string
    }
    fn usage_page(&self) -> Option<u16>{
        self.usage_page
    }
    fn usage(&self) -> u16{
        self.usage
    }
    fn interface_number(&self) -> i32{
        self.interface_number
    }
}

impl ApiDevice for WindowsDevice {}