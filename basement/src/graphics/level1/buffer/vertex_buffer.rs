use crate::graphics::{
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
    pub fn initiate()->VertexBuffer<V>{
        Self{
            buffer:Buffer::initiate(),
        }
    }

    #[inline(always)]
    pub fn new(vertices:&[V],usage:BufferUsage)->VertexBuffer<V>{
        unsafe{
            Self{
                buffer:Buffer::new(BufferTarget::ArrayBuffer,vertices,usage),
            }
        }
    }

    #[inline(always)]
    pub fn empty(size:isize,usage:BufferUsage)->VertexBuffer<V>{
        unsafe{
            Self{
                buffer:Buffer::empty(BufferTarget::ArrayBuffer,size,usage),
            }
        }
    }

    #[inline(always)]
    pub fn raw(&self)->&Buffer<V>{
        &self.buffer
    }

    #[inline(always)]
    pub fn into_raw(self)->Buffer<V>{
        self.buffer
    }

    #[inline(always)]
    pub fn bind(&self){
        self.buffer.bind(BufferTarget::ArrayBuffer).unwrap()
    }

    #[inline(always)]
    pub fn write(&self,offset:isize,vertices:&[V]){
        self.buffer.write(BufferTarget::ArrayBuffer,offset,vertices).unwrap()
    }

    #[inline(always)]
    pub fn rewrite(&self,vertices:&[V],usage:BufferUsage){
        self.buffer.rewrite(BufferTarget::ArrayBuffer,vertices,usage).unwrap()
    }
}