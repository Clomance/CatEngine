use crate::graphics::{
    GCore,
    core::GLError,
    core::texture::{
        TextureBindTarget,
        Texture2DRewriteTarget,
        Texture2DWriteTarget,
        Texture2DInternalFormat,
        ImageDataFormat,
        TextureParameterTarget,
        TextureMinFilter,
        TextureMagFilter
    },
};

use core::{
    ptr::null,
    mem::MaybeUninit
};

pub struct Texture{
    id:u32,
}

impl Texture{
    pub fn initiate()->Texture{
        unsafe{
            let mut id=MaybeUninit::uninit().assume_init();
            GCore.texture.generate_one(&mut id);

            Self{
                id,
            }
        }
    }

    // pub fn create(target:TextureBindTarget)->Result<Texture,GLError>{
    //     let texture=Texture::initiate();
    //     let error=texture.bind(target);
    //     if error.is_error(){
    //         Err(error)
    //     }
    //     else{
    //         Ok(texture)
    //     }
    // }

    pub fn new_2d<I:Sized>(
        internal_format:Texture2DInternalFormat,
        mag_filter:TextureMagFilter,
        min_filter:TextureMinFilter,
        size:[u32;2],
        image_data_format:ImageDataFormat,
        data:&[I]
    )->Texture{
        unsafe{
            let texture=Texture::initiate();

            GCore.texture.bind(TextureBindTarget::Texture2D,texture.id);

            let data=if data.is_empty(){
                null()
            }
            else{
                &data[0] as *const I
            };

            GCore.texture.set_min_filter(TextureParameterTarget::Texture2D,min_filter);
            GCore.texture.set_mag_filter(TextureParameterTarget::Texture2D,mag_filter);

            GCore.texture.rewrite_image_2d(
                Texture2DRewriteTarget::Texture2D,
                0,
                internal_format,
                [size[0] as i32,size[1] as i32],
                image_data_format,
                data
            );

            texture
        }
    }

    #[inline(always)]
    pub fn id(&self)->u32{
        self.id
    }

    /// Binds a texture to a texturing target.
    /// 
    /// When a texture is bound to a target,
    /// the previous binding for that target is automatically broken.
    /// 
    /// Returns `GLError::NoError` if no error has accured.
    /// 
    /// Returns `GLError::InvalidValue`
    /// if target is not a name returned from a previous call to glGenTextures.
    /// 
    /// Returns `GLError::InvalidOperation`
    /// if texture was previously created with a target that doesn't match that of target.
    pub fn bind(&self,target:TextureBindTarget)->GLError{
        unsafe{
            GCore.texture.bind(target,self.id);
            GCore.get_error()
        }
    }
}

impl Texture{
    pub fn rewrite_image_2d(
        &self,
        target:Texture2DRewriteTarget,
        texture_internal_format:Texture2DInternalFormat,
        size:[i32;2],
        image_data_format:ImageDataFormat,
        data:&[u8]
    ){
        unsafe{
            let data_ref=if data.len()!=0{
                (data as *const [u8]) as *const core::ffi::c_void
            }
            else{
                0 as *const core::ffi::c_void
            };
            GCore.texture.rewrite_image_2d(
                target,
                0,
                texture_internal_format,
                size,
                image_data_format,
                data_ref
            );
        }
    }

    pub fn write_image_2d(
        &self,
        target:Texture2DWriteTarget,
        [x,y,width,height]:[i32;4],
        image_data_format:ImageDataFormat,
        data:&[u8]
    ){
        unsafe{
            let data_ref=if data.len()!=0{
                (data as *const [u8]) as *const core::ffi::c_void
            }
            else{
                0 as *const core::ffi::c_void
            };
            GCore.texture.write_image_2d(
                target,
                0,
                [x,y,width,height],
                image_data_format,
                data_ref

            );
        }
    }
}

impl Drop for Texture{
    fn drop(&mut self){
        unsafe{
            GCore.texture.delete_one(&self.id)
        }
    }
}