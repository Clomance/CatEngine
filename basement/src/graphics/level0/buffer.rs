use crate::graphics::{
    GCore,
    core::GLError,
    core::buffer::{
        BufferTarget,
        BufferIndexedTarget,
        BufferUsage,
    },
};

use std::{
    marker::PhantomData,
    mem::{
        size_of,
        MaybeUninit
    },
};

pub struct Buffer<I:Sized>{
    id:u32,
    marker:PhantomData<*const I>,
}

impl<I:Sized> Buffer<I>{
    /// Creates a buffer without memory allocation.
    pub fn initiate()->Buffer<I>{
        unsafe{
            let mut id:u32=MaybeUninit::uninit().assume_init();
            GCore.buffer.generate_one(&mut id);

            Self{
                id,
                marker:PhantomData,
            }
        }
    }

    /// Creates a buffer with `size` capacity and writes data to it.
    /// 
    /// The size is in bytes.
    pub unsafe fn new_raw(target:BufferTarget,size:isize,data:*const I,usage:BufferUsage)->Buffer<I>{
        let buffer=Buffer::initiate();
        buffer.rewrite_raw(target,size,data,usage).unwrap();
        buffer
    }

    /// Creates a buffer with capacity equal to the data size and writes data to it.
    pub unsafe fn new(target:BufferTarget,data:&[I],usage:BufferUsage)->Buffer<I>{
        let buffer=Buffer::initiate();
        buffer.rewrite(target,data,usage).unwrap();
        buffer
    }

    /// Creates a buffer with `size * size_of::<I>()` capacity.
    pub unsafe fn empty(target:BufferTarget,size:isize,usage:BufferUsage)->Buffer<I>{
        let buffer=Buffer::initiate();
        buffer.rewrite_empty(target,size,usage).unwrap();
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
    /// 
    /// Returns `GLError::NoError` if no error has accured.
    /// 
    /// Returns `GLError::InvalidValue` is generated
    /// if `buffer` is not a name previously returned from a call to `Buffer::generate()`.
    pub fn bind(&self,target:BufferTarget)->GLError{
        unsafe{
            GCore.buffer.bind(target,self.id);
            GCore.get_error()
        }
    }

    /// Binds the zero-named buffer object.
    #[inline(always)]
    pub fn unbind(&self,target:BufferTarget){
        unsafe{
            GCore.buffer.bind(target,0)
        }
    }

    /// Binds a buffer to the specified binding with the specified target.
    /// 
    /// Returns `GLError::NoError` if no error has accured.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if `buffer` is not a name previously returned from a call to `Buffer::generate()`,
    /// if `index` is greater than or equal to the number of target-specific indexed binding points,
    /// if buffer does not have an associated data store, or if the size of that store is zero.
    #[inline(always)]
    pub fn bind_base(&self,target:BufferIndexedTarget,index:u32)->GLError{
        unsafe{
            GCore.buffer.bind_base(target,index,self.id);
            GCore.get_error()
        }
    }

    /// Binds a buffer to the specified binding with the specified target and buffer range.
    /// 
    /// Returns `GLError::NoError` if no error has accured.
    #[inline(always)]
    pub fn bind_range(&self,target:BufferIndexedTarget,index:u32,start:isize,count:isize)->GLError{
        unsafe{
            GCore.buffer.bind_range(target,index,self.id,start,count);
            GCore.get_error()
        }
    }
}

impl<I:Sized> Buffer<I>{
    /// Updates a subset of a buffer object's data store.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if `offset` or `size` is negative,
    /// or if together they define a region of memory
    /// that extends beyond the buffer object's allocated data store.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if the reserved buffer object name 0 is bound to `target`,
    /// or if the buffer object being updated is mapped.
    #[inline(always)]
    pub fn write_raw(&self,target:BufferTarget,offset:isize,size:isize,data:*const I)->GLError{
        unsafe{
            self.bind(target).unwrap();
            GCore.buffer.write(target,offset,size,data);
            self.unbind(target);
            GCore.get_error()
        }
    }

    /// Updates a subset of a buffer object's data store.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if `offset` is negative,
    /// or if `offset` and the size of data define a region of memory
    /// that extends beyond the buffer object's allocated data store.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if the reserved buffer object name 0 is bound to `target`,
    /// or if the buffer object being updated is mapped.
    /// 
    /// Panics if `data` is empty.
    pub fn write(&self,target:BufferTarget,offset:isize,data:&[I])->GLError{
        unsafe{
            self.bind(target).unwrap();
            let offset=size_of::<I>() as isize*offset;
            let size=size_of::<I>()*data.len();
            GCore.buffer.write(target,offset,size as isize,&data[0]);
            self.unbind(target);
            GCore.get_error()
        }
    }

    /// Creates and initializes a buffer object's data store.
    /// 
    /// Returns `GLError::NoError` if no error has accured.
    /// 
    /// Returns `GLError::InvalidValue` if `size` is negative.
    /// 
    /// Returns `GLError::InvalidOperation` if the reserved buffer object name 0 is bound to target.
    /// 
    /// Returns `GLError::OutOfMemory` if the GL is unable to create a data store with the specified size.
    #[inline(always)]
    pub fn rewrite_raw(&self,target:BufferTarget,size:isize,data:*const I,usage:BufferUsage)->GLError{
        unsafe{
            self.bind(target).unwrap();
            GCore.buffer.rewrite(target,size,data,usage);
            self.unbind(target);
            GCore.get_error()
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
    pub fn rewrite(&self,target:BufferTarget,data:&[I],usage:BufferUsage)->GLError{
        unsafe{
            self.bind(target).unwrap();
            let size=size_of::<I>()*data.len();
            GCore.buffer.rewrite(target,size as isize,&data[0],usage);
            self.unbind(target);
            GCore.get_error()
        }
    }

    /// Creates an uninitialized buffer object's data store.
    /// 
    /// The store size is `size * size_of::<I>()`.
    /// 
    /// Returns `GLError::NoError` if no error has accured.
    /// 
    /// Returns `GLError::InvalidValue` if `size` is negative.
    /// 
    /// Returns `GLError::InvalidOperation` if the reserved buffer object name 0 is bound to target.
    /// 
    /// Returns `GLError::OutOfMemory` if the GL is unable to create a data store with the specified size.
    pub fn rewrite_empty(&self,target:BufferTarget,size:isize,usage:BufferUsage)->GLError{
        unsafe{
            self.bind(target).unwrap();
            GCore.buffer.rewrite::<I>(target,size*size_of::<I>() as isize,core::ptr::null(),usage);
            self.unbind(target);
            GCore.get_error()
        }
    }
}

impl<I:Sized> Drop for Buffer<I>{
    fn drop(&mut self){
        unsafe{
            GCore.buffer.delete_one(&self.id);
        }
    }
}