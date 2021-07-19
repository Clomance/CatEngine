use crate::graphics::{
    GCore,
    core::texture::{
        TextureBindTarget,
        Texture2DRewriteTarget,
        Texture2DWriteTarget,
        Texture2DInternalFormat,
        ImageDataFormat,
        ImageDataType,
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

    pub fn new_2d<I:Sized>(
        internal_format:Texture2DInternalFormat,
        mag_filter:TextureMagFilter,
        min_filter:TextureMinFilter,
        size:[u32;2],
        image_data_format:ImageDataFormat,
        image_data_type:ImageDataType,
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
                image_data_type,
                data
            );

            texture
        }
    }

    #[inline(always)]
    pub fn id(&self)->u32{
        self.id
    }

    pub fn bind(&self,target:TextureBindTarget){
        unsafe{
            GCore.texture.bind(target,self.id)
        }
    }

    pub fn rewrite_image_2d(
        &self,
        target:Texture2DRewriteTarget,
        texture_internal_format:Texture2DInternalFormat,
        size:[u32;2],
        image_data_format:ImageDataFormat,
        image_data_type:ImageDataType,
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
                [size[0] as i32,size[1] as i32],
                image_data_format,
                image_data_type,
                data_ref
            );
        }
    }

    pub fn write_image_2d(
        &self,
        target:Texture2DWriteTarget,
        [x,y,width,height]:[i32;4],
        image_data_format:ImageDataFormat,
        image_data_type:ImageDataType,
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
                image_data_type,
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

    // pub fn rewrite_image_1d(
    //     &self,
    //     texture_internal_format:TextureInternalFormat,
    //     size:u32,
    //     image_data_format:ImageDataFormat,
    //     data:&[u8]
    // ){
    //     unsafe{
    //         // Arguments:
    //         // 1 - target
    //         // 2 - mipmap level - if we want to set mipmap textures manually
    //         // 3 - colour format of the texture
    //         // 4 - size
    //         // 5 - 
    //         // 6 - colour format of the image
    //         // 7 - byte format
    //         // 8 - data
    //         let [pixel_format,component_format]=image_data_format.as_gl_enums();
    //         let data_ref=if data.len()!=0{
    //             (data as *const [u8]) as *const core::ffi::c_void
    //         }
    //         else{
    //             0 as *const core::ffi::c_void
    //         };
    //         TexImage1D(
    //             self.target,
    //             0,
    //             texture_internal_format as i32,
    //             size as i32,
    //             0,
    //             pixel_format,
    //             component_format,
    //             data_ref
    //         );
    //     }
    // }

    // /// The target must be GL_TEXTURE_2D, GL_TEXTURE_CUBE_MAP_POSITIVE_X,
    // /// GL_TEXTURE_CUBE_MAP_NEGATIVE_X, GL_TEXTURE_CUBE_MAP_POSITIVE_Y,
    // /// GL_TEXTURE_CUBE_MAP_NEGATIVE_Y, GL_TEXTURE_CUBE_MAP_POSITIVE_Z, or GL_TEXTURE_CUBE_MAP_NEGATIVE_Z.
    // pub fn write_read_framebuffer(&self,buffer_offset:[u32;2],texture_offset:[u32;2],size:[u32;2]){
    //     unsafe{
    //         CopyTexSubImage2D(
    //             self.target,
    //             0,
    //             texture_offset[0] as i32,
    //             texture_offset[1] as i32,
    //             buffer_offset[0] as i32,
    //             buffer_offset[1] as i32,
    //             size[0] as i32,
    //             size[1] as i32,
    //         );
    //     }
    // }

// /// Settings parameters.
// impl<'a> BoundTexture<'a>{
//     #[inline(always)]
//     pub unsafe fn set_parameters(&self,parameter:u32,value:i32){
//         TexParameteri(self.target,parameter,value);
//     }

//     #[inline(always)]
//     pub fn set_mag_filter(&self,filter:TextureFilter){
//         unsafe{
//             TexParameteri(self.target,TEXTURE_MAG_FILTER,filter as i32);
//         }
//     }

//     #[inline(always)]
//     pub fn set_min_filter(&self,filter:TextureFilter){
//         unsafe{
//             TexParameteri(self.target,TEXTURE_MIN_FILTER,filter as i32);
//         }
//     }
// }