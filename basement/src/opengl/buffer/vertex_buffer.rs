use super::{
    Core,
    Buffer,
    BufferTarget,
    BufferUsage,
    Error,
};

use core::{
    mem::size_of,
    marker::PhantomData,
    ptr::null,
};

pub struct VertexBuffer<T>{
    buffer:Buffer<T>
}

impl<T> VertexBuffer<T>{
    pub fn new(data:&[T],usage:BufferUsage)->VertexBuffer<T>{
        unsafe{
            let buffer=Buffer::new();

            buffer.bind(BufferTarget::ArrayBuffer);

            Buffer::allocate(BufferTarget::ArrayBuffer,(size_of::<T>()*data.len()) as isize,data.get_unchecked(0),usage);

            Self{
                buffer
            }
        }
    }

    pub fn empty(capacity:usize,usage:BufferUsage)->VertexBuffer<T>{
        unsafe{
            let buffer=Buffer::new();

            buffer.bind(BufferTarget::ArrayBuffer);

            Buffer::<T>::allocate(BufferTarget::ArrayBuffer,(size_of::<T>()*capacity) as isize,null(),usage);

            Self{
                buffer
            }
        }
    }

    pub fn bind(&self){
        unsafe{
            self.buffer.bind(BufferTarget::ArrayBuffer)
        }
    }

    pub fn id(&self)->u32{
        self.buffer.id()
    }
}

impl<T> VertexBuffer<T>{
    pub fn reallocate(&self,data:&[T],usage:BufferUsage)->Result<(),Error>{
        unsafe{
            self.buffer.bind(BufferTarget::ArrayBuffer);
            Buffer::allocate(BufferTarget::ArrayBuffer,(size_of::<T>()*data.len()) as isize,data.get_unchecked(0),usage);
            let error=Core::get_error();
            if let Error::None=error{
                Result::Ok(())
            }
            else{
                Result::Err(error)
            }
        }
    }

    pub fn write(&self,start:usize,data:&[T])->Result<(),Error>{
        unsafe{
            self.buffer.bind(BufferTarget::ArrayBuffer);
            Buffer::write(BufferTarget::ArrayBuffer,(size_of::<T>()*start) as isize,(size_of::<T>()*data.len()) as isize,data.get_unchecked(0));
            let error=Core::get_error();
            if let Error::None=error{
                Result::Ok(())
            }
            else{
                Result::Err(error)
            }
        }
    }

    pub fn read(&self,start:usize,data:&mut [T])->Result<(),Error>{
        unsafe{
            self.buffer.bind(BufferTarget::ArrayBuffer);
            Buffer::read(BufferTarget::ArrayBuffer,(size_of::<T>()*start) as isize,(size_of::<T>()*data.len()) as isize,data.get_unchecked_mut(0));
            let error=Core::get_error();
            if let Error::None=error{
                Result::Ok(())
            }
            else{
                Result::Err(error)
            }
        }
    }
}