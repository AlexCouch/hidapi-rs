use crate::backend::*; 

/// The unsafe implementation of the enumerator. 
/// The enumerator must convert the data from unsafe data to usable, rust-safe, 
/// higher-level data.
/// 
/// H is a handle type. For Windows it will be winnt::HANDLE which translates to c_void which is *void.
pub trait UnsafeApiEnumerator<T: ApiDeviceInfo, H>
where
    Self: Sized
{
    unsafe fn create() -> ApiResult<Self>;
    unsafe fn enumerate(&self) -> ApiResult<Vec<T>>;
    unsafe fn open_device(&self, path: &str, enumerate: bool) -> ApiResult<H>;
}

/// This trait must be implemented in the unsafe/ffi layer of the api. We don't want
/// to expose this to the user layer.
pub trait ApiDevice: Write + Read {
    fn write_report_id(&mut self, report_id: u8, data: &[u8]) -> std::io::Result<usize> {
        let mut buf = Vec::with_capacity(data.len() + 1);
        buf.push(report_id);
        buf.extend_from_slice(data);

        self.write(buf.as_slice())
    }
}