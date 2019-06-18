use crate::backend::{ApiDeviceInfo, ApiResult};

pub trait ApiEnumerator<T: ApiDeviceInfo>
where
    Self: Sized
{
    fn create() -> ApiResult<Self>;
    fn enumerate(&self) -> ApiResult<Vec<T>>;
}