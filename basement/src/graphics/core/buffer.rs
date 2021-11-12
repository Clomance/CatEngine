#[cfg(all(target_os="windows",feature="windows"))]
use crate::windows::OpenGraphicsLibrary;

use core::mem::transmute;

// Buffer targets
const ARRAY_BUFFER:u32=0x8892;
const COPY_READ_BUFFER:u32=0x8F36;
const COPY_WRITE_BUFFER:u32=0x8F37;
const ELEMENT_ARRAY_BUFFER:u32=0x8893;
const PIXEL_PACK_BUFFER:u32=0x88EB;
const PIXEL_UNPACK_BUFFER:u32=0x88EC;
const TEXTURE_BUFFER:u32=0x8C2A;
const TRANSFORM_FEEDBACK_BUFFER:u32=0x8C8E;
const UNIFORM_BUFFER:u32=0x8A11;

// Buffer usages
const STREAM_DRAW:u32=0x88E0;
const STREAM_READ:u32=0x88E1;
const STREAM_COPY:u32=0x88E2;
const STATIC_DRAW:u32=0x88E4;
const STATIC_READ:u32=0x88E5;
const STATIC_COPY:u32=0x88E6;
const DYNAMIC_DRAW:u32=0x88E8;
const DYNAMIC_READ:u32=0x88E9;
const DYNAMIC_COPY:u32=0x88EA;

// the access policy
const READ_ONLY:u32=0x88B8;
const READ_WRITE:u32=0x88BA;
const WRITE_ONLY:u32=0x88B9;

const MAP_READ_BIT:u32=0x0001;
const MAP_UNSYNCHRONIZED_BIT:u32=0x0020;
const MAP_WRITE_BIT:u32=0x0002;
const MAP_FLUSH_EXPLICIT_BIT:u32=0x0010;
const MAP_INVALIDATE_BUFFER_BIT:u32=0x0008;
const MAP_INVALIDATE_RANGE_BIT:u32=0x0004;

// parameters
const BUFFER_MAP_POINTER:u32=0x88BD;

const BUFFER_ACCESS:u32=0x88BB;
const BUFFER_MAPPED:u32=0x88BC;
const BUFFER_SIZE:u32=0x8764;
const BUFFER_USAGE:u32=0x8765;

/// Specifies the target buffer object.
#[repr(u32)]
#[derive(Clone,Copy,Debug)]
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

/// Specifies the indexed target buffer object for.
#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum BufferIndexedTarget{
    TransformFeedbackBuffer=TRANSFORM_FEEDBACK_BUFFER,
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
#[derive(Clone,Copy,Debug)]
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

/// Specifies the access policy,
/// indicating whether it will be possible to read from,
/// write to, or both read from and write to the buffer object's mapped data store.
#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum BufferMapAccess{
    Read=READ_ONLY,
    Write=WRITE_ONLY,
    ReadWrite=READ_WRITE
}

/// Specifies the desired access to the buffer object's mapped data store.
#[repr(u32)]
#[derive(Clone,Copy,Debug)]
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
#[derive(Clone,Copy,Debug)]
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

/// Contains buffer object functions.
pub struct Buffer{
    glGenBuffers:usize, //
    glDeleteBuffers:usize, //
    glIsBuffer:usize, //

    glBindBuffer:usize, // rename functions in the docs
    glBindBufferBase:usize, //
    glBindBufferRange:usize, //

    glBufferData:usize, //
    glBufferSubData:usize, //
    glCopyBufferSubData:usize, //
    glGetBufferSubData:usize, //

    glMapBuffer:usize, //
    glMapBufferRange:usize, //
    glUnmapBuffer:usize, //
    glGetBufferPointerv:usize, //
    glFlushMappedBufferRange:usize, //

    glGetBufferParameteriv:usize, //
    glGetBufferParameteri64v:usize, //
}

impl Buffer{
    pub const fn new()->Buffer{
        Self{
            glGenBuffers:0,
            glDeleteBuffers:0,
            glIsBuffer:0,

            glBindBuffer:0,
            glBindBufferBase:0,
            glBindBufferRange:0,

            glBufferData:0,
            glBufferSubData:0,
            glCopyBufferSubData:0,
            glGetBufferSubData:0,

            glMapBuffer:0,
            glMapBufferRange:0,
            glUnmapBuffer:0,
            glGetBufferPointerv:0,
            glFlushMappedBufferRange:0,

            glGetBufferParameteriv:0,
            glGetBufferParameteri64v:0,
        }
    }

    #[cfg(all(target_os="windows",feature="windows"))]
    pub fn load(&mut self,library:&OpenGraphicsLibrary){
        unsafe{
            self.glGenBuffers=transmute(library.get_proc_address("glGenBuffers\0"));
            self.glDeleteBuffers=transmute(library.get_proc_address("glDeleteBuffers\0"));
            self.glIsBuffer=transmute(library.get_proc_address("glIsBuffer\0"));

            self.glBindBuffer=transmute(library.get_proc_address("glBindBuffer\0"));
            self.glBindBufferBase=transmute(library.get_proc_address("glBindBufferBase\0"));
            self.glBindBufferRange=transmute(library.get_proc_address("glBindBufferRange\0"));

            self.glBufferData=transmute(library.get_proc_address("glBufferData\0"));
            self.glBufferSubData=transmute(library.get_proc_address("glBufferSubData\0"));
            self.glCopyBufferSubData=transmute(library.get_proc_address("glCopyBufferSubData\0"));
            self.glGetBufferSubData=transmute(library.get_proc_address("glGetBufferSubData\0"));

            self.glMapBuffer=transmute(library.get_proc_address("glMapBuffer\0"));
            self.glMapBufferRange=transmute(library.get_proc_address("glMapBufferRange\0"));
            self.glUnmapBuffer=transmute(library.get_proc_address("glUnmapBuffer\0"));
            self.glGetBufferPointerv=transmute(library.get_proc_address("glGetBufferPointerv\0"));
            self.glFlushMappedBufferRange=transmute(library.get_proc_address("glFlushMappedBufferRange\0"));

            self.glGetBufferParameteriv=transmute(library.get_proc_address("glGetBufferParameteriv\0"));
            self.glGetBufferParameteri64v=transmute(library.get_proc_address("glGetBufferParameteri64v\0"));
        }
    }
}

impl Buffer{
    /// Generates a buffer object.
    /// 
    /// See `Buffer::generate`.
    #[inline(always)]
    pub fn generate_one(&self,buffer:&mut u32){
        unsafe{
            transmute::<usize,fn(i32,&mut u32)>(self.glGenBuffers)(1,buffer)
        }
    }

    /// Deletes a buffer object.
    /// 
    /// See `Buffer::delete`.
    #[inline(always)]
    pub fn delete_one(&self,buffer:&u32){
        unsafe{
            transmute::<usize,fn(i32,&u32)>(self.glDeleteBuffers)(1,buffer)
        }
    }

    /// Generates buffer objects.
    /// 
    /// Returns `buffers.len()` buffer object names in `buffers`.
    /// There is no guarantee that the names form a contiguous set of integers;
    /// however, it is guaranteed that none of the returned names was in use immediately before the call to `Buffer::generate`.
    /// 
    /// Buffer object names returned by a call to glGenBuffers are not returned by subsequent calls, unless they are first deleted with glDeleteBuffers.
    /// 
    /// No buffer objects are associated with the returned buffer object names until they are first bound by calling glBindBuffer.
    /// 
    /// `GLError::InvalidValue` is generated if `buffers.len()` is greater than `i32::MAX`.
    #[inline(always)]
    pub fn generate(&self,buffers:&mut [u32]){
        unsafe{
            transmute::<usize,fn(i32,&mut [u32])>(self.glGenBuffers)(buffers.len() as i32,buffers)
        }
    }

    /// Deletes buffer objects.
    /// 
    /// Deletes `buffers.len()` buffer objects named by the elements of the array buffers.
    /// After a buffer object is deleted, it has no contents,
    /// and its name is free for reuse (for example by `Buffer::generate`).
    /// If a buffer object that is currently bound is deleted,
    /// the binding reverts to 0 (the absence of any buffer object).
    /// 
    /// Silently ignores 0's and names that do not correspond to existing buffer objects.
    /// 
    /// `GLError::InvalidValue` is generated if `buffers.len()` is greater than `i32::MAX`.
    #[inline(always)]
    pub fn delete(&self,buffers:&[u32]){
        unsafe{
            transmute::<usize,fn(i32,&[u32])>(self.glDeleteBuffers)(buffers.len() as i32,buffers)
        }
    }

    /// Determines if a name corresponds to a buffer object.
    /// 
    /// Returns `true` if buffer is currently the name of a buffer object.
    /// If buffer is zero,
    /// or is a non-zero value that is not currently the name of a buffer object,
    /// or if an error occurs, returns `false`.
    /// 
    /// A name returned by `Buffer::generate`,
    /// but not yet associated with a buffer object by calling `Buffer::bind`,
    /// is not the name of a buffer object.
    #[inline(always)]
    pub fn is_buffer(&self,buffer:u32)->bool{
        unsafe{
            transmute::<usize,fn(u32)->bool>(self.glIsBuffer)(buffer)
        }
    }
}

impl Buffer{
    /// Binds a named buffer object.
    /// 
    /// Binds a buffer object to the specified buffer binding point.
    /// Calling `Buffer::bind` with `target` set to one of the accepted symbolic constants and `buffer`
    /// set to the name of a buffer object binds that buffer object name to the target.
    /// If no buffer object with name `buffer` exists, one is created with that name.
    /// When a buffer object is bound to a target,
    /// the previous binding for that target is automatically broken.
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
    /// While a non-zero buffer object name is bound, GL operations on the target to which it is bound affect the bound buffer object,
    /// and queries of the target to which it is bound return state from the bound buffer object.
    /// While buffer object name zero is bound, as in the initial state,
    /// attempts to modify or query state on the target to which it is bound generates an `GLError::InvalidOperation` error.
    /// 
    /// When a non-zero buffer object is bound to the `BufferTarget::ArrayBuffer` target,
    /// the vertex array pointer parameter is interpreted as an offset
    /// within the buffer object measured in basic machine units.
    /// 
    /// While a non-zero buffer object is bound to the `BufferTarget::ElementArrayBuffer` target,
    /// the indices parameter of `glDrawElements`, `glDrawElementsInstanced`, `glDrawElementsBaseVertex`,
    /// `glDrawRangeElements`, `glDrawRangeElementsBaseVertex`, `glMultiDrawElements`,
    /// or `glMultiDrawElementsBaseVertexis` interpreted as an offset within the buffer object measured in basic machine units.
    /// 
    /// While a non-zero buffer object is bound to the `BufferTarget::PixelPackBuffer` target,
    /// the following commands are affected: `glGetCompressedTexImage`, `glGetTexImage`, and `glReadPixels`.
    /// The pointer parameter is interpreted as an offset within the buffer object measured in basic machine units.
    /// 
    /// While a non-zero buffer object is bound to the `BufferTarget::PixelUnpackBuffer` target,
    /// the following commands are affected: `glCompressedTexImage1D`, `glCompressedTexImage2D`, `glCompressedTexImage3D`,
    /// `glCompressedTexSubImage1D`, `glCompressedTexSubImage2D`, `glCompressedTexSubImage3D`, `glTexImage1D`, `glTexImage2D`,
    /// `glTexImage3D`, `glTexSubImage1D`, `glTexSubImage2D`, and `glTexSubImage3D`.
    /// The pointer parameter is interpreted as an offset within the buffer object measured in basic machine units.
    /// 
    /// The buffer targets `BufferTarget::CopyReadBuffer` and `BufferTarget::CopyWriteBuffer` are provided
    /// to allow `Buffer::copy_buffer` to be used without disturbing the state of other bindings.
    /// However, `Buffer::copy_buffer` may be used with any pair of buffer binding points.
    /// 
    /// The `BufferTarget::TransformFeedbackBuffer` buffer binding point may be passed to `Buffer::bind`,
    /// but will not directly affect transform feedback state.
    /// Instead, the indexed `BufferTarget::TransformFeedbackBuffer` bindings
    /// must be used through a call to `Buffer::bind_base` or `Buffer::bind_range`.
    /// This will affect the generic `BufferTarget::TransformFeedbackBuffer` binding.
    /// 
    /// Likewise, the `BufferTarget::UniformBuffer` buffer binding point may be used,
    /// but does not directly affect uniform buffer state.
    /// `Buffer::bind_base` or `Buffer::bind_range` must be used to bind a buffer to an indexed uniform buffer binding point.
    /// 
    /// A buffer object binding created with `Buffer::bind` remains active
    /// until a different buffer object name is bound to the same target,
    /// or until the bound buffer object is deleted with `Buffer::delete`.
    /// 
    /// Once created, a named buffer object may be re-bound to any target as often as needed.
    /// However, the GL implementation may make choices about
    /// how to optimize the storage of a buffer object based on its initial binding target.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if `buffer` is not a name previously returned from a call to `Buffer::generate`.
    #[inline(always)]
    pub unsafe fn bind(&self,target:BufferTarget,buffer:u32){
        transmute::<usize,fn(BufferTarget,u32)>(self.glBindBuffer)(target,buffer)
    }

    /// Binds a range within a buffer object to an indexed buffer target.
    /// 
    /// Binds the buffer object `buffer` to the binding point at index `index` of the array of targets specified by `target`.
    /// Each target represents an indexed array of buffer binding points,
    /// as well as a single general binding point that can be used by other buffer manipulation functions such as `Buffer::bind` or `Buffer::map`.
    /// In addition to binding `buffer` to the indexed buffer binding target,
    /// `Buffer::bind_base` also binds `buffer` to the generic buffer binding point specified by `target`.
    /// 
    /// Calling `Buffer::bind_base` is equivalent to calling `Buffer::bind_range` with `offset` zero and `size` equal to the size of the buffer.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if `index` is greater than or equal to the number of target-specific indexed binding points,
    /// if `buffer` does not have an associated data store,
    /// or if the size of that store is zero.
    #[inline(always)]
    pub unsafe fn bind_base(&self,target:BufferIndexedTarget,index:u32,buffer:u32){
        transmute::<usize,fn(BufferIndexedTarget,u32,u32)>(self.glBindBufferBase)(target,index,buffer)
    }

    /// Binds a range within a buffer object to an indexed buffer target.
    /// 
    /// Binds a range the buffer object `buffer` represented by `offset` and `size`
    /// to the binding point at index `index` of the array of targets specified by `target`.
    /// Each `target` represents an indexed array of buffer binding points,
    /// as well as a single general binding point
    /// that can be used by other buffer manipulation functions such as `Buffer::bind` or `Buffer::map`.
    /// In addition to binding a range of `buffer` to the indexed buffer binding target,
    /// `Buffer::bind_range` also binds the range to the generic buffer binding point specified by `target`.
    /// 
    /// `offset` specifies the offset in basic machine units into the buffer object `buffer`
    /// and `size` specifies the amount of data
    /// that can be read from the buffer object while used as an indexed target.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if `index` is greater than or equal to the number of target-specific indexed binding points,
    /// if `size` is less than or equal to zero, or if `offset + size` is greater than the value of `GL_BUFFER_SIZE`.
    /// 
    /// Additional errors may be generated if offset violates any target-specific alignmemt restrictions.
    #[inline(always)]
    pub unsafe fn bind_range(&self,target:BufferIndexedTarget,index:u32,buffer:u32,offset:isize,size:isize){
        transmute::<usize,fn(BufferIndexedTarget,u32,u32,isize,isize)>(self.glBindBufferRange)(target,index,buffer,offset,size)
    }
}

impl Buffer{
    /// Creates and initializes a buffer object's data store.
    /// 
    /// Creates a new data store for the buffer object currently bound to `target`.
    /// Any pre-existing data store is deleted.
    /// The new data store is created with the specified `size` in bytes and `usage`.
    /// If `data` is not `None`, the data store is initialized with data from this pointer.
    /// In its initial state, the new data store is not mapped,
    /// it has a `NULL` mapped pointer, and its mapped access is `BufferMapAccess::ReadWrite`.
    /// 
    /// If `data` is `None`, a data store of the specified size is still created,
    /// but its contents remain uninitialized and thus undefined.
    /// 
    /// Clients must align data elements consistent with the requirements of the client platform,
    /// with an additional base-level requirement that an offset within a buffer to a datum comprising N bytes be a multiple of N.
    /// 
    /// `GLError::InvalidValue` is generated if `size` is negative.
    /// 
    /// `GLError::InvalidOperation` is generated if the reserved buffer object name 0 is bound to `target`.
    /// 
    /// `GLError::OutOfMemory` is generated if the GL is unable to create a data store with the specified size.
    #[inline(always)]
    pub unsafe fn rewrite<I:Sized>(&self,target:BufferTarget,size:isize,data:Option<&I>,usage:BufferUsage){
        transmute::<usize,fn(BufferTarget,isize,Option<&I>,BufferUsage)>(self.glBufferData)(target,size,data,usage)
    }

    /// Updates a subset of a buffer object's data store.
    /// 
    /// Redefines some or all of the data store for the buffer object currently bound to `target`.
    /// Data starting at byte offset `offset` and extending for `size` bytes is copied to the data store from the memory pointed to by `data`.
    /// 
    /// When replacing the entire data store,
    /// consider using `Buffer::write` rather than completely recreating the data store with `Buffer::rewrite`.
    /// This avoids the cost of reallocating the data store.
    /// 
    /// Consider using multiple buffer objects to avoid stalling the rendering pipeline during data store updates.
    /// If any rendering in the pipeline makes reference to data in the buffer object being updated by `Buffer::write`,
    /// especially from the specific region being updated,
    /// that rendering must drain from the pipeline before the data store can be updated.
    /// 
    /// Clients must align data elements consistent with the requirements of the client platform,
    /// with an additional base-level requirement that an offset within a buffer to a datum comprising N bytes be a multiple of N.
    /// 
    /// `GLError::InvalidValue` is generated if `offset` or `size` is negative,
    /// or if together they define a region of memory that extends beyond the buffer object's allocated data store.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if the reserved buffer object name 0 is bound to `target`,
    /// or if the buffer object being updated is mapped.
    #[inline(always)]
    pub unsafe fn write<I:Sized>(&self,target:BufferTarget,offset:isize,size:isize,data:&I){
        transmute::<usize,fn(BufferTarget,isize,isize,&I)>(self.glBufferSubData)(target,offset,size,data)
    }

    /// Copies part of the data store of a buffer object to the data store of another buffer object.
    /// 
    /// Copies part of the data store attached to `readtarget` to the data store attached to `writetarget`.
    /// The number of basic machine units indicated by `size` is copied from the source,
    /// at offset `readoffset` to the destination at `writeoffset`, also in basic machine units.
    /// 
    /// The targets `BufferTarget::CopyReadBuffer` and `BufferTarget::CopyWriteBuffer` are provided specifically
    /// to allow copies between buffers without disturbing other GL state.
    /// 
    /// Available only if the GL version is 3.1 or greater.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if any of `read_offset`, `write_offset` or `size` is negative,
    /// if `read_offset + size` exceeds the size of the buffer object bound to `read_target`,
    /// if `write_offset + size` exceeds the size of the buffer object bound to `write_target`,
    /// or if the same buffer object is bound to both `read_target` and `write_target`
    /// and the ranges `[read_offset, read_offset + size)` and `[write_offset, writeo_offset + size)` overlap.
    /// 
    /// `GLError::InvalidOperation` is generated if zero is bound to `read_target` or `write_target`,
    /// or if the buffer object bound to either `read_target` or `write_target` is mapped.
    #[inline(always)]
    pub unsafe fn copy_buffer(
        &self,
        read_target:BufferTarget,
        write_target:BufferTarget,
        read_offset:isize,
        write_offset:isize,
        size:isize,
    ){
        transmute::<usize,fn(
            BufferTarget,
            BufferTarget,
            isize,
            isize,
            isize
        )>(self.glCopyBufferSubData)(
            read_target,
            write_target,
            read_offset,
            write_offset,
            size
        )
    }

    /// Returns a subset of a buffer object's data store.
    /// 
    /// returns some or all of the data from the buffer object currently bound to target.
    /// Data starting at byte `offset` offset and extending for `size` bytes is copied from the data store to the memory pointed to by `data`.
    /// 
    /// If an error is generated, no change is made to the contents of `data`.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if `offset` or `size` is negative, or if together they define a region of memory
    /// that extends beyond the buffer object's allocated data store.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if the reserved buffer object name 0 is bound to `target`,
    /// if the buffer object being queried is mapped.
    #[inline(always)]
    pub unsafe fn read<I:Sized>(&self,target:BufferTarget,offset:isize,size:isize,data:&mut I){
        transmute::<usize,fn(BufferTarget,isize,isize,&mut I)>(self.glGetBufferSubData)(target,offset,size,data)
    }
}

impl Buffer{
    /// Maps a buffer object's data store.
    /// 
    /// Maps to the client's address space the entire data store of the buffer object currently bound to target.
    /// The data can then be directly read and/or written relative to the returned pointer,
    /// depending on the specified access policy.
    /// If the GL is unable to map the buffer object's data store,
    /// `Buffer::map` generates an error and returns `NULL`.
    /// This may occur for system-specific reasons, such as low virtual memory availability.
    /// 
    /// If a mapped data store is accessed in a way inconsistent with the specified access policy,
    /// no error is generated, but performance may be negatively impacted and system errors,
    /// including program termination, may result.
    /// Unlike the usage parameter of `Buffer::rewrite`, access is not a hint,
    /// and does in fact constrain the usage of the mapped data store on some GL implementations.
    /// In order to achieve the highest performance available,
    /// a buffer object's data store should be used in ways consistent
    /// with both its specified usage and access parameters.
    /// 
    /// A mapped data store must be unmapped with `Buffer::unmap` before its buffer object is used.
    /// Otherwise an error will be generated by any GL command that attempts to dereference the buffer object's data store.
    /// 
    /// A buffer object's mapped data store is automatically unmapped
    /// when the buffer object is deleted or its data store is recreated with `Buffer::rewrite`.
    /// 
    /// Parameter values passed to GL commands may not be sourced from the returned pointer.
    /// No error will be generated, but results will be undefined and will likely vary across GL implementations.
    /// 
    /// `GLError::OutOfMemory` is generated if he GL is unable to map the buffer object's data store.
    /// This may occur for a variety of system-specific reasons,
    /// such as the absence of sufficient remaining virtual memory.
    /// 
    /// `GLError::InvalidOperation` is generated if the reserved buffer object name 0 is bound to `target`,
    /// if the bound buffer object's data store is already mapped.
    #[inline(always)]
    pub unsafe fn map<I:Sized>(&self,target:BufferTarget,access:BufferMapAccess)->*mut I{
        transmute::<usize,fn(BufferTarget,BufferMapAccess)->*mut I>(self.glMapBuffer)(target,access)
    }

    /// Maps a section of a buffer object's data store.
    /// 
    /// Maps all or part of the data store of a buffer object into the client's address space.
    /// 
    /// `offset` and `size` indicate the range of data in the buffer object that is to be mapped,
    /// in terms of basic machine units.
    /// 
    /// If no error occurs, a pointer to the beginning of the mapped range is returned once all pending operations on that buffer have completed,
    /// and may be used to modify and/or query the corresponding range of the buffer,
    /// according to the following flag bits set in `access`.
    /// 
    /// If an error occurs, returns a `NULL` pointer.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if either of `offset` or `size` is negative,
    /// or if `offset + size` is greater than the value of `GL_BUFFER_SIZE`.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if the buffer is already in a mapped state,
    /// if neither `BufferMapRangeAccess::Read` or `BufferMapRangeAccess::Write` is set,
    /// if `BufferMapRangeAccess::Read` is set and any of `BufferMapRangeAccess::InvalidateRange`,
    /// `BufferMapRangeAccess::InvalidateBuffer`, or `BufferMapRangeAccess::Unsynchronized` is set,
    /// if `BufferMapRangeAccess::FlushExplicit` is set and `BufferMapRangeAccess::Write` is not set.
    /// 
    /// `GLError::OutOfMemory` is generated if memory for the mapping could not be obtained.
    #[inline(always)]
    pub unsafe fn map_range<I:Sized>(
        &self,
        target:BufferTarget,
        offset:isize,
        size:isize,
        access:BufferMapRangeAccessFlags
    )->*mut I{
        transmute::<usize,fn(BufferTarget,isize,isize,BufferMapRangeAccessFlags)->*mut I>(self.glMapBufferRange)(
            target,
            offset,
            size,
            access
        )
    }

    /// Unmaps a buffer object's data store.
    /// 
    /// When a data store is unmapped, the pointer to its data store becomes invalid.
    /// `Buffer::unmap` returns `true` unless the data store contents have become corrupt
    /// during the time the data store was mapped.
    /// This can occur for system-specific reasons that affect the availability of graphics memory,
    /// such as screen mode changes.
    /// In such situations, `false` is returned and the data store contents are undefined.
    /// An application must detect this rare condition and reinitialize the data store.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if the bound buffer object's data store is not currently mapped.
    #[inline(always)]
    pub unsafe fn unmap(&self,target:BufferTarget)->bool{
        transmute::<usize,fn(BufferTarget)->bool>(self.glUnmapBuffer)(target)
    }

    /// Returns the pointer to a mapped buffer object's data store.
    /// 
    /// Returns pointer information. The pointer to which the buffer object's data store is mapped.
    /// If the data store is not currently mapped, `NULL` is returned.
    /// `ptr` is a pointer to a location in which to place the returned pointer value.
    /// 
    /// If an error is generated, no change is made to the contents of `ptr`.
    /// 
    /// The initial value for the pointer is `NULL`.
    /// 
    /// `GLError::InvalidOperation` is generated if the reserved buffer object name 0 is bound to `target`.
    #[inline(always)]
    pub unsafe fn get_ptr<I:Sized>(&self,target:BufferTarget,ptr:&mut *mut I){
        transmute::<usize,fn(BufferTarget,u32,&mut *mut I)>(self.glGetBufferPointerv)(target,BUFFER_MAP_POINTER,ptr)
    }

    /// Indicates modifications to a range of a mapped buffer.
    /// 
    /// Indicates that modifications have been made to a range of a mapped buffer.
    /// The buffer must previously have been mapped with the `BufferMapRangeAccess::FlushExplicit` flag.
    /// `offset` and `size` indicate the modified subrange of the mapping, in basic units.
    /// The specified subrange to flush is relative to the start of the currently mapped range of the buffer.
    /// `Buffer::flush` may be called multiple times to indicate distinct subranges of the mapping which require flushing.
    /// 
    /// `GLError::InvalidValue` is generated
    /// if `offset` or `size` is negative, or if `offset + size` exceeds the size of the mapping.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if the reserved buffer object name 0 is bound to `target`,
    /// if the buffer bound to target is not mapped,
    /// or is mapped without the `BufferMapRangeAccess::FlushExplicit` flag.
    #[inline(always)]
    pub unsafe fn flush(
        &self,
        target:BufferTarget,
        offset:isize,
        size:isize,
    ){
        transmute::<usize,fn(BufferTarget,isize,isize)>(self.glFlushMappedBufferRange)(
            target,
            offset,
            size,
        )
    }
}

impl Buffer{
    /// Returns parameters of a buffer object.
    /// 
    /// Returns in `ptr` a selected parameter of the buffer object specified by `target`.
    /// 
    /// If an error is generated, no change is made to the contents of `ptr`.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if the reserved buffer object name 0 is bound to `target`.
    #[inline(always)]
    pub unsafe fn get_parameter_i32(&self,target:BufferTarget,parameter:BufferParameter,ptr:&mut i32){
        transmute::<usize,fn(BufferTarget,BufferParameter,&mut i32)>(self.glGetBufferParameteriv)(target,parameter,ptr)
    }

    /// Returns parameters of a buffer object.
    /// 
    /// Returns in `ptr` a selected parameter of the buffer object specified by `target`.
    /// 
    /// If an error is generated, no change is made to the contents of `ptr`.
    /// 
    /// Available only if the GL version is 3.2 or higher.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if the reserved buffer object name 0 is bound to `target`.
    #[inline(always)]
    pub unsafe fn get_parameter_i64(&self,target:BufferTarget,parameter:BufferParameter,ptr:&mut i64){
        transmute::<usize,fn(BufferTarget,BufferParameter,&mut i64)>(self.glGetBufferParameteri64v)(target,parameter,ptr)
    }
}