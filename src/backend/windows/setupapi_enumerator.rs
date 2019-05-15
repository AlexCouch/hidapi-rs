use winapi::setupapi;

pub struct Enumerator<'a>{
    current_device: &'a DeviceInfo,
}

impl<'a> Enumerator<'a>{
    pub fn create() -> HidResult<Self>{

    }
}