#[cfg(any(windows))]
use crate::windows::OpenGraphicsLibrary;

pub mod blend;
use blend::Blend;

pub mod viewport;
use viewport::Viewport;

use core::mem::transmute;

// Capabilities
const BLEND:u32=0x0BE2;
// pub const CLIP_DISTANCE0:u32=0x3000;
// pub const CLIP_DISTANCE1:u32=0x3001;
// pub const CLIP_DISTANCE2:u32=0x3002;
// pub const CLIP_DISTANCE3:u32=0x3003;
// pub const CLIP_DISTANCE4:u32=0x3004;
// pub const CLIP_DISTANCE5:u32=0x3005;
// pub const CLIP_DISTANCE6:u32=0x3006;
// pub const CLIP_DISTANCE7:u32=0x3007;
// pub const COLOR_LOGIC_OP:u32=0x0BF2;
// pub const CULL_FACE:u32=0x0B44;
const LINE_SMOOTH:u32=0x0B20;

// Draw buffer modes
const NONE:u32=0;
const FRONT_LEFT:u32=0x0400;
const FRONT_RIGHT:u32=0x0401;
const BACK_LEFT:u32=0x0402;
const BACK_RIGHT:u32=0x0403;
const FRONT:u32=0x0404;
const BACK:u32=0x0405;
const LEFT:u32=0x0406;
const RIGHT:u32=0x0407;
const FRONT_AND_BACK:u32=0x0408;

// Pixel storage parameters
pub const UNPACK_ALIGNMENT:u32=0x0CF5;

#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum GLCapability{
    /// If enabled, blend the computed fragment colour values with the values in the colour buffers.
    /// 
    /// See `Blend::set_function`.
    Blend=BLEND,

    // /// If enabled, clip geometry against user-defined half space i.
    // ClipDistance0=CLIP_DISTANCE0,
    // ClipDistance1=CLIP_DISTANCE1,
    // ClipDistance2=CLIP_DISTANCE2,
    // ClipDistance3=CLIP_DISTANCE3,
    // ClipDistance4=CLIP_DISTANCE4,
    // ClipDistance5=CLIP_DISTANCE5,
    // ClipDistance6=CLIP_DISTANCE6,
    // ClipDistance7=CLIP_DISTANCE7,

    // /// If enabled, apply the currently selected logical operation to the computed fragment colour and colour buffer values.
    // /// See `glLogicOp`.
    // ColourLogicOperation=COLOR_LOGIC_OP,

    // /// If enabled, cull polygons based on their winding in window coordinates.
    // /// See `glCullFace`.
    // CullFace=CULL_FACE,

    // /// If enabled, the −wc≤zc≤wc plane equation is ignored by view volume clipping (effectively, there is no near or far plane clipping).
    // /// See glDepthRange.
    // GL_DEPTH_CLAMP,

    // /// If enabled, do depth comparisons and update the depth buffer.
    // /// Note that even if the depth buffer exists and the depth mask is non-zero,
    // /// the depth buffer is not updated if the depth test is disabled.
    // /// See glDepthFunc and glDepthRange.
    // GL_DEPTH_TEST,

    // /// If enabled, dither colour components or indices before they are written to the colour buffer.
    // GL_DITHER,

    // /// If enabled and the value of GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING for the framebuffer attachment
    // /// corresponding to the destination buffer is GL_SRGB, the R, G, and B destination colour values (after conversion from fixed-point to floating-point)
    // /// are considered to be encoded for the sRGB colour space and hence are linearized prior to their use in blending.,
    // GL_FRAMEBUFFER_SRGB,

    /// If enabled, draw lines with correct filtering.
    /// Otherwise, draw aliased lines.
    /// See `GraphicsParameters::glLineWidth`.
    LineSmooth=LINE_SMOOTH,

    // /// If enabled, use multiple fragment samples in computing the final colour of a pixel.
    // /// See glSampleCoverage.
    // GL_MULTISAMPLE,

    // /// If enabled, and if the polygon is rendered in GL_FILL mode,
    // /// an offset is added to depth values of a polygon's fragments before the depth comparison is performed.
    // /// See glPolygonOffset.
    // GL_POLYGON_OFFSET_FILL,

    // /// If enabled, and if the polygon is rendered in GL_LINE mode,
    // /// an offset is added to depth values of a polygon's fragments before the depth comparison is performed.
    // /// See glPolygonOffset.
    // GL_POLYGON_OFFSET_LINE,

    // /// If enabled, an offset is added to depth values of a polygon's fragments before the depth comparison is performed,
    // /// if the polygon is rendered in GL_POINT mode.
    // /// See glPolygonOffset.,
    // GL_POLYGON_OFFSET_POINT,

    // /// If enabled, draw polygons with proper filtering.
    // /// Otherwise, draw aliased polygons.
    // /// For correct antialiased polygons, an alpha buffer is needed and the polygons must be sorted front to back.
    // GL_POLYGON_SMOOTH,

    // /// Enables primitive restarting.
    // /// If enabled, any one of the draw commands which transfers a set of generic attribute array elements to the GL will restart the primitive
    // /// when the index of the vertex is equal to the primitive restart index.
    // /// See glPrimitiveRestartIndex.
    // /// 
    // ///GL version is 3.1 or greater.
    // GL_PRIMITIVE_RESTART,

    // /// If enabled, compute a temporary coverage value where each bit is determined by the alpha value at the corresponding sample location.
    // /// The temporary coverage value is then ANDed with the fragment coverage value.
    // GL_SAMPLE_ALPHA_TO_COVERAGE,

    // /// If enabled, each sample alpha value is replaced by the maximum representable alpha value.
    // GL_SAMPLE_ALPHA_TO_ONE,

    // /// If enabled, the fragment's coverage is ANDed with the temporary coverage value.
    // /// If GL_SAMPLE_COVERAGE_INVERT is set to GL_TRUE, invert the coverage value.
    // /// See glSampleCoverage.,
    // GL_SAMPLE_COVERAGE,

    // /// If enabled, discard fragments that are outside the scissor rectangle.
    // /// See glScissor.
    // GL_SCISSOR_TEST,

    // /// If enabled, do stencil testing and update the stencil buffer.
    // /// See glStencilFunc and glStencilOp.
    // GL_STENCIL_TEST,

    // /// If enabled, modifies the way sampling is performed on cube map textures.
    // /// See the spec for more information.
    // GL_TEXTURE_CUBE_MAP_SEAMLESS,

    // /// If enabled and a vertex or geometry shader is active,
    // /// then the derived point size is taken from the (potentially clipped) shader builtin gl_PointSize
    // /// and clamped to the implementation-dependent point size range.
    // GL_PROGRAM_POINT_SIZE,
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum DrawBufferMode{
    /// No color buffers are written.
    None=NONE,

    /// Only the front left color buffer is written.
    FrontLeft=FRONT_LEFT,

    /// Only the front right color buffer is written.
    FrontRight=FRONT_RIGHT,

    /// Only the back left color buffer is written.
    BackLeft=BACK_LEFT,

    /// Only the back right color buffer is written.
    BackRight=BACK_RIGHT,

    /// Only the front left and front right color buffers are written.
    /// If there is no front right color buffer, only the front left color buffer is written.
    Front=FRONT,

    /// Only the back left and back right color buffers are written.
    /// If there is no back right color buffer, only the back left color buffer is written.
    Back=BACK,

    /// Only the front left and back left color buffers are written.
    /// If there is no back left color buffer, only the front left color buffer is written.
    Left=LEFT,

    /// Only the front right and back right color buffers are written.
    /// If there is no back right color buffer, only the front right color buffer is written.
    Right=RIGHT,

    /// All the front and back color buffers (front left, front right, back left, back right) are written.
    /// If there are no back color buffers, only the front left and front right color buffers are written.
    /// If there are no right color buffers, only the front left and back left color buffers are written.
    /// If there are no right or back color buffers, only the front left color buffer is written.
    FrontBack=FRONT_AND_BACK
}

pub struct GraphicsParameters{
    pub blend:Blend,
    pub viewport:Viewport,

    glEnable:usize,
    glDisable:usize,
    glIsEnabled:usize,

    glPixelStoref:usize,
    glPixelStorei:usize,

    glLineWidth:usize,

//     glClampColor:usize,

//     glScissor:usize,

//     glCullFace:usize,

//     glDepthFunc:usize,
//     glDepthMask:usize,
//     glDepthRange:usize,

    glClearColor:usize,

    glDrawBuffer:usize,
}

impl GraphicsParameters{
    pub const fn new()->GraphicsParameters{
        Self{
            blend:Blend::new(),
            viewport:Viewport::new(),

            glEnable:0,
            glDisable:0,
            glIsEnabled:0,

            glPixelStoref:0,
            glPixelStorei:0,

            glLineWidth:0,

            // glClampColor:0,

            // glScissor:0,

            // glCullFace:0,

            // glDepthFunc:0,
            // glDepthMask:0,
            // glDepthRange:0,

            glClearColor:0,

            glDrawBuffer:0,
        }
    }

    #[cfg(any(windows))]
    pub fn load(&mut self,library:&OpenGraphicsLibrary){
        self.blend.load(library);
        self.viewport.load(library);

        unsafe{
            self.glEnable=transmute(library.get_proc_address("glEnable\0"));
            self.glDisable=transmute(library.get_proc_address("glDisable\0"));
            self.glIsEnabled=transmute(library.get_proc_address("glIsEnabled\0"));

            self.glPixelStoref=transmute(library.get_proc_address("glPixelStoref\0"));
            self.glPixelStorei=transmute(library.get_proc_address("glPixelStorei\0"));

            self.glLineWidth=transmute(library.get_proc_address("glLineWidth\0"));

            // self.glClampColor=transmute(library.get_proc_address("glClampColor\0"));

            // self.glScissor=transmute(library.get_proc_address("glScissor\0"));

            // self.glCullFace=transmute(library.get_proc_address("glCullFace\0"));

            // self.glDepthFunc=transmute(library.get_proc_address("glDepthFunc\0"));
            // self.glDepthMask=transmute(library.get_proc_address("glDepthMask\0"));
            // self.glDepthRange=transmute(library.get_proc_address("glDepthRange\0"));

            self.glClearColor=transmute(library.get_proc_address("glClearColor\0"));

            self.glDrawBuffer=transmute(library.get_proc_address("glDrawBuffer\0"));
        }
    }
}

impl GraphicsParameters{
    /// Enables server-side GL capabilities.
    #[inline(always)]
    pub fn enable(&self,capability:GLCapability){
        unsafe{
            transmute::<usize,fn(GLCapability)>(self.glEnable)(capability)
        }
    }

    /// Disables server-side GL capabilities.
    #[inline(always)]
    pub fn disable(&self,capability:GLCapability){
        unsafe{
            transmute::<usize,fn(GLCapability)>(self.glDisable)(capability)
        }
    }

    /// Tests whether a capability is enabled
    #[inline(always)]
    pub fn is_enabled(&self,capability:GLCapability)->bool{
        unsafe{
            transmute::<usize,fn(GLCapability)->bool>(self.glIsEnabled)(capability)
        }
    }
}

impl GraphicsParameters{
    #[inline(always)]
    pub unsafe fn set_pixel_storage_modei(&self,parameter:u32,value:i32){
        transmute::<usize,fn(u32,i32)>(self.glPixelStorei)(parameter,value)
    }

    #[inline(always)]
    pub unsafe fn set_pixel_storage_modef(&self,parameter:u32,value:f32){
        transmute::<usize,fn(u32,f32)>(self.glPixelStoref)(parameter,value)
    }
}

impl GraphicsParameters{
    /// Specifies the width of rasterized lines.
    /// 
    /// The initial value is `1f32`.
    /// 
    /// Specifies the rasterized width of both aliased and antialiased lines.
    /// Using a line width other than 1 has different effects,
    /// depending on whether line antialiasing is enabled.
    /// To enable and disable line antialiasing,
    /// call `glEnable` and `glDisable` with argument `GL_LINE_SMOOTH`.
    /// Line antialiasing is initially disabled.
    /// 
    /// If line antialiasing is disabled,
    /// the actual width is determined by rounding the supplied width to the nearest integer.
    /// (If the rounding results in the value 0, it is as if the line width were 1.)
    /// If `∣Δx∣>=∣Δy∣`, i pixels are filled in each column that is rasterized,
    /// where i is the rounded value of width.
    /// Otherwise, i pixels are filled in each row that is rasterized.
    /// 
    /// If antialiasing is enabled, line rasterization produces a fragment for each pixel square
    /// that intersects the region lying within the rectangle having width
    /// equal to the current line width, length equal to the actual length of the line,
    /// and centered on the mathematical line segment.
    /// The coverage value for each fragment is the window coordinate area
    /// of the intersection of the rectangular region with the corresponding pixel square.
    /// This value is saved and used in the final rasterization step.
    /// 
    /// Not all widths can be supported when line antialiasing is enabled.
    /// If an unsupported width is requested, the nearest supported width is used.
    /// Only width 1 is guaranteed to be supported; others depend on the implementation.
    /// Likewise, there is a range for aliased line widths as well.
    /// To query the range of supported widths and the size difference between supported widths
    /// within the range, call `glGet` with arguments `GL_ALIASED_LINE_WIDTH_RANGE`,
    /// `GL_SMOOTH_LINE_WIDTH_RANGE`, and `GL_SMOOTH_LINE_WIDTH_GRANULARITY`.
    /// 
    /// The line width specified by `glLineWidth` is always returned when `GL_LINE_WIDTH` is queried.
    /// Clamping and rounding for aliased and antialiased lines have no effect on the specified value.
    /// 
    /// Nonantialiased line width may be clamped to an implementation-dependent maximum.
    /// Call glGet with `GL_ALIASED_LINE_WIDTH_RANGE` to determine the maximum width.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if `width` is less than or equal to `0f32`.
    pub unsafe fn set_line_width(&self,width:f32){
        transmute::<usize,fn(f32)>(self.glLineWidth)(width)
    }
}

impl GraphicsParameters{
    /// Specifies clear values for the colour buffers.
    #[inline(always)]
    pub fn set_clear_colour(&self,[red,greed,blue,alpha]:[f32;4]){
        unsafe{
            transmute::<usize,fn(f32,f32,f32,f32)>(self.glClearColor)(red,greed,blue,alpha)
        }
    }

    /// Specify which color buffers are to be drawn into.
    /// 
    /// When colors are written to the frame buffer,
    /// they are written into the color buffers specified by `GraphicsParameters::draw_buffer`.
    /// 
    /// If more than one color buffer is selected for drawing,
    /// then blending or logical operations are computed
    /// and applied independently for each color bufferand can produce different results in each buffer.
    /// 
    /// Monoscopic contexts include only left buffers,
    /// and stereoscopic contexts include both left and right buffers.
    /// Likewise, single-buffered contexts include only front buffers,
    /// and double-buffered contexts include both front and back buffers.
    /// The context is selected at GL initialization.
    /// 
    /// The initial value is `DrawBufferMode::Front` for single-buffered contexts,
    /// and `DrawBufferMode::Back` for double-buffered contexts.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if none of the buffers indicated by `mode` exists.
    #[inline(always)]
    pub fn draw_buffer(&self,mode:DrawBufferMode){
        unsafe{
            transmute::<usize,fn(DrawBufferMode)>(self.glDrawBuffer)(mode)
        }
    }
}