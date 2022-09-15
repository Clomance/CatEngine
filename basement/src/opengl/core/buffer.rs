#[cfg(target_os="windows")]
use crate::winapi::OpenGraphicsLibrary;

use super::{
    types::*,
    constants::*
};

use core::mem::transmute;

#[cfg(target_os="linux")]
extern "system"{
    fn glGenBuffers(number:GLsizei,buffers:*mut GLuint);
    fn glDeleteBuffers(number:GLsizei,buffers:*const GLuint);
    fn glIsBuffer(buffer:GLuint)->GLboolean;

    fn glBindBuffer(target:GLenum,buffer:GLuint);
    fn glBindBufferBase(target:GLenum,index:GLuint,buffer:GLuint);
    fn glBindBufferRange(target:GLenum,index:GLuint,buffer:GLuint,offset:GLintptr,size:GLintptr);

    fn glBufferData(target:GLenum,size:GLsizeiptr,data:*const GLvoid,usage:GLenum);
    fn glBufferSubData(target:GLenum,offset:GLintptr,size:GLsizeiptr,data:*const GLvoid);
    fn glGetBufferSubData(target:GLenum,offset:GLintptr,size:GLsizeiptr,data:*mut GLvoid);
}

#[cfg(target_os="windows")]
mod gl{
    pub static mut glGenBuffers:usize=0;
    pub static mut glDeleteBuffers:usize=0;
    pub static mut glIsBuffer:usize=0;

    pub static mut glBindBuffer:usize=0;
    pub static mut glBindBufferBase:usize=0;
    pub static mut glBindBufferRange:usize=0;

    pub static mut glBufferData:usize=0;
    pub static mut glBufferSubData:usize=0;
    pub static mut glCopyBufferSubData:usize=0;
    pub static mut glGetBufferSubData:usize=0;

    pub static mut glMapBuffer:usize=0;
    pub static mut glMapBufferRange:usize=0;
    pub static mut glUnmapBuffer:usize=0;
    pub static mut glGetBufferPointerv:usize=0;
    pub static mut glFlushMappedBufferRange:usize=0;

    pub static mut glGetBufferParameteriv:usize=0;
    pub static mut glGetBufferParameteri64v:usize=0;
}

#[cfg(target_os="windows")]
mod gl_functions{
    use super::*;

    #[inline(always)]
    pub unsafe fn glGenBuffers(number:GLsizei,buffers:*mut GLuint){
        transmute::<usize,extern "system" fn(GLsizei,*mut GLuint)>(gl::glGenBuffers)(number,buffers)
    }
    #[inline(always)]
    pub unsafe fn glDeleteBuffers(number:GLsizei,buffers:*const GLuint){
        transmute::<usize,extern "system" fn(GLsizei,*const GLuint)>(gl::glDeleteBuffers)(number,buffers)
    }
    #[inline(always)]
    pub unsafe fn glIsBuffer(buffer:GLuint)->GLboolean{
        transmute::<usize,extern "system" fn(GLuint)->GLboolean>(gl::glIsBuffer)(buffer)
    }


    #[inline(always)]
    pub unsafe fn glBindBuffer(target:GLenum,buffer:GLuint){
        transmute::<usize,extern "system" fn(GLenum,GLuint)>(gl::glBindBuffer)(target,buffer)
    }
    #[inline(always)]
    pub unsafe fn glBindBufferBase(target:GLenum,index:GLuint,buffer:GLuint){
        transmute::<usize,extern "system" fn(GLenum,GLuint,GLuint)>(gl::glBindBufferBase)(target,index,buffer)
    }
    #[inline(always)]
    pub unsafe fn glBindBufferRange(target:GLenum,index:GLuint,buffer:GLuint,offset:GLintptr,size:GLintptr){
        transmute::<usize,extern "system" fn(GLenum,GLuint,GLuint,GLintptr,GLintptr)>(gl::glBindBufferRange)(target,index,buffer,offset,size)
    }


    #[inline(always)]
    pub unsafe fn glBufferData(target:GLenum,size:GLsizeiptr,data:*const GLvoid,usage:GLenum){
        transmute::<usize,extern "system" fn(GLenum,GLsizeiptr,*const GLvoid,GLenum)>(gl::glBufferData)(target,size,data,usage)
    }

    #[inline(always)]
    pub unsafe fn glBufferSubData(target:GLenum,offset:GLintptr,size:GLsizeiptr,data:*const GLvoid){
        transmute::<usize,extern "system" fn(GLenum,GLintptr,GLsizeiptr,*const GLvoid)>(gl::glBufferSubData)(target,offset,size,data)
    }

    #[inline(always)]
    pub unsafe fn glGetBufferSubData(target:GLenum,offset:GLintptr,size:GLsizeiptr,data:*mut GLvoid){
        transmute::<usize,extern "system" fn(GLenum,GLintptr,GLsizeiptr,*const GLvoid)>(gl::glGetBufferSubData)(target,offset,size,data)
    }
}

#[cfg(target_os="windows")]
use gl_functions::*;

/// Specifies the target to which the buffer object is bound.
#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum BufferTarget{
    /// Vertex attributes.
    ArrayBuffer=ARRAY_BUFFER,

    /// Buffer copy source.
    /// 
    /// Available only if the GL version is 3.1 or greater.
    CopyReadBuffer=COPY_READ_BUFFER,

    /// Buffer copy destination.
    CopyWriteBuffer=COPY_WRITE_BUFFER,

    /// Vertex array indices.
    ElementArrayBuffer=ELEMENT_ARRAY_BUFFER,

    /// Pixel read target.
    PixelPackBuffer=PIXEL_PACK_BUFFER,

    /// Texture data source.
    PixelUnpackBuffer=PIXEL_UNPACK_BUFFER,

    /// Texture data buffer.
    /// 
    /// Available only if the GL version is 3.1 or greater.
    TextureBuffer=TEXTURE_BUFFER,

    /// Transform feedback buffer.
    TransformFeedbackBuffer=TRANSFORM_FEEDBACK_BUFFER,

    /// Uniform block storage.
    /// 
    /// Available only if the GL version is 3.1 or greater.
    UniformBuffer=UNIFORM_BUFFER,
}

/// Specifies the expected usage pattern of the data store.
/// 
/// A hint to the GL implementation as to how a buffer object's data store will be accessed.
/// This enables the GL implementation to make more intelligent decisions
/// that may significantly impact buffer object performance.
/// It does not, however, constrain the actual usage of the data store.
/// `BufferUsage` can be broken down into two parts:
/// first, the frequency of access (modification and usage),
/// and second, the nature of that access.
#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum BufferUsage{
    /// The data store contents will be modified once and used at most a few times.
    /// The data store contents are modified by the application,
    /// and used as the source for GL drawing and image specification commands.
    StreamDraw=STREAM_DRAW,

    /// The data store contents will be modified once and used at most a few times.
    /// The data store contents are modified by reading data from the GL,
    /// and used to return that data when queried by the application.
    StreamRead=STREAM_READ,

    /// The data store contents will be modified once and used at most a few times.
    /// The data store contents are modified by reading data from the GL,
    /// and used as the source for GL drawing and image specification commands.
    StreamCopy=STREAM_COPY,

    /// The data store contents will be modified once and used many times.
    /// The data store contents are modified by the application,
    /// and used as the source for GL drawing and image specification commands.
    StaticDraw=STATIC_DRAW,

    /// The data store contents will be modified once and used many times.
    /// The data store contents are modified by reading data from the GL,
    /// and used to return that data when queried by the application.
    StaticRead=STATIC_READ,

    /// The data store contents will be modified once and used many times.
    /// The data store contents are modified by reading data from the GL,
    /// and used as the source for GL drawing and image specification commands.
    StaticCopy=STATIC_COPY,

    /// The data store contents will be modified repeatedly and used many times.
    /// The data store contents are modified by the application,
    /// and used as the source for GL drawing and image specification commands.
    DynamicDraw=DYNAMIC_DRAW,

    /// The data store contents will be modified repeatedly and used many times.
    /// The data store contents are modified by reading data from the GL,
    /// and used to return that data when queried by the application.
    DynamicRead=DYNAMIC_READ,

    /// The data store contents will be modified repeatedly and used many times.
    /// The data store contents are modified by reading data from the GL,
    /// and used as the source for GL drawing and image specification commands.
    DynamicCopy=DYNAMIC_COPY,
}

pub struct Buffer;

impl Buffer{
    #[cfg(target_os="windows")]
    pub fn load(library:&OpenGraphicsLibrary){
        unsafe{
            use gl::*;

            glGenBuffers=transmute(library.get_proc_address("glGenBuffers\0"));
            glDeleteBuffers=transmute(library.get_proc_address("glDeleteBuffers\0"));
            glIsBuffer=transmute(library.get_proc_address("glIsBuffer\0"));

            glBindBuffer=transmute(library.get_proc_address("glBindBuffer\0"));
            glBindBufferBase=transmute(library.get_proc_address("glBindBufferBase\0"));
            glBindBufferRange=transmute(library.get_proc_address("glBindBufferRange\0"));

            glBufferData=transmute(library.get_proc_address("glBufferData\0"));
            glBufferSubData=transmute(library.get_proc_address("glBufferSubData\0"));
            glCopyBufferSubData=transmute(library.get_proc_address("glCopyBufferSubData\0"));
            glGetBufferSubData=transmute(library.get_proc_address("glGetBufferSubData\0"));

            glMapBuffer=transmute(library.get_proc_address("glMapBuffer\0"));
            glMapBufferRange=transmute(library.get_proc_address("glMapBufferRange\0"));
            glUnmapBuffer=transmute(library.get_proc_address("glUnmapBuffer\0"));
            glGetBufferPointerv=transmute(library.get_proc_address("glGetBufferPointerv\0"));
            glFlushMappedBufferRange=transmute(library.get_proc_address("glFlushMappedBufferRange\0"));

            glGetBufferParameteriv=transmute(library.get_proc_address("glGetBufferParameteriv\0"));
            glGetBufferParameteri64v=transmute(library.get_proc_address("glGetBufferParameteri64v\0"));
        }
    }
}

impl Buffer{
    /// Generates buffer object names.
    /// 
    /// `number` specifies the number of buffer object names to be generated.
    /// 
    /// `buffers` specifies an array in which the generated buffer object names are stored.
    /// 
    /// Returns `number` buffer object names in `buffers`.
    /// There is no guarantee that the names form a contiguous set of integers;
    /// however, it is guaranteed that none of the returned names was in use immediately before the call to `Buffer::generate`.
    /// 
    /// Buffer object names returned by a call to `Buffer::generate` are not returned by subsequent calls,
    /// unless they are first deleted with `Buffer::delete`.
    /// 
    /// No buffer objects are associated with the returned buffer object names until they are first bound by calling `Buffer::bind`.
    /// 
    /// `Error::InvalidValue` is generated if `number` is negative.
    #[inline(always)]
    pub unsafe fn generate(number:i32,buffers:*mut u32){
        glGenBuffers(number,buffers)
    }

    /// Deletes named buffer objects.
    /// 
    /// `number` specifies the number of buffer objects to be deleted.
    /// 
    /// `buffers` specifies an array of buffer objects to be deleted.
    /// 
    /// Deletes `number` buffer objects named by the elements of the array `buffers`.
    /// After a buffer object is deleted, it has no contents, and it's name is free for reuse (for example by `Buffer::generate`).
    /// If a buffer object that is currently bound is deleted,
    /// the binding reverts to 0 (the absence of any buffer object).
    /// 
    /// `Buffer::delete` silently ignores 0's and names that do not correspond to existing buffer objects.
    /// 
    /// `Error::InvalidValue` is generated if `number` is negative.
    #[inline(always)]
    pub unsafe fn delete(number:i32,buffers:*const u32){
        glDeleteBuffers(number,buffers)
    }

    /// Determines if a name corresponds to a buffer object.
    /// 
    /// `buffer` specifies a value that may be the name of a buffer object.
    /// 
    /// Returns `true` if buffer is currently the name of a buffer object.
    /// If buffer is zero, or is a non-zero value that is not currently the name of a buffer object, or if an error occurs,
    /// returns `false`.
    /// 
    /// A name returned by `Buffer::generate`,
    /// but not yet associated with a buffer object by calling `Buffer::bind`,
    /// is not the name of a buffer object.
    #[inline(always)]
    pub unsafe fn is_buffer(buffer:u32)->bool{
        transmute(glIsBuffer(buffer))
    }
}

#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum BufferIndexedTarget{
    TransformFeedbackBuffer=TRANSFORM_FEEDBACK_BUFFER,
    UniformBuffer=UNIFORM_BUFFER,
}

impl Buffer{
    /// Binds a named buffer object.
    /// 
    /// `target` specifies the target to which the buffer object is bound.
    /// 
    /// `buffer` specifies the name of a buffer object.
    /// 
    /// Binds a buffer object to the specified buffer binding point.
    /// Calling `Buffer::bind` with `target` set to one of the accepted symbolic constants
    /// and `buffer` set to the name of a buffer object binds that buffer object name to the target.
    /// If no buffer object with name buffer exists, one is created with that name.
    /// When a buffer object is bound to a target, the previous binding for that target is automatically broken.
    /// 
    /// Buffer object names are unsigned integers.
    /// The value zero is reserved, but there is no default buffer object for each buffer object target.
    /// Instead, `buffer` set to zero effectively unbinds any buffer object previously bound,
    /// and restores client memory usage for that buffer object target (if supported for that target).
    /// Buffer object names and the corresponding buffer object contents are local to the shared object space of the current GL rendering context;
    /// two rendering contexts share buffer object names
    /// only if they explicitly enable sharing between contexts through the appropriate GL windows interfaces functions.
    /// 
    /// `Buffer::generate` must be used to generate a set of unused buffer object names.
    /// 
    /// The state of a buffer object immediately after it is first bound
    /// is an unmapped zero-sized memory buffer with `BufferMapAccess::ReadWrite` access and `BufferUsage::StaticDraw` usage.
    /// 
    /// While a non-zero buffer object name is bound,
    /// GL operations on the target to which it is bound affect the bound buffer object,
    /// and queries of the target to which it is bound return state from the bound buffer object.
    /// While buffer object name zero is bound, as in the initial state,
    /// attempts to modify or query state on the target to which it is bound generates an `Error::InvalidOperator` error.
    /// 
    /// When a non-zero buffer object is bound to the `BufferTarget::ArrayBuffer` target,
    /// the vertex array pointer parameter is interpreted as an offset within the buffer object measured in basic machine units.
    /// 
    /// While a non-zero buffer object is bound to the `BufferTarget::ElementArrayBuffer` target,
    /// the indices parameter of `Draw::draw_elements`, `Draw::draw_elements_enstanced`,
    /// `Draw::draw_elements_base_vertex`, `Draw::draw_range_elements`, `Draw::draw_range_elements_base_vertex`,
    /// `Draw::multi_draw_elements`, or `Draw::multi_draw_elements_base_vertex` is interpreted as an offset within the buffer object measured in basic machine units.
    /// 
    /// While a non-zero buffer object is bound to the GL_PIXEL_PACK_BUFFER target,
    /// the following commands are affected: glGetCompressedTexImage, glGetTexImage, and glReadPixels.
    /// The pointer parameter is interpreted as an offset within the buffer object measured in basic machine units.
    /// 
    /// While a non-zero buffer object is bound to the GL_PIXEL_UNPACK_BUFFER target,
    /// the following commands are affected: glCompressedTexImage1D, glCompressedTexImage2D,
    /// glCompressedTexImage3D, glCompressedTexSubImage1D, glCompressedTexSubImage2D, glCompressedTexSubImage3D,
    /// glTexImage1D, glTexImage2D, glTexImage3D, glTexSubImage1D, glTexSubImage2D, and glTexSubImage3D.
    /// The pointer parameter is interpreted as an offset within the buffer object measured in basic machine units.
    /// 
    /// The buffer targets `GL_COPY_READ_BUFFER` and `GL_COPY_WRITE_BUFFER` are provided to allow glCopyBufferSubData to be used without disturbing the state of other bindings.
    /// However, glCopyBufferSubData may be used with any pair of buffer binding points.
    /// 
    /// The `GL_TRANSFORM_FEEDBACK_BUFFER` buffer binding point may be passed to glBindBuffer,
    /// but will not directly affect transform feedback state.
    /// Instead, the indexed `GL_TRANSFORM_FEEDBACK_BUFFER` bindings must be used through a call to `Buffer::bind_base` or `Buffer::bind_base_range`.
    /// This will affect the generic `GL_TRANSFORM_FEEDBACK_BUFFER` binding.
    /// 
    /// Likewise, the `GL_UNIFORM_BUFFER` buffer binding point may be used, but does not directly affect uniform buffer state.
    /// `Buffer::bind_base` or `Buffer::bind_base_range` must be used to bind a buffer to an indexed uniform buffer binding point.
    /// 
    /// A buffer object binding created with glBindBuffer remains active
    /// until a different buffer object name is bound to the same target,
    /// or until the bound buffer object is deleted with `Buffer::delete`.
    /// 
    /// Once created, a named buffer object may be re-bound to any target as often as needed.
    /// However, the GL implementation may make choices about how to optimize the storage of a buffer object based on its initial binding target.
    /// 
    /// The `GL_COPY_READ_BUFFER`, `GL_UNIFORM_BUFFER` and `GL_TEXTURE_BUFFER` targets are available only if the GL version is 3.1 or greater.
    /// 
    /// `GLError::InvalidEnum` is generated if `target` is not one of the allowable values.
    /// 
    /// `GLError::InvalidValue` is generated if `buffer` is not a name previously returned from a call to `Buffer::generate`.
    #[inline(always)]
    pub unsafe fn bind(target:BufferTarget,buffer:u32){
        glBindBuffer(target as GLenum,buffer)
    }

    /// Binds a buffer object to an indexed buffer target.
    /// 
    /// `target` specifies the target of the bind operation.
    /// 
    /// `index` specifies the index of the binding point within the array specified by `target`.
    /// 
    /// `buffer` is the name of a buffer object to bind to the specified binding point.
    /// 
    /// Binds the buffer object `buffer` to the binding point at index `index` of the array of targets specified by `target`.
    /// Each `target` represents an indexed array of buffer binding points,
    /// as well as a single general binding point that can be used by other buffer manipulation functions such as `Buffer::bind` or `Buffer::map`.
    /// In addition to binding `buffer` to the indexed buffer binding target,
    /// `Buffer::bind_base` also binds `buffer` to the generic buffer binding point specified by `target`.
    /// 
    /// Calling `Buffer::bind_base` is equivalent to calling `Buffer::bind_base_range` with `offset` zero and `size` equal to the size of the buffer.
    /// 
    /// `GLError::InvalidEnum` is generated if `target` is not `BufferIndexedTarget`.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if `index` is greater than or equal to the number of `target`-specific indexed binding points,
    /// if `buffer` does not have an associated data store, or if the size of that store is zero.
    #[inline(always)]
    pub unsafe fn bind_base(target:BufferIndexedTarget,index:u32,buffer:u32){
        glBindBufferBase(target as GLenum,index,buffer)
    }

    /// Binds a range within a buffer object to an indexed buffer target.
    /// 
    /// `target` specifies the target of the bind operation.
    /// 
    /// `index` specifies the index of the binding point within the array specified by `target`.
    /// 
    /// `buffer` is the name of a buffer object to bind to the specified binding point.
    /// 
    /// `offset` is the starting offset in basic machine units into the buffer object `buffer`.
    /// 
    /// `size` is the amount of data in machine units that can be read from the buffet object while used as an indexed target.
    /// 
    /// Binds a range the buffer object `buffer` represented by `offset` and `size` to the binding point at index `index` of the array of targets specified by `target`.
    /// Each `target` represents an indexed array of buffer binding points, as well as a single general binding point
    /// that can be used by other buffer manipulation functions such as `Buffer::bind` or `Buffer::map`.
    /// In addition to binding a range of `buffer` to the indexed buffer binding target,
    /// `Buffer::bind_base_range` also binds the range to the generic buffer binding point specified by `target`.
    /// 
    /// `offset` specifies the offset in basic machine units into the buffer object `buffer`
    /// and `size` specifies the amount of data that can be read from the buffer object while used as an indexed target.
    /// 
    /// `GLError::InvalidEnum` is generated if `target` is not `BufferIndexedTarget`.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if `index` is greater than or equal to the number of `target`-specific indexed binding points,
    /// if `buffer` does not have an associated data store, or if the size of that store is zero,
    /// if `size` is less than or equal to zero, or if `offset` + `size` is greater than the value of `GL_BUFFER_SIZE`.
    /// 
    /// Additional errors may be generated if `offset` violates any `target`-specific alignmemt restrictions.
    #[inline(always)]
    pub unsafe fn bind_range(target:BufferIndexedTarget,index:u32,buffer:u32,offset:isize,size:isize){
        glBindBufferRange(target as GLenum,index,buffer,offset,size)
    }
}

impl Buffer{
    /// Creates and initializes a buffer object's data store.
    /// 
    /// `target` specifies the target buffer object.
    /// 
    /// `size` specifies the size in bytes of the buffer object's new data store.
    /// 
    /// `data` specifies a pointer to data that will be copied into the data store for initialization,
    /// or `NULL` if no data is to be copied.
    /// 
    /// `usage` specifies the expected usage pattern of the data store.
    /// 
    /// Creates a new data store for the buffer object currently bound to `target`.
    /// Any pre-existing data store is deleted.
    /// The new data store is created with the specified `size` in bytes and `usage`.
    /// If `data` is not `NULL`, the data store is initialized with data from this pointer.
    /// In its initial state, the new data store is not mapped,
    /// it has a `NULL` mapped pointer, and it's mapped access is `BufferMapAccess::ReadWrite`.
    /// 
    /// `usage` is a hint to the GL implementation as to how a buffer object's data store will be accessed.
    /// This enables the GL implementation to make more intelligent decisions that may significantly impact buffer object performance.
    /// It does not, however, constrain the actual usage of the data store.
    /// `usage` can be broken down into two parts:
    /// first, the frequency of access (modification and usage),
    /// and second, the nature of that access.
    /// 
    /// If `data` is `NULL`, a data store of the specified size is still created,
    /// but it's contents remain uninitialized and thus undefined.
    /// 
    /// Clients must align data elements consistent with the requirements of the client platform, with an additional base-level requirement
    /// that an offset within a buffer to a datum comprising N bytes be a multiple of N.
    /// 
    /// `Error::InvalidEnum` is generated
    /// if `target` is not one of the accepted buffer targets,
    /// if `usage` is not `BufferUsage`.
    /// 
    /// `Error::InvalidValue` is generated if `size` is negative.
    /// 
    /// `Error::InvalidOperation` is generated if the reserved buffer object name 0 is bound to `target`.
    /// 
    /// `Error::OutOfMemory` is generated if the GL is unable to create a data store with the specified `size`.
    #[inline(always)]
    pub unsafe fn allocate<T>(target:BufferTarget,size:isize,data:*const T,usage:BufferUsage){
        glBufferData(target as GLenum,size,data as *const GLvoid,usage as GLenum)
    }

    /// Updates a subset of a buffer object's data store.
    /// 
    /// `target` specifies the target buffer object.
    /// 
    /// `offset` specifies the offset into the buffer object's data store where data replacement will begin, measured in bytes.
    /// 
    /// `size` specifies the size in bytes of the data store region being replaced.
    /// 
    /// `data` specifies a pointer to the new data that will be copied into the data store.
    /// 
    /// Redefines some or all of the data store for the buffer object currently bound to `target`.
    /// Data starting at byte offset `offset` and extending for `size` bytes is copied to the data store from the memory pointed to by `data`.
    /// An error is thrown if `offset` and `size` together define a range beyond the bounds of the buffer object's data store.
    /// 
    /// When replacing the entire data store, consider using `Buffer::write` rather than completely recreating the data store with `Buffer::allocate`.
    /// This avoids the cost of reallocating the data store.
    /// 
    /// Consider using multiple buffer objects to avoid stalling the rendering pipeline during data store updates.
    /// If any rendering in the pipeline makes reference to data in the buffer object being updated by `Buffer::write`,
    /// especially from the specific region being updated,
    /// that rendering must drain from the pipeline before the data store can be updated.
    /// 
    /// Clients must align data elements consistent with the requirements of the client platform,
    /// with an additional base-level requirement
    /// that an offset within a buffer to a datum comprising N bytes be a multiple of N.
    /// 
    /// `Error::InvalidEnum` is generated if `target` is not one of the accepted buffer targets.
    /// 
    /// `Error::InvalidValue` is generated
    /// if `offset` or `size` is negative,
    /// or if together they define a region of memory that extends beyond the buffer object's allocated data store.
    /// 
    /// `Error::InvalidOperation` is generated
    /// if the reserved buffer object name 0 is bound to `target`.
    /// if the buffer object being updated is mapped.
    #[inline(always)]
    pub unsafe fn write<T>(target:BufferTarget,offset:isize,size:isize,data:*const T){
        glBufferSubData(target as GLenum,offset,size,data as *const GLvoid)
    }

    // #[inline(always)]
    // pub unsafe fn copy_buffer(
    //     read_target:BufferTarget,
    //     write_target:BufferTarget,
    //     read_offset:isize,
    //     write_offset:isize,
    //     size:isize,
    // ){
    //     transmute::<usize,fn(
    //         BufferTarget,
    //         BufferTarget,
    //         isize,
    //         isize,
    //         isize
    //     )>(glCopyBufferSubData)(
    //         read_target,
    //         write_target,
    //         read_offset,
    //         write_offset,
    //         size
    //     )
    // }

    /// Returns a subset of a buffer object's data store.
    /// 
    /// `target` specifies the target buffer object.
    /// 
    /// `offset` specifies the offset into the buffer object's data store from which data will be returned, measured in bytes.
    /// 
    /// `size` specifies the size in bytes of the data store region being returned.
    /// 
    /// `data` specifies a pointer to the location where buffer object data is returned.
    /// 
    /// Returns some or all of the data from the buffer object currently bound to `target`.
    /// Data starting at byte offset `offset` and extending for `size` bytes is copied from the data store to the memory pointed to by `data`.
    /// An error is thrown if the buffer object is currently mapped,
    /// or if `offset` and `size` together define a range beyond the bounds of the buffer object's data store.
    /// 
    /// If an error is generated, no change is made to the contents of `data`.
    /// 
    /// `Error::InvalidEnum` is generated if `target` is not one of the accepted buffer targets.
    /// 
    /// `Error::InvalidValue` is generated
    /// if `offset` or `size` is negative,
    /// or if together they define a region of memory that extends beyond the buffer object's allocated data store.
    /// 
    /// `Error::InvalidOperation` is generated
    /// if the reserved buffer object name 0 is bound to `target`.
    /// if the buffer object being updated is mapped.
    #[inline(always)]
    pub unsafe fn read<T>(target:BufferTarget,offset:isize,size:isize,data:*mut T){
        glGetBufferSubData(target as GLenum,offset,size,data as *mut GLvoid)
    }
}

/// Specifies the access policy,
/// indicating whether it will be possible to read from,
/// write to, or both read from and write to the buffer object's mapped data store.
#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum BufferMapAccess{
    Read=READ_ONLY,
    Write=WRITE_ONLY,
    ReadWrite=READ_WRITE
}

/// Specifies the desired access to the buffer object's mapped data store.
#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum BufferMapRangeAccess{
    /// Indicates that the returned pointer may be used to read buffer object data.
    /// No GL error is generated if the pointer is used to query a mapping which excludes this flag,
    /// but the result is undefined and system errors (possibly including program termination) may occur.
    Read=MAP_READ_BIT,

    /// Indicates that the returned pointer may be used to modify buffer object data.
    /// No GL error is generated if the pointer is used to modify a mapping which excludes this flag,
    /// but the result is undefined and system errors (possibly including program termination) may occur.
    Write=MAP_WRITE_BIT,

    /// Indicates that the previous contents of the specified range may be discarded.
    /// Data within this range are undefined with the exception of subsequently written data.
    /// No GL error is generated if sub- sequent GL operations access unwritten data,
    /// but the result is undefined and system errors (possibly including program termination) may occur.
    /// This flag may not be used in combination with `GL_MAP_READ_BIT`.
    InvalidateRange=MAP_INVALIDATE_RANGE_BIT,

    /// Indicates that the previous contents of the entire buffer may be discarded.
    /// Data within the entire buffer are undefined with the exception of subsequently written data.
    /// No GL error is generated if subsequent GL operations access unwritten data,
    /// but the result is undefined and system errors (possibly including program termination) may occur.
    /// This flag may not be used in combination with `GL_MAP_READ_BIT`.
    InvalidateBuffer=MAP_INVALIDATE_BUFFER_BIT,

    /// Indicates that one or more discrete subranges of the mapping may be modified.
    /// When this flag is set,
    /// modifications to each subrange must be explicitly flushed by calling `glFlushMappedBufferRange`.
    /// No GL error is set if a subrange of the mapping is modified and not flushed,
    /// but data within the corresponding subrange of the buffer are undefined.
    /// This flag may only be used in conjunction with `GL_MAP_WRITE_BIT`.
    /// When this option is selected, flushing is strictly limited to regions
    /// that are explicitly indicated with calls to `glFlushMappedBufferRange` prior to unmap;
    /// if this option is not selected `Buffer::unmap` will automatically flush the entire mapped range when called.
    FlushExplicit=MAP_FLUSH_EXPLICIT_BIT,

    /// Indicates that the GL should not attempt to synchronize pending operations on the buffer prior to returning from `Buffer::map_range`.
    /// No GL error is generated if pending operations which source or modify the buffer overlap the mapped region,
    /// but the result of such previous and any subsequent operations is undefined.
    Unsynchronized=MAP_UNSYNCHRONIZED_BIT,
}

/// Specifies the symbolic name of a buffer object parameter.
#[repr(u32)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum BufferParameter{
    /// Returns the access policy set while mapping the buffer object.
    /// 
    /// The initial value is `BufferMapAccess::ReadWrite`.
    Access=BUFFER_ACCESS,

    /// Returns a flag indicating whether the buffer object is currently mapped.
    /// 
    /// The initial value is `false`.
    Mapped=BUFFER_MAPPED,

    /// Returns the size of the buffer object, measured in bytes.
    /// 
    /// The initial value is `0`.
    Size=BUFFER_SIZE,

    /// Returns the buffer object's usage pattern.
    /// 
    /// The initial value is `BufferUsage::StaticDraw`.
    Usage=BUFFER_USAGE
}

/// Specifies a combination of access flags indicating the desired access to the range.
pub struct BufferMapRangeAccessFlags{
    flag:u32,
}

impl BufferMapRangeAccessFlags{
    pub const fn new()->BufferMapRangeAccessFlags{
        Self{
            flag:0u32,
        }
    }

    pub const fn set(mut self,flag:BufferMapRangeAccess)->BufferMapRangeAccessFlags{
        self.flag|=flag as u32;
        self
    }

    pub const fn remove(mut self,flag:BufferMapRangeAccess)->BufferMapRangeAccessFlags{
        self.flag&=!(flag as u32);
        self
    }
}

impl Buffer{
    // #[inline(always)]
    // pub unsafe fn map<I:Sized>(target:BufferTarget,access:BufferMapAccess)->*mut I{
    //     transmute::<usize,fn(BufferTarget,BufferMapAccess)->*mut I>(glMapBuffer)(target,access)
    // }

    // #[inline(always)]
    // pub unsafe fn map_range<I:Sized>(
    //     target:BufferTarget,
    //     offset:isize,
    //     size:isize,
    //     access:BufferMapRangeAccessFlags
    // )->*mut I{
    //     transmute::<usize,fn(BufferTarget,isize,isize,BufferMapRangeAccessFlags)->*mut I>(glMapBufferRange)(
    //         target,
    //         offset,
    //         size,
    //         access
    //     )
    // }

    // #[inline(always)]
    // pub unsafe fn unmap(target:BufferTarget)->bool{
    //     transmute::<usize,fn(BufferTarget)->bool>(glUnmapBuffer)(target)
    // }

    // #[inline(always)]
    // pub unsafe fn get_ptr<I:Sized>(target:BufferTarget,ptr:&mut *mut I){
    //     transmute::<usize,fn(BufferTarget,u32,&mut *mut I)>(glGetBufferPointerv)(target,BUFFER_MAP_POINTER,ptr)
    // }

    // #[inline(always)]
    // pub unsafe fn flush(
    //     target:BufferTarget,
    //     offset:isize,
    //     size:isize,
    // ){
    //     transmute::<usize,fn(BufferTarget,isize,isize)>(glFlushMappedBufferRange)(
    //         target,
    //         offset,
    //         size,
    //     )
    // }
}

impl Buffer{
    // #[inline(always)]
    // pub unsafe fn get_parameter_i32(target:BufferTarget,parameter:BufferParameter,ptr:&mut i32){
    //     transmute::<usize,fn(BufferTarget,BufferParameter,&mut i32)>(glGetBufferParameteriv)(target,parameter,ptr)
    // }

    // #[inline(always)]
    // pub unsafe fn get_parameter_i64(target:BufferTarget,parameter:BufferParameter,ptr:&mut i64){
    //     transmute::<usize,fn(BufferTarget,BufferParameter,&mut i64)>(glGetBufferParameteri64v)(target,parameter,ptr)
    // }
}