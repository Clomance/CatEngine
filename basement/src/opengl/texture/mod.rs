pub mod texture_2d;

use crate::opengl::core::texture::{
    Texture as TextureFunctions,
};

pub use crate::opengl::core::texture::{
    TextureTarget,
    Texture2DAllocateTarget,
    Texture3DAllocateTarget,
    Texture3DWriteTarget,
    TextureInternalFormat,
    PixelFormat,
    PixelType,
    TextureParameterTarget,
    TextureParameter,
    TextureMinFilter,
    TextureMagFilter,
    TextureUnit,
};

use core::{
    mem::{
        size_of,
        MaybeUninit
    },
    marker::PhantomData,
    ptr::null,
};

use super::core::texture::Texture2DWriteTarget;

pub struct Texture{
    id:u32
}

impl Texture{
    pub fn new()->Texture{
        unsafe{
            let mut id=MaybeUninit::uninit();

            TextureFunctions::generate(1,id.as_mut_ptr());

            Self{
                id:id.assume_init(),
            }
        }
    }

    pub fn id(&self)->u32{
        self.id
    }

    pub fn bind(&self,target:TextureTarget){
        unsafe{
            TextureFunctions::bind(target,self.id)
        }
    }

    pub fn allocate_2d<P>(
        target:Texture2DAllocateTarget,
        level:i32,
        internal_format:TextureInternalFormat,
        width:i32,
        height:i32,
        pixel_format:PixelFormat,
        pixel_type:PixelType,
        data:*const P
    ){
        unsafe{
            TextureFunctions::allocate_2d(
                target,
                level,
                internal_format,
                width,
                height,
                0,
                pixel_format,
                pixel_type,
                data as *const _
            )
        }
    }

    pub fn allocate_3d<P>(
        target:Texture3DAllocateTarget,
        level:i32,
        internal_format:TextureInternalFormat,
        width:i32,
        height:i32,
        depth:i32,
        pixel_format:PixelFormat,
        pixel_type:PixelType,
        data:*const P
    ){
        unsafe{
            TextureFunctions::allocate_3d(
                target,
                level,
                internal_format,
                width,
                height,
                depth,
                0,
                pixel_format,
                pixel_type,
                data as *const _
            )
        }
    }

    pub fn write_2d<P>(
        target:Texture2DWriteTarget,
        level:i32,
        xoffset:i32,
        yoffset:i32,
        width:i32,
        height:i32,
        pixel_format:PixelFormat,
        pixel_type:PixelType,
        data:*const P
    ){
        unsafe{
            TextureFunctions::write_2d(
                target,
                level,
                xoffset,
                yoffset,
                width,
                height,
                pixel_format,
                pixel_type,
                data as *const _
            )
        }
    }

    pub fn write_3d<P>(
        target:Texture3DWriteTarget,
        level:i32,
        xoffset:i32,
        yoffset:i32,
        zoffset:i32,
        width:i32,
        height:i32,
        depth:i32,
        pixel_format:PixelFormat,
        pixel_type:PixelType,
        data:*const P
    ){
        unsafe{
            TextureFunctions::write_3d(
                target,
                level,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
                pixel_format,
                pixel_type,
                data as *const _
            )
        }
    }

    pub fn set_parameteri(target:TextureParameterTarget,parameter:TextureParameter,value:i32){
        unsafe{
            TextureFunctions::set_parameteri(target,parameter,value)
        }
    }
}

impl Drop for Texture{
    fn drop(&mut self){
        unsafe{
            TextureFunctions::delete(1,&self.id)
        }
    }
}