#[cfg(target_os="windows")]
use crate::windows::OpenGraphicsLibrary;

use core::mem::transmute;

use std::{
    ffi::CStr,
    mem::MaybeUninit,
};

// Data types
const BYTE:u32=0x1400;
const UNSIGNED_BYTE:u32=0x1401;
const SHORT:u32=0x1402;
const UNSIGNED_SHORT:u32=0x1403;
const INT:u32=0x1404;
const UNSIGNED_INT:u32=0x1405;
const FLOAT:u32=0x1406;
const DOUBLE:u32=0x140A;
const HALF_FLOAT:u32=0x140B;
const INT_2_10_10_10_REV:u32=0x8D9F;
const UNSIGNED_INT_2_10_10_10_REV:u32=0x8368;

/// Specifies the data type of each component in an array.
#[repr(u32)]
#[derive(Clone,Copy)]
pub enum DataType{
    I8=BYTE,
    U8=UNSIGNED_BYTE,
    I16=SHORT,
    U16=UNSIGNED_SHORT,
    I32=INT,
    U32=UNSIGNED_INT,
    F16=HALF_FLOAT,
    F32=FLOAT,
    F64=DOUBLE,
    I32_2_10_10_10_REV=INT_2_10_10_10_REV,
    U32_2_10_10_10_REV=UNSIGNED_INT_2_10_10_10_REV,
}

pub struct VertexArray{
    glGenVertexArrays:usize,
    glDeleteVertexArrays:usize,

    glBindVertexArray:usize,

    glVertexAttribPointer:usize,
    glEnableVertexAttribArray:usize,
    glDisableVertexAttribArray:usize,
}

impl VertexArray{
    pub const fn new()->VertexArray{
        Self{
            glGenVertexArrays:0,
            glDeleteVertexArrays:0,

            glBindVertexArray:0,

            glVertexAttribPointer:0,
            glEnableVertexAttribArray:0,
            glDisableVertexAttribArray:0,
        }
    }

    #[cfg(target_os="windows")]
    pub fn load(&mut self,library:&OpenGraphicsLibrary){
        unsafe{
            self.glGenVertexArrays=transmute(library.get_proc_address("glGenVertexArrays\0"));
            self.glDeleteVertexArrays=transmute(library.get_proc_address("glDeleteVertexArrays\0"));

            self.glBindVertexArray=transmute(library.get_proc_address("glBindVertexArray\0"));

            self.glVertexAttribPointer=transmute(library.get_proc_address("glVertexAttribPointer\0"));
            self.glEnableVertexAttribArray=transmute(library.get_proc_address("glEnableVertexAttribArray\0"));
            self.glDisableVertexAttribArray=transmute(library.get_proc_address("glDisableVertexAttribArray\0"));
        }
    }
}

impl VertexArray{
    #[inline(always)]
    pub fn generate_one(&self,array:&mut u32){
        unsafe{
            transmute::<usize,fn(i32,&mut u32)>(self.glGenVertexArrays)(1,array)
        }
    }

    #[inline(always)]
    pub unsafe fn delete_one(&self,array:&u32){
        transmute::<usize,fn(i32,&u32)>(self.glDeleteVertexArrays)(1,array)
    }

    #[inline(always)]
    pub fn generate(&self,arrays:&mut [u32]){
        unsafe{
            transmute::<usize,fn(i32,&mut u32)>(self.glGenVertexArrays)(arrays.len() as i32,&mut arrays[0])
        }
    }

    #[inline(always)]
    pub unsafe fn delete(&self,arrays:&[u32]){
        transmute::<usize,fn(i32,&u32)>(self.glDeleteVertexArrays)(arrays.len() as i32,&arrays[0])
    }
    
    #[inline(always)]
    pub unsafe fn bind(&self,array_id:u32){
        transmute::<usize,fn(u32)>(self.glBindVertexArray)(array_id)
    }

    /// `index` - Specifies the index of the generic vertex attribute to be modified.
    /// 
    /// `offset` - Specifies an offset of the first component of the first generic vertex attribute
    /// in the array in the data store of the buffer currently bound to the `BufferTarget::ArrayBuffer` target.
    /// The initial value is 0.
    /// 
    /// `size` - Specifies the number of components per generic vertex attribute. Must be 1, 2, 3, 4.
    /// Additionally, the symbolic constant GL_BGRA is accepted. The initial value is 4.
    /// 
    /// `stride` - Specifies the byte offset between consecutive generic vertex attributes.
    /// If stride is 0, the generic vertex attributes are understood to be tightly packed in the array.
    /// The initial value is 0.
    /// 
    /// `data_type` - Specifies the data type of each component in the array.
    /// 
    /// `normalized` - Specifies whether fixed-point data values should be normalized (`true`)
    /// or converted directly as fixed-point values (`false`) when they are accessed.
    #[inline(always)]
    pub unsafe fn attribute_pointer(
        &self,
        index:u32,
        offset:isize,
        size:i32,
        stride:i32,
        data_type:DataType,
        normalized:bool
    ){
        transmute::<usize,fn(
            u32,
            i32,
            DataType,
            bool,
            i32,
            isize
        )>(self.glVertexAttribPointer)(
            index,
            size,
            data_type,
            normalized,
            stride,
            offset
        )
    }

    /// Enables a generic vertex attribute array.
    #[inline(always)]
    pub unsafe fn enable_attribute(&self,index:u32){
        transmute::<usize,fn(u32)>(self.glEnableVertexAttribArray)(index)
    }

    /// Disables a generic vertex attribute array
    #[inline(always)]
    pub unsafe fn disable_attribute(&self,index:u32){
        transmute::<usize,fn(u32)>(self.glDisableVertexAttribArray)(index)
    }
}