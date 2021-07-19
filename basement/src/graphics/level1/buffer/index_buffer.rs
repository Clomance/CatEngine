use crate::graphics::{
    core::buffer::{
        BufferTarget,
        BufferIndexedTarget,
        BufferUsage,
    },
    level0::Buffer,
};

use std::marker::PhantomData;

pub struct IndexBuffer<I:Sized>{
    buffer:Buffer<I>,
}

impl<I:Sized> IndexBuffer<I>{
    #[inline(always)]
    pub fn initiate()->IndexBuffer<I>{
        Self{
            buffer:Buffer::initiate(),
        }
    }

    #[inline(always)]
    pub fn new(indices:&[I],usage:BufferUsage)->IndexBuffer<I>{
        unsafe{
            Self{
                buffer:Buffer::new(BufferTarget::ElementArrayBuffer,indices,usage),
            }
        }
    }

    #[inline(always)]
    pub fn empty(size:isize,usage:BufferUsage)->IndexBuffer<I>{
        unsafe{
            Self{
                buffer:Buffer::empty(BufferTarget::ElementArrayBuffer,size,usage),
            }
        }
    }

    #[inline(always)]
    pub fn raw(&self)->&Buffer<I>{
        &self.buffer
    }

    #[inline(always)]
    pub fn into_raw(self)->Buffer<I>{
        self.buffer
    }

    #[inline(always)]
    pub fn bind(&self){
        self.buffer.bind(BufferTarget::ElementArrayBuffer).unwrap()
    }

    #[inline(always)]
    pub fn write(&self,offset:isize,indices:&[I]){
        self.buffer.write(BufferTarget::ElementArrayBuffer,offset,indices).unwrap()
    }

    #[inline(always)]
    pub fn rewrite(&self,indices:&[I],usage:BufferUsage){
        self.buffer.rewrite(BufferTarget::ElementArrayBuffer,indices,usage).unwrap()
    }
}