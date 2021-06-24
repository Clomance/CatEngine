use super::level0::{
    Buffer,
    BoundBuffer,
    BufferTarget,
    BufferUsage,
};

pub struct IndexBuffer<I:Sized>{
    buffer:Buffer<I>,
}

impl<I:Sized> IndexBuffer<I>{
    #[inline(always)]
    pub fn initialize()->IndexBuffer<I>{
        Self{
            buffer:Buffer::initialize(),
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
    pub fn empty(size:usize,usage:BufferUsage)->IndexBuffer<I>{
        unsafe{
            Self{
                buffer:Buffer::empty(BufferTarget::ElementArrayBuffer,size as isize,usage),
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
}

impl<I:Sized> IndexBuffer<I>{
    #[inline(always)]
    pub fn bind(&self)->BoundIndexBuffer<I>{
        unsafe{
            BoundIndexBuffer{
                marker:self.buffer.bind(BufferTarget::ElementArrayBuffer)
            }
        }
    }
}

pub struct BoundIndexBuffer<'a,I:Sized>{
    marker:BoundBuffer<'a,I>
}

impl<'a,I> BoundIndexBuffer<'a,I>{
    #[inline(always)]
    pub fn write(&self,offset:usize,indices:&[I]){
        unsafe{
            self.marker.write(offset as isize,indices)
        }
    }

    #[inline(always)]
    pub fn rewrite(&self,indices:&[I],usage:BufferUsage){
        unsafe{
            self.marker.rewrite(indices,usage)
        }
    }
}