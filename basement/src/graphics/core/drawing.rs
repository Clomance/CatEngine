#[cfg(all(target_os="windows",feature="windows"))]
use crate::windows::OpenGraphicsLibrary;

use super::types::*;

use core::mem::transmute;

// Index value types
const UNSIGNED_BYTE:GLenum=0x1401;
const UNSIGNED_SHORT:GLenum=0x1403;
const UNSIGNED_INT:GLenum=0x1405;

// Primitive types
const POINTS:GLenum=0x0000;
const LINES:GLenum=0x0001;
const LINE_LOOP:GLenum=0x0002;
const LINE_STRIP:GLenum=0x0003;
const TRIANGLES:GLenum=0x0004;
const TRIANGLE_STRIP:GLenum=0x0005;
const TRIANGLE_FAN:GLenum=0x0006;
const LINES_ADJACENCY:GLenum=0x000A;
const LINE_STRIP_ADJACENCY:GLenum=0x000B;
const TRIANGLES_ADJACENCY:GLenum=0x000C;
const TRIANGLE_STRIP_ADJACENCY:GLenum=0x000D;
const PATCHES:GLenum=0x000E;

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
    fn gl_enum()->GLenum;

    fn index_type()->IndexType;

    fn offset(elements:isize)->isize{
        core::mem::size_of::<Self>() as isize*elements
    }
}

impl AvailableIndexType for u8{
    #[inline(always)]
    fn gl_enum()->GLenum{
        IndexType::U8 as GLenum
    }

    #[inline(always)]
    fn index_type()->IndexType{
        IndexType::U8
    }
}

impl AvailableIndexType for u16{
    #[inline(always)]
    fn gl_enum()->GLenum{
        IndexType::U16 as GLenum
    }

    #[inline(always)]
    fn index_type()->IndexType{
        IndexType::U16
    }
}

impl AvailableIndexType for u32{
    #[inline(always)]
    fn gl_enum()->GLenum{
        IndexType::U32 as GLenum
    }

    #[inline(always)]
    fn index_type()->IndexType{
        IndexType::U32
    }
}

/// Specifies the kind of primitives.
#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,PartialOrd,Eq,Ord)]
pub enum PrimitiveType{
    Points=POINTS,
    Lines=LINES,
    LineLoop=LINE_LOOP,
    LineStrip=LINE_STRIP,
    Triangles=TRIANGLES,
    TriangleStrip=TRIANGLE_STRIP,
    TriangleFan=TRIANGLE_FAN,

    /// Available only if the GL version is 3.2 or greater.
    LinesAdjacency=LINES_ADJACENCY,

    /// Available only if the GL version is 3.2 or greater.
    LineStripAdjacency=LINE_STRIP_ADJACENCY,

    /// Available only if the GL version is 3.2 or greater.
    TrianglesAdjacency=TRIANGLES_ADJACENCY,

    /// Available only if the GL version is 3.2 or greater.
    TriangleStripAdjacency=TRIANGLE_STRIP_ADJACENCY,
}

// #[cfg_attr(windows,link(name="opengl32"))]
#[cfg(target_os="linux")]
extern "system"{
    fn glDrawArrays(
        mode:GLenum,
        first:GLint,
        count:GLsizei
    )->();
    fn glDrawArraysInstanced(
        mode:GLenum,
        first:GLint,
        count:GLsizei,
        primcount:GLsizei
    )->();

    fn glDrawElements(
        mode:GLenum,
        count:GLsizei,
        index_type:GLenum,
        index_start:*const GLvoid
    )->();
    fn glDrawElementsBaseVertex(
        mode:GLenum,
        count:GLsizei,
        index_type:GLenum,
        index_start:*const GLvoid,
        base_vertex:i32
    )->();
    fn glDrawElementsInstanced(
        mode:GLenum,
        count:GLsizei,
        index_type:GLenum,
        index_start:*const GLvoid,
        primcount:GLint
    )->();
    fn glDrawElementsInstancedBaseVertex(
        mode:GLenum,
        count:GLsizei,
        index_type:GLenum,
        index_start:*const GLvoid,
        primcount:GLint,
        base_vertex:i32
    )->();

    fn glDrawRangeElements(
        mode:GLenum,
        start:u32,
        end:u32,
        count:i32,
        index_type:GLenum,
        indices:*const GLvoid
    )->();
    fn glDrawRangeElementsBaseVertex(
        mode:GLenum,
        start:u32,
        end:u32,
        count:i32,
        index_type:GLenum,
        indices:*const GLvoid,
        base_vertex:i32
    )->();

    fn glMultiDrawArrays(
        mode:GLenum,
        first:*const i32,
        count:*const i32,
        primcount:GLint
    )->();
    fn glMultiDrawElements(
        mode:GLenum,
        count:*const i32,
        index_type:GLenum,
        indices_start:*const *const GLvoid,
        primcount:GLint,
    )->();
    fn glMultiDrawElementsBaseVertex(
        mode:GLenum,
        count:*const GLsizei,
        index_type:GLenum,
        indices_start:*const *const GLvoid,
        primcount:GLint,
        base_vertex:*const GLint
    )->();
}

#[cfg(all(target_os="windows",feature="windows"))]
mod gl{
    pub static mut glDrawArrays:usize=0;
    pub static mut glDrawArraysInstanced:usize=0;

    pub static mut glDrawElements:usize=0;
    pub static mut glDrawElementsBaseVertex:usize=0;
    pub static mut glDrawElementsInstanced:usize=0;
    pub static mut glDrawElementsInstancedBaseVertex:usize=0;

    pub static mut glDrawRangeElements:usize=0;
    pub static mut glDrawRangeElementsBaseVertex:usize=0;

    pub static mut glMultiDrawArrays:usize=0;
    pub static mut glMultiDrawElements:usize=0;
    pub static mut glMultiDrawElementsBaseVertex:usize=0;
}

#[cfg(all(target_os="windows",feature="windows"))]
mod gl_functions{
    use super::*;

    pub unsafe extern "system" fn glDrawArrays(
        mode:GLenum,
        first:GLint,
        count:GLsizei
    ){
        transmute::<usize,fn(GLenum,GLint,GLsizei)>(gl::glDrawArrays)(
            mode,first,count
        )
    }
    pub unsafe extern "system" fn glDrawArraysInstanced(
        mode:GLenum,
        first:GLint,
        count:GLsizei,
        primcount:GLsizei
    ){
        transmute::<usize,fn(GLenum,GLint,GLsizei,GLsizei)>(
            gl::glDrawArraysInstanced
        )(
            mode,first,count,primcount
        )
    }

    pub unsafe extern "system" fn glDrawElements(
        mode:GLenum,
        count:GLsizei,
        index_type:GLenum,
        indices:*const GLvoid
    ){
        transmute::<usize,fn(GLenum,GLsizei,GLenum,*const GLvoid)>(
            gl::glDrawElements
        )(
            mode,count,index_type,indices
        )
    }
    pub unsafe extern "system" fn glDrawElementsBaseVertex(
        mode:GLenum,
        count:GLsizei,
        index_type:GLenum,
        indices:*const GLvoid,
        base_vertex:i32
    ){
        transmute::<usize,fn(GLenum,GLsizei,GLenum,*const GLvoid,i32)>(
            gl::glDrawElementsBaseVertex
        )(
            mode,count,index_type,indices,base_vertex
        )
    }
    pub unsafe extern "system" fn glDrawElementsInstanced(
        mode:GLenum,
        count:GLsizei,
        index_type:GLenum,
        indices:*const GLvoid,
        primcount:GLint
    ){
        transmute::<usize,fn(GLenum,GLsizei,GLenum,*const GLvoid,i32)>(
            gl::glDrawElementsInstanced
        )(
            mode,count,index_type,indices,primcount
        )
    }
    pub unsafe extern "system" fn glDrawElementsInstancedBaseVertex(
        mode:GLenum,
        count:GLsizei,
        index_type:GLenum,
        indices:*const GLvoid,
        primcount:GLint,
        base_vertex:i32
    ){
        transmute::<usize,fn(GLenum,GLsizei,GLenum,*const GLvoid,GLsizei,i32)>(
            gl::glDrawElementsInstancedBaseVertex
        )(
            mode,count,index_type,indices,primcount,base_vertex
        )
    }

    pub unsafe extern "system" fn glDrawRangeElements(
        mode:GLenum,
        start:u32,
        end:u32,
        count:GLsizei,
        index_type:GLenum,
        indices:*const GLvoid
    ){
        transmute::<usize,fn(GLenum,u32,u32,GLsizei,GLenum,*const GLvoid)>(
            gl::glDrawRangeElements
        )(
            mode,start,end,count,index_type,indices
        )
    }
    pub unsafe extern "system" fn glDrawRangeElementsBaseVertex(
        mode:GLenum,
        start:u32,
        end:u32,
        count:GLsizei,
        index_type:GLenum,
        indices:*const GLvoid,
        base_vertex:i32
    ){
        transmute::<usize,fn(GLenum,u32,u32,i32,GLenum,*const GLvoid,i32)>(
            gl::glDrawRangeElementsBaseVertex
        )(
            mode,start,end,count,index_type,indices,base_vertex
        )
    }

    pub unsafe extern "system" fn glMultiDrawArrays(
        mode:GLenum,
        first:*const GLint,
        count:*const GLsizei,
        primcount:GLint
    ){
        transmute::<usize,fn(GLenum,*const i32,*const i32,i32)>(
            gl::glMultiDrawArrays
        )(
            mode,first,count,primcount
        )
    }
    pub unsafe extern "system" fn glMultiDrawElements(
        mode:GLenum,
        count:*const GLsizei,
        index_type:GLenum,
        indices_start:*const *const GLvoid,
        primcount:GLint,
    ){
        transmute::<usize,fn(GLenum,*const i32,GLenum,*const *const GLvoid,i32)>(
            gl::glMultiDrawElements
        )(
            mode,count,index_type,indices_start,primcount
        )
    }
    pub unsafe extern "system" fn glMultiDrawElementsBaseVertex(
        mode:GLenum,
        count:*const GLsizei,
        index_type:GLenum,
        indices_start:*const *const GLvoid,
        primcount:GLint,
        base_vertex:*const GLint
    ){
        transmute::<usize,fn(GLenum,*const GLsizei,GLenum,*const *const GLvoid,GLint,*const GLint)>(
            gl::glMultiDrawElementsBaseVertex
        )(
            mode,count,index_type,indices_start,primcount,base_vertex
        )
    }
}

#[cfg(all(target_os="windows",feature="windows"))]
use gl_functions::*;

pub struct Drawing;

impl Drawing{
    pub const fn new()->Drawing{
        Self
    }

    #[cfg(all(target_os="windows",feature="windows"))]
    pub fn load(&mut self,library:&OpenGraphicsLibrary){
        unsafe{
            use gl::*;

            glDrawArrays=transmute(library.get_proc_address("glDrawArrays\0"));
            glDrawArraysInstanced=transmute(library.get_proc_address("glDrawArraysInstanced\0"));

            glDrawElements=transmute(library.get_proc_address("glDrawElements\0"));
            glDrawElementsBaseVertex=transmute(library.get_proc_address("glDrawElementsBaseVertex\0"));
            glDrawElementsInstanced=transmute(library.get_proc_address("glDrawElementsInstanced\0"));
            glDrawElementsInstancedBaseVertex=transmute(library.get_proc_address("glDrawElementsInstancedBaseVertex\0"));

            glDrawRangeElements=transmute(library.get_proc_address("glDrawRangeElements\0"));
            glDrawRangeElementsBaseVertex=transmute(library.get_proc_address("glDrawRangeElementsBaseVertex\0"));

            glMultiDrawArrays=transmute(library.get_proc_address("glMultiDrawArrays\0"));
            glMultiDrawElements=transmute(library.get_proc_address("glMultiDrawElements\0"));
            glMultiDrawElementsBaseVertex=transmute(library.get_proc_address("glMultiDrawElementsBaseVertex\0"));
        }
    }
}

impl Drawing{
    /// Renders primitives from array data.
    /// 
    /// `start` specifies the starting index in the enabled arrays.
    /// 
    /// `count` specifies the number of indices to be rendered.
    /// 
    /// `mode` specifies what kind of primitives to render.
    /// 
    /// Specifies multiple geometric primitives with very few subroutine calls.
    /// Instead of calling a GL procedure to pass each individual vertex, normal, texture coordinate, edge flag, or color,
    /// you can prespecify separate arrays of vertices, normals, and colors
    /// and use them to construct a sequence of primitives with a single call to `Drawing::draw_arrays`.
    /// 
    /// When `Drawing::draw_arrays` is called,
    /// it uses count sequential elements from each enabled array
    /// to construct a sequence of geometric primitives, beginning with element `start`.
    /// `mode` specifies what kind of primitives are constructed
    /// and how the array elements construct those primitives.
    /// 
    /// Vertex attributes that are modified by `Drawing::draw_arrays` have an unspecified value
    /// after `Drawing::draw_arrays` returns.
    /// Attributes that aren't modified remain well defined.
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
        glDrawArrays(mode as GLenum,start,count)
    }

    /// Draws multiple instances of a range of elements.
    /// 
    /// `start` specifies the starting index in the enabled arrays.
    /// 
    /// `count` specifies the number of indices to be rendered.
    /// 
    /// `instances` specifies the number of instances of the specified range of indices to be rendered.
    /// 
    /// `mode` specifies what kind of primitives to render.
    /// 
    /// Behaves identically to `Drawing::draw_arrays` except that `instances` instances of the range of elements are executed
    /// and the value of the internal counter instanceID advances for each iteration.
    /// `instanceID` is an internal 32-bit integer counter
    /// that may be read by a vertex shader as `gl_InstanceID`.
    /// 
    /// Has the same effect as:
    /// ```Rust
    /// if ( mode or count is invalid ){
    ///     generate appropriate error
    /// }
    /// else {
    ///     for i in 0..primcount {
    ///         instanceID = i;
    ///         Drawing::draw_arrays(start, count, mode);
    ///     }
    ///     instanceID = 0;
    /// }
    /// ```
    /// 
    /// Available only if the GL version is 3.1 or greater.
    /// 
    /// `GLError::InvalidValue` is generated if `count` or `instances` are negative.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if a non-zero buffer object name is bound to an enabled array
    /// and the buffer object's data store is currently mapped,
    /// or if a geometry shader is active and mode is incompatible
    /// with the input primitive type of the geometry shader in the currently installed program object.
    #[inline(always)]
    pub unsafe fn draw_arrays_instanced(&self,start:i32,count:i32,instances:i32,mode:PrimitiveType){
        glDrawArraysInstanced(mode as GLenum,start,count,instances)
    }
}

impl Drawing{
    /// Renders primitives from array data.
    /// 
    /// `start` specifies an offset of the first index in the array in the data store
    /// of the buffer currently bound to the `BufferTarget::ElementArrayBuffer` target.
    /// 
    /// `count` specifies the number of elements to be rendered.
    /// 
    /// `index_type` specifies the type of indices.
    /// 
    /// `mode` specifies what kind of primitives to render.
    /// 
    /// Specifies multiple geometric primitives with very few subroutine calls.
    /// Instead of calling a GL function to pass each individual vertex, normal, texture coordinate, edge flag, or color,
    /// you can prespecify separate arrays of vertices, normals, and so on,
    /// and use them to construct a sequence of primitives with a single call to `Drawing::draw_elements`.
    /// 
    /// Uses `count` sequential elements from an enabled array,
    /// starting at `start` to construct a sequence of geometric primitives.
    /// `mode` specifies what kind of primitives are constructed and how the array elements construct these primitives. If more than one array is enabled, each is used.
    /// 
    /// Vertex attributes that are modified by `Drawing::draw_elements` have an unspecified value after `Drawing::draw_elements` returns.
    /// Attributes that aren't modified maintain their previous values.
    /// 
    /// `GLError::InvalidValue` is generated if `count` is negative.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if a non-zero buffer object name is bound to an enabled array or the element array
    /// and the buffer object's data store is currently mapped,
    /// or if a geometry shader is active and mode is incompatible
    /// with the input primitive type of the geometry shader in the currently installed program object.
    #[inline(always)]
    pub unsafe fn draw_elements(
        &self,
        start:isize,
        count:i32,
        index_type:IndexType,
        mode:PrimitiveType
    ){
        glDrawElements(
            mode as GLenum,
            count,
            index_type as GLenum,
            start as *const _
        )
    }

    /// Same as `Drawing::draw_elements`, but with a static index type.
    #[inline(always)]
    pub unsafe fn draw_elements_typed<T:AvailableIndexType>(
        &self,
        start:isize,
        count:i32,
        mode:PrimitiveType
    ){
        glDrawElements(
            mode as GLenum,
            count,
            T::gl_enum(),
            start as *const _
        )
    }

    /// Renders primitives from array data with a per-element offset.
    /// 
    /// `start` specifies an offset of the first index in the array in the data store
    /// of the buffer currently bound to the `BufferTarget::ElementArrayBuffer` target.
    /// 
    /// `count` specifies the number of elements to be rendered.
    /// 
    /// `base_vertex` specifies a constant
    /// that should be added to each element of indices
    /// when chosing elements from the enabled vertex arrays.
    /// 
    /// `index_type` specifies the type of indices.
    /// 
    /// `mode` specifies what kind of primitives to render.
    /// 
    /// Behaves identically to `Drawing::draw_elements` except that the `i`th element transferred
    /// by the corresponding draw call will be taken from element `indices[i] + base_vertex` of each enabled array.
    /// If the resulting value is larger than the maximum value representable by `index_type`,
    /// it is as if the calculation were upconverted to 32-bit unsigned integers (with wrapping on overflow conditions).
    /// The operation is undefined if the sum would be negative.
    /// 
    /// Supported if the GL version is 3.2 or greater,
    /// or if the `ARB_draw_elements_base_vertex` extension is supported.
    /// 
    /// `GLError::InvalidValue` is generated if `count` is negative.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if a non-zero buffer object name is bound to an enabled array or the element array
    /// and the buffer object's data store is currently mapped,
    /// or if a geometry shader is active and mode is incompatible
    /// with the input primitive type of the geometry shader in the currently installed program object.
    #[inline(always)]
    pub unsafe fn draw_elements_base_vertex(
        &self,
        start:isize,
        count:i32,
        base_vertex:i32,
        index_type:IndexType,
        mode:PrimitiveType
    ){
        glDrawElementsBaseVertex(
            mode as GLenum,
            count,
            index_type as GLenum,
            start as *const _,
            base_vertex
        )
    }

    /// Same as `Drawing::draw_elements_base_vertex`, but with a static index type.
    #[inline(always)]
    pub unsafe fn draw_elements_base_vertex_typed<T:AvailableIndexType>(
        &self,
        start:isize,
        count:i32,
        base_vertex:i32,
        mode:PrimitiveType
    ){
        glDrawElementsBaseVertex(
            mode as GLenum,
            count,
            T::gl_enum(),
            start as *const _,
            base_vertex
        )
    }

    /// Draws multiple instances of a set of elements.
    /// 
    /// `start` specifies an offset of the first index in the array in the data store
    /// of the buffer currently bound to the `BufferTarget::ElementArrayBuffer` target.
    /// 
    /// `count` specifies the number of elements to be rendered.
    /// 
    /// `instances` specifies the number of instances of the specified range of indices to be rendered.
    /// 
    /// `index_type` specifies the type of indices.
    /// 
    /// `mode` specifies what kind of primitives to render.
    /// 
    /// Behaves identically to `Drawing::draw_elements` except that `instances` instances of the range of elements are executed
    /// and the value of the internal counter instanceID advances for each iteration.
    /// `instanceID` is an internal 32-bit integer counter
    /// that may be read by a vertex shader as `gl_InstanceID`.
    /// 
    /// Has the same effect as:
    /// ```Rust
    /// if ( mode as GLenum, count or index_type is invalid ){
    ///     generate appropriate error
    /// }
    /// else {
    ///     for i in 0..primcount {
    ///         instanceID = i;
    ///         Drawing::draw_elements(start, count, index_type, mode);
    ///     }
    ///     instanceID = 0;
    /// }
    /// ```
    /// 
    /// Available only if the GL version is 3.1 or greater.
    /// 
    /// `GLError::InvalidValue` is generated if `count` or `instances` are negative.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if a non-zero buffer object name is bound to an enabled array
    /// and the buffer object's data store is currently mapped,
    /// or if a geometry shader is active and mode is incompatible
    /// with the input primitive type of the geometry shader in the currently installed program object.
    #[inline(always)]
    pub unsafe fn draw_elements_instanced(
        &self,
        start:isize,
        count:i32,
        instances:i32,
        index_type:IndexType,
        mode:PrimitiveType
    ){
        glDrawElementsInstanced(
            mode as GLenum,
            count,
            index_type as GLenum,
            start as *const _,
            instances
        )
    }

    /// Same as `Drawing::draw_elements_instanced`, but with a static index type.
    #[inline(always)]
    pub unsafe fn draw_elements_instanced_typed<T:AvailableIndexType>(
        &self,
        start:isize,
        count:i32,
        instances:i32,
        mode:PrimitiveType
    ){
        glDrawElementsInstanced(
            mode as GLenum,
            count,
            T::gl_enum(),
            start as *const _,
            instances
        )
    }

    /// Renders multiple instances of a set of primitives from array data with a per-element offset.
    /// 
    /// `start` specifies an offset of the first index in the array in the data store
    /// of the buffer currently bound to the `BufferTarget::ElementArrayBuffer` target.
    /// 
    /// `count` specifies the number of elements to be rendered.
    /// 
    /// `base_vertex` specifies a constant
    /// that should be added to each element of indices
    /// when chosing elements from the enabled vertex arrays.
    /// 
    /// `instances` specifies the number of instances of the specified range of indices to be rendered.
    /// 
    /// `index_type` specifies the type of indices.
    /// 
    /// `mode` specifies what kind of primitives to render.
    /// 
    /// Behaves identically to `Drawing::draw_elements_instanced` except that the `i`th element transferred
    /// by the corresponding draw call will be taken from element `indices[i] + base_vertex` of each enabled array.
    /// If the resulting value is larger than the maximum value representable by `index_type`,
    /// it is as if the calculation were upconverted to 32-bit unsigned integers (with wrapping on overflow conditions).
    /// The operation is undefined if the sum would be negative.
    /// 
    /// Supported if the GL version is 3.2 or greater.
    /// 
    /// `GLError::InvalidValue` is generated if `count` or `instances` are negative.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if a non-zero buffer object name is bound to an enabled array
    /// and the buffer object's data store is currently mapped,
    /// or if a geometry shader is active and mode is incompatible
    /// with the input primitive type of the geometry shader in the currently installed program object.
    #[inline(always)]
    pub unsafe fn draw_elements_base_vertex_instanced(
        &self,
        start:isize,
        count:i32,
        base_vertex:i32,
        instances:i32,
        index_type:IndexType,
        mode:PrimitiveType
    ){
        glDrawElementsInstancedBaseVertex(
            mode as GLenum,
            count,
            index_type as GLenum,
            start as *const _,
            instances,
            base_vertex
        )
    }

    /// Same as `Drawing::draw_elements_base_vertex_instanced`, but with a static index type.
    #[inline(always)]
    pub unsafe fn draw_elements_base_vertex_instanced_typed<T:AvailableIndexType>(
        &self,
        start:isize,
        count:i32,
        base_vertex:i32,
        instances:i32,
        mode:PrimitiveType
    ){
        glDrawElementsInstancedBaseVertex(
            mode as GLenum,
            count,
            T::gl_enum(),
            start as *const _,
            instances,
            base_vertex
        )
    }
}

impl Drawing{
    /// Renders primitives from array data.
    /// 
    /// `start` specifies an offset of the first index in the array in the data store
    /// of the buffer currently bound to the `BufferTarget::ElementArrayBuffer` target.
    /// 
    /// `count` specifies the number of elements to be rendered.
    /// 
    /// `range_start` specifies the minimum array index contained in indices.
    /// 
    /// `range_end` specifies the maximum array index contained in indices.
    /// 
    /// `index_type` specifies the type of indices.
    /// 
    /// `mode` specifies what kind of primitives to render.
    /// 
    /// `Drawing::draw_range_elements` is a restricted form of `Drawing::draw_elements`.
    /// `mode`, `range_start`, `range_end`, and `count` match the corresponding arguments to `Drawing::draw_elements`,
    /// with the additional constraint that all values in the arrays count must lie between `range_start` and `range_end`, inclusive.
    /// 
    /// Implementations denote recommended maximum amounts of vertex and index data,
    /// which may be queried by calling `glGet` with argument `GL_MAX_ELEMENTS_VERTICES` and `GL_MAX_ELEMENTS_INDICES`.
    /// If `range_end−range_start+1` is greater than the value of `GL_MAX_ELEMENTS_VERTICES`,
    /// or if `count` is greater than the value of `GL_MAX_ELEMENTS_INDICES`, then the call may operate at reduced performance.
    /// There is no requirement that all vertices in the range `[range_start,range_end]` be referenced.
    /// However, the implementation may partially process unused vertices,
    /// reducing performance from what could be achieved with an optimal index set.
    /// 
    /// Uses `count` sequential elements from an enabled array,
    /// starting at `start` to construct a sequence of geometric primitives.
    /// `mode` specifies what kind of primitives are constructed,
    /// and how the array elements construct these primitives.
    /// If more than one array is enabled, each is used.
    /// 
    /// Vertex attributes that are modified by `Drawing::draw_range_elements` have an unspecified value after `Drawing::draw_range_elements` returns.
    /// Attributes that aren't modified maintain their previous values.
    /// 
    /// It is an error for indices to lie outside the range `[range_start,range_end]`,
    /// but implementations may not check for this situation.
    /// Such indices cause implementation-dependent behavior.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if `count` is negative,
    /// or if `range_end` < `range_start`.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if a non-zero buffer object name is bound to an enabled array
    /// and the buffer object's data store is currently mapped,
    /// or if a geometry shader is active and mode is incompatible
    /// with the input primitive type of the geometry shader in the currently installed program object.
    #[inline(always)]
    pub unsafe fn draw_range_elements(
        &self,
        start:isize,
        count:i32,
        range_start:u32,
        range_end:u32,
        index_type:IndexType,
        mode:PrimitiveType
    ){
        glDrawRangeElements(
            mode as GLenum,
            range_start,
            range_end,
            count,
            index_type as GLenum,
            start as *const _
        )
    }

    /// Same as `Drawing::draw_range_elements`, but with a static index type.
    #[inline(always)]
    pub unsafe fn draw_range_elements_typed<T:AvailableIndexType>(
        &self,
        start:isize,
        count:i32,
        range_start:u32,
        range_end:u32,
        mode:PrimitiveType
    ){
        glDrawRangeElements(
            mode as GLenum,
            range_start,
            range_end,
            count,
            T::gl_enum(),
            start as *const _
        )
    }

    /// Render primitives from array data with a per-element offset.
    /// 
    /// `start` specifies an offset of the first index in the array in the data store
    /// of the buffer currently bound to the `BufferTarget::ElementArrayBuffer` target.
    /// 
    /// `count` specifies the number of elements to be rendered.
    /// 
    /// `range_start` specifies the minimum array index contained in indices.
    /// 
    /// `range_end` specifies the maximum array index contained in indices.
    /// 
    /// `base_vertex` specifies a constant
    /// that should be added to each element of indices
    /// when chosing elements from the enabled vertex arrays.
    /// 
    /// `index_type` specifies the type of indices.
    /// 
    /// `mode` specifies what kind of primitives to render.
    /// 
    /// `Drawing::draw_range_elements_base_vertex` is a restricted form of `Drawing::draw_elements_base_vertex`.
    /// `mode`, `range_start`, `range_end`, `count` and `base_vertex` match the corresponding arguments to glDrawElementsBaseVertex,
    /// with the additional constraint that all values in the array indices must lie
    /// between `range_start`and `range_end`, inclusive, prior to adding basevertex.
    /// Index values lying outside the range `[range_start, range_end]` are treated in the same way as `Drawing::draw_elements_base_vertex`.
    /// The `i`th element transferred by the corresponding draw call will be taken from `element indices[i] + base_vertex` of each enabled array.
    /// If the resulting value is larger than the maximum value representable by type,
    /// it is as if the calculation were upconverted to 32-bit unsigned integers (with wrapping on overflow conditions).
    /// The operation is undefined if the sum would be negative.
    /// 
    /// Supported if the GL version is 3.2 or greater,
    /// or if the `ARB_draw_elements_base_vertex extension` is supported.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if `count` is negative,
    /// or if `range_end` < `range_start`.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if a non-zero buffer object name is bound to an enabled array
    /// and the buffer object's data store is currently mapped,
    /// or if a geometry shader is active and mode is incompatible
    /// with the input primitive type of the geometry shader in the currently installed program object.
    #[inline(always)]
    pub unsafe fn draw_range_elements_base_vertex(
        &self,
        start:isize,
        count:i32,
        range_start:u32,
        range_end:u32,
        index_type:IndexType,
        base_vertex:i32,
        mode:PrimitiveType
    ){
        glDrawRangeElementsBaseVertex(
            mode as GLenum,
            range_start,
            range_end,
            count,
            index_type as GLenum,
            start as *const _,
            base_vertex
        )
    }

    /// Same as `Drawing::draw_range_elements_base_vertex`, but with a static index type.
    #[inline(always)]
    pub unsafe fn draw_range_elements_base_vertex_typed<T:AvailableIndexType>(
        &self,
        start:isize,
        count:i32,
        range_start:u32,
        range_end:u32,
        base_vertex:i32,
        mode:PrimitiveType
    ){
        glDrawRangeElementsBaseVertex(
            mode as GLenum,
            range_start,
            range_end,
            count,
            T::gl_enum(),
            start as *const _,
            base_vertex
        )
    }
}

impl Drawing{
    /// Renders multiple sets of primitives from array data.
    /// 
    /// `start` points to an array of starting indices in the enabled arrays.
    /// 
    /// `count` points to an array of the number of indices to be rendered.
    /// 
    /// `size` specifies the size of the `start` and `count`.
    /// 
    /// `mode` specifies what kind of primitives to render.
    /// 
    /// Specifies multiple sets of geometric primitives with very few subroutine calls.
    /// Instead of calling a GL procedure to pass each individual vertex, normal, texture coordinate, edge flag, or color,
    /// you can prespecify separate arrays of vertices, normals, and colors
    /// and use them to construct a sequence of primitives with a single call to `Drawing::draw_arrays`.
    /// 
    /// Behaves identically to `Drawing::draw_arrays` except that `size` separate ranges of elements are specified instead.
    /// 
    /// It uses `count` sequential elements from each enabled array to construct a sequence of geometric primitives, beginning with element `start`.
    /// `mode` specifies what kind of primitives are constructed, and how the array elements construct those primitives.
    /// 
    /// Vertex attributes that are modified by `Drawing::multi_draw_arrays` have an unspecified value
    /// after `Drawing::multi_draw_arrays` returns.
    /// Attributes that aren't modified remain well defined.
    /// 
    /// `GLError::InvalidValue` is generated if `size` is negative.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if a non-zero buffer object name is bound to an enabled array
    /// and the buffer object's data store is currently mapped.
    #[inline(always)]
    pub unsafe fn multi_draw_arrays(
        &self,
        start:*const i32,
        count:*const i32,
        size:i32,
        mode:PrimitiveType
    ){
        glMultiDrawArrays(
            mode as GLenum,
            start,
            count,
            size
        )
    }

    /// Renders multiple sets of primitives by specifying indices of array data elements.
    /// 
    /// `start` points to an array of starting indices in the enabled arrays.
    /// 
    /// `count` points to an array of the number of indices to be rendered.
    /// 
    /// `size` specifies the size of the `start` and `count`.
    /// 
    /// `index_type` specifies the type of indices.
    /// 
    /// `mode` specifies what kind of primitives to render.
    /// 
    /// Specifies multiple sets of geometric primitives with very few subroutine calls.
    /// Instead of calling a GL function to pass each individual vertex, normal, texture coordinate, edge flag, or color,
    /// you can prespecify separate arrays of vertices, normals, and so on,
    /// and use them to construct a sequence of primitives with a single call to `Drawing::multi_draw_elements`.
    /// 
    /// `Drawing::multi_draw_elements` is identical in operation to `Drawing::draw_elements`
    /// except that `size` separate lists of elements are specified.
    /// 
    /// Vertex attributes that are modified by `Drawing::multi_draw_elements` have an unspecified value
    /// after `Drawing::multi_draw_elements` returns.
    /// Attributes that aren't modified maintain their previous values.
    /// 
    /// `GLError::InvalidValue` is generated if `size` is negative.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if a non-zero buffer object name is bound to an enabled array or the element array
    /// and the buffer object's data store is currently mapped.
    #[inline(always)]
    pub unsafe fn multi_draw_elements(
        &self,
        start:*const isize,
        count:*const i32,
        size:i32,
        index_type:IndexType,
        mode:PrimitiveType
    ){
        glMultiDrawElements(
            mode as GLenum,
            count,
            index_type as GLenum,
            start as *const _,
            size
        )
    }

    /// Same as `Drawing::multi_draw_elements`, but with a static index type.
    #[inline(always)]
    pub unsafe fn multi_draw_elements_typed<T:AvailableIndexType>(
        &self,
        start:*const isize,
        count:*const i32,
        size:i32,
        mode:PrimitiveType
    ){
        glMultiDrawElements(
            mode as GLenum,
            count,
            T::gl_enum(),
            start as *const _,
            size
        )
    }

    /// Renders multiple sets of primitives by specifying indices of array data elements and an index to apply to each index.
    /// 
    /// `start` points to an array of starting indices in the enabled arrays.
    /// 
    /// `count` points to an array of the number of indices to be rendered.
    /// 
    /// `size` specifies the size of the `start` and `count`.
    /// 
    /// `index_type` specifies the type of indices.
    /// 
    /// `mode` specifies what kind of primitives to render.
    /// 
    /// Behaves identically to `Drawing::draw_elements_base_vertex`,
    /// except that `size` separate lists of elements are specifried instead.
    /// 
    /// It has the same effect as:
    /// ```rust
    /// for i in 0..size{
    ///     if count[i] > 0{
    ///         Drawing::draw_elements_base_vertex(
    ///             start[i],
    ///             count[i],
    ///             base_vertex[i],
    ///             index_type,
    ///             mode
    ///         )
    ///     }
    /// }
    /// ```
    /// 
    /// Available only if the GL version is 3.1 or greater.
    /// 
    /// `GLError::InvalidValue` is generated if `size` is negative.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if a non-zero buffer object name is bound to an enabled array or the element array
    /// and the buffer object's data store is currently mapped.
    #[inline(always)]
    pub unsafe fn multi_draw_elements_base_vertex(
        &self,
        start:*const isize,
        count:*const i32,
        base_vertex:*const i32,
        size:i32,
        index_type:IndexType,
        mode:PrimitiveType
    ){
        glMultiDrawElementsBaseVertex(
            mode as GLenum,
            count,
            index_type as GLenum,
            start as *const _,
            size,
            base_vertex
        )
    }

    /// Same as `Drawing::multi_draw_elements_base_vertex`, but with a static index type.
    #[inline(always)]
    pub unsafe fn multi_draw_elements_base_vertex_typed<T:AvailableIndexType>(
        &self,
        start:*const isize,
        count:*const i32,
        base_vertex:*const i32,
        size:i32,
        mode:PrimitiveType
    ){
        glMultiDrawElementsBaseVertex(
            mode as GLenum,
            count,
            T::gl_enum(),
            start as *const _,
            size,
            base_vertex
        )
    }
}