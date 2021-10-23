use crate::windows::{
    WinCore,
};

#[derive(Clone,Copy,Debug)]
pub struct Error{
    code:u32,
}

impl Error{
    pub fn get_last_error()->Error{
        unsafe{
            Self{
                code:WinCore.get_last_error(),
            }
        }
    }

    pub fn is_ok(&self)->bool{
        self.code==0
    }

    pub fn code(&self)->u32{
        self.code
    }
}