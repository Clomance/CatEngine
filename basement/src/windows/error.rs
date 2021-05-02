use winapi::{
    shared::ntdef::{
        MAKELANGID,
        LANG_NEUTRAL,
        SUBLANG_DEFAULT,
    },
    um::{
        errhandlingapi::GetLastError,
        winbase::{
            FORMAT_MESSAGE_FROM_SYSTEM,
            FORMAT_MESSAGE_IGNORE_INSERTS,
            FORMAT_MESSAGE_ALLOCATE_BUFFER,

            FormatMessageW,
            FormatMessageA,
        },
    }
};

use std::{
    ffi::CString,
    ptr::null_mut
};

#[derive(Debug)]
pub struct WinError{
    code:u32,
}

impl WinError{
    pub (crate) fn get_last_error()->WinError{
        let code=unsafe{GetLastError()};
        WinError{
            code,
        }
    }

    pub fn to_string(&self)->String{
        unsafe{
            let mut buffer=null_mut();

            let size=FormatMessageA(
                FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS | FORMAT_MESSAGE_ALLOCATE_BUFFER,
                null_mut(),
                self.code,
                MAKELANGID(LANG_NEUTRAL,SUBLANG_DEFAULT) as u32,
                buffer,
                0,
                null_mut()
            );

            let vec=Vec::from_raw_parts(buffer as *mut u8,size as usize,size as usize+1); // (+1 - the terminating null)

            String::from_utf8(vec).unwrap()
        }
    }
}