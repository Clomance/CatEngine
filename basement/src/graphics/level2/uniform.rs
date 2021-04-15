use super::Program;

use gl::{
    Uniform1f,
    Uniform2f,
    Uniform3f,
    Uniform4f,
    Uniform1i,
    Uniform2i,
    Uniform3i,
    Uniform4i,
    Uniform1ui,
    Uniform2ui,
    Uniform3ui,
    Uniform4ui,
    Uniform1fv,
    Uniform2fv,
    Uniform3fv,
    Uniform4fv,
    Uniform1iv,
    Uniform2iv,
    Uniform3iv,
    Uniform4iv,
    Uniform1uiv,
    Uniform2uiv,
    Uniform3uiv,
    Uniform4uiv,
    UniformMatrix2fv,
    UniformMatrix3fv,
    UniformMatrix4fv,
    UniformMatrix2x3fv,
    UniformMatrix3x2fv,
    UniformMatrix2x4fv,
    UniformMatrix4x2fv,
    UniformMatrix3x4fv,
    UniformMatrix4x3fv,
};

use std::marker::PhantomData;

pub struct Uniform<'a>{
    id:i32,
    marker:PhantomData<&'a Program>,
}

impl<'a> Uniform<'a>{
    pub fn new(program:&'a Program,name:&str)->Option<Uniform<'a>>{
        if let Some(id)=program.get_uniform_location(name){
            Some(Self{
                id,
                marker:PhantomData,
            })
        }
        else{
            None
        }
    }

    pub fn set<V:UniformValue>(&self,value:V){
        UniformValue::set(self,value)
    }

    pub fn set_f32_1(&self,value:f32){
        unsafe{
            Uniform1f(self.id,value)
        }
    }

    pub fn set_f32_2(&self,[v1,v2]:[f32;2]){
        unsafe{
            Uniform2f(self.id,v1,v2)
        }
    }

    pub fn set_f32_3(&self,[v1,v2,v3]:[f32;3]){
        unsafe{
            Uniform3f(self.id,v1,v2,v3)
        }
    }

    pub fn set_f32_4(&self,[v1,v2,v3,v4]:[f32;4]){
        unsafe{
            Uniform4f(self.id,v1,v2,v3,v4)
        }
    }

    pub fn set_i32_1(&self,value:i32){
        unsafe{
            Uniform1i(self.id,value)
        }
    }

    pub fn set_i32_2(&self,[v1,v2]:[i32;2]){
        unsafe{
            Uniform2i(self.id,v1,v2)
        }
    }

    pub fn set_i32_3(&self,[v1,v2,v3]:[i32;3]){
        unsafe{
            Uniform3i(self.id,v1,v2,v3)
        }
    }

    pub fn set_i32_4(&self,[v1,v2,v3,v4]:[i32;4]){
        unsafe{
            Uniform4i(self.id,v1,v2,v3,v4)
        }
    }

    pub fn set_u32_1(&self,value:u32){
        unsafe{
            Uniform1ui(self.id,value)
        }
    }

    pub fn set_u32_2(&self,[v1,v2]:[u32;2]){
        unsafe{
            Uniform2ui(self.id,v1,v2)
        }
    }

    pub fn set_u32_3(&self,[v1,v2,v3]:[u32;3]){
        unsafe{
            Uniform3ui(self.id,v1,v2,v3)
        }
    }

    pub fn set_u32_4(&self,[v1,v2,v3,v4]:[u32;4]){
        unsafe{
            Uniform4ui(self.id,v1,v2,v3,v4)
        }
    }
}

pub trait UniformValue{
    fn set(uniform:&Uniform,value:Self);
}

impl UniformValue for f32{
    fn set(uniform:&Uniform,value:f32){
        uniform.set_f32_1(value);
    }
}

impl UniformValue for [f32;2]{
    fn set(uniform:&Uniform,value:[f32;2]){
        uniform.set_f32_2(value);
    }
}

impl UniformValue for (f32,f32){
    fn set(uniform:&Uniform,(v1,v2):(f32,f32)){
        uniform.set_f32_2([v1,v2]);
    }
}

impl UniformValue for [f32;3]{
    fn set(uniform:&Uniform,value:[f32;3]){
        uniform.set_f32_3(value);
    }
}

impl UniformValue for (f32,f32,f32){
    fn set(uniform:&Uniform,(v1,v2,v3):(f32,f32,f32)){
        uniform.set_f32_3([v1,v2,v3]);
    }
}

impl UniformValue for [f32;4]{
    fn set(uniform:&Uniform,value:[f32;4]){
        uniform.set_f32_4(value);
    }
}

impl UniformValue for (f32,f32,f32,f32){
    fn set(uniform:&Uniform,(v1,v2,v3,v4):(f32,f32,f32,f32)){
        uniform.set_f32_4([v1,v2,v3,v4]);
    }
}

impl UniformValue for i32{
    fn set(uniform:&Uniform,value:i32){
        uniform.set_i32_1(value);
    }
}

impl UniformValue for [i32;2]{
    fn set(uniform:&Uniform,value:[i32;2]){
        uniform.set_i32_2(value);
    }
}

impl UniformValue for (i32,i32){
    fn set(uniform:&Uniform,(v1,v2):(i32,i32)){
        uniform.set_i32_2([v1,v2]);
    }
}

impl UniformValue for [i32;3]{
    fn set(uniform:&Uniform,value:[i32;3]){
        uniform.set_i32_3(value);
    }
}

impl UniformValue for (i32,i32,i32){
    fn set(uniform:&Uniform,(v1,v2,v3):(i32,i32,i32)){
        uniform.set_i32_3([v1,v2,v3]);
    }
}

impl UniformValue for [i32;4]{
    fn set(uniform:&Uniform,value:[i32;4]){
        uniform.set_i32_4(value);
    }
}

impl UniformValue for (i32,i32,i32,i32){
    fn set(uniform:&Uniform,(v1,v2,v3,v4):(i32,i32,i32,i32)){
        uniform.set_i32_4([v1,v2,v3,v4]);
    }
}

impl UniformValue for u32{
    fn set(uniform:&Uniform,value:u32){
        uniform.set_u32_1(value);
    }
}

impl UniformValue for [u32;2]{
    fn set(uniform:&Uniform,value:[u32;2]){
        uniform.set_u32_2(value);
    }
}

impl UniformValue for (u32,u32){
    fn set(uniform:&Uniform,(v1,v2):(u32,u32)){
        uniform.set_u32_2([v1,v2]);
    }
}

impl UniformValue for [u32;3]{
    fn set(uniform:&Uniform,value:[u32;3]){
        uniform.set_u32_3(value);
    }
}

impl UniformValue for (u32,u32,u32){
    fn set(uniform:&Uniform,(v1,v2,v3):(u32,u32,u32)){
        uniform.set_u32_3([v1,v2,v3]);
    }
}

impl UniformValue for [u32;4]{
    fn set(uniform:&Uniform,value:[u32;4]){
        uniform.set_u32_4(value);
    }
}

impl UniformValue for (u32,u32,u32,u32){
    fn set(uniform:&Uniform,(v1,v2,v3,v4):(u32,u32,u32,u32)){
        uniform.set_u32_4([v1,v2,v3,v4]);
    }
}