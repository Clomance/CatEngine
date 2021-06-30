use super::level0::{
    Buffer,
    BoundBuffer,
    BufferTarget,
    BufferUsage
};

pub struct VertexBuffer<V:Sized>{
    buffer:Buffer<V>
}

impl<V:Sized> VertexBuffer<V>{
    #[inline(always)]
    pub fn initialize()->VertexBuffer<V>{
        Self{
            buffer:Buffer::initialize(),
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
    pub fn empty(size:usize,usage:BufferUsage)->VertexBuffer<V>{
        unsafe{
            Self{
                buffer:Buffer::empty(BufferTarget::ArrayBuffer,size as isize,usage),
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
}

impl<V:Sized> VertexBuffer<V>{
    #[inline(always)]
    pub fn bind(&self)->BoundVertexBuffer<V>{
        unsafe{
            BoundVertexBuffer{
                marker:self.buffer.bind(BufferTarget::ArrayBuffer)
            }
        }
    }
}

pub struct BoundVertexBuffer<'a,V:Sized>{
    marker:BoundBuffer<'a,V>
}

impl<'a,V> BoundVertexBuffer<'a,V>{
    #[inline(always)]
    pub fn raw(&self)->&BoundBuffer<'a,V>{
        &self.marker
    }

    #[inline(always)]
    pub fn into_raw(self)->BoundBuffer<'a,V>{
        self.marker
    }

    #[inline(always)]
    pub fn write(&self,offset:usize,vertices:&[V]){
        unsafe{
            self.marker.write(offset as isize,vertices)
        }
    }

    #[inline(always)]
    pub fn rewrite(&self,vertices:&[V],usage:BufferUsage){
        unsafe{
            self.marker.rewrite(vertices,usage)
        }
    }
}