#[cfg(target_os="windows")]
use crate::windows::OpenGraphicsLibrary;

use core::mem::transmute;

pub struct Uniform{
    glUniform1f:usize,
    glUniform2f:usize,
    glUniform3f:usize,
    glUniform4f:usize,

    glUniform1i:usize,
    glUniform2i:usize,
    glUniform3i:usize,
    glUniform4i:usize,

    glUniform1ui:usize,
    glUniform2ui:usize,
    glUniform3ui:usize,
    glUniform4ui:usize,

    // glUniform1fv:usize,
    // glUniform2fv:usize,
    // glUniform3fv:usize,
    // glUniform4fv:usize,

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
    // glUniformMatrix4fv:usize,
    // glUniformMatrix2x3fv:usize,
    // glUniformMatrix3x2fv:usize,
    // glUniformMatrix2x4fv:usize,
    // glUniformMatrix4x2fv:usize,
    // glUniformMatrix3x4fv:usize,
    // glUniformMatrix4x3fv:usize,
}

impl Uniform{
    pub const fn new()->Uniform{
        Self{
            glUniform1f:0,
            glUniform2f:0,
            glUniform3f:0,
            glUniform4f:0,

            glUniform1i:0,
            glUniform2i:0,
            glUniform3i:0,
            glUniform4i:0,

            glUniform1ui:0,
            glUniform2ui:0,
            glUniform3ui:0,
            glUniform4ui:0,

            // glUniform1fv:0,
            // glUniform2fv:0,
            // glUniform3fv:0,
            // glUniform4fv:0,

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
            // glUniformMatrix4fv:usize,
            // glUniformMatrix2x3fv:usize,
            // glUniformMatrix3x2fv:usize,
            // glUniformMatrix2x4fv:usize,
            // glUniformMatrix4x2fv:usize,
            // glUniformMatrix3x4fv:usize,
            // glUniformMatrix4x3fv:usize,
        }
    }

    #[cfg(target_os="windows")]
    pub fn load(&mut self,library:&OpenGraphicsLibrary){
        unsafe{
            self.glUniform1f=transmute(library.get_proc_address("glUniform1f\0"));
            self.glUniform2f=transmute(library.get_proc_address("glUniform2f\0"));
            self.glUniform3f=transmute(library.get_proc_address("glUniform3f\0"));
            self.glUniform4f=transmute(library.get_proc_address("glUniform4f\0"));

            self.glUniform1i=transmute(library.get_proc_address("glUniform1i\0"));
            self.glUniform2i=transmute(library.get_proc_address("glUniform2i\0"));
            self.glUniform3i=transmute(library.get_proc_address("glUniform3i\0"));
            self.glUniform4i=transmute(library.get_proc_address("glUniform4i\0"));

            self.glUniform1ui=transmute(library.get_proc_address("glUniform1ui\0"));
            self.glUniform2ui=transmute(library.get_proc_address("glUniform2ui\0"));
            self.glUniform3ui=transmute(library.get_proc_address("glUniform3ui\0"));
            self.glUniform4ui=transmute(library.get_proc_address("glUniform4ui\0"));

            // self.glUniform1fv=transmute(library.get_proc_address("glUniform1fv\0"));
            // self.glUniform2fv=transmute(library.get_proc_address("glUniform2fv\0"));
            // self.glUniform3fv=transmute(library.get_proc_address("glUniform3fv\0"));
            // self.glUniform4fv=transmute(library.get_proc_address("glUniform4fv\0"));

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
            // glUniformMatrix4fv:usize,
            // glUniformMatrix2x3fv:usize,
            // glUniformMatrix3x2fv:usize,
            // glUniformMatrix2x4fv:usize,
            // glUniformMatrix4x2fv:usize,
            // glUniformMatrix3x4fv:usize,
            // glUniformMatrix4x3fv:usize,
        }
    }
}

impl Uniform{
    pub unsafe fn set_f32_1(&self,location:i32,value:f32){
        transmute::<usize,fn(i32,f32)>(self.glUniform1f)(location,value)
    }

    pub unsafe fn set_f32_2(&self,location:i32,[v1,v2]:[f32;2]){
        transmute::<usize,fn(i32,f32,f32)>(self.glUniform2f)(location,v1,v2)
    }

    pub unsafe fn set_f32_3(&self,location:i32,[v1,v2,v3]:[f32;3]){
        transmute::<usize,fn(i32,f32,f32,f32)>(self.glUniform3f)(location,v1,v2,v3)
    }

    pub unsafe fn set_f32_4(&self,location:i32,[v1,v2,v3,v4]:[f32;4]){
        transmute::<usize,fn(i32,f32,f32,f32,f32)>(self.glUniform4f)(location,v1,v2,v3,v4)
    }

    pub unsafe fn set_i32_1(&self,location:i32,value:i32){
        transmute::<usize,fn(i32,i32)>(self.glUniform1i)(location,value)
    }

    pub unsafe fn set_i32_2(&self,location:i32,[v1,v2]:[i32;2]){
        transmute::<usize,fn(i32,i32,i32)>(self.glUniform2i)(location,v1,v2)
    }

    pub unsafe fn set_i32_3(&self,location:i32,[v1,v2,v3]:[i32;3]){
        transmute::<usize,fn(i32,i32,i32,i32)>(self.glUniform3i)(location,v1,v2,v3)
    }

    pub unsafe fn set_i32_4(&self,location:i32,[v1,v2,v3,v4]:[i32;4]){
        transmute::<usize,fn(i32,i32,i32,i32,i32)>(self.glUniform4i)(location,v1,v2,v3,v4)
    }

    pub unsafe fn set_u32_1(&self,location:i32,value:u32){
        transmute::<usize,fn(i32,u32)>(self.glUniform1ui)(location,value)
    }

    pub unsafe fn set_u32_2(&self,location:i32,[v1,v2]:[u32;2]){
        transmute::<usize,fn(i32,u32,u32)>(self.glUniform2ui)(location,v1,v2)
    }

    pub unsafe fn set_u32_3(&self,location:i32,[v1,v2,v3]:[u32;3]){
        transmute::<usize,fn(i32,u32,u32,u32)>(self.glUniform3ui)(location,v1,v2,v3)
    }

    pub unsafe fn set_u32_4(&self,location:i32,[v1,v2,v3,v4]:[u32;4]){
        transmute::<usize,fn(i32,u32,u32,u32,u32)>(self.glUniform4ui)(location,v1,v2,v3,v4)
    }

    // pub unsafe fn set_f32_1(&self,location:i32,value:f32){
    //     transmute::<usize,fn(i32,f32)>(self.glUniform1fv)(location,value)
    // }

    // pub unsafe fn set_f32_2(&self,location:i32,[v1,v2]:[f32;2]){
    //     transmute::<usize,fn(i32,f32,f32)>(self.glUniform2fv)(location,v1,v2)
    // }

    // pub unsafe fn set_f32_3(&self,location:i32,[v1,v2,v3]:[f32;3]){
    //     transmute::<usize,fn(i32,f32,f32,f32)>(self.glUniform3fv)(location,v1,v2,v3)
    // }

    // pub unsafe fn set_f32_4(&self,location:i32,[v1,v2,v3,v4]:[f32;4]){
    //     transmute::<usize,fn(i32,f32,f32,f32,f32)>(self.glUniform4fv)(location,v1,v2,v3,v4)
    // }
}