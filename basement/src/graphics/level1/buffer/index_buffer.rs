use crate::graphics::{
    core::GLError,
    core::buffer::{
        BufferTarget,
        BufferUsage,
    },
    level0::Buffer,
};

pub struct IndexBuffer<I:Sized>{
    buffer:Buffer<I>,
}

impl<I:Sized> IndexBuffer<I>{
    #[inline(always)]
    pub fn generate()->IndexBuffer<I>{
        Self{
            buffer:Buffer::generate(),
        }
    }

    #[inline(always)]
    pub unsafe fn raw(buffer:Buffer<I>)->IndexBuffer<I>{
        Self{
            buffer,
        }
    }

    pub fn new(indices:&[I],usage:BufferUsage)->Result<IndexBuffer<I>,GLError>{
        let buffer=IndexBuffer::generate();
        let result=buffer.rewrite(indices,usage);
        if result.is_error(){
            Err(result)
        }
        else{
            Ok(buffer)
        }
    }

    pub fn empty(size:isize,usage:BufferUsage)->Result<IndexBuffer<I>,GLError>{
        let buffer=IndexBuffer::generate();
        let result=buffer.rewrite_empty(size,usage);
        if result.is_error(){
            Err(result)
        }
        else{
            Ok(buffer)
        }
    }

    #[inline(always)]
    pub fn as_raw(&self)->&Buffer<I>{
        &self.buffer
    }

    #[inline(always)]
    pub fn into_raw(self)->Buffer<I>{
        self.buffer
    }

    #[inline(always)]
    pub fn bind(&self)->GLError{
        self.buffer.bind(BufferTarget::ElementArrayBuffer)
    }

    pub fn write(&self,offset:isize,indices:&[I])->GLError{
        let result=self.bind();
        if result.is_error(){
            result
        }
        else{
            Buffer::write(BufferTarget::ElementArrayBuffer,offset,indices)
        }
    }

    pub fn rewrite(&self,indices:&[I],usage:BufferUsage)->GLError{
        let result=self.bind();
        if result.is_error(){
            result
        }
        else{
            Buffer::rewrite(BufferTarget::ElementArrayBuffer,indices,usage)
        }
    }

    pub fn rewrite_empty(&self,size:isize,usage:BufferUsage)->GLError{
        let result=self.bind();
        if result.is_error(){
            result
        }
        else{
            Buffer::<I>::rewrite_empty(BufferTarget::ElementArrayBuffer,size,usage)
        }
    }
}