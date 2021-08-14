use crate::graphics::{
    core::GLError,
    core::buffer::{
        BufferTarget,
        BufferIndexedTarget,
        BufferUsage,
    },
    level0::Buffer,
};

pub struct VertexBuffer<V:Sized>{
    buffer:Buffer<V>
}

impl<V:Sized> VertexBuffer<V>{
    #[inline(always)]
    pub fn generate()->VertexBuffer<V>{
        Self{
            buffer:Buffer::generate(),
        }
    }

    #[inline(always)]
    pub unsafe fn raw(buffer:Buffer<V>)->VertexBuffer<V>{
        Self{
            buffer,
        }
    }

    pub fn new(vertices:&[V],usage:BufferUsage)->Result<VertexBuffer<V>,GLError>{
        let buffer=VertexBuffer::generate();
        let result=buffer.rewrite(vertices,usage);
        if result.is_error(){
            Err(result)
        }
        else{
            Ok(buffer)
        }
    }

    pub fn empty(size:isize,usage:BufferUsage)->Result<VertexBuffer<V>,GLError>{
        let buffer=VertexBuffer::generate();
        let result=buffer.rewrite_empty(size,usage);
        if result.is_error(){
            Err(result)
        }
        else{
            Ok(buffer)
        }
    }

    #[inline(always)]
    pub fn as_raw(&self)->&Buffer<V>{
        &self.buffer
    }

    #[inline(always)]
    pub fn into_raw(self)->Buffer<V>{
        self.buffer
    }

    #[inline(always)]
    pub fn bind(&self)->GLError{
        self.buffer.bind(BufferTarget::ArrayBuffer)
    }

    pub fn write(&self,offset:isize,vertices:&[V])->GLError{
        let result=self.bind();
        if result.is_error(){
            result
        }
        else{
            Buffer::write(BufferTarget::ArrayBuffer,offset,vertices)
        }
    }

    /// Creates and initializes a buffer object's data store.
    /// 
    /// Returns `GLError::NoError` if no error has accured.
    /// 
    /// Returns `GLError::InvalidOperation` if the reserved buffer object name 0 is bound to target.
    /// 
    /// Returns `GLError::OutOfMemory` if the GL is unable to create a data store with the specified size.
    /// 
    /// Panics if `data` is empty.
    pub fn rewrite(&self,vertices:&[V],usage:BufferUsage)->GLError{
        let result=self.bind();
        if result.is_error(){
            result
        }
        else{
            Buffer::rewrite(BufferTarget::ArrayBuffer,vertices,usage)
        }
    }

    pub fn rewrite_empty(&self,size:isize,usage:BufferUsage)->GLError{
        let result=self.bind();
        if result.is_error(){
            result
        }
        else{
            Buffer::<V>::rewrite_empty(BufferTarget::ArrayBuffer,size,usage)
        }
    }
}