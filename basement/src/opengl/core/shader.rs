#[cfg(target_os="windows")]
use crate::winapi::OpenGraphicsLibrary;

use super::{
    types::*,
    constants::*
};

use core::mem::transmute;

#[cfg(target_os="linux")]
extern "system"{
    fn glCreateShader(shader_type:GLenum)->GLuint;
    fn glDeleteShader(shader:GLuint);
    fn glIsShader(shader:GLuint)->GLboolean;

    fn glShaderSource(shader:GLuint,count:GLsizei,string:*const *const GLchar,length:*const GLint);
    fn glGetShaderSource(shader:GLuint,buffer_size:GLsizei,length:*mut GLint,source:*mut GLchar);

    fn glCompileShader(shader:GLuint);

    fn glGetShaderiv(shader:GLuint,parameter:GLenum,values:*mut GLint);
    fn glGetShaderInfoLog(shader:GLuint,max_length:GLsizei,length:*mut GLsizei,info_log:*mut GLchar);
}

#[cfg(target_os="windows")]
mod gl{
    pub static mut glCreateShader:usize=0;
    pub static mut glDeleteShader:usize=0;
    pub static mut glIsShader:usize=0;

    pub static mut glShaderSource:usize=0;
    pub static mut glGetShaderSource:usize=0;

    pub static mut glCompileShader:usize=0;

    pub static mut glGetShaderiv:usize=0;
    pub static mut glGetShaderInfoLog:usize=0;
}

#[cfg(target_os="windows")]
mod gl_functions{
    use super::*;

    #[inline(always)]
    pub unsafe fn glCreateShader(shader_type:GLenum)->GLuint{
        transmute::<usize,fn(GLenum)->GLuint>(gl::glCreateShader)(shader_type)
    }
    #[inline(always)]
    pub unsafe fn glDeleteShader(shader:GLuint){
        transmute::<usize,fn(GLuint)>(gl::glDeleteShader)(shader)
    }
    #[inline(always)]
    pub unsafe fn glIsShader(shader:GLuint)->GLboolean{
        transmute::<usize,fn(GLuint)->GLboolean>(gl::glIsShader)(shader)
    }


    #[inline(always)]
    pub unsafe fn glShaderSource(shader:GLuint,count:GLsizei,string:*const *const GLchar,length:*const GLint){
        transmute::<usize,fn(GLuint,GLsizei,*const *const GLchar,*const GLint)>(gl::glShaderSource)(shader,count,string,length)
    }
    #[inline(always)]
    pub unsafe fn glGetShaderSource(shader:GLuint,buffer_size:GLsizei,length:*mut GLint,source:*mut GLchar){
        transmute::<usize,fn(GLuint,GLsizei,*mut GLint,*mut GLchar)>(gl::glGetShaderSource)(shader,buffer_size,length,source)
    }


    #[inline(always)]
    pub unsafe fn glCompileShader(shader:GLuint){
        transmute::<usize,fn(GLuint)>(gl::glCompileShader)(shader)
    }


    #[inline(always)]
    pub unsafe fn glGetShaderiv(shader:GLuint,parameter:GLenum,values:*mut GLint){
        transmute::<usize,fn(GLuint,GLenum,*mut GLint)>(gl::glGetShaderiv)(shader,parameter,values)
    }
    #[inline(always)]
    pub unsafe fn glGetShaderInfoLog(shader:GLuint,max_length:GLsizei,length:*mut GLsizei,info_log:*mut GLchar){
        transmute::<usize,fn(GLuint,GLsizei,*mut GLsizei,*mut GLchar)>(gl::glGetShaderInfoLog)(shader,max_length,length,info_log)
    }
}

#[cfg(target_os="windows")]
use gl_functions::*;

pub struct Shader;

impl Shader{
    #[cfg(target_os="windows")]
    pub fn load(library:&OpenGraphicsLibrary){
        unsafe{
            use gl::*;

            glCreateShader=transmute(library.get_proc_address("glCreateShader\0"));
            glDeleteShader=transmute(library.get_proc_address("glDeleteShader\0"));
            glIsShader=transmute(library.get_proc_address("glIsShader\0"));

            glShaderSource=transmute(library.get_proc_address("glShaderSource\0"));
            glGetShaderSource=transmute(library.get_proc_address("glGetShaderSource\0"));

            glCompileShader=transmute(library.get_proc_address("glCompileShader\0"));

            glGetShaderiv=transmute(library.get_proc_address("glGetShaderiv\0"));
            glGetShaderInfoLog=transmute(library.get_proc_address("glGetShaderInfoLog\0"));
        }
    }
}

/// Specifies a shader type.
#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum ShaderType{
    FragmentShader=FRAGMENT_SHADER,
    VertexShader=VERTEX_SHADER,
    GeometryShader=GEOMETRY_SHADER,
}

impl Shader{
    /// Creates a shader object.
    /// 
    /// `shader_type` specifies the type of shader to be created.
    /// 
    /// Creates an empty shader object and returns a non-zero value by which it can be referenced.
    /// A shader object is used to maintain the source code strings that define a shader.
    /// `shader_type` indicates the type of shader to be created.
    /// Three types of shaders are supported.
    /// 
    /// A shader of type `ShaderType::VertexShader` is a shader that is intended to run on the programmable vertex processor.
    /// A shader of type `ShaderType::GeometryShader` is a shader that is intended to run on the programmable geometry processor.
    /// A shader of type `ShaderType::FragmentShader` is a shader that is intended to run on the programmable fragment processor.
    /// 
    /// When created, a shader object's `GL_SHADER_TYPE` parameter is set to either
    /// `ShaderType::VertexShader`, `ShaderType::GeometryShader` or `ShaderType::FragmentShader`,
    /// depending on the value of `shader_type`.
    /// 
    /// Like buffer and texture objects, the name space for shader objects may be shared across a set of contexts,
    /// as long as the server sides of the contexts share the same address space.
    /// If the name space is shared across contexts, any attached objects and the data associated with those attached objects are shared as well.
    /// 
    /// Applications are responsible for providing the synchronization across API calls when objects are accessed from different execution threads.
    /// 
    /// This function returns 0 if an error occurs creating the shader object.
    /// 
    /// `Error::InvalidEnum` is generated if `shader_type` is not an accepted value.
    #[inline(always)]
    pub unsafe fn create(shader_type:ShaderType)->u32{
        glCreateShader(shader_type as GLenum)
    }

    /// Deletes a shader object.
    /// 
    /// `shader` specifies the shader object to be deleted.
    /// 
    /// Frees the memory and invalidates the name associated with the shader object specified by `shader`.
    /// This command effectively undoes the effects of a call to `Shader::create`.
    /// 
    /// If a shader object to be deleted is attached to a program object, it will be flagged for deletion,
    /// but it will not be deleted until it is no longer attached to any program object,
    /// for any rendering context (i.e., it must be detached from wherever it was attached before it will be deleted).
    /// A value of 0 for `shader` will be silently ignored.
    /// 
    /// To determine whether an object has been flagged for deletion,
    /// call `Shader::get` with arguments `shader` and `ShaderParameter::DeleteStatus`.
    /// 
    /// `Error::InvalidValue` is generated if `shader` is not a value generated by OpenGL.
    #[inline(always)]
    pub unsafe fn delete(shader:u32){
        glDeleteShader(shader)
    }

    /// Determines if a name corresponds to a shader object.
    /// 
    /// `shader` specifies a potential shader object.
    /// 
    /// Returns `true` if `shader` is the name of a shader object previously
    /// created with `Shader::create` and not yet deleted with `Shader::delete`.
    /// If `shader` is zero or a non-zero value that is not the name of a shader object,
    /// or if an error occurs, `Shader::is_shader` returns `false`.
    /// 
    /// No error is generated if `shader` is not a valid shader object name.
    /// 
    /// A shader object marked for deletion with `Shader::delete`
    /// but still attached to a program object is still considered a shader object and `Shader::is_shader` will return `true`.
    #[inline(always)]
    pub unsafe fn is_shader(shader:u32)->bool{
        transmute(glIsShader(shader))
    }
}



impl Shader{
    /// Replaces the source code in a shader object.
    /// 
    /// `shader` specifies the handle of the shader object whose source code is to be replaced.
    /// 
    /// `count` specifies the number of elements in the `string` and `length` arrays.
    /// 
    /// `string` specifies an array of pointers to strings containing the source code to be loaded into the shader.
    /// 
    /// `length` specifies an array of string lengths.
    /// 
    /// Sets the source code in `shader` to the source code in the array of strings specified by `string`.
    /// Any source code previously stored in the shader object is completely replaced.
    /// The number of strings in the array is specified by `count`.
    /// If `length` is `NULL`, each string is assumed to be null terminated.
    /// If `length` is a value other than `NULL`, it points to an array containing a string length for each of the corresponding elements of `string`.
    /// Each element in the `length` array may contain the length of the corresponding string (the null character is not counted as part of the string length)
    /// or a value less than 0 to indicate that the string is null terminated.
    /// The source code strings are not scanned or parsed at this time; they are simply copied into the specified shader object.
    /// 
    /// OpenGL copies the shader source code strings when `Shader::set_source` is called,
    /// so an application may free its copy of the source code strings immediately after the function returns.
    /// 
    /// `Error::InvalidValue` is generated
    /// if `shader` is not a value generated by OpenGL.
    /// if `count` is less than 0.
    /// 
    /// `Error::InvalidOparation` is generated if `shader` is not a shader object.
    #[inline(always)]
    pub unsafe fn set_source(shader:u32,count:i32,string:*const *const i8,length:*const i32){
        glShaderSource(shader,count,string,length)
    }

    /// Returns the source code string from a shader object.
    /// 
    /// `shader` specifies the shader object to be queried.
    /// 
    /// `buffet_size` specifies the size of the character buffer for storing the returned source code string.
    /// 
    /// `length` returns the length of the string returned in source (excluding the null terminator).
    /// 
    /// `source` specifies an array of characters that is used to return the source code string.
    /// 
    /// Returns the concatenation of the source code strings from the shader object specified by `shader`.
    /// The source code strings for a shader object are the result of a previous call to `Shader::set_source`.
    /// The string returned by the function will be null terminated.
    /// 
    /// Returns in `source` as much of the source code string as it can, up to a maximum of `buffer_size` characters.
    /// The number of characters actually returned, excluding the null termination character, is specified by `length`.
    /// If the length of the returned string is not required, a value of `NULL` can be passed in the `length` argument.
    /// The size of the buffer required to store the returned source code string can be obtained
    /// by calling `Shader::get` with the value `ShaderParameter::SourceLength`.
    /// 
    /// `Error::InvalidValue` is generated
    /// if `shader` is not a value generated by OpenGL.
    /// if `buffet_size` is less than 0.
    /// 
    /// `Error::InvalidOparation` is generated if `shader` is not a shader object.
    #[inline(always)]
    pub unsafe fn get_source(shader:u32,buffer_size:i32,length:*mut i32,source:*mut i8){
        glGetShaderSource(shader,buffer_size,length,source)
    }

    /// Compiles a shader object.
    /// 
    /// `shader` specifies the shader object to be compiled.
    /// 
    /// Compiles the source code strings that have been stored in the shader object specified by `shader`.
    /// 
    /// The compilation status will be stored as part of the shader object's state.
    /// This value will be set to `true` if the shader was compiled without errors and is ready for use, and `false` otherwise.
    /// It can be queried by calling `Shader::get` with arguments `shader` and `ShaderParameter::CompileStatus`.
    /// 
    /// Compilation of a shader can fail for a number of reasons as specified by the OpenGL Shading Language Specification.
    /// Whether or not the compilation was successful,
    /// information about the compilation can be obtained from the shader object's information log by calling `Shader::get_info_log`.
    /// 
    /// `Error::InvalidValue` is generated if `shader` is not a value generated by OpenGL.
    /// 
    /// `Error::InvalidOparation` is generated if `shader` is not a shader object.
    #[inline(always)]
    pub unsafe fn compile(shader:u32){
        glCompileShader(shader)
    }
}

/// Specifies a shader object parameter.
#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum ShaderParameter{
    /// Returns `ShaderType::FragmentShader` if `shader` is a vertex shader object,
    /// `ShaderType::GeometryShader` if `shader` is a geometry shader object,
    /// and `ShaderType::FragmentShader` if `shader` is a fragment shader object.
    Type=SHADER_TYPE,

    /// Returns `true` if `shader` is currently flagged for deletion,
    /// and `false` otherwise.
    DeleteStatus=DELETE_STATUS,

    /// Returns `true` if the last compile operation on `shader` was successful,
    /// and `false` otherwise.
    CompileStatus=COMPILE_STATUS,

    /// Returns the number of characters in the information log for `shader` including the null termination character
    /// (i.e., the size of the character buffer required to store the information log).
    /// If `shader` has no information log, a value of 0 is returned.
    InfoLogLength=INFO_LOG_LENGTH,

    /// Returns the length of the concatenation of the source strings that make up the shader source for the `shader`,
    /// including the null termination character (i.e., the size of the character buffer required to store the shader source).
    /// If no source code exists, 0 is returned.
    SourceLength=SHADER_SOURCE_LENGTH,
}

impl Shader{
    /// Returns a parameter from a shader object.
    /// 
    /// `shader` specifies the shader object to be queried.
    /// 
    /// `parameter` specifies the object parameter.
    /// 
    /// `values` returns the requested object parameter.
    /// 
    /// Returns in `parameter` the value of a parameter for a specific shader object.
    /// (See `ShaderParameter`.)
    /// 
    /// If an error is generated, no change is made to the contents of `values`.
    /// 
    /// `Error::InvalidValue` is generated if `shader` is not a value generated by OpenGL.
    /// 
    /// `Error::InvalidOparation` is generated if `shader` is not a shader object.
    /// 
    /// `Error::InvalidEnum` is generated if `parameter` is not an accepted value.
    #[inline(always)]
    pub unsafe fn get(shader:u32,parameter:ShaderParameter,values:*mut i32){
        glGetShaderiv(shader,parameter as GLenum,values)
    }

    /// Returns the information log for a shader object.
    /// 
    /// `shader` specifies the shader object whose information log is to be queried.
    /// 
    /// `max_length` specifies the size of the character buffer for storing the returned information log.
    /// 
    /// `length` returns the length of the string returned in `info_log` (excluding the null terminator).
    /// 
    /// `info_log` specifies an array of characters that is used to return the information log.
    /// 
    /// Returns the information log for the specified shader object.
    /// The information log for a shader object is modified when the shader is compiled.
    /// The string that is returned will be null terminated.
    /// 
    /// Returns in `info_log` as much of the information log as it can, up to a maximum of `max_length` characters.
    /// 
    /// The number of characters actually returned, excluding the null termination character, is specified by `length`.
    /// 
    ///  If the length of the returned string is not required, a value of `NULL` can be passed in the `length` argument.
    /// 
    /// The size of the buffer required to store the returned information log
    /// can be obtained by calling `Shader::get` with the value `ShaderParameter::InfoLogLength`.
    /// 
    /// The information log for a shader object is a string that may contain diagnostic messages,
    /// warning messages, and other information about the last compile operation.
    /// When a shader object is created, its information log will be a string of length 0.
    /// 
    /// The information log for a shader object is the OpenGL implementer's primary mechanism for conveying information about the compilation process.
    /// Therefore, the information log can be helpful to application developers during the development process, even when compilation is successful.
    /// Application developers should not expect different OpenGL implementations to produce identical information logs.
    /// 
    /// `Error::InvalidValue` is generated
    /// if `shader` is not a value generated by OpenGL.
    /// if `max_length` is less than 0.
    /// 
    /// `Error::InvalidOparation` is generated if `shader` is not a shader object.
    #[inline(always)]
    pub unsafe fn get_info_log(shader:u32,max_length:i32,length:*mut i32,info_log:*mut i8){
        glGetShaderInfoLog(shader,max_length,length,info_log)
    }
}