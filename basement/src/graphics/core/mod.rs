#[cfg(all(target_os="windows",feature="windows"))]
use crate::windows::OpenGraphicsLibrary;

pub mod parameters;
use parameters::GraphicsParameters;

pub mod buffer;
use buffer::Buffer;

pub mod drawing;
use drawing::Drawing;

pub mod framebuffer;
use framebuffer::Framebuffer;

pub mod program;
use program::Program;

pub mod shader;
use shader::Shader;

pub mod texture;
use texture::Texture;

pub mod uniform;
use uniform::Uniform;

pub mod vertex_array;
use vertex_array::VertexArray;

use std::{
    ffi::CStr,
    mem::transmute,
};

pub mod types{
    use std::{
        marker::{
            PhantomData,
            PhantomPinned
        },
        os::raw::{
            c_uchar,
            c_char,
            c_short,
            c_ushort,
            c_uint,
            c_int,
            c_float,
            c_double,
            c_void
        }
    };

    pub struct __GLsync{
        _data:[u8;0],
        _marker:PhantomData<(*mut u8,PhantomPinned)>,
    }

    pub type GLboolean=c_uchar;
    pub type GLbyte=c_char;
    pub type GLubyte=c_uchar;
    pub type GLshort=c_short;
    pub type GLushort=c_ushort;
    pub type GLint=c_int;
    pub type GLuint=c_uint;
    pub type GLfixed=c_uint;
    pub type GLint64=i64;
    pub type GLuint64=u64;
    pub type GLsizei=c_int;
    pub type GLenum=c_uint;
    pub type GLintptr=isize;
    pub type GLsizeiptr=isize;
    pub type GLsync=*const __GLsync;
    pub type GLbitfield=c_int;
    pub type GLhalf=c_ushort;
    pub type GLfloat=c_float;
    pub type GLclampf=c_float;
    pub type GLdouble=c_double;
    pub type GLclampd=c_double;
    pub type GLvoid=c_void;
}

const MAJOR_VERSION:u32=0x821B;
const MINOR_VERSION:u32=0x821C;
const VERSION:u32=0x1F02;

// Clear mask bits
const COLOR_BUFFER_BIT:u32=0x00004000;
const DEPTH_BUFFER_BIT:u32=0x00000100;
const STENCIL_BUFFER_BIT:u32=0x00000400;

// Errors
const NO_ERROR:u32=0;
const INVALID_ENUM:u32=0x0500;
const INVALID_VALUE:u32=0x0501;
const INVALID_OPERATION:u32=0x0502;
const INVALID_FRAMEBUFFER_OPERATION:u32=0x0506;
const OUT_OF_MEMORY:u32=0x0505;


/// Mask that indicate the buffers to be cleared.
#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum ClearMask{
    Colour=COLOR_BUFFER_BIT,
    Depth=DEPTH_BUFFER_BIT,
    Stencil=STENCIL_BUFFER_BIT,
    ColourDepth=COLOR_BUFFER_BIT|DEPTH_BUFFER_BIT,
    ColourStencil=COLOR_BUFFER_BIT|STENCIL_BUFFER_BIT,
    DepthStencil=DEPTH_BUFFER_BIT|STENCIL_BUFFER_BIT,
    ColourDepthStencil=COLOR_BUFFER_BIT|DEPTH_BUFFER_BIT|STENCIL_BUFFER_BIT,
}

#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum GLError{
    /// No error has been recorded.
    /// The value of this symbolic constant is guaranteed to be 0.
    NoError=NO_ERROR,

    /// An unacceptable value is specified for an enumerated argument.
    /// The offending command is ignored and has no other side effect than to set the error flag.
    InvalidEnum=INVALID_ENUM,

    /// A numeric argument is out of range.
    /// The offending command is ignored and has no other side effect than to set the error flag.
    InvalidValue=INVALID_VALUE,

    /// The specified operation is not allowed in the current state.
    /// The offending command is ignored and has no other side effect than to set the error flag.
    InvalidOperation=INVALID_OPERATION,

    /// The framebuffer object is not complete.
    /// The offending command is ignored and has no other side effect than to set the error flag.
    InvalidFramebufferOperation=INVALID_FRAMEBUFFER_OPERATION,

    /// There is not enough memory left to execute the command.
    /// The state of the GL is undefined, except for the state of the error flags, after this error is recorded.
    OutOfMemory=OUT_OF_MEMORY,
}

impl GLError{
    pub fn no_error(&self)->bool{
        *self==GLError::NoError
    }

    pub fn is_error(&self)->bool{
        *self!=GLError::NoError
    }

    pub fn unwrap(&self){
        if self.is_error(){
            panic!("{:?}",self)
        }
    }

    pub fn expect(&self,message:&str){
        if self.is_error(){
            panic!("{:?}, Message: {}",self,message)
        }
    }
}

pub struct GraphicsCore{
    pub parameters:GraphicsParameters,
    pub buffer:Buffer,
    pub drawing:Drawing,
    pub framebuffer:Framebuffer,
    pub program:Program,
    pub shader:Shader,
    pub texture:Texture,
    pub uniform:Uniform,
    pub vertex_array:VertexArray,

    glGetIntegerv:usize,
    glGetFloatv:usize,
    glGetString:usize,

    glClear:usize,

    glGetError:usize,

    glDrawBuffer:usize,

    glFinish:usize,
    glFlush:usize,
}

impl GraphicsCore{
    pub const fn new()->GraphicsCore{
        Self{
            parameters:GraphicsParameters::new(),
            buffer:Buffer::new(),
            drawing:Drawing::new(),
            framebuffer:Framebuffer::new(),
            program:Program::new(),
            shader:Shader::new(),
            texture:Texture::new(),
            uniform:Uniform::new(),
            vertex_array:VertexArray::new(),

            glGetIntegerv:0,
            glGetFloatv:0,
            glGetString:0,

            glClear:0,

            glGetError:0,

            glDrawBuffer:0,

            glFinish:0,
            glFlush:0,
        }
    }

    #[cfg(all(target_os="windows",feature="windows"))]
    pub fn load_functions(&mut self,library:&OpenGraphicsLibrary){
        self.parameters.load(library);
        self.buffer.load(library);
        self.drawing.load(library);
        self.framebuffer.load(library);
        self.program.load(library);
        self.shader.load(library);
        self.texture.load(library);
        self.uniform.load(library);
        self.vertex_array.load(library);

        unsafe{
            self.glGetIntegerv=transmute(library.get_proc_address("glGetIntegerv\0"));
            self.glGetFloatv=transmute(library.get_proc_address("glGetFloatv\0"));
            self.glGetString=transmute(library.get_proc_address("glGetString\0"));

            self.glClear=transmute(library.get_proc_address("glClear\0"));

            self.glGetError=transmute(library.get_proc_address("glGetError\0"));

            self.glFinish=transmute(library.get_proc_address("glFinish\0"));
            self.glFlush=transmute(library.get_proc_address("glFlush\0"));
        }
    }
}

impl GraphicsCore{
    #[inline(always)]
    pub fn get_error(&self)->GLError{
        unsafe{
            transmute::<usize,fn()->GLError>(self.glGetError)()
        }
    }

    /// Blocks the current thread until all GL execution is complete.
    /// 
    /// Does not return until the effects of all previously called GL commands are complete.
    /// Such effects include all changes to GL state,
    /// all changes to connection state,
    /// and all changes to the frame buffer contents.
    /// 
    /// Requires a round trip to the server.
    #[inline(always)]
    pub fn finish(&self){
        unsafe{
            transmute::<usize,fn()>(self.glFinish)()
        }
    }

    /// Force execution of GL commands in finite time.
    /// 
    /// Different GL implementations buffer commands in several different locations,
    /// including network buffers and the graphics accelerator itself.
    /// `GraphicsCore::flush` empties all of these buffers,
    /// causing all issued commands to be executed as quickly
    /// as they are accepted by the actual rendering engine.
    /// Though this execution may not be completed in any particular time period,
    /// it does complete in finite time.
    /// 
    /// Because any GL program might be executed over a network,
    /// or on an accelerator that buffers commands,
    /// all programs should call `GraphicsCore::flush`
    /// whenever they count on having all of their previously issued commands completed.
    /// For example, call `GraphicsCore::flush` before waiting for user input that depends on the generated image.
    /// 
    /// `GraphicsCore::flush` can return at any time.
    /// It does not wait until the execution of all previously issued GL commands is complete.
    #[inline(always)]
    pub fn flush(&self){
        unsafe{
            transmute::<usize,fn()>(self.glFlush)()
        }
    }
}

impl GraphicsCore{
    #[inline(always)]
    pub unsafe fn get_bool_v(&self,parameter:u32,value:&mut bool){
        transmute::<usize,fn(u32,&mut bool)>(self.glGetIntegerv)(parameter,value)
    }

    #[inline(always)]
    pub unsafe fn get_integer_v(&self,parameter:u32,value:&mut i32){
        transmute::<usize,fn(u32,&mut i32)>(self.glGetIntegerv)(parameter,value)
    }

    #[inline(always)]
    pub unsafe fn get_float_v(&self,parameter:u32,value:&mut f32){
        transmute::<usize,fn(u32,&mut f32)>(self.glGetFloatv)(parameter,value)
    }

    // GL_VENDOR, GL_RENDERER, GL_VERSION, or GL_SHADING_LANGUAGE_VERSION,
    #[inline(always)]
    pub unsafe fn get_string(&self,connection:u32)->&CStr{
        CStr::from_ptr(transmute::<usize,fn(u32)->*const i8>(self.glGetString)(connection))
    }
}

/// Versions.
impl GraphicsCore{
    #[inline(always)]
    pub fn get_major_version(&self)->i32{
        unsafe{
            let mut version=0i32;
            self.get_integer_v(MAJOR_VERSION,&mut version);
            version
        }
    }

    #[inline(always)]
    pub fn get_minor_version(&self)->i32{
        unsafe{
            let mut version=0i32;
            self.get_integer_v(MINOR_VERSION,&mut version);
            version
        }
    }

    #[inline(always)]
    pub fn get_version_string(&self)->&CStr{
        unsafe{
            self.get_string(VERSION)
        }
    }
}

impl GraphicsCore{
    #[inline(always)]
    pub fn clear(&self,mask:ClearMask){
        unsafe{
            transmute::<usize,fn(ClearMask)>(self.glClear)(mask)
        }
    }
}