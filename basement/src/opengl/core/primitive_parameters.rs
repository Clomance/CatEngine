#[cfg(target_os="windows")]
use crate::winapi::OpenGraphicsLibrary;

use super::{
    types::*,
    constants::*
};

use core::mem::transmute;

#[cfg(target_os="linux")]
extern "system"{
    fn glFrontFace(mode:GLenum);

    fn glLineWidth(width:GLfloat);

    fn glPointSize(size:GLfloat);

    fn glPointParameterf(parameter:GLenum,value:GLfloat);
    fn glPointParameteri(parameter:GLenum,value:GLint);
    fn glPointParameterfv(parameter:GLenum,values:*const GLfloat);
    fn glPointParameteriv(parameter:GLenum,values:*const GLint);
}

#[cfg(target_os="windows")]
mod gl{
    pub static mut glFrontFace:usize=0;

    pub static mut glLineWidth:usize=0;

    pub static mut glPointSize:usize=0;

    pub static mut glPointParameterf:usize=0;
    pub static mut glPointParameteri:usize=0;
    pub static mut glPointParameterfv:usize=0;
    pub static mut glPointParameteriv:usize=0;
}

#[cfg(target_os="windows")]
mod gl_functions{
    use super::*;

    pub unsafe fn glFrontFace(mode:GLenum){
        transmute::<usize,extern "system" fn(GLenum)>(gl::glFrontFace)(mode)
    }

    pub unsafe fn glLineWidth(width:GLfloat){
        transmute::<usize,extern "system" fn(GLfloat)>(gl::glLineWidth)(width)
    }

    pub unsafe fn glPointSize(size:GLfloat){
        transmute::<usize,extern "system" fn(GLfloat)>(gl::glPointSize)(size)
    }

    pub unsafe fn glPointParameterf(parameter:GLenum,value:GLfloat){
        transmute::<usize,extern "system" fn(GLenum,GLfloat)>(gl::glPointParameterf)(parameter,value)
    }
    pub unsafe fn glPointParameteri(parameter:GLenum,value:GLint){
        transmute::<usize,extern "system" fn(GLenum,GLint)>(gl::glPointParameteri)(parameter,value)
    }
    pub unsafe fn glPointParameterfv(parameter:GLenum,values:*const GLfloat){
        transmute::<usize,extern "system" fn(GLenum,*const GLfloat)>(gl::glPointParameterfv)(parameter,values)
    }
    pub unsafe fn glPointParameteriv(parameter:GLenum,values:*const GLint){
        transmute::<usize,extern "system" fn(GLenum,*const GLint)>(gl::glPointParameteriv)(parameter,values)
    }
}

#[cfg(target_os="windows")]
use gl_functions::*;

pub struct PrimitiveParameters;

impl PrimitiveParameters{
    #[cfg(target_os="windows")]
    pub fn load(library:&OpenGraphicsLibrary){
        unsafe{
            use gl::*;

            glFrontFace=transmute(library.get_proc_address("glFrontFace\0"));

            glLineWidth=transmute(library.get_proc_address("glLineWidth\0"));

            glPointSize=transmute(library.get_proc_address("glPointSize\0"));

            glPointParameterf=transmute(library.get_proc_address("glPointParameterf\0"));
            glPointParameteri=transmute(library.get_proc_address("glPointParameteri\0"));
            glPointParameterfv=transmute(library.get_proc_address("glPointParameterfv\0"));
            glPointParameteriv=transmute(library.get_proc_address("glPointParameteriv\0"));
        }
    }
}

#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum FrontFaceMode{
    Clockwise=CW,
    Counterclockwise=CCW,
}

impl PrimitiveParameters{
    /// Defines front- and back-facing polygons.
    /// 
    /// `mode` specifies the orientation of front-facing polygons.
    /// 
    /// The initial value is `FrontFaceMode::Counterclockwise`.
    /// 
    /// In a scene composed entirely of opaque closed surfaces, back-facing polygons are never visible.
    /// Eliminating these invisible polygons has the obvious benefit of speeding up the rendering of the image.
    /// To enable and disable elimination of back-facing polygons,
    /// call `Core::enable` and `Core::disable` with argument `CULL_FACE`.
    /// 
    /// The projection of a polygon to window coordinates is said to have clockwise winding
    /// if an imaginary object following the path from its first vertex, its second vertex,
    /// and so on, to its last vertex, and finally back to its first vertex, moves in a clockwise direction about the interior of the polygon.
    /// The polygon's winding is said to be counterclockwise
    /// if the imaginary object following the same path moves in a counterclockwise direction about the interior of the polygon.
    /// `PrimitiveParameters::set_front_face` specifies whether polygons with clockwise winding in window coordinates,
    /// or counterclockwise winding in window coordinates, are taken to be front-facing.
    /// Passing `FrontFaceMode::Counterclockwise` to `mode` selects counterclockwise polygons as front-facing;
    /// `FrontFaceMode::Clockwise` selects clockwise polygons as front-facing.
    /// By default, counterclockwise polygons are taken to be front-facing.
    /// 
    /// `Error::InvalidEnum` is generated if `mode` is not an accepted value.
    pub unsafe fn set_front_face(mode:FrontFaceMode){
        glFrontFace(mode as GLenum)
    }

    /// Specifies the width of rasterized lines.
    /// 
    /// `width` specifies the width of rasterized lines.
    /// 
    /// The initial value is 1.
    /// 
    /// Specifies the rasterized width of both aliased and antialiased lines.
    /// Using a line width other than 1 has different effects, depending on whether line antialiasing is enabled.
    /// To enable and disable line antialiasing, call `Core::enable` and `Core::disable` with argument `LINE_SMOOTH`.
    /// Line antialiasing is initially disabled.
    /// 
    /// If line antialiasing is disabled, the actual width is determined by rounding the supplied width to the nearest integer.
    /// (If the rounding results in the value 0, it is as if the line width were 1.)
    /// If `∣Δx∣>=∣Δy∣`, `i` pixels are filled in each column that is rasterized, where `i` is the rounded value of width.
    /// Otherwise, `i` pixels are filled in each row that is rasterized.
    /// 
    /// If antialiasing is enabled, line rasterization produces a fragment for each pixel square
    /// that intersects the region lying within the rectangle having width equal to the current line width,
    /// length equal to the actual length of the line, and centered on the mathematical line segment.
    /// The coverage value for each fragment is the window coordinate area of the intersection of the rectangular region with the corresponding pixel square.
    /// This value is saved and used in the final rasterization step.
    /// 
    /// Not all widths can be supported when line antialiasing is enabled.
    /// If an unsupported width is requested, the nearest supported width is used.
    /// Only width 1 is guaranteed to be supported; others depend on the implementation.
    /// Likewise, there is a range for aliased line widths as well.
    /// To query the range of supported widths and the size difference between supported widths within the range,
    /// call `ParameterInfo::get` with arguments `ALIASED_LINE_WIDTH_RANGE`, `SMOOTH_LINE_WIDTH_RANGE`, and `SMOOTH_LINE_WIDTH_GRANULARITY`.
    /// 
    /// The line width specified by `PrimitiveParameters::set_line_width` is always returned when `LINE_WIDTH` is queried.
    /// Clamping and rounding for aliased and antialiased lines have no effect on the specified value.
    /// 
    /// Nonantialiased line width may be clamped to an implementation-dependent maximum.
    /// Call `ParameterInfo::get` with `ALIASED_LINE_WIDTH_RANGE` to determine the maximum width.
    /// 
    /// In OpenGL 1.2, the tokens GL_LINE_WIDTH_RANGE and GL_LINE_WIDTH_GRANULARITY were replaced
    /// by GL_ALIASED_LINE_WIDTH_RANGE, GL_SMOOTH_LINE_WIDTH_RANGE, and GL_SMOOTH_LINE_WIDTH_GRANULARITY.
    /// The old names are retained for backward compatibility, but should not be used in new code.
    /// 
    /// `Error::InvalidValue` is generated if `width` is less than or equal to 0.
    pub unsafe fn set_line_width(width:f32){
        glLineWidth(width)
    }

    /// Specifies the diameter of rasterized points.
    /// 
    /// `size` specifies the diameter of rasterized points.
    /// The initial value is 1.
    /// 
    /// Specifies the rasterized diameter of points.
    /// If point size mode is disabled (see `Core::Enable` with parameter `PROGRAM_POINT_SIZE`),
    /// this value will be used to rasterize points.
    /// Otherwise, the value written to the shading language built-in variable `gl_PointSize` will be used.
    /// 
    /// The point size specified by `PrimitiveParameters::set_point_size` is always returned when `POINT_SIZE` is queried.
    /// Clamping and rounding for points have no effect on the specified value.
    /// 
    /// `Error::InvalidValue` is generated if `size` is less than or equal to 0.
    pub unsafe fn set_point_size(size:GLfloat){
        glPointSize(size)
    }
}

#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum PointParameter{
    /// A single floating-point value that specifies the threshold value to which point sizes are clamped if they exceed the specified value.
    /// The default value is `1.0`.
    PointFadeThresholdSize=POINT_FADE_THRESHOLD_SIZE,

    /// A single enum specifying the point sprite texture coordinate origin (`SpriteCoordinateOrigin`).
    /// 
    /// The default value is `SpriteCoordinateOrigin::UpperLeft`.
    PointSpriteCoordinateOrigin=POINT_SPRITE_COORD_ORIGIN
}

#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum SpriteCoordinateOrigin{
    LowerLeft=LOWER_LEFT,
    UpperLeft=UPPER_LEFT
}

/// Specifies point parameters.
/// 
/// `parameter` specifies a single-valued point parameter.
/// 
/// `value`/`values` specifies the value that `parameter` will be set to.
/// 
/// `Error::InvalidValue` is generated
/// if the value specified for `PointParameter::PointFadeThresholdSize` is less than zero.
/// 
/// `Error::InvalidEnum` is generated
/// If the value specified for `PointParameter::PointSpriteCoordinateOrigin` is not `SpriteCoordinateOrigin`.
impl PrimitiveParameters{
    pub unsafe fn set_point_parameterf(parameter:PointParameter,value:f32){
        glPointParameterf(parameter as GLenum,value)
    }
    pub unsafe fn set_point_parameteri(parameter:PointParameter,value:i32){
        glPointParameteri(parameter as GLenum,value)
    }
    pub unsafe fn set_point_parameterfv(parameter:PointParameter,values:*const f32){
        glPointParameterfv(parameter as GLenum,values)
    }
    pub unsafe fn set_point_parameteriv(parameter:PointParameter,values:*const i32){
        glPointParameteriv(parameter as GLenum,values)
    }
}