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

pub struct IndexBuffer<T>{
    buffer:Buffer<T>
}

impl<T> IndexBuffer<T>{
    pub fn new(data:&[T],usage:BufferUsage)->IndexBuffer<T>{
        unsafe{
            let buffer=Buffer::new();

            buffer.bind(BufferTarget::ElementArrayBuffer);

            Buffer::allocate(BufferTarget::ElementArrayBuffer,(size_of::<T>()*data.len()) as isize,data.get_unchecked(0),usage);

            Self{
                buffer
            }
        }
    }

    pub fn empty(capacity:usize,usage:BufferUsage)->IndexBuffer<T>{
        unsafe{
            let buffer=Buffer::new();

            buffer.bind(BufferTarget::ElementArrayBuffer);

            Buffer::<T>::allocate(BufferTarget::ElementArrayBuffer,(size_of::<T>()*capacity) as isize,null(),usage);

            Self{
                buffer
            }
        }
    }

    pub fn bind(&self){
        unsafe{
            self.buffer.bind(BufferTarget::ElementArrayBuffer)
        }
    }

    pub fn id(&self)->u32{
        self.buffer.id()
    }
}

impl<T> IndexBuffer<T>{
    pub fn reallocate(&self,data:&[T],usage:BufferUsage)->Result<(),Error>{
        unsafe{
            self.buffer.bind(BufferTarget::ElementArrayBuffer);
            Buffer::allocate(BufferTarget::ElementArrayBuffer,(size_of::<T>()*data.len()) as isize,data.get_unchecked(0),usage);
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
            self.buffer.bind(BufferTarget::ElementArrayBuffer);
            Buffer::write(BufferTarget::ElementArrayBuffer,(size_of::<T>()*start) as isize,(size_of::<T>()*data.len()) as isize,data.get_unchecked(0));
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