use crate::graphics::GLCore;

use super::Program;

use std::marker::PhantomData;

pub struct Uniform<'a>{
    id:i32,
    marker:PhantomData<&'a Program>,
}

impl<'a> Uniform<'a>{
    pub unsafe fn raw(id:i32)->Uniform<'a>{
        Self{
            id,
            marker:PhantomData,
        }
    }

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
            GLCore.uniform.set_f32_1(self.id,value)
        }
    }

    pub fn set_f32_2(&self,values:[f32;2]){
        unsafe{
            GLCore.uniform.set_f32_2(self.id,values)
        }
    }

    pub fn set_f32_3(&self,values:[f32;3]){
        unsafe{
            GLCore.uniform.set_f32_3(self.id,values)
        }
    }

    pub fn set_f32_4(&self,values:[f32;4]){
        unsafe{
            GLCore.uniform.set_f32_4(self.id,values)
        }
    }

    pub fn set_i32_1(&self,value:i32){
        unsafe{
            GLCore.uniform.set_i32_1(self.id,value)
        }
    }

    pub fn set_i32_2(&self,values:[i32;2]){
        unsafe{
            GLCore.uniform.set_i32_2(self.id,values)
        }
    }

    pub fn set_i32_3(&self,values:[i32;3]){
        unsafe{
            GLCore.uniform.set_i32_3(self.id,values)
        }
    }

    pub fn set_i32_4(&self,values:[i32;4]){
        unsafe{
            GLCore.uniform.set_i32_4(self.id,values)
        }
    }

    pub fn set_u32_1(&self,value:u32){
        unsafe{
            GLCore.uniform.set_u32_1(self.id,value)
        }
    }

    pub fn set_u32_2(&self,values:[u32;2]){
        unsafe{
            GLCore.uniform.set_u32_2(self.id,values)
        }
    }

    pub fn set_u32_3(&self,values:[u32;3]){
        unsafe{
            GLCore.uniform.set_u32_3(self.id,values)
        }
    }

    pub fn set_u32_4(&self,values:[u32;4]){
        unsafe{
            GLCore.uniform.set_u32_4(self.id,values)
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