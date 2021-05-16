use super::GLError;

use std::{
    marker::PhantomData,
    mem::{
        size_of,
        MaybeUninit
    },
};

use gl::{
    // constants
    // usages
    STREAM_DRAW,
    STREAM_READ,
    STREAM_COPY,
    STATIC_DRAW,
    STATIC_READ,
    STATIC_COPY,
    DYNAMIC_DRAW,
    DYNAMIC_READ,
    DYNAMIC_COPY,
    // targets
    ARRAY_BUFFER,
    ATOMIC_COUNTER_BUFFER,
    COPY_READ_BUFFER,
    COPY_WRITE_BUFFER,
    DISPATCH_INDIRECT_BUFFER,
    DRAW_INDIRECT_BUFFER,
    ELEMENT_ARRAY_BUFFER,
    PIXEL_PACK_BUFFER,
    PIXEL_UNPACK_BUFFER,
    QUERY_BUFFER,
    SHADER_STORAGE_BUFFER,
    TEXTURE_BUFFER,
    TRANSFORM_FEEDBACK_BUFFER,
    UNIFORM_BUFFER,
    // functions
    GenBuffers,
    BindBuffer,
    BufferData,
    BufferSubData,
    DeleteBuffers,
    CopyBufferSubData,
};

/// Specifies the expected usage pattern of the data store.
#[derive(Clone,Copy,Debug)]
pub enum BufferUsage{
    /// The data store contents will be modified once and used at most a few times.
    /// The data store contents are modified by the application,
    /// and used as the source for GL drawing and image specification commands.
    StreamDraw=STREAM_DRAW as isize,

    /// The data store contents will be modified once and used at most a few times.
    /// The data store contents are modified by reading data from the GL,
    /// and used to return that data when queried by the application.
    StreamRead=STREAM_READ as isize,

    /// The data store contents will be modified once and used at most a few times.
    /// The data store contents are modified by reading data from the GL,
    /// and used as the source for GL drawing and image specification commands.
    StreamCopy=STREAM_COPY as isize,

    /// The data store contents will be modified once and used many times.
    /// The data store contents are modified by the application,
    /// and used as the source for GL drawing and image specification commands.
    StaticDraw=STATIC_DRAW as isize,

    /// The data store contents will be modified once and used many times.
    /// The data store contents are modified by reading data from the GL,
    /// and used to return that data when queried by the application.
    StaticRead=STATIC_READ as isize,

    /// The data store contents will be modified once and used many times.
    /// The data store contents are modified by reading data from the GL,
    /// and used as the source for GL drawing and image specification commands.
    StaticCopy=STATIC_COPY as isize,

    /// The data store contents will be modified repeatedly and used many times.
    /// The data store contents are modified by the application,
    /// and used as the source for GL drawing and image specification commands.
    DynamicDraw=DYNAMIC_DRAW as isize,

    /// The data store contents will be modified repeatedly and used many times.
    /// The data store contents are modified by reading data from the GL,
    /// and used to return that data when queried by the application.
    DynamicRead=DYNAMIC_READ as isize,

    /// The data store contents will be modified repeatedly and used many times.
    /// The data store contents are modified by reading data from the GL,
    /// and used as the source for GL drawing and image specification commands.
    DynamicCopy=DYNAMIC_COPY as isize,
}

#[derive(Clone,Copy,Debug)]
pub enum BufferTarget{
    /// Vertex attributes.
    ArrayBuffer=ARRAY_BUFFER as isize,
    /// Atomic counter storage.
    /// 
    /// Since OpenGL 4.2.
    AtomicCounterBuffer=ATOMIC_COUNTER_BUFFER as isize,
    /// Buffer copy source.
    /// 
    /// Since OpenGL 3.1.
    CopyReadBuffer=COPY_READ_BUFFER as isize,
    /// Buffer copy destination.
    /// 
    /// Since OpenGL 3.1.
    CopyWriteBuffer=COPY_WRITE_BUFFER as isize,
    /// Indirect compute dispatch commands.
    /// 
    /// Since OpenGL 4.3.
    DispatchIndirectBuffer=DISPATCH_INDIRECT_BUFFER as isize,
    /// Indirect command arguments.
    DrawIndirectBuffer=DRAW_INDIRECT_BUFFER as isize,
    /// Vertex array indices.
    ElementArrayBuffer=ELEMENT_ARRAY_BUFFER as isize,
    /// Pixel read target.
    PixelPackBuffer=PIXEL_PACK_BUFFER as isize,
    /// Texture data source.
    PixelUnpackBuffer=PIXEL_UNPACK_BUFFER as isize,
    /// Query result buffer.
    /// 
    /// Since OpenGL 4.4.
    QueryBuffer=QUERY_BUFFER as isize,
    /// Read-write storage for shaders.
    /// 
    /// Since OpenGL 4.3.
    ShaderStorageBuffer=SHADER_STORAGE_BUFFER as isize,
    /// Texture data buffer.
    /// 
    /// Since OpenGL 3.1.
    TextureBuffer=TEXTURE_BUFFER as isize,
    /// Transform feedback buffer.
    TransformFeedbackBuffer=TRANSFORM_FEEDBACK_BUFFER as isize,
    /// Uniform block storage.
    UniformBuffer=UNIFORM_BUFFER as isize,
}

pub struct Buffer<I:Sized>{
    id:u32,
    marker:PhantomData<I>,
}

impl<I:Sized> Buffer<I>{
    pub fn initialize()->Buffer<I>{
        unsafe{
            let mut id:u32=MaybeUninit::uninit().assume_init();
            GenBuffers(1,&mut id as *mut u32);

            Self{
                id,
                marker:PhantomData,
            }
        }
    }

    pub unsafe fn new(target:BufferTarget,items:&[I],usage:BufferUsage)->Buffer<I>{
        let buffer=Buffer::initialize();
        buffer.bind(target).rewrite(items,usage);
        buffer
    }

    pub unsafe fn new_value<Item>(target:BufferTarget,value:&Item,usage:BufferUsage)->Buffer<I>{
        let buffer=Buffer::initialize();
        buffer.bind(target);

        let data_ref=(value as *const Item) as *const core::ffi::c_void;
        BufferData(target as u32,size_of::<Item>() as isize,data_ref,usage as u32);

        buffer
    }

    /// The size is the amount of vertices.
    pub unsafe fn empty(target:BufferTarget,size:usize,usage:BufferUsage)->Buffer<I>{
        let buffer=Buffer::initialize();
        buffer.bind(target);

        // Arguments:
        // 1 - the target (the type of the buffer)
        // 2 - the size of the vertices
        // 3 - the data
        // 4 - type of managing the data
        let data_ref=core::ptr::null();
        BufferData(target as u32,(size*size_of::<I>()) as isize,data_ref,usage as u32);

        buffer
    }

    #[inline(always)]
    pub fn id(&self)->u32{
        self.id
    }
}

impl<I:Sized> Buffer<I>{
    pub unsafe fn bind<'a>(&'a self,target:BufferTarget)->BoundBuffer<'a,I>{
        BindBuffer(target as u32,self.id);
        BoundBuffer{
            target:target as u32,
            marker:PhantomData,
        }
    }

    pub unsafe fn write(&self,target:BufferTarget,offset:usize,items:&[I]){
        BindBuffer(target as u32,self.id);
        let data_ref=(items as *const [I]) as *const core::ffi::c_void;
        BufferSubData(target as u32,(offset*size_of::<I>()) as isize,(items.len()*size_of::<I>()) as isize,data_ref)
    }

    /// Offset in bytes.
    pub unsafe fn write_value(&self,target:BufferTarget,offset:usize,value:&I){
        BindBuffer(target as u32,self.id);
        let data_ref=(value as *const I) as *const core::ffi::c_void;
        BufferSubData(target as u32,offset as isize,size_of::<I>() as isize,data_ref)
    }

    /// Offset in bytes.
    /// 
    /// Since OpenGL 3.1.
    pub unsafe fn write_buffer<I2>(&self,offset:usize,buffer:&Buffer<I2>,from:usize,size:usize){
        self.bind(BufferTarget::CopyWriteBuffer);
        buffer.bind(BufferTarget::CopyReadBuffer);
        CopyBufferSubData(COPY_READ_BUFFER,COPY_WRITE_BUFFER,offset as isize,from as isize,size as isize)
    }
}

impl<I:Sized> Drop for Buffer<I>{
    fn drop(&mut self){
        unsafe{
            DeleteBuffers(1,&self.id as *const u32);
        }
    }
}

pub struct BoundBuffer<'a,I:Sized>{
    target:u32,
    marker:PhantomData<&'a Buffer<I>>,
}

impl<'a,I:Sized> BoundBuffer<'a,I>{
    pub unsafe fn write(&self,offset:usize,items:&[I]){
        let data_ref=(items as *const [I]) as *const core::ffi::c_void;
        BufferSubData(self.target,(offset*size_of::<I>()) as isize,(items.len()*size_of::<I>()) as isize,data_ref)
    }

    pub unsafe fn rewrite(&self,items:&[I],usage:BufferUsage){
        let data_ref=(items as *const [I]) as *const core::ffi::c_void;
        // Arguments:
        // 1 - the target (the type of the buffer)
        // 2 - the size of the vertices
        // 3 - the data
        // 4 - type of managing the data
        BufferData(self.target,(items.len()*size_of::<I>()) as isize,data_ref,usage as u32);
    }
}