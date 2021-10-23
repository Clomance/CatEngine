use crate::graphics::{
    GLCore,
    core::GLError,
    core::buffer::{
        BufferTarget,
        BufferIndexedTarget,
        BufferUsage,
    },
};

use core::{
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
    pub fn generate()->Buffer<I>{
        unsafe{
            let mut id:u32=MaybeUninit::uninit().assume_init();
            GLCore.buffer.generate_one(&mut id);

            Self{
                id,
                marker:PhantomData,
            }
        }
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
            GLCore.buffer.bind(target,self.id);
            GLCore.get_error()
        }
    }

    /// Binds the zero-named buffer object.
    /// 
    /// Returns `GLError::NoError` if no error has accured.
    /// 
    /// 
    #[inline(always)]
    pub fn unbind(target:BufferTarget)->GLError{
        unsafe{
            GLCore.buffer.bind(target,0);
            GLCore.get_error()
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
            GLCore.buffer.bind_base(target,index,self.id);
            GLCore.get_error()
        }
    }

    /// Binds a buffer to the specified binding with the specified target and buffer range.
    /// 
    /// Returns `GLError::NoError` if no error has accured.
    #[inline(always)]
    pub fn bind_range(&self,target:BufferIndexedTarget,index:u32,start:isize,count:isize)->GLError{
        unsafe{
            GLCore.buffer.bind_range(target,index,self.id,start,count);
            GLCore.get_error()
        }
    }
}

impl<I:Sized> Buffer<I>{
    /// Updates a subset of a buffer object's data store.
    /// 
    /// `size` is the number of bytes.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if `offset` or `size` is negative,
    /// or if together they define a region of memory
    /// that extends beyond the buffer object's allocated data store.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if the reserved buffer object name 0 is bound to `target`,
    /// or if the buffer object being updated is mapped.
    pub fn write_raw(target:BufferTarget,offset:isize,size:isize,data:&I)->GLError{
        unsafe{
            GLCore.buffer.write(target,offset,size,data);
            GLCore.get_error()
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
    pub fn write(target:BufferTarget,offset:isize,data:&[I])->GLError{
        unsafe{
            let offset=size_of::<I>() as isize*offset;
            let size=size_of::<I>()*data.len();
            let data=&*data.as_ptr();
            GLCore.buffer.write::<I>(target,offset,size as isize,data);
            GLCore.get_error()
        }
    }

    /// Creates and initializes a buffer object's data store.
    /// 
    /// `size` is the number of bytes.
    /// 
    /// Returns `GLError::NoError` if no error has accured.
    /// 
    /// Returns `GLError::InvalidValue` if `size` is negative.
    /// 
    /// Returns `GLError::InvalidOperation` if the reserved buffer object name 0 is bound to target.
    /// 
    /// Returns `GLError::OutOfMemory` if the GL is unable to create a data store with the specified size.
    pub fn rewrite_raw(target:BufferTarget,size:isize,data:Option<&I>,usage:BufferUsage)->GLError{
        unsafe{
            GLCore.buffer.rewrite(target,size,data,usage);
            GLCore.get_error()
        }
    }

    /// Creates and initializes a buffer object's data store.
    /// 
    /// Returns `GLError::NoError` if no error has accured.
    /// 
    /// Returns `GLError::InvalidOperation` if the reserved buffer object name 0 is bound to target.
    /// 
    /// Returns `GLError::OutOfMemory` if the GL is unable to create a data store with the specified size.
    pub fn rewrite(target:BufferTarget,data:&[I],usage:BufferUsage)->GLError{
        unsafe{
            let size=size_of::<I>()*data.len();
            let data=&*data.as_ptr();
            GLCore.buffer.rewrite::<I>(target,size as isize,Some(data),usage);
            GLCore.get_error()
        }
    }

    /// Creates an uninitialized buffer object's data store.
    /// 
    /// `size` is the number of items.
    /// 
    /// Returns `GLError::NoError` if no error has accured.
    /// 
    /// Returns `GLError::InvalidValue` if `size` is negative.
    /// 
    /// Returns `GLError::InvalidOperation` if the reserved buffer object name 0 is bound to target.
    /// 
    /// Returns `GLError::OutOfMemory` if the GL is unable to create a data store with the specified size.
    pub fn rewrite_empty(target:BufferTarget,size:isize,usage:BufferUsage)->GLError{
        unsafe{
            GLCore.buffer.rewrite::<I>(target,size*size_of::<I>() as isize,None,usage);
            GLCore.get_error()
        }
    }
}

impl<I:Sized> Drop for Buffer<I>{
    fn drop(&mut self){
        unsafe{
            GLCore.buffer.delete_one(&self.id);
        }
    }
}