use gl::{
    // constants
    UNSIGNED_BYTE,
    UNSIGNED_SHORT,
    UNSIGNED_INT,
    // Primitive types
    POINTS,
    LINES,
    LINE_LOOP,
    LINE_STRIP,
    TRIANGLES,
    TRIANGLE_STRIP,
    TRIANGLE_FAN,
    LINES_ADJACENCY,
    TRIANGLES_ADJACENCY,
    TRIANGLE_STRIP_ADJACENCY,
    PATCHES,
    // functions
    DrawArrays,
    DrawElements,
    MultiDrawArrays,
    MultiDrawElements,
};

use core::mem::transmute;

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
    /// Can only be used when tessellation is active.
    Patches=PATCHES,
}

pub struct Drawing;

impl Drawing{
    #[inline(always)]
    pub fn draw_arrays(&self,start:i32,count:i32,mode:PrimitiveType){
        unsafe{
            DrawArrays(mode as u32,start,count)
        }
    }

    pub fn draw_elements(&self,start:i32,count:i32,index_type:IndexType,mode:PrimitiveType){
        unsafe{
            let offset=match index_type{
                IndexType::U8=>u8::offset(start as isize),
                IndexType::U16=>u16::offset(start as isize),
                IndexType::U32=>u32::offset(start as isize),
            };

            DrawElements(mode as u32,count as i32,index_type as u32,offset as *const _)
        }
    }

    #[inline]
    pub fn draw_elements_typed<T:AvailableIndexType>(&self,start:i32,count:i32,mode:PrimitiveType){
        unsafe{
            let offset=T::offset(start as isize);
            let gl_enum=T::gl_enum();
            DrawElements(mode as u32,count as i32,gl_enum,offset as *const _)
        }
    }

    #[inline(always)]
    pub fn multi_draw_arrays(&self,start:&[i32],count:&[i32],mode:PrimitiveType){
        unsafe{
            MultiDrawArrays(mode as u32,&start[0],&count[0],start.len() as i32)
        }
    }

    #[inline(always)]
    pub fn multi_draw_elements(
        &self,
        start:&[isize],
        count:&[i32],
        index_type:IndexType,
        mode:PrimitiveType
    ){
        unsafe{
            MultiDrawElements(
                mode as u32,
                transmute(&count[0]),
                index_type as u32,
                transmute(&start[0]),
                start.len() as i32
            )
        }
    }

    pub fn multi_draw_elements_typed<T:AvailableIndexType>(
        &self,
        start:&[isize],
        count:&[i32],
        mode:PrimitiveType
    ){
        unsafe{
            MultiDrawElements(
                mode as u32,
                transmute(&count[0]),
                T::gl_enum(),
                transmute(&start[0]),
                start.len() as i32
            )
        }
    }
}