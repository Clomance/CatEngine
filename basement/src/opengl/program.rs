use crate::opengl::{
    core::{
        Core,
        constants::INVALID_INDEX,
        program::{
            Program as ProgramFunctions,
        },
    },
    shader::{
        Shader,
    }
};

pub use crate::opengl::{
    core::{
        program::{
            ProgramParameter
        },
    },
};


use core::{
    mem::MaybeUninit,
    ptr::null_mut
};

pub struct Program{
    id:u32,
}

impl Program{
    pub fn new()->Program{
        unsafe{
            let id=ProgramFunctions::create();
            Self{
                id,
            }
        }
    }

    pub fn attach_shader(&self,shader:&Shader){
        unsafe{
            ProgramFunctions::attach_shader(self.id,shader.id());
        }
    }

    pub fn link(&self)->Result<(),String>{
        unsafe{
            ProgramFunctions::link(self.id);

            let mut result=MaybeUninit::uninit();
            ProgramFunctions::get_parameter(self.id,ProgramParameter::LinkStatus,result.as_mut_ptr());

            if result.assume_init()==0{
                let mut len=MaybeUninit::uninit();
                ProgramFunctions::get_parameter(self.id,ProgramParameter::InfoLogLength,len.as_mut_ptr());
                let len=len.assume_init() as usize;

                let mut log=String::with_capacity(len);

                ProgramFunctions::get_info_log(self.id,log.len() as i32,null_mut(),log.as_mut_ptr() as *mut _);

                log.as_mut_vec().set_len(len);

                return Err(log);
            }

            Ok(())
        }
    }

    pub fn id(&self)->u32{
        self.id
    }

    pub fn bind(&self){
        unsafe{
            ProgramFunctions::bind(self.id);
        }
    }

    pub fn get_uniform_location(&self,name:&str)->Option<i32>{
        unsafe{
            let id=ProgramFunctions::get_uniform_location(self.id,name.as_ptr() as *const i8);

            if id==-1{
                None
            }
            else{
                Some(id)
            }
        }
    }

    pub fn get_uniform_block_index(&self,name:&str)->Option<u32>{
        unsafe{
            let id=ProgramFunctions::get_uniform_block_index(self.id,name.as_ptr() as *const i8);

            if id==INVALID_INDEX{
                None
            }
            else{
                Some(id)
            }
        }
    }

    pub fn bind_uniform_block(&self,name:&str,binding_index:u32)->bool{
        if let Some(index)=self.get_uniform_block_index(name){
            unsafe{
                ProgramFunctions::set_uniform_block_binding(self.id,index,binding_index)
            }
            true
        }
        else{
            false
        }
    }
}

impl Drop for Program{
    fn drop(&mut self){
        unsafe{
            ProgramFunctions::delete(self.id);
        }
    }
}