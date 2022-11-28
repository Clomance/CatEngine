use crate::opengl::core::{
    Core,
    Error,
    shader::{
        Shader as ShaderFunctions,
    },
};

pub use crate::opengl::core::{
    shader::{
        ShaderParameter,
        ShaderType,
    },
};

use core::{
    mem::MaybeUninit,
    ptr::null_mut,
};

pub struct Shader{
    id:u32,
}

impl Shader{
    pub fn empty(shader_type:ShaderType)->Shader{
        unsafe{
            Self{
                id:ShaderFunctions::create(shader_type),
            }
        }
    }

    pub unsafe fn new_unchecked(source:&str,shader_type:ShaderType)->Shader{
        let id=ShaderFunctions::create(shader_type);
        
        ShaderFunctions::set_source(id,1,&(source.as_ptr() as *const i8) as *const _,&(source.len() as i32));

        ShaderFunctions::compile(id);

        Self{
            id,
        }
    }

    pub fn new(source:&str,shader_type:ShaderType)->Result<Shader,String>{
        unsafe{
            let shader=Shader::new_unchecked(source,shader_type);

            let mut result=MaybeUninit::uninit();
            ShaderFunctions::get(shader.id,ShaderParameter::CompileStatus,result.as_mut_ptr());

            if result.assume_init()==0{
                let mut len=MaybeUninit::uninit();
                ShaderFunctions::get(shader.id,ShaderParameter::InfoLogLength,len.as_mut_ptr());
                let len=len.assume_init() as usize;

                let mut log=String::with_capacity(len);

                ShaderFunctions::get_info_log(shader.id,log.len() as i32,null_mut(),log.as_mut_ptr() as *mut _);

                return Err(log);
            }

            Ok(shader)
        }
    }

    #[inline(always)]
    pub fn id(&self)->u32{
        self.id
    }
}

impl Shader{
    pub fn get_parameter(&self,parameter:ShaderParameter,values:*mut i32)->Error{
        unsafe{
            ShaderFunctions::get(self.id,parameter,values);
            Core::get_error()
        }
    }

    pub fn get_info_log(&self,log:&mut String)->Error{
        unsafe{
            let mut len=MaybeUninit::uninit();
            ShaderFunctions::get(self.id,ShaderParameter::InfoLogLength,len.as_mut_ptr());
            let len=len.assume_init() as usize;

            log.clear();
            if log.capacity()<len{
                log.reserve(len-log.capacity())
            }

            ShaderFunctions::get_info_log(self.id,log.len() as i32,null_mut(),log.as_mut_ptr() as *mut _);
            Core::get_error()
        }
    }
}

impl Drop for Shader{
    fn drop(&mut self){
        unsafe{
            ShaderFunctions::delete(self.id);
        }
    }
}