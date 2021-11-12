use super::display::{
    DisplayHandle,
};

use std::mem::{
    transmute,
    transmute_copy,
};

use x11::xlib::{
    XStoreName,
};


#[derive(Debug,Clone,Copy)]
#[repr(transparent)]
pub struct WindowID{
    inner:std::num::NonZeroU64,
}
implement_handle_wrapper!(WindowID,u64);

pub struct Window{

}



impl Window{
    // pub fn store_name(display:DisplayHandle,window:WindowID,name:*const i8){
    //     unsafe{
    //         XStoreName(display.as_raw(),window.as_raw(),name)
    //     }
    // }

    // pub fn set_attributes(display:DisplayHandle,window:WindowID,){
    //     unsafe{
    //         XGetWindowAttributes(display.as_raw(),window.as_raw(), &gwa)
    //     }
    // }
}