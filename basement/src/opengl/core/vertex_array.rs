#[cfg(target_os="windows")]
use crate::winapi::OpenGraphicsLibrary;

use super::{
    types::*,
    constants::*
};

use core::mem::transmute;

#[cfg(target_os="linux")]
extern "system"{
    fn glGenVertexArrays(number:GLsizei,arrays:*mut GLuint);
    fn glDeleteVertexArrays(number:GLsizei,arrays:*const GLuint);
    fn glIsVertexArray(array:GLuint)->GLboolean;

    fn glBindVertexArray(array:GLuint);

    fn glVertexAttribPointer(index:GLuint,size:GLint,data_type:GLenum,normalized:GLboolean,stride:GLsizei,pointer:*const GLvoid);
    fn glVertexAttribIPointer(index:GLuint,size:GLint,data_type:GLenum,stride:GLsizei,pointer:*const GLvoid);
    fn glGetVertexAttribPointerv(index:GLuint,parameter:GLenum,pointer:*mut *mut GLvoid);

    fn glEnableVertexAttribArray(index:GLuint);
    fn glDisableVertexAttribArray(index:GLuint);

    fn glVertexAttribDivisor(index:GLuint,divisor:GLuint);
}

#[cfg(target_os="windows")]
mod gl{
    pub static mut glGenVertexArrays:usize=0;
    pub static mut glDeleteVertexArrays:usize=0;
    pub static mut glIsVertexArray:usize=0;

    pub static mut glBindVertexArray:usize=0;

    pub static mut glVertexAttribPointer:usize=0;
    pub static mut glVertexAttribIPointer:usize=0;
    pub static mut glGetVertexAttribPointerv:usize=0;

    pub static mut glEnableVertexAttribArray:usize=0;
    pub static mut glDisableVertexAttribArray:usize=0;

    pub static mut glVertexAttribDivisor:usize=0;
}

#[cfg(target_os="windows")]
mod gl_functions{
    use super::*;

    #[inline(always)]
    pub unsafe fn glGenVertexArrays(number:GLsizei,arrays:*mut GLuint){
        transmute::<usize,fn(GLsizei,*mut GLuint)>(gl::glGenVertexArrays)(number,arrays)
    }

    #[inline(always)]
    pub unsafe fn glDeleteVertexArrays(number:GLsizei,arrays:*const GLuint){
        transmute::<usize,fn(GLsizei,*const GLuint)>(gl::glDeleteVertexArrays)(number,arrays)
    }

    #[inline(always)]
    pub unsafe fn glIsVertexArray(array:GLuint)->GLboolean{
        transmute::<usize,fn(GLuint)->GLboolean>(gl::glIsVertexArray)(array)
    }


    #[inline(always)]
    pub unsafe fn glBindVertexArray(array:GLuint){
        transmute::<usize,fn(GLuint)>(gl::glBindVertexArray)(array)
    }


    #[inline(always)]
    pub unsafe fn glVertexAttribPointer(index:GLuint,size:GLint,data_type:GLenum,normalized:GLboolean,stride:GLsizei,pointer:*const GLvoid){
        transmute::<usize,fn(GLuint,GLint,GLenum,GLboolean,GLsizei,*const GLvoid)>(gl::glVertexAttribPointer)(index,size,data_type,normalized,stride,pointer)
    }

    #[inline(always)]
    pub unsafe fn glVertexAttribIPointer(index:GLuint,size:GLint,data_type:GLenum,stride:GLsizei,pointer:*const GLvoid){
        transmute::<usize,fn(GLuint,GLint,GLenum,GLsizei,*const GLvoid)>(gl::glVertexAttribIPointer)(index,size,data_type,stride,pointer)
    }

    #[inline(always)]
    pub unsafe fn glGetVertexAttribPointerv(index:GLuint,parameter:GLenum,pointer:*mut *mut GLvoid){
        transmute::<usize,fn(GLuint,GLenum,*mut *mut GLvoid)>(gl::glGetVertexAttribPointerv)(index,parameter,pointer)
    }


    #[inline(always)]
    pub unsafe fn glEnableVertexAttribArray(index:GLuint){
        transmute::<usize,fn(GLuint)>(gl::glEnableVertexAttribArray)(index)
    }

    #[inline(always)]
    pub unsafe fn glDisableVertexAttribArray(index:GLuint){
        transmute::<usize,fn(GLuint)>(gl::glDisableVertexAttribArray)(index)
    }


    #[inline(always)]
    pub unsafe fn glVertexAttribDivisor(index:GLuint,divisor:GLuint){
        transmute::<usize,fn(GLuint,GLuint)>(gl::glVertexAttribDivisor)(index,divisor)
    }
}

#[cfg(target_os="windows")]
use gl_functions::*;

/// Specifies the data type of each component in an array.
#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
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
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum IntegerDataType{
    I8=BYTE,
    U8=UNSIGNED_BYTE,
    I16=SHORT,
    U16=UNSIGNED_SHORT,
    I32=INT,
    U32=UNSIGNED_INT,
}

#[repr(i32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum VertexComponents{
    One=1i32,
    Two=2i32,
    Three=3i32,
    Four=4i32,
    BGRA=BGRA as i32,
}

#[repr(i32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum VertexIntegerComponents{
    One=1i32,
    Two=2i32,
    Three=3i32,
    Four=4i32,
}

#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum VertexAttributePointer{
    VertexAttributePointer=VERTEX_ATTRIB_ARRAY_POINTER
}

pub struct VertexArray;

impl VertexArray{
    #[cfg(target_os="windows")]
    pub fn load(library:&OpenGraphicsLibrary){
        unsafe{
            use gl::*;

            glGenVertexArrays=transmute(library.get_proc_address("glGenVertexArrays\0"));
            glDeleteVertexArrays=transmute(library.get_proc_address("glDeleteVertexArrays\0"));
            glIsVertexArray=transmute(library.get_proc_address("glIsVertexArray\0"));

            glBindVertexArray=transmute(library.get_proc_address("glBindVertexArray\0"));

            glVertexAttribPointer=transmute(library.get_proc_address("glVertexAttribPointer\0"));
            glVertexAttribIPointer=transmute(library.get_proc_address("glVertexAttribIPointer\0"));
            glGetVertexAttribPointerv=transmute(library.get_proc_address("glGetVertexAttribPointerv\0"));

            glEnableVertexAttribArray=transmute(library.get_proc_address("glEnableVertexAttribArray\0"));
            glDisableVertexAttribArray=transmute(library.get_proc_address("glDisableVertexAttribArray\0"));

            glVertexAttribDivisor=transmute(library.get_proc_address("glVertexAttribDivisor\0"));
        }
    }
}

impl VertexArray{
    /// Generates vertex array object names.
    /// 
    /// `number` specifies the number of vertex array object names to generate.
    /// 
    /// `arrays` specifies an array in which the generated vertex array object names are stored.
    /// 
    /// Returns `number` vertex array object names in arrays.
    /// There is no guarantee that the names form a contiguous set of integers;
    /// however, it is guaranteed that none of the returned names was in use immediately before the call to `VertexArray::generate`.
    /// 
    /// Vertex array object names returned by a call to `VertexArray::generate` are not returned by subsequent calls,
    /// unless they are first deleted with `VertexArray::delete`.
    /// 
    /// The names returned in `arrays` are marked as used,
    /// for the purposes of `VertexArray::generate` only,
    /// but they acquire state and type only when they are first bound.
    /// 
    /// `Error::InvalidValue` is generated if `number` is negative.
    #[inline(always)]
    pub unsafe fn generate(number:i32,arrays:*mut u32){
        glGenVertexArrays(number,arrays)
    }

    /// Deletes vertex array objects.
    /// 
    /// `number` specifies the number of vertex array objects to be deleted.
    /// 
    /// `arrays` specifies the address of an array containing the n names of the objects to be deleted.
    /// 
    /// Deletes `number` vertex array objects whose names are stored in the array addressed by `arrays`.
    /// Once a vertex array object is deleted it has no contents and its name is again unused.
    /// If a vertex array object that is currently bound is deleted,
    /// the binding for that object reverts to zero and the default vertex array becomes current.
    /// Unused names in `arrays` are silently ignored, as is the value zero.
    /// 
    /// `Error::InvalidValue` is generated if `number` is negative.
    #[inline(always)]
    pub unsafe fn delete(number:i32,arrays:*const u32){
        glDeleteVertexArrays(number,arrays)
    }

    /// Determines if a name corresponds to a vertex array object.
    /// 
    /// `array` specifies a value that may be the name of a vertex array object.
    /// 
    /// Returns `true` if `array` is currently the name of a renderbuffer object.
    /// If `renderbuffer` is zero,
    /// or if `array` is not the name of a renderbuffer object, or if an error occurs,
    /// returns `true`.
    /// If `array` is a name returned by `VertexArray::generate`,
    /// by that has not yet been bound through a call to `VertexArray::bind`,
    /// then the name is not a vertex array object and `VertexArray::is_vertex_array` returns `false`.
    #[inline(always)]
    pub unsafe fn is_vertex_array(array:u32)->bool{
        transmute(glIsVertexArray(array))
    }
}

impl VertexArray{
    /// Binds a vertex array object.
    /// 
    /// `array` specifies the name of the vertex array to bind.
    /// 
    ///  Binds the vertex array object with name `array`.
    /// `array` is the name of a vertex array object previously returned from a call to `VertexArray::generate`,
    /// or zero to break the existing vertex array object binding.
    /// 
    /// `Error::InvalidOperation` is generated
    /// if `array` is not zero or the name of a vertex array object previously
    /// returned from a call to `VertexArray::generate`.
    #[inline(always)]
    pub unsafe fn bind(array:u32){
        glBindVertexArray(array)
    }

    /// Enables a generic vertex attribute array.
    /// 
    /// `index` specifies the index of the generic vertex attribute to be enabled.
    /// 
    /// Enables the generic vertex attribute array specified by `index`.
    /// 
    /// By default, all client-side capabilities are disabled, including all generic vertex attribute arrays.
    /// If enabled, the values in the generic vertex attribute array will be accessed
    /// and used for rendering when calls are made to vertex array commands such as
    /// `Drawing::draw_arrays`, `Drawing::draw_elements`, `Drawing::draw_range_elements`, `Drawing::multi_draw_elements`, or Drawing::multi_draw_arrays`.
    /// 
    /// `Error::InvalidValue` is generated
    /// if `index` is greater than or equal to `GL_MAX_VERTEX_ATTRIBS`.
    /// 
    /// `Error::InvalidOperation` is generated
    /// if there is no current vertex array object.
    #[inline(always)]
    pub unsafe fn enable_attribute(index:u32){
        glEnableVertexAttribArray(index)
    }

    /// Disables a generic vertex attribute array.
    /// 
    /// `index` specifies the index of the generic vertex attribute to be disabled.
    /// 
    /// Disables the generic vertex attribute array specified by `index`.
    /// 
    /// By default, all client-side capabilities are disabled, including all generic vertex attribute arrays.
    /// If enabled, the values in the generic vertex attribute array will be accessed
    /// and used for rendering when calls are made to vertex array commands such as
    /// `Drawing::draw_arrays`, `Drawing::draw_elements`, `Drawing::draw_range_elements`, `Drawing::multi_draw_elements`, or `Drawing::multi_draw_arrays`.
    /// 
    /// `Error::InvalidValue` is generated
    /// if `index` is greater than or equal to `GL_MAX_VERTEX_ATTRIBS`.
    /// 
    /// `Error::InvalidOperation` is generated
    /// if there is no current vertex array object.
    #[inline(always)]
    pub unsafe fn disable_attribute(index:u32){
        glDisableVertexAttribArray(index)
    }
}

impl VertexArray{
    /// Defines an array of generic vertex attribute data.
    /// 
    /// `index` specifies the index of the generic vertex attribute to be modified.
    /// 
    /// `size` specifies the number of components per generic vertex attribute.
    /// The initial value is `VertexComponents::Four`.
    /// 
    /// `data_type` specifies the data type of each component in the array.
    /// The initial value is `DataType::F32`.
    /// 
    /// `normalized` specifies whether fixed-point data values should be normalized (`true`)
    /// or converted directly as fixed-point values (`false`) when they are accessed.
    /// 
    /// `stride` specifies the byte offset between consecutive generic vertex attributes.
    /// If `stride` is 0, the generic vertex attributes are understood to be tightly packed in the array.
    /// The initial value is 0.
    /// 
    /// `pointer` specifies a offset of the first component of the first generic vertex attribute
    /// in the array in the data store of the buffer currently bound to the `BufferTarget::ArrayBuffer` target.
    /// The initial value is 0.
    /// 
    /// Specifies the location and data format of the array of generic vertex attributes at index `index` to use when rendering.
    /// `size` specifies the number of components per attribute.
    /// `data_type` specifies the data type of each component,
    /// and `stride` specifies the byte stride from one attribute to the next,
    /// allowing vertices and attributes to be packed into a single array or stored in separate arrays.
    /// 
    /// If `normalized` is set to `true`, it indicates that values stored in an integer format are to be mapped to the range [-1,1] (for signed values)
    /// or [0,1] (for unsigned values) when they are accessed and converted to floating point.
    /// Otherwise, values will be converted to floats directly without normalization.
    /// 
    /// If `pointer` is not `NULL`,
    /// a non-zero named buffer object must be bound to the `BufferTarget::ArrayBuffer` target (see `Buffer::bind`),
    /// otherwise an error is generated.
    /// `pointer` is treated as a byte offset into the buffer object's data store.
    /// The buffer object binding (GL_ARRAY_BUFFER_BINDING) is saved as generic vertex attribute array state (GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING) for index index.
    /// 
    /// When a generic vertex attribute array is specified, `size`, `data_type`, `normalized`, `stride`, and `pointer` are saved as vertex array state,
    /// in addition to the current vertex array buffer object binding.
    /// 
    /// To enable and disable a generic vertex attribute array, call `VertexArray::enable_attribute` and `VertexArray::disable_attribute` with `index`.
    /// If enabled, the generic vertex attribute array is used when `Drawing::draw_arrays`, `Drawing::draw_elements`,
    /// `Drawing::draw_range_elements`, `Drawing::multi_draw_elements`, `Drawing::multi_draw_arrays` or `Drawing::draw_range_elements` is called.
    /// 
    /// Each generic vertex attribute array is initially disabled and isn't accessed
    /// when `Drawing::draw_arrays`, `Drawing::draw_elements`,
    /// `Drawing::draw_range_elements` or `Drawing::multi_draw_elements` is called.
    /// 
    /// 
    /// `Error::InvalidValue` is generated
    /// if `index` is greater than or equal to `GL_MAX_VERTEX_ATTRIBS`,
    /// if `size` is not `VertexIntegerComponents`,
    /// if `stride` is negative.
    /// 
    /// `Error::InvalidEnum` is generated if `data_type` is not an accepted value.
    /// 
    /// `Error::InvalidOperation` is generated
    /// if `size` is `VertexComponents::BGRA` and type is not `DataType::U8`,
    /// `DataType::I32_2_10_10_10_REV` or `DataType::U32_2_10_10_10_REV`,
    /// if `data_type` is `DataType::I32_2_10_10_10_REV` or `DataType::U32_2_10_10_10_REV`
    /// and `size` is not `VertexComponents::Four` or `VertexComponents::BGRA`,
    /// if `size` is `VertexComponents::BGRA` and `noramlized` is `false`,
    /// if zero is bound to the `BufferTarget::ArrayBuffer` buffer object binding point and the `pointer` argument is not `NULL`.
    /// (Note: In the core context, the old method of passing `VertexArray::attribute_pointer`
    /// and `Drawing::draw_arrays` pointers to mesh data in main memory is no longer allowed.
    /// You must create a Vertex Buffer Object and fill it with your mesh data.)
    /// 
    /// `Error::InvalidOperation` is generated in the core context if there is no Vertex Array Object bound.
    #[inline(always)]
    pub unsafe fn attribute_pointer(
        index:GLuint,
        size:VertexComponents,
        data_type:DataType,
        normalized:bool,
        stride:i32,
        pointer:*const GLvoid
    ){
        glVertexAttribPointer(
            index,
            size as GLint,
            data_type as GLenum,
            normalized as GLboolean,
            stride,
            pointer
        )
    }

    /// Defines an array of generic vertex attribute data.
    /// 
    /// `index` specifies the index of the generic vertex attribute to be modified.
    /// 
    /// `size` specifies the number of components per generic vertex attribute.
    /// The initial value is `VertexComponents::Four`.
    /// 
    /// `data_type` specifies the data type of each component in the array.
    /// The initial value is `DataType::F32`.
    /// 
    /// `stride` specifies the byte offset between consecutive generic vertex attributes.
    /// If `stride` is 0, the generic vertex attributes are understood to be tightly packed in the array.
    /// The initial value is 0.
    /// 
    /// `pointer` specifies a offset of the first component of the first generic vertex attribute
    /// in the array in the data store of the buffer currently bound to the `BufferTarget::ArrayBuffer` target.
    /// The initial value is 0.
    /// 
    /// Specifies the location and data format of the array of generic vertex attributes at index `index` to use when rendering.
    /// `size` specifies the number of components per attribute.
    /// `data_type` specifies the data type of each component,
    /// and `stride` specifies the byte stride from one attribute to the next,
    /// allowing vertices and attributes to be packed into a single array or stored in separate arrays.
    /// 
    /// Values are always left as integer values.
    /// 
    /// If `pointer` is not `NULL`,
    /// a non-zero named buffer object must be bound to the `BufferTarget::ArrayBuffer` target (see `Buffer::bind`),
    /// otherwise an error is generated.
    /// `pointer` is treated as a byte offset into the buffer object's data store.
    /// The buffer object binding (GL_ARRAY_BUFFER_BINDING) is saved as generic vertex attribute array state (GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING) for index index.
    /// 
    /// When a generic vertex attribute array is specified, `size`, `data_type`, `stride`, and `pointer` are saved as vertex array state,
    /// in addition to the current vertex array buffer object binding.
    /// 
    /// To enable and disable a generic vertex attribute array, call `VertexArray::enable_attribute` and `VertexArray::disable_attribute` with `index`.
    /// If enabled, the generic vertex attribute array is used when `Drawing::draw_arrays`, `Drawing::draw_elements`,
    /// `Drawing::draw_range_elements`, `Drawing::multi_draw_elements`, `Drawing::multi_draw_arrays` or `Drawing::draw_range_elements` is called.
    /// 
    /// Each generic vertex attribute array is initially disabled and isn't accessed
    /// when `Drawing::draw_arrays`, `Drawing::draw_elements`,
    /// `Drawing::draw_range_elements` or `Drawing::multi_draw_elements` is called.
    /// 
    /// Available only if the GL version is 3.0 or higher.
    /// 
    /// `Error::InvalidValue` is generated
    /// if `index` is greater than or equal to `GL_MAX_VERTEX_ATTRIBS`,
    /// if `size` is not `VertexIntegerComponents`,
    /// if `stride` is negative.
    /// 
    /// `Error::InvalidEnum` is generated if `data_type` is not an accepted value.
    /// 
    /// Error::InvalidOperation` is generated
    /// if zero is bound to the `BufferTarget::ArrayBuffer` buffer object binding point and the `pointer` argument is not `NULL`.
    /// (Note: In the core context, the old method of passing `VertexArray::attribute_pointer`
    /// and `Drawing::draw_arrays` pointers to mesh data in main memory is no longer allowed.
    /// You must create a Vertex Buffer Object and fill it with your mesh data.)
    /// 
    /// `Error::InvalidOperation` is generated in the core context if there is no Vertex Array Object bound.
    #[inline(always)]
    pub unsafe fn attribute_pointer_integer(
        index:GLuint,
        size:VertexIntegerComponents,
        data_type:IntegerDataType,
        stride:i32,
        pointer:*const GLvoid
    ){
        glVertexAttribIPointer(
            index,
            size as GLint,
            data_type as GLenum,
            stride,
            pointer
        )
    }

        /// Returns the address of the specified generic vertex attribute pointer.
    /// 
    /// `index` specifies the generic vertex attribute parameter to be returned.
    /// 
    /// `parameter` specifies the symbolic name of the generic vertex attribute parameter to be returned.
    /// 
    /// `pointer` returns the pointer value.
    /// 
    /// Returns pointer information.
    /// `index` is the generic vertex attribute to be queried,
    /// `parameter` is a symbolic constant indicating the pointer to be returned,
    /// and `pointer` is a pointer to a location in which to place the returned data.
    /// 
    /// The `pointer` returned is a byte offset into the data store of the buffer object
    /// that was bound to the `BufferTarget::ArrayBuffer` target (see `Buffer::bind`)
    /// when the desired pointer was previously specified.
    /// 
    /// The state returned is retrieved from the currently bound vertex array object.
    /// 
    /// The initial value for each pointer is 0.
    /// 
    /// `Error::InvalidOperation` is generated if no vertex array object is currently bound.
    /// 
    /// `Error::InvalidValue` is generated if `index` is greater than or equal to `GL_MAX_VERTEX_ATTRIBS`.
    /// 
    /// `Error::InvalidEnum` is generated if `parameter` is not an accepted value.
    pub unsafe fn get_attribute_pointer(index:u32,parameter:VertexAttributePointer,pointer:*mut *mut GLvoid){
        glGetVertexAttribPointerv(index,parameter as GLenum,pointer)
    }
}

impl VertexArray{
    /// Modifies the rate at which generic vertex attributes advance during instanced rendering.
    /// 
    /// `index` specify the index of the generic vertex attribute.
    /// 
    /// `divisor` specify the number of instances that will pass between updates of the generic attribute at slot `index`.
    /// 
    /// Mmodifies the rate at which generic vertex attributes advance when rendering multiple instances of primitives in a single draw call.
    /// If `divisor` is zero, the attribute at slot index advances once per vertex.
    /// If `divisor` is non-zero, the attribute advances once per `divisor` instances of the set(s) of vertices being rendered.
    /// An attribute is referred to as instanced if its `GL_VERTEX_ATTRIB_ARRAY_DIVISOR` value is non-zero.
    /// 
    /// `index` must be less than the value of `GL_MAX_VERTEX_ATTRIBS`.
    /// 
    /// Available only if the GL version is 3.3 or higher.
    /// 
    /// `Error::InvalidValue` is generated if `index` is greater than or equal to `GL_MAX_VERTEX_ATTRIBS`.
    pub unsafe fn set_attribute_divisor(index:u32,divisor:u32){
        glVertexAttribDivisor(index,divisor)
    }
}