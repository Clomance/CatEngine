#[cfg(any(windows))]
use crate::windows::OpenGraphicsLibrary;

use core::mem::transmute;

const READ_FRAMEBUFFER:u32=0x8CA8;
const DRAW_FRAMEBUFFER:u32=0x8CA9;
const FRAMEBUFFER:u32=0x8D40;

const COLOR_ATTACHMENT0:u32=0x8CE0;
const DEPTH_ATTACHMENT:u32=0x8D00;
const STENCIL_ATTACHMENT:u32=0x8D20;
const DEPTH_STENCIL_ATTACHMENT:u32=0x821A;

const RENDERBUFFER:u32=0x8D41;

const COLOR_BUFFER_BIT:u32=0x00004000;
const DEPTH_BUFFER_BIT:u32=0x00000100;
const STENCIL_BUFFER_BIT:u32=0x00000400;

const NEAREST:u32=0x2600;
const LINEAR:u32=0x2601;

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum FramebufferTarget{
    Read=READ_FRAMEBUFFER,
    Draw=DRAW_FRAMEBUFFER,
    ReadDraw=FRAMEBUFFER,
}

#[derive(Clone,Copy,Debug)]
pub struct FramebufferAttachement{
    attachement:u32
}

impl FramebufferAttachement{
    /// `attachment` must be in the range zero to the value of GL_MAX_COLOR_ATTACHMENTS - 1
    pub const fn colour(attachment:u32)->FramebufferAttachement{
        Self{
            attachement:COLOR_ATTACHMENT0+attachment
        }
    }

    pub const fn depth()->FramebufferAttachement{
        Self{
            attachement:DEPTH_ATTACHMENT
        }
    }

    pub const fn stencil()->FramebufferAttachement{
        Self{
            attachement:STENCIL_ATTACHMENT
        }
    }

    pub const fn depth_stencil()->FramebufferAttachement{
        Self{
            attachement:DEPTH_STENCIL_ATTACHMENT
        }
    }
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum BlitMask{
    Colour=COLOR_BUFFER_BIT,
    Depth=DEPTH_BUFFER_BIT,
    Stencil=STENCIL_BUFFER_BIT,
    ColourDepth=COLOR_BUFFER_BIT|DEPTH_BUFFER_BIT,
    ColourStencil=COLOR_BUFFER_BIT|STENCIL_BUFFER_BIT,
    DepthStencil=DEPTH_BUFFER_BIT|STENCIL_BUFFER_BIT,
    ColourDepthStencil=COLOR_BUFFER_BIT|DEPTH_BUFFER_BIT|STENCIL_BUFFER_BIT
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum FramebufferFilter{
    /// Returns the value of the texture element
    /// that is nearest (in Manhattan distance) to the specified texture coordinates.
    Nearest=NEAREST,

    /// Returns the weighted average of the four texture elements that are closest to the specified texture coordinates.
    /// These can include items wrapped or repeated from other parts of a texture,
    /// depending on the values of `GL_TEXTURE_WRAP_S` and `GL_TEXTURE_WRAP_T`,and on the exact mapping.
    Linear=LINEAR,
}

pub struct Framebuffer{
    glGenFramebuffers:usize,
    glDeleteFramebuffers:usize,

    glBindFramebuffer:usize,

    glFramebufferTexture:usize,
    glFramebufferTextureLayer:usize,
    glFramebufferTexture1D:usize,
    glFramebufferTexture2D:usize,
    glFramebufferTexture3D:usize,

    glFramebufferRenderbuffer:usize,

    glBlitFramebuffer:usize,

    glCheckFramebufferStatus:usize,
    glGetFramebufferAttachmentParameteriv:usize,
    glIsFramebuffer:usize,
}

impl Framebuffer{
    pub const fn new()->Framebuffer{
        Self{
            glGenFramebuffers:0,
            glDeleteFramebuffers:0,

            glBindFramebuffer:0,

            glFramebufferTexture:0,
            glFramebufferTextureLayer:0,
            glFramebufferTexture1D:0,
            glFramebufferTexture2D:0,
            glFramebufferTexture3D:0,

            glFramebufferRenderbuffer:0,

            glBlitFramebuffer:0,

            glCheckFramebufferStatus:0,
            glGetFramebufferAttachmentParameteriv:0,
            glIsFramebuffer:0,
        }
    }

    #[cfg(any(windows))]
    pub fn load(&mut self,library:&OpenGraphicsLibrary){
        unsafe{
            self.glGenFramebuffers=transmute(library.get_proc_address("glGenFramebuffers\0"));
            self.glDeleteFramebuffers=transmute(library.get_proc_address("glDeleteFramebuffers\0"));

            self.glBindFramebuffer=transmute(library.get_proc_address("glBindFramebuffer\0"));

            self.glFramebufferTexture=transmute(library.get_proc_address("glFramebufferTexture\0"));
            self.glFramebufferTextureLayer=transmute(library.get_proc_address("glFramebufferTextureLayer\0"));
            self.glFramebufferTexture1D=transmute(library.get_proc_address("glFramebufferTexture1D\0"));
            self.glFramebufferTexture2D=transmute(library.get_proc_address("glFramebufferTexture2D\0"));
            self.glFramebufferTexture3D=transmute(library.get_proc_address("glFramebufferTexture3D\0"));

            self.glFramebufferRenderbuffer=transmute(library.get_proc_address("glFramebufferRenderbuffer\0"));

            self.glBlitFramebuffer=transmute(library.get_proc_address("glBlitFramebuffer\0"));

            self.glCheckFramebufferStatus=transmute(library.get_proc_address("glCheckFramebufferStatus\0"));
            self.glGetFramebufferAttachmentParameteriv=transmute(library.get_proc_address("glGetFramebufferAttachmentParameteriv\0"));
            self.glIsFramebuffer=transmute(library.get_proc_address("glIsFramebuffer\0"));
        }
    }
}

impl Framebuffer{
    /// Generates a framebuffer object name.
    /// 
    /// See `Framebuffer::generate`.
    #[inline(always)]
    pub fn generate_one(&self,framebuffer:&mut u32){
        unsafe{
            transmute::<usize,fn(i32,&mut u32)>(self.glGenFramebuffers)(1,framebuffer)
        }
    }

    /// Deletes a buffer object.
    /// 
    /// See `Framebuffer::delete`.
    #[inline(always)]
    pub fn delete_one(&self,framebuffer:&u32){
        unsafe{
            transmute::<usize,fn(i32,&u32)>(self.glDeleteFramebuffers)(1,framebuffer)
        }
    }

    /// Generates framebuffer object names.
    /// 
    /// Returns `framebuffers.len()` framebuffer object names in `framebuffers`.
    /// There is no guarantee that the names form a contiguous set of integers;
    /// however, it is guaranteed that none of the returned names was in use immediately
    /// before the call to `Framebuffer::generate`.
    /// 
    /// Framebuffer object names returned by a call to `Framebuffer::generate` are not returned by subsequent calls,
    /// unless they are first deleted with glDeleteFramebuffers.
    /// 
    /// The names returned in `framebuffers` are marked as used,
    /// for the purposes of `Framebuffer::generate` only,
    /// but they acquire state and type only when they are first bound.
    /// 
    /// `GLError::InvalidValue` is generated if `framebuffers.len()` is greater than `i32::MAX`.
    #[inline(always)]
    pub fn generate(&self,framebuffers:&mut [u32]){
        unsafe{
            transmute::<usize,fn(i32,&mut [u32])>(self.glGenFramebuffers)(framebuffers.len() as i32,framebuffers)
        }
    }

    /// Deletes buffer objects.
    /// 
    /// Deletes the `framebuffers.len()` framebuffer objects
    /// whose name are stored in the array addressed by `framebuffers`.
    /// The name zero is reserved by the GL and is silently ignored,
    /// should it occur in `framebuffers`, as are other unused names.
    /// Once a framebuffer object is deleted, its name is again unused and it has no attachments.
    /// If a framebuffer that is currently bound to one or more of the targets `GL_DRAW_FRAMEBUFFER`
    /// or `GL_READ_FRAMEBUFFER` is deleted,
    /// it is as though `Framebuffer::bind` had been executed with the corresponding `target` and `framebuffer` zero.
    /// 
    /// `GLError::InvalidValue` is generated if `framebuffers.len()` is greater than `i32::MAX`.
    #[inline(always)]
    pub fn delete(&self,framebuffers:&[u32]){
        unsafe{
            transmute::<usize,fn(i32,&[u32])>(self.glDeleteFramebuffers)(framebuffers.len() as i32,framebuffers)
        }
    }

    /// Determines if a name corresponds to a framebuffer object.
    /// 
    /// Returns `true` if `framebuffer` is currently the name of a framebuffer object.
    /// If `framebuffer` is zero, or if framebuffer is not the name of a framebuffer object,
    /// or if an error occurs, `Framebuffer::is_framebuffer` returns `false`.
    /// If `framebuffer` is a name returned by `Framebuffer::generate`,
    /// by that has not yet been bound through a call to `Framebuffer::bind`,
    /// then the name is not a framebuffer object and `Framebuffer::is_framebuffer` returns `false`.
    #[inline(always)]
    pub fn is_framebuffer(&self,framebuffer:u32)->bool{
        unsafe{
            transmute::<usize,fn(u32)->bool>(self.glIsFramebuffer)(framebuffer)
        }
    }
}

impl Framebuffer{
    /// Binds a framebuffer to a framebuffer target.
    /// 
    /// Binds the framebuffer object with name `framebuffer` to the framebuffer target specified by `target`.
    /// If a framebuffer object is bound to `FramebufferTarget::Draw` or `FramebufferTarget::Read`,
    /// it becomes the target for rendering or readback operations, respectively,
    /// until it is deleted or another framebuffer is bound to the corresponding bind point.
    /// Calling `Framebuffer::bind` with target set to `Framebuffer::ReadDraw` binds `framebuffer` to both
    /// the read and draw framebuffer targets.
    /// `framebuffer` is the name of a framebuffer object
    /// previously returned from a call to `Framebuffer::generate`,
    /// or zero to break the existing binding of a framebuffer object to target.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if `buffer` is not a name previously returned from a call to `Buffer::generate`.
    #[inline(always)]
    pub unsafe fn bind(&self,target:FramebufferTarget,framebuffer:u32){
        transmute::<usize,fn(FramebufferTarget,u32)>(self.glBindFramebuffer)(target,framebuffer)
    }
}

impl Framebuffer{
    /// Attaches a level of a texture object as a logical buffer to the currently bound framebuffer object.
    /// 
    /// Attachs a selected mipmap level or image of a texture object
    /// as one of the logical buffers of the framebuffer object currently bound to `target`.
    /// 
    /// `attachment` specifies the logical attachment of the framebuffer.
    /// Attaching a level of a texture to `FramebufferAttachement::depth_stencil()` is equivalent to attaching
    /// that level to both the `FramebufferAttachement::depth()`
    /// and the `FramebufferAttachement::stencil()` attachment points simultaneously.
    /// 
    /// `texture_target` specifies what type of texture is named by `texture`,
    /// and for cube map textures, specifies the face that is to be attached.
    /// If `texture` is not zero, it must be the name of an existing texture with type `texture_target`,
    /// unless it is a cube map texture, in which case `texture_target` must be
    /// GL_TEXTURE_CUBE_MAP_POSITIVE_X GL_TEXTURE_CUBE_MAP_NEGATIVE_X, GL_TEXTURE_CUBE_MAP_POSITIVE_Y,
    /// GL_TEXTURE_CUBE_MAP_NEGATIVE_Y, GL_TEXTURE_CUBE_MAP_POSITIVE_Z, or GL_TEXTURE_CUBE_MAP_NEGATIVE_Z.
    /// 
    /// If `texture` is non-zero,
    /// the specified level of the texture object named `texture` is attached
    /// to the framebfufer attachment point named by attachment.
    /// For `glFramebufferTexture1D`, `glFramebufferTexture2D`, and `glFramebufferTexture3D`,
    /// `texture` must be zero or the name of an existing texture with a target of `texture_target`,
    /// or `texture` must be the name of an existing cube-map texture and `texture_target`
    /// must be one of GL_TEXTURE_CUBE_MAP_POSITIVE_X, GL_TEXTURE_CUBE_MAP_POSITIVE_Y,
    /// GL_TEXTURE_CUBE_MAP_POSITIVE_Z, GL_TEXTURE_CUBE_MAP_NEGATIVE_X,
    /// GL_TEXTURE_CUBE_MAP_NEGATIVE_Y, or GL_TEXTURE_CUBE_MAP_NEGATIVE_Z.
    /// 
    /// If `texture_target` is GL_TEXTURE_RECTANGLE, GL_TEXTURE_2D_MULTISAMPLE,
    /// or GL_TEXTURE_2D_MULTISAMPLE_ARRAY,then level must be zero.
    /// If `texture_target` is GL_TEXTURE_3D, then level must be greater than or equal to zero
    /// and less than or equal to log2 of the value of GL_MAX_3D_TEXTURE_SIZE.
    /// If `texture_target` is one of GL_TEXTURE_CUBE_MAP_POSITIVE_X, GL_TEXTURE_CUBE_MAP_POSITIVE_Y,
    /// GL_TEXTURE_CUBE_MAP_POSITIVE_Z, GL_TEXTURE_CUBE_MAP_NEGATIVE_X,
    /// GL_TEXTURE_CUBE_MAP_NEGATIVE_Y, or GL_TEXTURE_CUBE_MAP_NEGATIVE_Z, then level must be greater than
    /// or equal to zero and less than or equal to log2 of the value of GL_MAX_CUBE_MAP_TEXTURE_SIZE.
    /// For all other values of `texture_target`, level must be greater than
    /// or equal to zero and no larger than log2 of the value of GL_MAX_TEXTURE_SIZE.
    /// 
    /// `GLError::InvalidEnum` is generated
    /// if `attachment` is not one of the accepted tokens.
    /// 
    /// `GLError::InvalidOperation` is generated if zero is bound to `target`,
    /// if `texture_target` and `texture` are not compatible.
    /// 
    /// Available only if the GL version is 3.2 or greater.
    #[inline(always)]
    pub unsafe fn texture(
        &self,
        target:FramebufferTarget,
        attachment:FramebufferAttachement,
        texture:u32,
        mipmap_level:i32,
    ){
        transmute::<usize,fn(
            FramebufferTarget,
            FramebufferAttachement,
            u32,
            i32
        )>(self.glFramebufferTexture)(target,attachment,texture,mipmap_level)
    }

    /// Attaches a single layer of a texture to a framebuffer.
    /// 
    /// Operates like `Framebuffer::texture`, except that only a single layer of the texture level,
    /// given by layer, is attached to the attachment point.
    /// If `texture` is not zero, layer must be greater than or equal to zero.
    /// `texture` must either be zero or the name of an existing three-dimensional texture,
    /// one- or two-dimensional array texture, or multisample array texture.
    /// 
    /// `GLError::InvalidEnum` is generated
    /// if `attachment` is not one of the accepted tokens.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if `texture` is not zero or the name of an existing texture object,
    /// `texture` is not zero and layer is negative.
    /// 
    /// `GLError::InvalidOperation` is generated if zero is bound to `target`,
    /// if `texture` is not zero or the name of an existing cube map texture.
    /// 
    /// Available only if the GL version is 3.2 or greater.
    #[inline(always)]
    pub unsafe fn texture_layer(
        &self,
        target:FramebufferTarget,
        attachment:FramebufferAttachement,
        texture:u32,
        mipmap_level:i32,
        layer:i32,
    ){
        transmute::<usize,fn(
            FramebufferTarget,
            FramebufferAttachement,
            u32,
            i32,
            i32
        )>(self.glFramebufferTexture)(target,attachment,texture,mipmap_level,layer)
    }

    /// Attaches a level of a texture object as a logical buffer to the currently bound framebuffer object.
    /// 
    /// Attachs a selected mipmap level or image of a texture object
    /// as one of the logical buffers of the framebuffer object currently bound to `target`.
    /// 
    /// If `texture` is not zero, then `texture_target` must be GL_TEXTURE_1D.
    /// 
    /// For more details see `Framebuffer::texture`.
    #[inline(always)]
    pub unsafe fn texture_1d(
        &self,
        target:FramebufferTarget,
        attachment:FramebufferAttachement,
        texture_target:u32,
        texture:u32,
        mipmap_level:i32,
    ){
        transmute::<usize,fn(
            FramebufferTarget,
            FramebufferAttachement,
            u32,
            u32,
            i32
        )>(self.glFramebufferTexture1D)(target,attachment,texture_target,texture,mipmap_level)
    }

    /// Attaches a level of a texture object as a logical buffer to the currently bound framebuffer object.
    /// 
    /// Attachs a selected mipmap level or image of a texture object
    /// as one of the logical buffers of the framebuffer object currently bound to `target`.
    /// 
    /// If `texture` is not zero, `texture_target` must be one of GL_TEXTURE_2D,
    /// GL_TEXTURE_RECTANGLE, GL_TEXTURE_CUBE_MAP_POSITIVE_X, GL_TEXTURE_CUBE_MAP_POSITIVE_Y,
    /// GL_TEXTURE_CUBE_MAP_POSITIVE_Z, GL_TEXTURE_CUBE_MAP_NEGATIVE_X, GL_TEXTURE_CUBE_MAP_NEGATIVE_Y,
    /// GL_TEXTURE_CUBE_MAP_NEGATIVE_Z, or GL_TEXTURE_2D_MULTISAMPLE.
    /// 
    /// For more details see `Framebuffer::texture`.
    #[inline(always)]
    pub unsafe fn texture_2d(
        &self,
        target:FramebufferTarget,
        attachment:FramebufferAttachement,
        texture_target:u32,
        texture:u32,
        mipmap_level:i32,
    ){
        transmute::<usize,fn(
            FramebufferTarget,
            FramebufferAttachement,
            u32,
            u32,
            i32
        )>(self.glFramebufferTexture2D)(target,attachment,texture_target,texture,mipmap_level)
    }

    /// Attaches a level of a texture object as a logical buffer to the currently bound framebuffer object.
    /// 
    /// Attachs a selected mipmap level or image of a texture object
    /// as one of the logical buffers of the framebuffer object currently bound to `target`.
    /// 
    /// `layer` specifies the layer of a 2-dimensional image within a 3-dimensional texture.
    /// 
    /// If `texture` is not zero, then `texture_target` must be GL_TEXTURE_3D.
    /// 
    /// For more details see `Framebuffer::texture`.
    #[inline(always)]
    pub unsafe fn texture_3d(
        &self,
        target:FramebufferTarget,
        attachment:FramebufferAttachement,
        texture_target:u32,
        texture:u32,
        mipmap_level:i32,
        layer:i32
    ){
        transmute::<usize,fn(
            FramebufferTarget,
            FramebufferAttachement,
            u32,
            u32,
            i32,
            i32
        )>(self.glFramebufferTexture3D)(target,attachment,texture_target,texture,mipmap_level,layer)
    }

    /// Attaches a renderbuffer as a logical buffer to the currently bound framebuffer object.
    /// 
    /// Attaches a renderbuffer as one of the logical buffers of the currently bound framebuffer object.
    /// `renderbuffer` is the name of the renderbuffer object to attach and must be either zero,
    /// or the name of an existing renderbuffer object of type `Renderbuffer`.
    /// If `renderbuffer` is not zero and if `Framebuffer::renderbuffer` is successful,
    /// then the renderbuffer name `renderbuffer` will be usedas the logical buffer identified
    /// by `attachment` of the framebuffer currently bound to `target`.
    /// 
    /// The value of `GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE`
    /// for the specified attachment point is set to `Renderbuffer`
    /// and the value of `GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME` is set to `renderbuffer`.
    /// All other state values of the attachment point
    /// specified by `attachment` are set to their default values.
    /// No change is made to the state of the renderbuuffer object
    /// and any previous attachment to the `attachment` logical buffer of the framebuffer `target` is broken.
    /// 
    /// Calling `Framebuffer::renderbuffer` with the renderbuffer name zero will detach the image,
    /// if any, identified by `attachment`, in the framebuffer currently bound to target.
    /// All state values of the attachment point specified by attachment
    /// in the objectbound to `target` are set to their default values.
    /// 
    /// Setting attachment to the value `GL_DEPTH_STENCIL_ATTACHMENT` is a special case
    /// causing both the depth and stencil attachments of the framebuffer object to be set to `renderbuffer`,
    /// which should have the base internal format `GL_DEPTH_STENCIL`.
    /// 
    /// `GLError::InvalidOperation` is generated if zero is bound to `target`.
    #[inline(always)]
    pub unsafe fn renderbuffer(
        &self,
        target:FramebufferTarget,
        attachment:FramebufferAttachement,
        renderbuffer:u32
    ){
        transmute::<usize,fn(
            FramebufferTarget,
            FramebufferAttachement,
            u32,
            u32
        )>(self.glFramebufferRenderbuffer)(target,attachment,RENDERBUFFER,renderbuffer)
    }
}

impl Framebuffer{
    /// Copies a block of pixels from the read framebuffer to the draw.
    /// 
    /// Transfers a rectangle of pixel values from one region of the read framebuffer
    /// to another region in the draw framebuffer.
    /// `mask` indicats which buffers are to be copied.
    /// The pixels corresponding to these buffers
    /// are copied from the source rectangle bounded by the locations (srcX0; srcY0)
    /// and (srcX1; srcY1) to the destination rectangle bounded by the locations (dstX0; dstY0)
    /// and (dstX1; dstY1).
    /// The lower bounds of the rectangle are inclusive,
    /// while the upper bounds are exclusive.
    /// 
    /// The actual region taken from the read framebuffer is limited
    /// to the intersection of the source buffers being transferred,
    /// which may include the color buffer selected by the read buffer,
    /// the depth buffer, and/or the stencil buffer depending on mask.
    /// The actual region written to the draw framebuffer is limited
    /// to the intersection of the destination buffers being written,
    /// which may include multiple draw buffers, the depth buffer,
    /// and/or the stencil buffer depending on mask.
    /// Whether or not the source or destination regions are altered due to these limits,
    /// the scaling and offset applied to pixels being transferred
    /// is performedas though no such limits were present.
    /// 
    /// If the sizes of the source and destination rectangles are not equal,
    /// filter specifies the interpolation method that will be applied to resize the source image.
    /// GL_LINEAR is only a valid interpolation method for the color buffer.
    /// If filter is not GL_NEAREST and mask includes GL_DEPTH_BUFFER_BIT or GL_STENCIL_BUFFER_BIT,
    /// no data is transferred and a GL_INVALID_OPERATION error is generated.
    /// 
    /// If filter is GL_LINEAR and the source rectangle would require
    /// sampling outside the bounds of the source framebuffer,
    /// values are read as if the GL_CLAMP_TO_EDGE texture wrapping mode were applied.
    /// 
    /// When the color buffer is transferred,
    /// values are taken from the read buffer of the read framebuffer
    /// and written to each of the draw buffers of the draw framebuffer.
    /// 
    /// If the source and destination rectangles overlap or are the same,
    /// and the read and draw buffers are the same,
    /// the result of the operation is undefined.
    /// 
    /// GL_INVALID_OPERATION is generated if mask contains any of the GL_DEPTH_BUFFER_BIT or GL_STENCIL_BUFFER_BIT and filter is not GL_NEAREST.
    /// 
    /// GL_INVALID_OPERATION is generated if mask contains GL_COLOR_BUFFER_BIT and any of the following conditions hold:
    /// 
    /// The read buffer contains fixed-point or floating-point values and any draw buffer contains neither fixed-point nor floating-point values.
    /// 
    /// The read buffer contains unsigned integer values and any draw buffer does not contain unsigned integer values.
    /// 
    /// The read buffer contains signed integer values and any draw buffer does not contain signed integer values.
    /// 
    /// GL_INVALID_OPERATION is generated if mask contains GL_DEPTH_BUFFER_BIT or GL_STENCIL_BUFFER_BIT and the source and destination depth and stencil formats do not match.
    /// 
    /// GL_INVALID_OPERATION is generated if filter is GL_LINEAR and the read buffer contains integer data.
    /// 
    /// GL_INVALID_OPERATION is generated if the value of GL_SAMPLES for the read and draw buffers is not identical.
    /// 
    /// GL_INVALID_OPERATION is generated if GL_SAMPLE_BUFFERS for both read and draw buffers greater than zero and the dimensions of the source and destination rectangles is not identical.
    /// 
    /// GL_INVALID_FRAMEBUFFER_OPERATION is generated if the objects bound to GL_DRAW_FRAMEBUFFER_BINDING or GL_READ_FRAMEBUFFER_BINDING are not framebuffer complete.
    #[inline(always)]
    pub unsafe fn blit(
        &self,
        source:[i32;4],
        destination:[i32;4],
        mask:BlitMask,
        filter:FramebufferFilter
    ){
        transmute::<usize,fn(
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            BlitMask,
            FramebufferFilter
        )>(self.glBlitFramebuffer)(
            source[0],
            source[1],
            source[2],
            source[3],
            destination[0],
            destination[1],
            destination[2],
            destination[3],
            mask,
            filter,
        )
    }
}

const FRAMEBUFFER_COMPLETE:u32=0x8CD5;
const FRAMEBUFFER_UNDEFINED:u32=0x8219;

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum FramebufferStatus{
    /// Returned if the framebuffer bound to `target` is complete.
    Complete=FRAMEBUFFER_COMPLETE,

    /// Returned if `target` is the default framebuffer,
    /// but the default framebuffer does not exist.
    Undefined=FRAMEBUFFER_UNDEFINED,

    /// Returned if any of the framebuffer attachment points are framebuffer incomplete.
    FRAMEBUFFER_INCOMPLETE_ATTACHMENT,

    /// Returned if the framebuffer does not have at least one image attached to it.
    FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT,

    /// Returned if the value of `GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE`
    /// is `GL_NONE` for any color attachment point(s) named by `GL_DRAW_BUFFERi`.
    FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER,

    /// returned if `GL_READ_BUFFER` is not `GL_NONE` and the value of `GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE`
    /// is `GL_NONE` for the color attachment point named by `GL_READ_BUFFER`.
    FRAMEBUFFER_INCOMPLETE_READ_BUFFER,

    /// Returned if the combination of internal formats of the attached images
    /// violates an implementation-dependent set of restrictions.
    FRAMEBUFFER_UNSUPPORTED,

    /// Returned if the value of `GL_RENDERBUFFER_SAMPLES` is not the same for all attached renderbuffers;
    /// if the value of `GL_TEXTURE_SAMPLES` is the not same for all attached textures;
    /// if the attached images are a mix of renderbuffers and textures,
    /// the value of `GL_RENDERBUFFER_SAMPLES` does not match the value of `GL_TEXTURE_SAMPLES`;
    /// if the value of `GL_TEXTURE_FIXED_SAMPLE_LOCATIONS` is not the same for all attached textures;
    /// or, if the attached images are a mix of renderbuffers and textures,
    /// the value of `GL_TEXTURE_FIXED_SAMPLE_LOCATIONS` is not GL_TRUE for all attached textures.
    FRAMEBUFFER_INCOMPLETE_MULTISAMPLE,

    /// Returned if any framebuffer attachment is layered,
    /// and any populated attachment is not layered,
    /// or if all populated color attachments are not from textures of the same target.
    FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS

}

pub enum FramebufferParameter{

}

impl Framebuffer{
    /// Checks the completeness status of a framebuffer.
    /// 
    /// Queries the completeness status of the framebuffer object currently bound to `target`.
    #[inline(always)]
    pub fn check_status(&self,target:FramebufferTarget)->FramebufferStatus{
        unsafe{
            transmute::<usize,fn(FramebufferTarget)->FramebufferStatus>(self.glIsFramebuffer)(target)
        }
    }

    /// Retrieve information about attachments of a bound framebuffer object.
    /// 
    /// Returns information about attachments of a bound framebuffer object.
    /// 
    /// If the default framebuffer is bound to target
    /// then attachment must be one of `GL_FRONT_LEFT`,
    /// `GL_FRONT_RIGHT`, `GL_BACK_LEFT`, or `GL_BACK_RIGHT`,
    /// identifying a color buffer, `GL_DEPTH`, identifying the depth buffer,
    /// or GL_STENCIL, identifying the stencil buffer.
    /// 
    /// If attachment is `FramebufferAttachment::DepthStencil`
    /// and different objects are bound to the depth
    /// and stencil attachment points of targetthe query will fail.
    /// If the same object is bound to both attachment points,
    /// information about that object will be returned.
    /// 
    /// Upon successful return from glGetFramebufferAttachmentParameteriv,
    /// if pname is `GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE`,
    /// then params will contain one of GL_NONE,
    /// GL_FRAMEBUFFER_DEFAULT GL_TEXTURE, or GL_RENDERBUFFER,
    /// identifying the type of object which contains the attached image.
    /// Other values accepted for pname depend on the type of object, as described below.
    /// 
    /// If the value of GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE is GL_NONE,
    /// no framebuffer is bound to target.
    /// In this case querying pname GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME will return zero,
    /// and all other queries will generate an error.
    /// 
    /// If the value of GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE is not GL_NONE,
    /// these queries apply to all other framebuffer types:
    /// 
    /// If pname is GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE, GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE, GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE, GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE, GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE, or GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE, then params will contain the number of bits in the corresponding red, green, blue, alpha, depth, or stencil component of the specified attachment. Zero is returned if the requested component is not present in attachment.
    /// 
    /// If pname is GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE, params will contain the format of components of the specified attachment, one of GL_FLOAT, GL_INT, GL_UNSIGNED_INT, GL_SIGNED_NORMALIZED, or GL_UNSIGNED_NORMALIZED for floating-point, signed integer, unsigned integer, signed normalized fixed-point, or unsigned normalized fixed-point components respectively. Only color buffers may have integer components.
    /// 
    /// If pname is GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING, param will contain the encoding of components of the specified attachment, one of GL_LINEAR or GL_SRGB for linear or sRGB-encoded components, respectively. Only color buffer components may be sRGB-encoded; such components are treated as described in sections 4.1.7 and 4.1.8. For the default framebuffer, color encoding is determined by the implementation. For framebuffer objects, components are sRGB-encoded if the internal format of a color attachment is one of the color-renderable SRGB formats.
    /// 
    /// If the value of GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE is GL_RENDERBUFFER, then:
    /// 
    /// If pname is GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME, params will contain the name of the renderbuffer object which contains the attached image.
    /// 
    /// If the value of GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE is GL_TEXTURE, then:
    /// 
    /// If pname is GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME, then params will contain the name of the texture object which contains the attached image.
    /// 
    /// If pname is GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL, then params will contain the mipmap level of the texture object which contains the attached image.
    /// 
    /// If pname is GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE and the texture object named GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME is a cube map texture, then params will contain the cube map face of the cubemap texture object which contains the attached image. Otherwise params will contain the value zero.
    /// 
    /// If pname is GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER and the texture object named GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME is a layer of a three-dimensional texture or a one-or two-dimensional array texture, then params will contain the number of the texture layer which contains the attached image. Otherwise params will contain the value zero.
    /// 
    /// If pname is GL_FRAMEBUFFER_ATTACHMENT_LAYERED, then params will contain GL_TRUE if an entire level of a three-dimesional texture, cube map texture, or one-or two-dimensional array texture is attached. Otherwise, params will contain GL_FALSE.
    /// 
    /// Any combinations of framebuffer type and pname not described above will generate an error.
    /// 
    /// GL_INVALID_ENUM is generated if target is not one of the accepted tokens.
    /// 
    /// GL_INVALID_ENUM is generated if pname is not valid for the value of GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE.
    /// 
    /// GL_INVALID_OPERATION is generated if attachment is not the accepted values for target.
    /// 
    /// GL_INVALID_OPERATION is generated if attachment is GL_DEPTH_STENCIL_ATTACHMENT and different objects are bound to the depth and stencil attachment points of target.
    /// 
    /// GL_INVALID_OPERATION is generated if the value of GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE is GL_NONE and pname is not GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME.
    #[inline(always)]
    pub fn get_attachment_parameter(
        &self,
        target:FramebufferTarget,
        attachment:u32,
        paramenter:FramebufferParameter,
        parameters:&mut [i32]
    ){
        unsafe{
            transmute::<usize,fn(
                FramebufferTarget,
                u32,
                FramebufferParameter,
                &mut [i32]
            )>(self.glGetFramebufferAttachmentParameteriv)(
                target,
                attachment,
                paramenter,
                parameters
            )
        }
    }
}