use winapi::um::{winbase, winnt, fileapi, setupapi};

use WindowsDeviceInfo;

impl UnsafeApiEnumerator<WindowsDeviceInfo, > for WindowsEnumerator{
    fn create() -> ApiResult<Self>{
        unimplemented!()
    }

    unsafe fn enumerate(&self) -> ApiResult<Vec<WindowsDeviceInfo>>{
        let v = Vec::<WindowsDeviceInfo>::new();
        let interface_class_guid = winapi::shared::guiddef::GUID{
            Data1: 0x4d1e55b2, 
            Data2: 0xf16f, 
            Data3: 0x11cf, 
            Data4: [0x88, 0xcb, 0x00, 0x11, 0x11, 0x00, 0x00, 0x30]
        };
        let device_info_data: setupapi::PSP_DEVINFO_DATA;
        let device_info_set = setupapi::SetupDiGetClassDevsA(&interface_class_guid, std::ptr::null(), std::ptr::null_mut(), setupapi::DIGCF_PRESENT | setupapi::DIGCF_DEVICEINTERFACE);
        let mut res: i32;
        let device_index = 0;
        let mut device_interface_data = setupapi::SP_INTERFACE_DEVICE_DATA{
            cbSize: std::mem::size_of::<setupapi::SP_INTERFACE_DEVICE_DATA>() as u32,
            InterfaceClassGuid: interface_class_guid.clone(),
            Flags: 0,
            Reserved: 0
        };
        let mut required_size = 0;
        let mut device_interface_detail_data: setupapi::SP_DEVICE_INTERFACE_DETAIL_DATA_A;
        loop{
            res = setupapi::SetupDiEnumDeviceInterfaces(device_info_set, std::ptr::null_mut(), &interface_class_guid, device_index, &mut device_interface_data);
            if res != 0 {
                break;
            }

            res = setupapi::SetupDiGetDeviceInterfaceDetailA(device_info_set.clone(), &mut device_interface_data, std::ptr::null_mut(), 0, &mut required_size, std::ptr::null_mut());
            device_interface_detail_data = setupapi::SP_DEVICE_INTERFACE_DETAIL_DATA_A{
                cbSize: std::mem::size_of::<setupapi::SP_DEVICE_INTERFACE_DETAIL_DATA_A>() as u32,
                DevicePath: [0i8; winnt::ANYSIZE_ARRAY]
            };

            res = setupapi::SetupDiGetDeviceInterfaceDetailA(device_info_set.clone(), &mut device_interface_data, &mut device_interface_detail_data, required_size.clone(), std::ptr::null_mut(), std::ptr::null_mut());

            if res != 0{
                break;
            }

            let mut i = 0;
            loop{
                let mut driver_name = CString::new("").unwrap();
                let mut attrib = 

                res = setupapi::SetupDiEnumDeviceInfo(device_info_set.clone(), i, device_info_data.clone());
                if res != 0{
                    break;
                }

                res = setupapi::SetupDiGetDeviceRegistryPropertyA(device_info_set.clone(), device_info_data.clone(), setupapi::SPDRP_CLASS, std::ptr::null_mut(), driver_name.as_bytes().as_mut_ptr(), std::mem::size_of::<*mut u8>() as u32, std::ptr::null_mut());
                if res != 0{
                    break;
                }

                if driver_name.to_str().unwrap() == "HIDClass"{
                    res = setupapi::SetupDiGetDeviceRegistryPropertyA(device_info_set.clone(), device_info_data.clone(), setupapi::SPDRP_CLASS, std::ptr::null_mut(), driver_name.as_bytes().as_mut_ptr(), std::mem::size_of::<*mut u8>() as u32, std::ptr::null_mut());

                    if res == 0 {
                        break;
                    } 
                }

                let write_handle = self.open_device(CStr::from_ptr(&device_interface_detail_data.DevicePath[0]).to_str().unwrap(), true);
                if write_handle.unwrap() == handleapi::INVALID_HANDLE_VALUE { 
                    handleapi::CloseHandle(write_handle.unwrap());
                    device_index += 1;
                    continue;
                }
            }
        }
        unimplemented!()
    }

    fn open_device(&self, path: &str, enumerate: bool) -> ApiResult<winapi::um::winnt::HANDLE>{
        let handle = fileapi::CreateFileA(CString::new(path).unwrap().as_c_str().to_bytes().as_mut_ptr() as *const i8, if enumerate { 0 } else { winnt::GENERIC_WRITE | winnt::GENERIC_READ }, winnt::FILE_SHARE_READ | winnt::FILE_SHARE_WRITE, std::ptr::null_mut(), fileapi::OPEN_EXISTING, winbase::FILE_FLAG_OVERLAPPED, std::ptr::null_mut());
        Ok(handle)
    }
}