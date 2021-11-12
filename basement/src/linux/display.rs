use super::{
    XCore,
    core::display::DisplayHandle,
};

use std::ptr::null;

pub struct Display{
    handle:DisplayHandle
}

impl Display{
    pub fn open(name:Option<&str>)->Option<Display>{
        let name=if let Some(name)=name{
            name.as_ptr() as *const i8
        }
        else{
            null()
        };
        if let Some(handle)=unsafe{XCore.display.open(name)}{
            Some(
                Self{
                    handle,
                }
            )
        }
        else{
            None
        }
    }

    pub fn handle(&self)->DisplayHandle{
        self.handle
    }
}

impl Drop for Display{
    fn drop(&mut self){
        unsafe{
            XCore.display.close(self.handle);
        }
    }
}