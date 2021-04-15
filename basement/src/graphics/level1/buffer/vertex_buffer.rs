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
    pub fn initialize()->VertexBuffer<V>{
        Self{
            buffer:Buffer::initialize(),
        }
    }

    pub fn new(vertices:&[V],usage:BufferUsage)->VertexBuffer<V>{
        unsafe{
            Self{
                buffer:Buffer::new(BufferTarget::ArrayBuffer,vertices,usage),
            }
        }
    }

    /// The size is the amount of vertices.
    pub fn empty(size:usize,usage:BufferUsage)->VertexBuffer<V>{
        unsafe{
            Self{
                buffer:Buffer::empty(BufferTarget::ArrayBuffer,size,usage),
            }
        }
    }
}

impl<V:Sized> VertexBuffer<V>{
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
    pub fn write(&self,offset:usize,vertices:&[V]){
        unsafe{
            self.marker.write(offset,vertices)
        }
    }

    pub fn rewrite(&self,vertices:&[V],usage:BufferUsage){
        unsafe{
            self.marker.rewrite(vertices,usage)
        }
    }
}