#[cfg(any(windows))]
use crate::windows::OpenGraphicsLibrary;

use core::mem::{
    MaybeUninit,
    transmute,
};

use std::ffi::CStr;

// Shader types
const FRAGMENT_SHADER:u32=0x8B30;
const VERTEX_SHADER:u32=0x8B31;
const GEOMETRY_SHADER :u32=0x8DD9;

/// Shader parameters
const SHADER_TYPE:u32=0x8B4F;
const DELETE_STATUS:u32=0x8B80;
const COMPILE_STATUS:u32=0x8B81;
const INFO_LOG_LENGTH:u32=0x8B84;
const SHADER_SOURCE_LENGTH:u32=0x8B88;

/// Specifies a shader type.
#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum ShaderType{
    FragmentShader=FRAGMENT_SHADER,
    VertexShader=VERTEX_SHADER,
    GeometryShader=GEOMETRY_SHADER,
}

/// Specifies a shader object parameter.
#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum ShaderParameter{
    /// Returns `ShaderType::FragmentShader` if shader is a vertex shader object,
    /// `ShaderType::GeometryShader` if shader is a geometry shader object,
    /// and `ShaderType::FragmentShader` if shader is a fragment shader object.
    ShaderType=SHADER_TYPE,

    /// Returns `true` if shader is currently flagged for deletion,
    /// and `false` otherwise.
    DeleteStatus=DELETE_STATUS,

    /// Returns `true` if the last compile operation on shader was successful,
    /// and `false` otherwise.
    CompileStatus=COMPILE_STATUS,

    /// Returns the number of characters in the information log for shader including the null termination character
    /// (i.e., the size of the character buffer required to store the information log).
    /// If shader has no information log, a value of 0 is returned.
    InfoLogLength=INFO_LOG_LENGTH,

    /// Returns the length of the concatenation of the source strings that make up the shader source for the shader,
    /// including the null termination character (i.e., the size of the character buffer required to store the shader source).
    /// If no source code exists, 0 is returned.
    ShaderSourceLength=SHADER_SOURCE_LENGTH,
}

pub struct Shader{
    glCreateShader:usize,
    glDeleteShader:usize,

    glShaderSource:usize,
    glCompileShader:usize,

    glGetShaderiv:usize,
    glGetShaderInfoLog:usize,
}

impl Shader{
    pub const fn new()->Shader{
        Self{
            glCreateShader:0,
            glDeleteShader:0,

            glShaderSource:0,
            glCompileShader:0,

            glGetShaderiv:0,
            glGetShaderInfoLog:0,
        }
    }

    #[cfg(any(windows))]
    pub fn load(&mut self,library:&OpenGraphicsLibrary){
        unsafe{
            self.glCreateShader=transmute(library.get_proc_address("glCreateShader\0"));
            self.glDeleteShader=transmute(library.get_proc_address("glDeleteShader\0"));

            self.glShaderSource=transmute(library.get_proc_address("glShaderSource\0"));
            self.glCompileShader=transmute(library.get_proc_address("glCompileShader\0"));

            self.glGetShaderiv=transmute(library.get_proc_address("glGetShaderiv\0"));
            self.glGetShaderInfoLog=transmute(library.get_proc_address("glGetShaderInfoLog\0"));
        }
    }
}

impl Shader{
    /// Creates a shader object.
    /// 
    /// Returns a non-zero value by which it can be referenced.
    #[inline(always)]
    pub fn create(&self,shader_type:ShaderType)->u32{
        unsafe{
            transmute::<usize,fn(ShaderType)->u32>(self.glCreateShader)(shader_type)
        }
    }

    /// Deletes a shader object.
    /// 
    /// If a shader object to be deleted is attached to a program object,
    /// it will be flagged for deletion,
    /// but it will not be deleted until it is no longer attached to any program object,
    /// for any rendering context (i.e., it must be detached from
    /// wherever it was attached before it will be deleted).
    /// A value of 0 for shader will be silently ignored.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if `shader` is not a value generated by OpenGL.
    #[inline(always)]
    pub unsafe fn delete(&self,shader:u32){
        transmute::<usize,fn(u32)>(self.glDeleteShader)(shader)
    }

    /// Replaces the source code in a shader object.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if `shader` is not a value generated by OpenGL.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if `shader` is not a shader object.
    #[inline(always)]
    pub unsafe fn source(&self,shader:u32,source:&str){
        let ptr=&source.as_ptr() as *const *const u8;
        let length=&[source.bytes().len() as i32];
        transmute::<usize,fn(u32,i32,*const*const u8,&i32)>(self.glShaderSource)(shader,1,ptr,&length[0])
    }

    /// Compiles a shader object.
    /// 
    /// The compilation status will be stored as part of the shader object's state.
    /// This value will be set to `true`
    /// if the shader was compiled without errors and is ready for use,
    /// and `false` otherwise.
    /// It can be queried by calling `Shader::get_parameter`
    /// with arguments shader and `ShaderParameter::CompileStatus`.
    /// 
    /// `GLError::InvalidValue` is generated if `shader` is not a value generated by OpenGL.
    /// 
    /// `GLError::InvalidOperation` is generated if `shader` is not a shader object.
    #[inline(always)]
    pub unsafe fn compile(&self,shader:u32){
        transmute::<usize,fn(u32)>(self.glCompileShader)(shader)
    }

    /// Returns a parameter from a shader object.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if `shader` is not a value generated by OpenGL.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if `shader` is not a shader object.
    #[inline(always)]
    pub unsafe fn get_parameter(&self,shader:u32,parameter:ShaderParameter,value:&mut i32){
        transmute::<usize,fn(u32,ShaderParameter,&mut i32)>(self.glGetShaderiv)(shader,parameter,value)
    }

    /// Returns the information log for a shader object.
    /// 
    /// Fills `log` without allocation.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if `shader` is not a value generated by OpenGL.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if `shader` is not a shader object.
    pub unsafe fn get_info_log(&self,shader:u32,log:&mut String){
        let buffer=log.as_mut_vec();
        let mut length:i32=MaybeUninit::uninit().assume_init();
        transmute::<usize,fn(u32,i32,&mut i32,*mut u8)>(self.glGetShaderInfoLog)(
            shader,
            buffer.capacity() as i32,
            &mut length,
            buffer.as_mut_ptr()
        );
        if length!=0{
            length-=1;
        }
        buffer.set_len(length as usize);
    }
}