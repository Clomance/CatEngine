use crate::graphics::{
    GCore,
    core::GLError,
    core::drawing::{
        PrimitiveType,
        IndexType,
        AvailableIndexType,
    },
};

pub struct Drawing;

impl Drawing{
    /// Renders primitives from array data.
    /// 
    /// `GLError::InvalidValue` is generated if `count` is negative.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if a non-zero buffer object name is bound to an enabled array
    /// and the buffer object's data store is currently mapped,
    /// or if a geometry shader is active and mode is incompatible
    /// with the input primitive type of the geometry shader in the currently installed program object.
    pub fn draw_arrays(start:i32,count:i32,mode:PrimitiveType)->GLError{
        unsafe{
            GCore.drawing.draw_arrays(start,count,mode);
            GCore.get_error()
        }
    }

    /// `GLError::InvalidValue` is generated if `count` is negative.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if a non-zero buffer object name is bound to an enabled array or the element array
    /// and the buffer object's data store is currently mapped,
    /// or if a geometry shader is active and mode is incompatible
    /// with the input primitive type of the geometry shader in the currently installed program object.
    pub fn draw_elements(start:i32,count:i32,index_type:IndexType,mode:PrimitiveType)->GLError{
        unsafe{
            GCore.drawing.draw_elements(start,count,index_type,mode);
            GCore.get_error()
        }
    }

    /// `GLError::InvalidValue` is generated if `count` is negative.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if a non-zero buffer object name is bound to an enabled array or the element array
    /// and the buffer object's data store is currently mapped,
    /// or if a geometry shader is active and mode is incompatible
    /// with the input primitive type of the geometry shader in the currently installed program object.
    pub fn draw_elements_typed<T:AvailableIndexType>(start:i32,count:i32,mode:PrimitiveType)->GLError{
        unsafe{
            GCore.drawing.draw_elements_typed::<T>(start,count,mode);
            GCore.get_error()
        }
    }

    /// Renders multiple sets of primitives from array data.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if a non-zero buffer object name is bound to an enabled array
    /// and the buffer object's data store is currently mapped.
    pub fn multi_draw_arrays(&self,start:&[i32],count:&[i32],mode:PrimitiveType)->GLError{
        unsafe{
            GCore.drawing.multi_draw_arrays(start,count,mode);
            GCore.get_error()
        }
    }

    /// Renders multiple sets of primitives by specifying indices of array data elements.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if a non-zero buffer object name is bound to an enabled array or the element array
    /// and the buffer object's data store is currently mapped.
    pub fn multi_draw_elements(
        &self,
        start:&[isize],
        count:&[i32],
        index_type:IndexType,
        mode:PrimitiveType
    )->GLError{
        unsafe{
            GCore.drawing.multi_draw_elements(start,count,index_type,mode);
            GCore.get_error()
        }
    }

    /// Renders multiple sets of primitives by specifying indices of array data elements.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if a non-zero buffer object name is bound to an enabled array or the element array
    /// and the buffer object's data store is currently mapped.
    pub fn multi_draw_elements_typed<T:AvailableIndexType>(
        &self,
        start:&[isize],
        count:&[i32],
        mode:PrimitiveType
    )->GLError{
        unsafe{
            GCore.drawing.multi_draw_elements_typed::<T>(start,count,mode);
            GCore.get_error()
        }
    }
}