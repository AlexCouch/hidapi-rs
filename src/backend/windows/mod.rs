extern crate winapi;
extern crate backend;

use crate::backend::ApiBackend;
use setupapi;

pub struct WindowsBackend;

impl ApiBackend for WindowsBackend{
    type Device = Device;
    type DeviceInfo = DeviceInfo;
    type DeviceInfoIter = Enumerator;

    fn create() -> HidResult<Self> {

        Ok()
    }
    fn open_device(&self, vid: u16, pid: u16) -> HidResult<Self::Device> {

    }
    fn open_device_with_serial(&self, vid: u16, pid: u16, serial: &str) -> HidResult<Self::Device> {
    }
    fn enumerate(&mut self) -> HidResult<Self::DeviceInfoIter> {

    }
}

pub struct Device;

impl Write for Device {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {

    }
    fn flush(&mut self) -> io::Result<()> {
        unimplemented!()
    }
}

impl Read for Device {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        unimplemented!()
    }
}

impl ApiDevice for Device {}