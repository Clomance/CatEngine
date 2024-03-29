use crate::graphics::{
    GLCore,
    core::GLError,
    core::shader::{
        ShaderType,
        ShaderParameter,
    }
};

use core::mem::MaybeUninit;

pub struct Shader{
    id:u32,
}

impl Shader{
    pub fn generate(shader_type:ShaderType)->Shader{
        unsafe{
            Self{
                id:GLCore.shader.create(shader_type),
            }
        }
    }

    /// Creates and compiles a shader without checking the result.
    /// 
    /// Создаёт и компилирует шейдер без проверки результата.
    pub unsafe fn new_unchecked(source:&str,shader_type:ShaderType)->Shader{
        // Создание шейдера
        let id=GLCore.shader.create(shader_type);
        // Загрузка кода
        GLCore.shader.source(id,source);
        // Компиляция
        GLCore.shader.compile(id);

        Self{
            id,
        }
    }

    /// Creates and compiles a shader.
    /// 
    /// Создаёт и компилирует шейдер.
    pub fn new(source:&str,shader_type:ShaderType)->Result<Shader,String>{
        unsafe{
            let shader=Shader::new_unchecked(source,shader_type);

            // Проверка компиляции
            let mut result:i32=MaybeUninit::uninit().assume_init();
            GLCore.shader.get_parameter(shader.id(),ShaderParameter::CompileStatus,&mut result);

            if result==0{
                let mut length=MaybeUninit::uninit().assume_init();
                GLCore.shader.get_parameter(shader.id(),ShaderParameter::InfoLogLength,&mut length);

                let mut log=String::with_capacity(length as usize);

                GLCore.shader.get_info_log(shader.id(),&mut log);

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
    pub fn compile(&self)->GLError{
        unsafe{
            GLCore.shader.compile(self.id);
            GLCore.get_error()
        }
    }

    pub fn get_parameter(&self,parameter:ShaderParameter,value:&mut i32)->GLError{
        unsafe{
            GLCore.shader.get_parameter(self.id,parameter,value);
            GLCore.get_error()
        }
    }

    pub fn get_info_log(&self,log:&mut String)->GLError{
        unsafe{
            GLCore.shader.get_info_log(self.id,log);
            GLCore.get_error()
        }
    }
}

impl Drop for Shader{
    fn drop(&mut self){
        unsafe{
            GLCore.shader.delete(self.id);
        }
    }
}