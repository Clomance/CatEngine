pub mod level0;

pub mod level1;

pub mod level2;

pub use gl;

pub type ColourComponent=f32;
pub type Colour=[ColourComponent;4];

use gl::{
    // Primitive types
    POINTS,
    LINES,
    LINE_LOOP,
    LINE_STRIP,
    TRIANGLES,
    TRIANGLE_STRIP,
    TRIANGLE_FAN,
    LINES_ADJACENCY,
    TRIANGLES_ADJACENCY,
    TRIANGLE_STRIP_ADJACENCY,
    COLOR_BUFFER_BIT,
    STENCIL_BUFFER_BIT,
    DEPTH_BUFFER_BIT,

    // Errors
    NO_ERROR,
    INVALID_ENUM,
    INVALID_VALUE,
    INVALID_OPERATION,
    INVALID_FRAMEBUFFER_OPERATION,
    OUT_OF_MEMORY,

    // functions
    GetError
};

use std::mem::transmute;

#[repr(u32)]
#[derive(Clone,Copy)]
pub enum PrimitiveType{
    Points=POINTS,
    Lines=LINES,
    LineLoop=LINE_LOOP,
    LineStrip=LINE_STRIP,
    Triangles=TRIANGLES,
    TriangleStrip=TRIANGLE_STRIP,
    TriangleFan=TRIANGLE_FAN,
    LinesAdjacency=LINES_ADJACENCY,
    TrianglesAdjacency=TRIANGLES_ADJACENCY,
    TriangleStripAdjacency=TRIANGLE_STRIP_ADJACENCY,
}

#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum GLError{
    NoError=NO_ERROR,
    InvalidEnum=INVALID_ENUM,
    InvalidValue=INVALID_VALUE,
    InvalidOperation=INVALID_OPERATION,
    InvalidFramebufferOperation=INVALID_FRAMEBUFFER_OPERATION,
    OutOfMemory=OUT_OF_MEMORY,
}

impl GLError{
    /// Returns a error.
    pub fn get_error()->GLError{
        unsafe{
            transmute(GetError())
        }
    }
}