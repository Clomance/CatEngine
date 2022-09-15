#[cfg(target_os="windows")]
use crate::winapi::OpenGraphicsLibrary;

use super::{
    types::*,
    constants::*
};

use core::mem::transmute;

#[cfg(target_os="linux")]
extern "system"{
    fn glUniform1i(location:GLint,v0:GLint);

    fn glUniform1ui(location:GLint,v0:GLuint);

    fn glUniformMatrix4fv(location:GLint,count:GLsizei,transpose:GLboolean,value:*const GLfloat);
}

#[cfg(target_os="windows")]
mod gl{
    pub static mut glUniform1f:usize=0;
    pub static mut glUniform2f:usize=0;
    pub static mut glUniform3f:usize=0;
    pub static mut glUniform4f:usize=0;

    pub static mut glUniform1i:usize=0;
    pub static mut glUniform2i:usize=0;
    pub static mut glUniform3i:usize=0;
    pub static mut glUniform4i:usize=0;

    pub static mut glUniform1ui:usize=0;
    pub static mut glUniform2ui:usize=0;
    pub static mut glUniform3ui:usize=0;
    pub static mut glUniform4ui:usize=0;

    pub static mut glUniform1fv:usize=0;
    pub static mut glUniform2fv:usize=0;
    pub static mut glUniform3fv:usize=0;
    pub static mut glUniform4fv:usize=0;

    // glUniform1iv:usize,
    // glUniform2iv:usize,
    // glUniform3iv:usize,
    // glUniform4iv:usize,

    // glUniform1uiv:usize,
    // glUniform2uiv:usize,
    // glUniform3uiv:usize,
    // glUniform4uiv:usize,

    // glUniformMatrix2fv:usize,
    // glUniformMatrix3fv:usize,
    pub static mut glUniformMatrix4fv:usize=0;

    // glUniformMatrix2x3fv:usize,
    // glUniformMatrix3x2fv:usize,
    // glUniformMatrix2x4fv:usize,
    // glUniformMatrix4x2fv:usize,
    // glUniformMatrix3x4fv:usize,
    // glUniformMatrix4x3fv:usize,
}

#[cfg(target_os="windows")]
mod gl_functions{
    use super::*;

    #[inline(always)]
    pub unsafe fn glUniform1ui(location:GLint,v0:GLuint){
        transmute::<usize,extern "system" fn(GLint,GLuint)>(gl::glUniform1ui)(location,v0)
    }

    #[inline(always)]
    pub unsafe fn glUniform1i(location:GLint,v0:GLint){
        transmute::<usize,extern "system" fn(GLint,GLint)>(gl::glUniform1i)(location,v0)
    }


    #[inline(always)]
    pub unsafe fn glUniformMatrix4fv(location:GLint,count:GLsizei,transpose:GLboolean,value:*const GLfloat){
        transmute::<usize,extern "system" fn(GLint,GLsizei,GLboolean,*const GLfloat)>(gl::glUniformMatrix4fv)(location,count,transpose,value)
    }
}

#[cfg(target_os="windows")]
use gl_functions::*;

pub struct Uniform;

impl Uniform{
    #[cfg(target_os="windows")]
    pub fn load(library:&OpenGraphicsLibrary){
        use gl::*;

        unsafe{
            glUniform1f=transmute(library.get_proc_address("glUniform1f\0"));
            glUniform2f=transmute(library.get_proc_address("glUniform2f\0"));
            glUniform3f=transmute(library.get_proc_address("glUniform3f\0"));
            glUniform4f=transmute(library.get_proc_address("glUniform4f\0"));

            glUniform1i=transmute(library.get_proc_address("glUniform1i\0"));
            glUniform2i=transmute(library.get_proc_address("glUniform2i\0"));
            glUniform3i=transmute(library.get_proc_address("glUniform3i\0"));
            glUniform4i=transmute(library.get_proc_address("glUniform4i\0"));

            glUniform1ui=transmute(library.get_proc_address("glUniform1ui\0"));
            glUniform2ui=transmute(library.get_proc_address("glUniform2ui\0"));
            glUniform3ui=transmute(library.get_proc_address("glUniform3ui\0"));
            glUniform4ui=transmute(library.get_proc_address("glUniform4ui\0"));

            glUniform1fv=transmute(library.get_proc_address("glUniform1fv\0"));
            glUniform2fv=transmute(library.get_proc_address("glUniform2fv\0"));
            glUniform3fv=transmute(library.get_proc_address("glUniform3fv\0"));
            glUniform4fv=transmute(library.get_proc_address("glUniform4fv\0"));

            glUniformMatrix4fv=transmute(library.get_proc_address("glUniformMatrix4fv\0"));
        }
    }
}

impl Uniform{
    pub unsafe fn set_i32_1(location:i32,v0:i32){
        glUniform1i(location,v0)
    }

    pub unsafe fn set_u32_1(location:i32,v0:u32){
        glUniform1ui(location,v0)
    }
}

impl Uniform{
    pub unsafe fn set_matrix_4_f32(location:i32,count:i32,transpose:bool,value:*const f32){
        glUniformMatrix4fv(location,count,transpose as u8,value)
    }
}