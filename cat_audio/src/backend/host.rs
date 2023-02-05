use super::device::{
    Device,
    DeviceInterface
};

use std::ptr::null_mut;

use winapi::{
    Interface,
    ctypes::c_void,
    um::{
        mmdeviceapi::{
            eAll,
            eRender,
            eCapture,
            eConsole,
            eMultimedia,
            eCommunications,
            IMMDeviceEnumerator,
            CLSID_MMDeviceEnumerator
        },
        combaseapi::{
            CLSCTX_ALL,
            COINITBASE_MULTITHREADED,

            CoInitializeEx,
            CoUninitialize,
            CoCreateInstance
        }
    }
};



#[repr(u32)]
pub enum DataFlow{
    Render=eRender,
    Capture=eCapture,
    All=eAll,
}



#[repr(u32)]
pub enum DeviceRole{
    Console=eConsole,
    Multimedia=eMultimedia,
    Communications=eCommunications
}



pub struct DeviceEnumeratorInterface{
    handle:*mut IMMDeviceEnumerator
}

impl DeviceEnumeratorInterface{
    pub fn null()->DeviceEnumeratorInterface{
        Self{
            handle:null_mut()
        }
    }

    pub fn create(device_enumerator_interface:&mut DeviceEnumeratorInterface)->i32{
        unsafe{
            CoCreateInstance(
                &CLSID_MMDeviceEnumerator,
                null_mut(),
                CLSCTX_ALL,
                &IMMDeviceEnumerator::uuidof(),
                &mut device_enumerator_interface.handle as *mut *mut IMMDeviceEnumerator as *mut *mut c_void
            )
        }
    }

    pub fn get_default_audio_endpoint(&self,data_flow:DataFlow,device_role:DeviceRole,device_interface:&mut DeviceInterface)->i32{
        unsafe{
            (&mut *self.handle).GetDefaultAudioEndpoint(data_flow as u32,device_role as u32,&mut device_interface.handle)
        }
    }
}

impl Drop for DeviceEnumeratorInterface{
    fn drop(&mut self){
        unsafe{
            (&mut *self.handle).Release();
        }
    }
}



pub struct DeviceEnumerator{
    interface:DeviceEnumeratorInterface
}

impl DeviceEnumerator{
    pub fn new()->Result<DeviceEnumerator,()>{
        let mut device_enumerator_interface=DeviceEnumeratorInterface::null();
        let result=DeviceEnumeratorInterface::create(&mut device_enumerator_interface);
        if result!=0{
            return Err(())
        }

        Ok(
            DeviceEnumerator{
                interface:device_enumerator_interface
            }
        )
    }

    pub fn default_render_device(&self,role:DeviceRole)->Result<Device,()>{
        let mut interface=DeviceInterface::null();
        let result=self.interface.get_default_audio_endpoint(DataFlow::Render,role,&mut interface);

        if result!=0{
            return Err(())
        }

        Ok(
            Device{
                interface
            }
        )
    }
}



pub struct Host{}

impl Host{
    pub fn initialize()->Result<Host,()>{
        unsafe{
            let result=CoInitializeEx(null_mut(),COINITBASE_MULTITHREADED);

            if result!=0{
                return Err(())
            }
        }

        Ok(Host{})
    }
}

impl Drop for Host{
    fn drop(&mut self){
        unsafe{
            CoUninitialize()
        }
    }
}