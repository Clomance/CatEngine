#[cfg(target_os="windows")]
use crate::windows::OpenGraphicsLibrary;

use core::mem::transmute;

// Index value types
const UNSIGNED_BYTE:u32=0x1401;
const UNSIGNED_SHORT:u32=0x1403;
const UNSIGNED_INT:u32=0x1405;

// Primitive types
const POINTS:u32=0x0000;
const LINES:u32=0x0001;
const LINE_LOOP:u32=0x0002;
const LINE_STRIP:u32=0x0003;
const TRIANGLES:u32=0x0004;
const TRIANGLE_STRIP:u32=0x0005;
const TRIANGLE_FAN:u32=0x0006;
const LINES_ADJACENCY:u32=0x000A;
const LINE_STRIP_ADJACENCY:u32=0x000B;
const TRIANGLES_ADJACENCY:u32=0x000C;
const TRIANGLE_STRIP_ADJACENCY:u32=0x000D;
const PATCHES:u32=0x000E;

#[repr(u32)]
#[derive(Clone,Copy)]
pub enum IndexType{
    U8=UNSIGNED_BYTE,
    U16=UNSIGNED_SHORT,
    U32=UNSIGNED_INT,
}

/// Specifies the type of indices.
/// 
/// Don't implement for any other types.
pub trait AvailableIndexType:Sized{
    fn gl_enum()->u32;

    fn offset(elements:isize)->isize{
        core::mem::size_of::<Self>() as isize*elements
    }
}

impl AvailableIndexType for u8{
    #[inline(always)]
    fn gl_enum()->u32{
        IndexType::U8 as u32
    }
}

impl AvailableIndexType for u16{
    #[inline(always)]
    fn gl_enum()->u32{
        IndexType::U16 as u32
    }
}

impl AvailableIndexType for u32{
    #[inline(always)]
    fn gl_enum()->u32{
        IndexType::U32 as u32
    }
}

/// Specifies the kind of primitives.
#[repr(u32)]
#[derive(Clone,Copy)]
pub enum PrimitiveType{
    Points=POINTS,
    Lines=LINES,
    LineLoop=LINE_LOOP,
    LineStrip=LINE_STRIP,
    Triangles=TRIANGLES,
    TriangleStrip=TRIANGLE_STRIP,
    TriangleFan=TRIANGLE_FAN,
    LinesAdjacency=LINES_ADJACENCY,
    TrianglesAdjacency=TRIANGLES_ADJACENCY,
    TriangleStripAdjacency=TRIANGLE_STRIP_ADJACENCY,
}

pub struct Drawing{
    glDrawArrays:usize,
    glDrawElements:usize,
    glMultiDrawArrays:usize,
    glMultiDrawElements:usize,
}

impl Drawing{
    pub const fn new()->Drawing{
        Self{
            glDrawArrays:0,
            glDrawElements:0,
            glMultiDrawArrays:0,
            glMultiDrawElements:0,
        }
    }

    #[cfg(target_os="windows")]
    pub fn load(&mut self,library:&OpenGraphicsLibrary){
        unsafe{
            self.glDrawArrays=transmute(library.get_proc_address("glDrawArrays\0"));
            self.glDrawElements=transmute(library.get_proc_address("glDrawElements\0"));
            self.glMultiDrawArrays=transmute(library.get_proc_address("glMultiDrawArrays\0"));
            self.glMultiDrawElements=transmute(library.get_proc_address("glMultiDrawElements\0"));
        }
    }
}

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
    #[inline(always)]
    pub unsafe fn draw_arrays(&self,start:i32,count:i32,mode:PrimitiveType){
        transmute::<usize,fn(PrimitiveType,i32,i32)>(self.glDrawArrays)(mode,start,count)
    }

    /// Render primitives from array data.
    /// 
    /// `GLError::InvalidValue` is generated if `count` is negative.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if a non-zero buffer object name is bound to an enabled array or the element array
    /// and the buffer object's data store is currently mapped,
    /// or if a geometry shader is active and mode is incompatible
    /// with the input primitive type of the geometry shader in the currently installed program object.
    pub unsafe fn draw_elements(&self,start:i32,count:i32,index_type:IndexType,mode:PrimitiveType){
        let offset=match index_type{
            IndexType::U8=>u8::offset(start as isize),
            IndexType::U16=>u16::offset(start as isize),
            IndexType::U32=>u32::offset(start as isize),
        };
        transmute::<usize,fn(PrimitiveType,i32,IndexType,isize)>(self.glDrawElements)(mode,count,index_type,offset)
    }

    /// Renders primitives from array data.
    /// 
    /// `GLError::InvalidValue` is generated if `count` is negative.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if a non-zero buffer object name is bound to an enabled array or the element array
    /// and the buffer object's data store is currently mapped,
    /// or if a geometry shader is active and mode is incompatible
    /// with the input primitive type of the geometry shader in the currently installed program object.
    pub unsafe fn draw_elements_typed<T:AvailableIndexType>(&self,start:i32,count:i32,mode:PrimitiveType){
        let offset=T::offset(start as isize);
        let gl_enum=T::gl_enum();
        transmute::<usize,fn(PrimitiveType,i32,u32,isize)>(self.glDrawElements)(mode,count,gl_enum,offset)
    }

    /// Renders multiple sets of primitives from array data.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if a non-zero buffer object name is bound to an enabled array
    /// and the buffer object's data store is currently mapped.
    #[inline(always)]
    pub unsafe fn multi_draw_arrays(&self,start:&[i32],count:&[i32],mode:PrimitiveType){
        transmute::<usize,fn(PrimitiveType,&i32,&i32,i32)>(self.glMultiDrawArrays)(mode,&start[0],&count[0],start.len() as i32)
    }

    /// Renders multiple sets of primitives by specifying indices of array data elements.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if a non-zero buffer object name is bound to an enabled array or the element array
    /// and the buffer object's data store is currently mapped.
    #[inline(always)]
    pub unsafe fn multi_draw_elements(
        &self,
        start:&[isize],
        count:&[i32],
        index_type:IndexType,
        mode:PrimitiveType
    ){
        transmute::<usize,fn(PrimitiveType,&i32,IndexType,&isize,i32)>(self.glMultiDrawElements)(
            mode,
            &count[0],
            index_type,
            &start[0],
            start.len() as i32
        )
    }

    /// Renders multiple sets of primitives by specifying indices of array data elements.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if a non-zero buffer object name is bound to an enabled array or the element array
    /// and the buffer object's data store is currently mapped.
    pub unsafe fn multi_draw_elements_typed<T:AvailableIndexType>(
        &self,
        start:&[isize],
        count:&[i32],
        mode:PrimitiveType
    ){
        transmute::<usize,fn(PrimitiveType,&i32,u32,&isize,i32)>(self.glMultiDrawElements)(
            mode,
            &count[0],
            T::gl_enum(),
            &start[0],
            start.len() as i32
        )
    }
}