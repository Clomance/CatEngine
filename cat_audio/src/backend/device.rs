use super::{
    client::{
        AudioClient,
        AudioClientInterface,
    }
};

use std::ptr::null_mut;

use winapi::{
    ctypes::c_void,
    um::{
        combaseapi::CLSCTX_ALL,
        mmdeviceapi::IMMDevice,
        audioclient::{
            IID_IAudioClient,
            IAudioClient
        }
    }
};



pub struct DeviceInterface{
    pub handle:*mut IMMDevice
}

impl DeviceInterface{
    pub const  fn null()->DeviceInterface{
        Self{
            handle:null_mut()
        }
    }

    #[inline]
    pub fn activate(&self,client_interface:&mut AudioClientInterface)->i32{
        unsafe{
            (&*self.handle).Activate(&IID_IAudioClient,CLSCTX_ALL,null_mut(),&mut client_interface.handle as *mut *mut IAudioClient as *mut *mut c_void)
        }
    }

    #[inline]
    pub fn release(&self)->u32{
        unsafe{
            (&*self.handle).Release()
        }
    }
}



pub struct Device{
    pub interface:DeviceInterface
}

impl Device{
    pub fn activate(&self)->Result<AudioClient,()>{
        let mut client_interface=AudioClientInterface::null();
        let result=self.interface.activate(&mut client_interface);

        if result!=0{
            return Err(())
        }

        Ok(
            AudioClient{
                interface:client_interface
            }
        )
    }
}

impl Drop for Device{
    fn drop(&mut self){
        self.interface.release();
    }
}
