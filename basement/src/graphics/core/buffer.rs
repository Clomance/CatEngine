#[cfg(target_os="windows")]
use crate::windows::OpenGraphicsLibrary;

use core::mem::transmute;

// Buffer targets
const ARRAY_BUFFER:u32=0x8892;
const COPY_READ_BUFFER:u32=0x8F36;
const COPY_WRITE_BUFFER:u32=0x8F37;
const ELEMENT_ARRAY_BUFFER:u32=0x8893;
const PIXEL_PACK_BUFFER:u32=0x88EB;
const PIXEL_UNPACK_BUFFER:u32=0x88EC;
const TEXTURE_BUFFER:u32=0x8C2A;
const TRANSFORM_FEEDBACK_BUFFER:u32=0x8C8E;
const UNIFORM_BUFFER:u32=0x8A11;

// Buffer usages
const STREAM_DRAW:u32=0x88E0;
const STREAM_READ:u32=0x88E1;
const STREAM_COPY:u32=0x88E2;
const STATIC_DRAW:u32=0x88E4;
const STATIC_READ:u32=0x88E5;
const STATIC_COPY:u32=0x88E6;
const DYNAMIC_DRAW:u32=0x88E8;
const DYNAMIC_READ:u32=0x88E9;
const DYNAMIC_COPY:u32=0x88EA;


#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum BufferTarget{
    /// Vertex attributes.
    ArrayBuffer=ARRAY_BUFFER,

    /// Buffer copy source.
    CopyReadBuffer=COPY_READ_BUFFER,

    /// Buffer copy destination.
    CopyWriteBuffer=COPY_WRITE_BUFFER,

    /// Vertex array indices.
    ElementArrayBuffer=ELEMENT_ARRAY_BUFFER,

    /// Pixel read target.
    PixelPackBuffer=PIXEL_PACK_BUFFER,

    /// Texture data source.
    PixelUnpackBuffer=PIXEL_UNPACK_BUFFER,

    /// Texture data buffer.
    TextureBuffer=TEXTURE_BUFFER,

    /// Transform feedback buffer.
    TransformFeedbackBuffer=TRANSFORM_FEEDBACK_BUFFER,

    /// Uniform block storage.
    UniformBuffer=UNIFORM_BUFFER,
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum BufferIndexedTarget{
    TransformFeedbackBuffer=TRANSFORM_FEEDBACK_BUFFER,
    UniformBuffer=UNIFORM_BUFFER,
}

/// Specifies the expected usage pattern of the data store.
#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum BufferUsage{
    /// The data store contents will be modified once and used at most a few times.
    /// The data store contents are modified by the application,
    /// and used as the source for GL drawing and image specification commands.
    StreamDraw=STREAM_DRAW,

    /// The data store contents will be modified once and used at most a few times.
    /// The data store contents are modified by reading data from the GL,
    /// and used to return that data when queried by the application.
    StreamRead=STREAM_READ,

    /// The data store contents will be modified once and used at most a few times.
    /// The data store contents are modified by reading data from the GL,
    /// and used as the source for GL drawing and image specification commands.
    StreamCopy=STREAM_COPY,

    /// The data store contents will be modified once and used many times.
    /// The data store contents are modified by the application,
    /// and used as the source for GL drawing and image specification commands.
    StaticDraw=STATIC_DRAW,

    /// The data store contents will be modified once and used many times.
    /// The data store contents are modified by reading data from the GL,
    /// and used to return that data when queried by the application.
    StaticRead=STATIC_READ,

    /// The data store contents will be modified once and used many times.
    /// The data store contents are modified by reading data from the GL,
    /// and used as the source for GL drawing and image specification commands.
    StaticCopy=STATIC_COPY,

    /// The data store contents will be modified repeatedly and used many times.
    /// The data store contents are modified by the application,
    /// and used as the source for GL drawing and image specification commands.
    DynamicDraw=DYNAMIC_DRAW,

    /// The data store contents will be modified repeatedly and used many times.
    /// The data store contents are modified by reading data from the GL,
    /// and used to return that data when queried by the application.
    DynamicRead=DYNAMIC_READ,

    /// The data store contents will be modified repeatedly and used many times.
    /// The data store contents are modified by reading data from the GL,
    /// and used as the source for GL drawing and image specification commands.
    DynamicCopy=DYNAMIC_COPY,
}

pub struct Buffer{
    glGenBuffers:usize,
    glDeleteBuffers:usize,

    glBindBuffer:usize,
    glBindBufferBase:usize,
    glBindBufferRange:usize,

    glBufferData:usize,
    glBufferSubData:usize,

    glCopyBufferSubData:usize,
}

impl Buffer{
    pub const fn new()->Buffer{
        Self{
            glGenBuffers:0,
            glDeleteBuffers:0,

            glBindBuffer:0,
            glBindBufferBase:0,
            glBindBufferRange:0,

            glBufferData:0,
            glBufferSubData:0,

            glCopyBufferSubData:0,
        }
    }

    #[cfg(target_os="windows")]
    pub fn load(&mut self,library:&OpenGraphicsLibrary){
        unsafe{
            self.glGenBuffers=transmute(library.get_proc_address("glGenBuffers\0"));
            self.glDeleteBuffers=transmute(library.get_proc_address("glDeleteBuffers\0"));

            self.glBindBuffer=transmute(library.get_proc_address("glBindBuffer\0"));
            self.glBindBufferBase=transmute(library.get_proc_address("glBindBufferBase\0"));
            self.glBindBufferRange=transmute(library.get_proc_address("glBindBufferRange\0"));

            self.glBufferData=transmute(library.get_proc_address("glBufferData\0"));
            self.glBufferSubData=transmute(library.get_proc_address("glBufferSubData\0"));

            self.glCopyBufferSubData=transmute(library.get_proc_address("glCopyBufferSubData\0"));
        }
    }
}

impl Buffer{
    /// Generates a buffer object.
    #[inline(always)]
    pub fn generate_one(&self,buffer:&mut u32){
        unsafe{
            transmute::<usize,fn(i32,&mut u32)>(self.glGenBuffers)(1,buffer)
        }
    }

    /// Deletes a buffer object.
    #[inline(always)]
    pub fn delete_one(&self,buffer:&u32){
        unsafe{
            transmute::<usize,fn(i32,&u32)>(self.glDeleteBuffers)(1,buffer)
        }
    }

    /// Generates buffer objects.
    #[inline(always)]
    pub fn generate(&self,buffers:&mut [u32]){
        unsafe{
            transmute::<usize,fn(i32,&mut u32)>(self.glGenBuffers)(buffers.len() as i32,&mut buffers[0])
        }
    }

    /// Deletes buffer objects.
    #[inline(always)]
    pub fn delete(&self,buffers:&[u32]){
        unsafe{
            transmute::<usize,fn(i32,&u32)>(self.glDeleteBuffers)(buffers.len() as i32,&buffers[0])
        }
    }

    /// Binds a named buffer object.
    /// 
    /// When a buffer object is bound to a target, the previous binding for that target is automatically broken.
    /// 
    /// `GLError::InvalidValue` is generated if `buffer` is not a name previously returned from a call to `generate`.
    #[inline(always)]
    pub unsafe fn bind(&self,target:BufferTarget,buffer:u32){
        transmute::<usize,fn(u32,u32)>(self.glBindBuffer)(target as u32,buffer)
    }

    /// Binds a range within a buffer object to an indexed buffer target.
    /// 
    /// `GLError::InvalidValue` is generated if `buffer` is not a name previously returned from a call to `Buffer::generate`,
    /// if `index` is greater than or equal to the number of target-specific indexed binding points,
    /// if buffer does not have an associated data store, or if the size of that store is zero.
    #[inline(always)]
    pub unsafe fn bind_base(&self,target:BufferIndexedTarget,index:u32,buffer:u32){
        transmute::<usize,fn(u32,u32,u32)>(self.glBindBufferBase)(target as u32,index,buffer)
    }

    /// Binds a range within a buffer object to an indexed buffer target.
    /// 
    /// `GLError::InvalidValue` is generated if `buffer` is not a name previously returned from a call to `Buffer::generate`,
    /// if `index` is greater than or equal to the number of target-specific indexed binding points,
    /// if `size` is less than or equal to zero, or if `offset + size` is greater than the value of GL_BUFFER_SIZE.
    /// 
    /// Additional errors may be generated if offset violates any target-specific alignmemt restrictions.
    #[inline(always)]
    pub unsafe fn bind_range(&self,target:BufferIndexedTarget,index:u32,buffer:u32,offset:isize,size:isize){
        transmute::<usize,fn(u32,u32,u32,isize,isize)>(self.glBindBufferRange)(target as u32,index,buffer,offset,size)
    }

    /// Creates and initializes a buffer object's data store.
    /// 
    /// `GLError::InvalidValue` is generated if `size` is negative.
    /// 
    /// `GLError::InvalidOperation` is generated if the reserved buffer object name 0 is bound to `target`.
    /// 
    /// `GLError::OutOfMemory` is generated if the GL is unable to create a data store with the specified size.
    #[inline(always)]
    pub unsafe fn rewrite<I:Sized>(&self,target:BufferTarget,size:isize,data:*const I,usage:BufferUsage){
        transmute::<usize,fn(BufferTarget,isize,*const I,BufferUsage)>(self.glBufferData)(target,size,data,usage)
    }

    /// Updates a subset of a buffer object's data store.
    /// 
    /// `GLError::InvalidValue` is generated if `offset` or `size` is negative,
    /// or if together they define a region of memory that extends beyond the buffer object's allocated data store.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if the reserved buffer object name 0 is bound to `target`,
    /// or if the buffer object being updated is mapped.
    #[inline(always)]
    pub unsafe fn write<I:Sized>(&self,target:BufferTarget,offset:isize,size:isize,data:*const I){
        transmute::<usize,fn(BufferTarget,isize,isize,*const I)>(self.glBufferSubData)(target,offset,size,data)
    }

    /// Copies part of the data store of a buffer object to the data store of another buffer object.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if any of `read_offset`, `write_offset` or `size` is negative,
    /// if `read_offset + size` exceeds the size of the buffer object bound to `read_target`,
    /// if `write_offset + size` exceeds the size of the buffer object bound to `write_target`,
    /// or if the same buffer object is bound to both `read_target` and `write_target`
    /// and the ranges `[read_offset, read_offset + size)` and `[write_offset, writeo_offset + size)` overlap.
    /// 
    /// `GLError::InvalidOperation` is generated if zero is bound to `read_target` or `write_target`,
    /// or if the buffer object bound to either `read_target` or `write_target` is mapped.
    #[inline(always)]
    pub unsafe fn copy_buffer(
        &self,
        read_target:BufferTarget,
        write_target:BufferTarget,
        read_offset:isize,
        write_offset:isize,
        size:isize,
    ){
        transmute::<usize,fn(
            BufferTarget,
            BufferTarget,
            isize,
            isize,
            isize
        )>(self.glCopyBufferSubData)(
            read_target,
            write_target,
            read_offset,
            write_offset,
            size
        )
    }
}