pub use crate::graphics::{
    GLCore,
    core::program::{
        INVALID_INDEX,
        ProgramParameter
    },
    level1::{
        VertexShader,
        FragmentShader,
    }
};

use super::{
    Uniform,
    UniformValue,
};

use std::{
    ffi::CString,
    mem::MaybeUninit
};

pub struct Program{
    id:u32,
}

impl Program{
    pub fn new(vertex_shader:&VertexShader,fragment_shader:&FragmentShader)->Result<Program,String>{
        unsafe{
            let id=GLCore.program.create();

            GLCore.program.attach_shader(id,vertex_shader.id());
            GLCore.program.attach_shader(id,fragment_shader.id());

            GLCore.program.link(id);

            let mut result:i32=MaybeUninit::uninit().assume_init();
            GLCore.program.get_parameter(id,ProgramParameter::LinkStatus,&mut result);

            if result==0{
                let mut len:i32=MaybeUninit::uninit().assume_init();
                GLCore.program.get_parameter(id,ProgramParameter::InfoLogLength,&mut len);
                let mut log=String::with_capacity(len as usize);

                GLCore.program.get_info_log(id,&mut log);

                log.as_mut_vec().set_len(len as usize);

                return Err(log);
            }

            Ok(Self{
                id,
            })
        }
    }

    pub fn id(&self)->u32{
        self.id
    }

    pub fn bind(&self){
        unsafe{
            GLCore.program.bind(self.id);
        }
    }

    pub fn get_uniform_location(&self,name:&str)->Option<i32>{
        unsafe{
            if let Ok(name)=CString::new(name){
                let id=GLCore.program.get_uniform_location(self.id,name.as_c_str().to_str().unwrap());

                if id==-1{
                    None
                }
                else{
                    Some(id)
                }
            }
            else{
                None
            }
        }
    }

    pub unsafe fn set_uniform_value_raw<V:UniformValue>(&self,uniform_id:i32,value:V){
        Uniform::raw(uniform_id).set(value)
    }

    pub fn set_uniform_value<V:UniformValue>(&self,name:&str,value:V)->bool{
        if let Some(uniform)=Uniform::new(self,name){
            uniform.set(value);
            true
        }
        else{
            false
        }
    }

    pub fn get_uniform_block_index(&self,name:&str)->Option<u32>{
        unsafe{
            if let Ok(name)=CString::new(name){
                let id=GLCore.program.get_uniform_block_index(self.id,name.as_c_str().to_str().unwrap());

                if id==INVALID_INDEX{
                    None
                }
                else{
                    Some(id)
                }
            }
            else{
                None
            }
        }
    }

    // pub unsafe fn bind_uniform_block_raw(&self,uniform_block_index:u32,binding_index:u32){
    //     UniformBlockBinding(self.id,uniform_block_index,binding_index);
    // }

    pub fn bind_uniform_block(&self,name:&str,binding_index:u32)->bool{
        if let Some(index)=self.get_uniform_block_index(name){
            unsafe{
                GLCore.program.set_uniform_block_binding(self.id,index,binding_index)
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
            GLCore.program.delete(self.id);
        }
    }
}