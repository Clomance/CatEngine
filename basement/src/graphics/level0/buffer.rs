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
    BindBufferBase,
    BindBufferRange,
    BufferData as glBufferData,
    BufferSubData,
    DeleteBuffers,
    CopyBufferSubData,
};

use std::mem::transmute;

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

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum BufferTarget{
    /// Vertex attributes.
    ArrayBuffer=ARRAY_BUFFER,
    /// Atomic counter storage.
    /// 
    /// Since OpenGL 4.2.
    AtomicCounterBuffer=ATOMIC_COUNTER_BUFFER,
    /// Buffer copy source.
    /// 
    /// Since OpenGL 3.1.
    CopyReadBuffer=COPY_READ_BUFFER,
    /// Buffer copy destination.
    /// 
    /// Since OpenGL 3.1.
    CopyWriteBuffer=COPY_WRITE_BUFFER,
    /// Indirect compute dispatch commands.
    /// 
    /// Since OpenGL 4.3.
    DispatchIndirectBuffer=DISPATCH_INDIRECT_BUFFER,
    /// Indirect command arguments.
    DrawIndirectBuffer=DRAW_INDIRECT_BUFFER,
    /// Vertex array indices.
    ElementArrayBuffer=ELEMENT_ARRAY_BUFFER,
    /// Pixel read target.
    PixelPackBuffer=PIXEL_PACK_BUFFER,
    /// Texture data source.
    PixelUnpackBuffer=PIXEL_UNPACK_BUFFER,
    /// Query result buffer.
    /// 
    /// Since OpenGL 4.4.
    QueryBuffer=QUERY_BUFFER,
    /// Read-write storage for shaders.
    /// 
    /// Since OpenGL 4.3.
    ShaderStorageBuffer=SHADER_STORAGE_BUFFER,
    /// Texture data buffer.
    /// 
    /// Since OpenGL 3.1.
    TextureBuffer=TEXTURE_BUFFER,
    /// Transform feedback buffer.
    TransformFeedbackBuffer=TRANSFORM_FEEDBACK_BUFFER,
    /// Uniform block storage.
    /// 
    /// Since OpenGL 3.1.
    UniformBuffer=UNIFORM_BUFFER,
}

pub trait BufferData{
    /// Returns the whole size of data.
    fn size(&self)->isize;
    /// Returns a pointer to data.
    fn ptr(&self)->*const core::ffi::c_void;
    /// Returns an offset of `items` elements.
    /// 
    /// `Result = items * element_size`
    fn offset(&self,items:isize)->isize;
}

impl<I:Sized> BufferData for &'_ I{
    fn size(&self)->isize{
        size_of::<I>() as isize
    }

    fn ptr(&self)->*const core::ffi::c_void{
        unsafe{
            transmute(*self)
        }
    }

    fn offset(&self,items:isize)->isize{
        size_of::<I>() as isize*items
    }
}

impl<I:Sized> BufferData for &'_ [I]{
    fn size(&self)->isize{
        (self.len()*size_of::<I>()) as isize
    }

    fn ptr(&self)->*const core::ffi::c_void{
        unsafe{
            transmute(&self[0])
        }
    }

    fn offset(&self,items:isize)->isize{
        size_of::<I>() as isize*items
    }
}

/// A low-level buffer.
/// 
/// All the errors are ignored.
/// If you want to check whether any error accured,
/// use the `GLError::get_error()` function.
pub struct Buffer<I:Sized>{
    id:u32,
    marker:PhantomData<I>,
}

impl<I:Sized> Buffer<I>{
    /// Creates a buffer without memory allocation.
    pub fn initialize()->Buffer<I>{
        unsafe{
            let mut id:u32=MaybeUninit::uninit().assume_init();
            GenBuffers(1,&mut id);

            Self{
                id,
                marker:PhantomData,
            }
        }
    }

    /// Creates a buffer with `size` capacity and writes data to it.
    /// 
    /// The size is in bytes.
    pub unsafe fn raw(target:BufferTarget,data:*const I,size:isize,usage:BufferUsage)->Buffer<I>{
        let buffer=Buffer::initialize();
        buffer.bind(target).rewrite_raw(data,size,usage);
        buffer
    }

    /// Creates a buffer with capacity equal to the data size and writes data to it.
    pub unsafe fn new<Data:BufferData>(target:BufferTarget,data:Data,usage:BufferUsage)->Buffer<I>{
        let buffer=Buffer::initialize();
        buffer.bind(target).rewrite(data,usage);
        buffer
    }

    /// Creates a buffer with `size * size_of::<I>()` capacity in bytes.
    pub unsafe fn empty(target:BufferTarget,size:isize,usage:BufferUsage)->Buffer<I>{
        let buffer=Buffer::initialize();
        buffer.bind(target).rewrite_empty(size,usage);
        buffer
    }

    /// Returns the identifier of a buffer.
    #[inline(always)]
    pub fn id(&self)->u32{
        self.id
    }
}

impl<I:Sized> Buffer<I>{
    /// Binds a buffer to the specified target.
    pub unsafe fn bind<'a>(&'a self,target:BufferTarget)->BoundBuffer<'a,I>{
        BindBuffer(target as u32,self.id);
        BoundBuffer{
            target:target as u32,
            marker:PhantomData,
        }
    }

    /// Binds a buffer to the specified binding with the specified target.
    /// 
    /// Only these targets are available:
    /// - `BufferTarget::AtomicCounterBuffer`,
    /// - `BufferTarget::ShaderStorageBuffer`,
    /// - `BufferTarget::TransformFeedbackBuffer`,
    /// - `BufferTarget::UniformBuffer`.
    #[inline(always)]
    pub unsafe fn bind_base(&self,target:BufferTarget,binding_index:u32){
        BindBufferBase(target as u32,binding_index,self.id)
    }

    /// Binds a buffer to the specified binding with the specified target and buffer range.
    /// 
    /// The offset and size are in bytes.
    /// 
    /// Only these targets are available:
    /// - `BufferTarget::AtomicCounterBuffer`,
    /// - `BufferTarget::ShaderStorageBuffer`,
    /// - `BufferTarget::TransformFeedbackBuffer`,
    /// - `BufferTarget::UniformBuffer`.
    #[inline(always)]
    pub unsafe fn bind_range(&self,target:BufferTarget,binding_index:u32,offset:isize,size:isize){
        BindBufferRange(target as u32,binding_index,self.id,offset,size)
    }
}

impl<I:Sized> Drop for Buffer<I>{
    fn drop(&mut self){
        unsafe{
            DeleteBuffers(1,&self.id as *const u32);
        }
    }
}

/// A representation of a bound buffer.
/// 
/// Does not guarantee a buffer be bound all the time.
/// 
/// ```
/// let b1 = buffer1.bind(BufferTarget::ArrayBuffer);
/// 
/// // `b1` bound
/// b1.write(0,data);
/// 
/// let b2 = buffer2.bind(BufferTarget::ArrayBuffer);
/// 
/// // `b2` bound, but `b1` not bound
/// b1.write(0,data); // this will do the same
/// b2.write(0,data); // as this
/// ```
pub struct BoundBuffer<'a,I:Sized>{
    target:u32,
    marker:PhantomData<&'a I>,
}

impl<'a,I:Sized> BoundBuffer<'a,I>{
    /// Writes data to a buffer.
    /// 
    /// The offset and size are in bytes.
    #[inline(always)]
    pub unsafe fn write_raw(&self,data:*const I,offset:isize,size:isize){
        BufferSubData(self.target,offset,size,data as *const core::ffi::c_void)
    }

    /// Writes data to a buffer.
    /// 
    /// The offset is the amount of items.
    pub unsafe fn write<Data:BufferData>(&self,offset:isize,data:Data){
        BufferSubData(self.target,data.offset(offset),data.size(),data.ptr())
    }

    /// Reallocates and rewrites data of a buffer.
    /// 
    /// The new capacity is `size` in bytes.
    #[inline(always)]
    pub unsafe fn rewrite_raw(&self,data:*const I,size:isize,usage:BufferUsage){
        glBufferData(self.target,size,data as *const core::ffi::c_void,usage as u32)
    }

    /// Reallocates and rewrites data of a buffer.
    /// 
    /// The new capacity is the data size.
    pub unsafe fn rewrite<Data:BufferData>(&self,data:Data,usage:BufferUsage){
        glBufferData(self.target,data.size(),data.ptr(),usage as u32);
    }

    /// Reallocates data of a buffer.
    /// 
    /// The new capacity is `size * size_of::<I>()` in bytes.
    pub unsafe fn rewrite_empty(&self,size:isize,usage:BufferUsage){
        glBufferData(self.target,size*size_of::<I>() as isize,core::ptr::null(),usage as u32);
    }
}