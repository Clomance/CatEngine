#[cfg(any(windows))]
use crate::windows::OpenGraphicsLibrary;

use core::mem::transmute;

use std::ffi::CStr;

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

// Vertex components
const BGRA:u32=0x80E1;

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

#[repr(u32)]
#[derive(Clone,Copy)]
pub enum IntegerDataType{
    I8=BYTE,
    U8=UNSIGNED_BYTE,
    I16=SHORT,
    U16=UNSIGNED_SHORT,
    I32=INT,
    U32=UNSIGNED_INT,
}

#[repr(i32)]
#[derive(Clone,Copy)]
pub enum VertexComponents{
    One=1i32,
    Two=2i32,
    Three=3i32,
    Four=4i32,
    BGRA=BGRA as i32,
}

#[repr(i32)]
#[derive(Clone,Copy)]
pub enum VertexIntegerComponents{
    One=1i32,
    Two=2i32,
    Three=3i32,
    Four=4i32,
}

pub struct VertexArray{
    glGenVertexArrays:usize,
    glDeleteVertexArrays:usize,

    glBindVertexArray:usize,

    glVertexAttribPointer:usize,
    glVertexAttribIPointer:usize,

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
            glVertexAttribIPointer:0,

            glEnableVertexAttribArray:0,
            glDisableVertexAttribArray:0,
        }
    }

    #[cfg(any(windows))]
    pub fn load(&mut self,library:&OpenGraphicsLibrary){
        unsafe{
            self.glGenVertexArrays=transmute(library.get_proc_address("glGenVertexArrays\0"));
            self.glDeleteVertexArrays=transmute(library.get_proc_address("glDeleteVertexArrays\0"));

            self.glBindVertexArray=transmute(library.get_proc_address("glBindVertexArray\0"));

            self.glVertexAttribPointer=transmute(library.get_proc_address("glVertexAttribPointer\0"));
            self.glVertexAttribIPointer=transmute(library.get_proc_address("glVertexAttribIPointer\0"));

            self.glEnableVertexAttribArray=transmute(library.get_proc_address("glEnableVertexAttribArray\0"));
            self.glDisableVertexAttribArray=transmute(library.get_proc_address("glDisableVertexAttribArray\0"));
        }
    }
}

impl VertexArray{
    /// Generates a vertex array object name.
    #[inline(always)]
    pub fn generate_one(&self,array:&mut u32){
        unsafe{
            transmute::<usize,fn(i32,&mut u32)>(self.glGenVertexArrays)(1,array)
        }
    }

    /// Deletes a vertex array object.
    /// 
    /// If a vertex array object that is currently bound is deleted,
    /// the binding for that object reverts to zero and the default vertex array becomes current.
    /// 
    /// Unused names in arrays are silently ignored, as is the value zero.
    #[inline(always)]
    pub fn delete_one(&self,array:&u32){
        unsafe{
            transmute::<usize,fn(i32,&u32)>(self.glDeleteVertexArrays)(1,array)
        }
    }

    /// Generates vertex array object names.
    #[inline(always)]
    pub fn generate(&self,arrays:&mut [u32]){
        unsafe{
            transmute::<usize,fn(i32,&mut u32)>(self.glGenVertexArrays)(arrays.len() as i32,&mut arrays[0])
        }
    }

    /// Deletes vertex array objects
    /// 
    /// If a vertex array object that is currently bound is deleted,
    /// the binding for that object reverts to zero and the default vertex array becomes current.
    /// 
    /// Unused names in arrays are silently ignored, as is the value zero.
    #[inline(always)]
    pub fn delete(&self,arrays:&[u32]){
        unsafe{
            transmute::<usize,fn(i32,&u32)>(self.glDeleteVertexArrays)(arrays.len() as i32,&arrays[0])
        }
    }

    /// Binds a vertex array object.
    /// 
    ///  If the bind is successful no change is made to the state of the vertex array object,
    /// and any previous vertex array object binding is broken.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if array is not zero or the name of a vertex array object previously
    /// returned from a call to `VertexArray::generate`.
    #[inline(always)]
    pub unsafe fn bind(&self,array_id:u32){
        transmute::<usize,fn(u32)>(self.glBindVertexArray)(array_id)
    }

    /// Define an array of generic vertex attribute data.
    /// 
    /// `index` - Specifies the index of the generic vertex attribute to be modified.
    /// 
    /// `offset` - Specifies an offset of the first component of the first generic vertex attribute
    /// in the array in the data store of the buffer currently bound to the `BufferTarget::ArrayBuffer` target.
    /// The initial value is `0`.
    /// 
    /// `size` - Specifies the number of components per generic vertex attribute.
    /// The initial value is `4`.
    /// 
    /// `stride` - Specifies the byte offset between consecutive generic vertex attributes.
    /// If stride is `0`, the generic vertex attributes are understood to be tightly packed in the array.
    /// The initial value is `0`.
    /// 
    /// `data_type` - Specifies the data type of each component in the array.
    /// 
    /// `normalized` - Specifies whether fixed-point data values should be normalized (`true`)
    /// or converted directly as fixed-point values (`false`) when they are accessed.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if index is greater than or equal to `GL_MAX_VERTEX_ATTRIBS`,
    /// if stride is negative.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if `size` is `VertexComponents::BGRA` and type is not `DataType::U8`,
    /// `DataType::I32_2_10_10_10_REV` or `DataType::U32_2_10_10_10_REV`,
    /// if `type` is `DataType::I32_2_10_10_10_REV` or `DataType::U32_2_10_10_10_REV`
    /// and `size` is not `VertexComponents::Four` or `VertexComponents::BGRA`,
    /// if `size` is `DataType::BGRA` and noramlized is `false`,
    /// if zero is bound to the `BufferTarget::ArrayBuffer` buffer object binding point
    /// and `offset` is not `0`
    /// (Note: In the core context, the old method of passing `glVertexAttribPointer`
    /// and `glDrawArrays` pointers to mesh data in main memory is no longer allowed.
    /// You must create a Vertex Buffer Object and fill it with your mesh data.),
    /// if there is no Vertex Array Object bound.
    #[inline(always)]
    pub unsafe fn attribute_pointer(
        &self,
        index:u32,
        offset:isize,
        size:VertexComponents,
        stride:i32,
        data_type:DataType,
        normalized:bool
    ){
        transmute::<usize,fn(
            u32,
            VertexComponents,
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

    /// Define an array of generic vertex attribute data.
    /// 
    /// Only the integer types.
    /// Values are always left as integer values.
    /// 
    /// `index` - Specifies the index of the generic vertex attribute to be modified.
    /// 
    /// `offset` - Specifies an offset of the first component of the first generic vertex attribute
    /// in the array in the data store of the buffer currently bound to the `BufferTarget::ArrayBuffer` target.
    /// The initial value is `0`.
    /// 
    /// `size` - Specifies the number of components per generic vertex attribute.
    /// The initial value is `4`.
    /// 
    /// `stride` - Specifies the byte offset between consecutive generic vertex attributes.
    /// If stride is `0`, the generic vertex attributes are understood to be tightly packed in the array.
    /// The initial value is `0`.
    /// 
    /// `data_type` - Specifies the data type of each component in the array.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if index is greater than or equal to `GL_MAX_VERTEX_ATTRIBS`,
    /// if stride is negative.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if zero is bound to the `BufferTarget::ArrayBuffer` buffer object binding point
    /// and `offset` is not `0`
    /// (note: in the core context, the old method of passing `glVertexAttribPointer`
    /// and `glDrawArrays` pointers to mesh data in main memory is no longer allowed;
    /// you must create a Vertex Buffer Object and fill it with your mesh data),
    /// if there is no Vertex Array Object bound.
    #[inline(always)]
    pub unsafe fn attribute_pointer_integer(
        &self,
        index:u32,
        offset:isize,
        size:VertexIntegerComponents,
        stride:i32,
        data_type:IntegerDataType,
    ){
        transmute::<usize,fn(
            u32,
            VertexIntegerComponents,
            IntegerDataType,
            i32,
            isize
        )>(self.glVertexAttribIPointer)(
            index,
            size,
            data_type,
            stride,
            offset
        )
    }

    /// Enables a generic vertex attribute array.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if index is greater than or equal to `GL_MAX_VERTEX_ATTRIBS`.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if there is no current vertex array object.
    #[inline(always)]
    pub unsafe fn enable_attribute(&self,index:u32){
        transmute::<usize,fn(u32)>(self.glEnableVertexAttribArray)(index)
    }

    /// Disables a generic vertex attribute array.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if index is greater than or equal to `GL_MAX_VERTEX_ATTRIBS`.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if there is no current vertex array object.
    #[inline(always)]
    pub unsafe fn disable_attribute(&self,index:u32){
        transmute::<usize,fn(u32)>(self.glDisableVertexAttribArray)(index)
    }
}