use super::{
    Texture,
    TextureFilter,
};

use std::{
    marker::PhantomData,
    mem::{
        size_of,
        MaybeUninit
    },
};

use gl::{
    // constants
    READ_FRAMEBUFFER,
    DRAW_FRAMEBUFFER,
    FRAMEBUFFER,
    COLOR_ATTACHMENT0,
    DEPTH_ATTACHMENT,
    STENCIL_ATTACHMENT,
    // functions
    GenFramebuffers,
    BindFramebuffer,
    FramebufferTexture2D,
    BlitFramebuffer,
    DeleteFramebuffers,
};

#[derive(Clone,Copy,Debug)]
pub enum FrameBufferTarget{
    Read=READ_FRAMEBUFFER as isize,
    Draw=DRAW_FRAMEBUFFER as isize,
    FrameBuffer=FRAMEBUFFER as isize,
}

#[derive(Clone,Copy,Debug)]
pub enum FrameBufferAttachment{
    ColourAttachment0=COLOR_ATTACHMENT0 as isize,
    DepthAttachment=DEPTH_ATTACHMENT as isize,
    StencilAttachment=STENCIL_ATTACHMENT as isize,
}

/// Since OpenGL 3.0.
pub struct FrameBuffer<'a>{
    id:u32,
    texture:&'a Texture,
}

impl<'a> FrameBuffer<'a>{
    pub unsafe fn new_2d(target:FrameBufferTarget,attachement:FrameBufferAttachment,texture:&'a Texture,texture_target:u32)->FrameBuffer<'a>{
        let mut id:u32=MaybeUninit::uninit().assume_init();
        GenFramebuffers(1,&mut id as *mut u32);
        BindFramebuffer(target as u32,id);

        texture.bind(texture_target);

        FramebufferTexture2D(target as u32,attachement as u32,texture_target,texture.id(),0);

        BindFramebuffer(target as u32,0);

        Self{
            id,
            texture,
        }
    }

    #[inline(always)]
    pub fn id(&self)->u32{
        self.id
    }

    pub fn texture(&self)->&Texture{
        self.texture
    }
}

impl<'a> FrameBuffer<'a>{
    pub unsafe fn bind(&'a self,target:FrameBufferTarget)->BoundFrameBuffer<'a>{
        BindFramebuffer(target as u32,self.id);
        BoundFrameBuffer{
            target:target as u32,
            marker:PhantomData,
        }
    }

    pub unsafe fn unbind(&self,target:FrameBufferTarget){
        BindFramebuffer(target as u32,0);
    }

    // pub unsafe fn write(&self,target:u32,offset:usize,items:&[I]){
    //     BindFramebuffer(target,self.id);
    //     let data_ref=(items as *const [I]) as *const core::ffi::c_void;
    //     BufferSubData(target,(offset*size_of::<I>()) as isize,(items.len()*size_of::<I>()) as isize,data_ref)
    // }

    // /// Offset in bytes.
    // pub unsafe fn write_value(&self,target:u32,offset:usize,value:&I){
    //     BindFramebuffer(target,self.id);
    //     let data_ref=(value as *const I) as *const core::ffi::c_void;
    //     BufferSubData(target,offset as isize,size_of::<I>() as isize,data_ref)
    // }    

    /// Offset in bytes.
    pub unsafe fn write_frame_buffer(&self,[x1,y1,x2,y2]:[i32;4],frame_buffer:&FrameBuffer,[src_x1,src_y1,src_x2,src_y2]:[i32;4],mask:u32,filter:TextureFilter){
        self.bind(FrameBufferTarget::Draw);
        frame_buffer.bind(FrameBufferTarget::Read);
        BlitFramebuffer(src_x1,src_y1,src_x2,src_y2,x1,y1,x2,y2,mask,filter as u32);

        BindFramebuffer(DRAW_FRAMEBUFFER,0);
        BindFramebuffer(READ_FRAMEBUFFER,0);
    }

    pub unsafe fn write_screen_buffer(&self,[x1,y1,x2,y2]:[i32;4],[src_x1,src_y1,src_x2,src_y2]:[i32;4],mask:u32,filter:TextureFilter){
        self.bind(FrameBufferTarget::Draw);

        BlitFramebuffer(src_x1,src_y1,src_x2,src_y2,x1,y1,x2,y2,mask,filter as u32);
        BindFramebuffer(DRAW_FRAMEBUFFER,0);
    }

    // pub unsafe fn rewrite(&self,target:u32,items:&[I]){
    //     BindFramebuffer(target,self.id);
    //     let data_ref=(items as *const [I]) as *const core::ffi::c_void;
    //     BufferData(target,(items.len()*size_of::<I>()) as isize,data_ref,DYNAMIC_DRAW);
    // }
}

impl<'a> Drop for FrameBuffer<'a>{
    fn drop(&mut self){
        unsafe{
            DeleteFramebuffers(1,&self.id as *const u32);
        }
    }
}

pub struct BoundFrameBuffer<'a>{
    target:u32,
    marker:PhantomData<&'a FrameBuffer<'a>>,
}

impl<'a> BoundFrameBuffer<'a>{
    // pub unsafe fn write(&self,offset:usize,items:&[I]){
    //     let data_ref=(items as *const [I]) as *const core::ffi::c_void;
    //     BufferSubData(self.target,(offset*size_of::<I>()) as isize,(items.len()*size_of::<I>()) as isize,data_ref)
    // }

    // pub unsafe fn rewrite(&self,items:&[I]){
    //     let data_ref=(items as *const [I]) as *const core::ffi::c_void;
    //     BufferData(self.target,(items.len()*size_of::<I>()) as isize,data_ref,DYNAMIC_DRAW);
    // }
}