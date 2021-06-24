pub use super::level1::shader::{
    VertexShader,
    FragmentShader,
};

use super::{
    Uniform,
    UniformValue,
};

use std::{
    ffi::CString,
    mem::MaybeUninit
};

use gl::{
    // constants
    LINK_STATUS,
    INVALID_INDEX,
    MAX_UNIFORM_BUFFER_BINDINGS,
    // functions
    CreateProgram,
    AttachShader,
    LinkProgram,
    GetProgramiv,
    GetProgramInfoLog,
    GetUniformLocation,
    GetUniformBlockIndex,
    UniformBlockBinding,
    UseProgram,
    DeleteProgram,
};

pub struct Program{
    id:u32,
}

impl Program{
    pub fn new(vertex_shader:&VertexShader,fragment_shader:&FragmentShader)->Result<Program,String>{
        unsafe{
            let id=CreateProgram();

            AttachShader(id,vertex_shader.id());
            AttachShader(id,fragment_shader.id());

            LinkProgram(id);
            let mut result:i32=MaybeUninit::uninit().assume_init();
            GetProgramiv(id,LINK_STATUS,&mut result as *mut i32);

            if result==0{
                let mut log=String::with_capacity(512);
                let log_ref=log.as_ptr() as *mut i8;
                let mut len=0i32;
                GetProgramInfoLog(id,512,&mut len as *mut i32,log_ref);

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
            UseProgram(self.id);
        }
    }

    pub fn get_uniform_location(&self,name:&str)->Option<i32>{
        unsafe{
            if let Ok(name)=CString::new(name){
                let id=GetUniformLocation(self.id,name.as_ptr());

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
                let id=GetUniformBlockIndex(self.id,name.as_ptr());

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

    pub unsafe fn bind_uniform_block_raw(&self,uniform_block_index:u32,binding_index:u32){
        UniformBlockBinding(self.id,uniform_block_index,binding_index);
    }

    pub fn bind_uniform_block(&self,name:&str,binding_index:u32)->bool{
        if binding_index<MAX_UNIFORM_BUFFER_BINDINGS{
            if let Some(index)=self.get_uniform_block_index(name){
                unsafe{
                    UniformBlockBinding(self.id,index,binding_index);
                }
                true
            }
            else{
                false
            }
        }
        else{
            false
        }
    }
}

impl Drop for Program{
    fn drop(&mut self){
        unsafe{
            DeleteProgram(self.id);
        }
    }
}