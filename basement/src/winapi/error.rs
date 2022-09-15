use crate::winapi::backend::core::WindowsCore;

use winapi::{
    shared::ntdef::{
        MAKELANGID,
        LANG_NEUTRAL,
        SUBLANG_DEFAULT,
    },
    um::{
        winbase::{
            FORMAT_MESSAGE_FROM_SYSTEM,
            FORMAT_MESSAGE_IGNORE_INSERTS,
            FORMAT_MESSAGE_ALLOCATE_BUFFER,

            FormatMessageW,
        },
    }
};

use std::ptr::null_mut;

pub struct Error{
    code:u32,
}

impl Error{
    pub fn get_last_error()->Error{
        Error{
            code:WindowsCore::get_last_error(),
        }
    }

    pub fn raw(code:u32)->Error{
        Error{
            code,
        }
    }

    pub fn code(&self)->u32{
        self.code
    }

    pub fn to_string(&self)->String{
        unsafe{
            let mut buffer:*mut u16=null_mut();

            let size=FormatMessageW(
                FORMAT_MESSAGE_FROM_SYSTEM|FORMAT_MESSAGE_IGNORE_INSERTS|FORMAT_MESSAGE_ALLOCATE_BUFFER,
                null_mut(),
                self.code,
                MAKELANGID(LANG_NEUTRAL,SUBLANG_DEFAULT) as u32,
                &mut buffer as *mut*mut u16 as *mut u16,
                0,
                null_mut()
            );

            let vec=Vec::from_raw_parts(buffer as *mut u16,size as usize,size as usize+1); // +1 - the terminating null

            String::from_utf16(&vec).unwrap()
        }
    }
}

impl std::fmt::Debug for Error{
    fn fmt(&self,f:&mut std::fmt::Formatter)->Result<(),std::fmt::Error>{
        let text=self.to_string();
        let args=format!("Error {{ code: {}, description: {} }}",&self.code,&text);
        f.write_str(&args)
    }
}

#[test]
fn test_error_to_string() {
    let error=Error{
        code:6, // ERROR_INVALID_HANDLE
    };

    assert_eq!("The handle is invalid",error.to_string());
}