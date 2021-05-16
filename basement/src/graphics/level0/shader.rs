use gl::{
    // constants
    VERTEX_SHADER,
    FRAGMENT_SHADER,
    COMPILE_STATUS,
    // functions
    CreateShader,
    ShaderSource,
    CompileShader,
    GetShaderiv,
    GetShaderInfoLog,
    DeleteShader,
};

use std::{
    ffi::CString,
    mem::MaybeUninit,
};

pub enum ShaderType{
    FragmentShader=FRAGMENT_SHADER as isize,
    VertexShader=VERTEX_SHADER as isize,
}

pub struct Shader{
    id:u32,
}

impl Shader{
    /// Creates and compiles a shader.
    /// 
    /// Создаёт и компилирует шейдер.
    pub fn new(source:&str,shader_type:ShaderType)->Result<Shader,String>{
        unsafe{
            // Создание шейдера
            let id=CreateShader(shader_type as u32);

            let string=CString::new(source).expect("NullByte");

            // Загрузка данных
            let string_ptr=string.as_ptr() as *const i8;
            //let length=source.len() as i32;
            ShaderSource(id,1,&string_ptr as *const *const i8,core::ptr::null());
            // Компиляция
            CompileShader(id);

            // Проверка компиляции
            let mut result:i32=MaybeUninit::uninit().assume_init();
            GetShaderiv(id,COMPILE_STATUS,&mut result as *mut _);

            if result==0{
                let mut log=String::with_capacity(512);
                let log_ref=log.as_ptr() as *mut i8;
                let mut len=0i32;

                GetShaderInfoLog(id,512,&mut len as *mut i32,log_ref);

                log.as_mut_vec().set_len(len as usize);

                return Err(log);
            }

            Ok(Self{
                id,
            })
        }
    }

    #[inline(always)]
    pub fn id(&self)->u32{
        self.id
    }
}

impl Drop for Shader{
    fn drop(&mut self){
        unsafe{
            DeleteShader(self.id);
        }
    }
}