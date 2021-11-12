use crate::graphics::{
    GLCore,
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
    pub fn draw_arrays(start:i32,count:i32,mode:PrimitiveType)->GLError{
        unsafe{
            GLCore.drawing.draw_arrays(start,count,mode);
            GLCore.get_error()
        }
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
    pub fn draw_arrays_instanced(start:i32,count:i32,instances:i32,mode:PrimitiveType)->GLError{
        unsafe{
            GLCore.drawing.draw_arrays_instanced(start,count,instances,mode);
            GLCore.get_error()
        }
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
    pub fn draw_elements(start:isize,count:i32,index_type:IndexType,mode:PrimitiveType)->GLError{
        unsafe{
            GLCore.drawing.draw_elements(start,count,index_type,mode);
            GLCore.get_error()
        }
    }

    /// Same as `Drawing::draw_elements_base_vertex`, but with a static index type.
    pub fn draw_elements_typed<T:AvailableIndexType>(start:isize,count:i32,mode:PrimitiveType)->GLError{
        unsafe{
            GLCore.drawing.draw_elements_typed::<T>(start,count,mode);
            GLCore.get_error()
        }
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
    pub fn draw_elements_base_vertex(
        start:isize,
        count:i32,
        base_vertex:i32,
        index_type:IndexType,
        mode:PrimitiveType
    )->GLError{
        unsafe{
            GLCore.drawing.draw_elements_base_vertex(
                start,
                count,
                base_vertex,
                index_type,
                mode
            );
            GLCore.get_error()
        }
    }

    /// Same as `Drawing::draw_elements_base_vertex`, but with a static index type.
    pub fn draw_elements_base_vertex_typed<T:AvailableIndexType>(
        start:isize,
        count:i32,
        base_vertex:i32,
        mode:PrimitiveType
    )->GLError{
        unsafe{
            GLCore.drawing.draw_elements_base_vertex_typed::<T>(
                start,
                count,
                base_vertex,
                mode
            );
            GLCore.get_error()
        }
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
    /// if ( mode, count or index_type is invalid ){
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
    pub fn draw_elements_instanced(
        start:isize,
        count:i32,
        instances:i32,
        index_type:IndexType,
        mode:PrimitiveType
    )->GLError{
        unsafe{
            GLCore.drawing.draw_elements_instanced(
                start,
                count,
                instances,
                index_type,
                mode
            );
            GLCore.get_error()
        }
    }

    /// Same as `Drawing::draw_elements_instanced`, but with a static index type.
    pub fn draw_elements_instanced_typed<T:AvailableIndexType>(
        start:isize,
        count:i32,
        instances:i32,
        mode:PrimitiveType
    )->GLError{
        unsafe{
            GLCore.drawing.draw_elements_instanced_typed::<T>(
                start,
                count,
                instances,
                mode
            );
            GLCore.get_error()
        }
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
    pub fn draw_elements_base_vertex_instanced(
        start:isize,
        count:i32,
        base_vertex:i32,
        instances:i32,
        index_type:IndexType,
        mode:PrimitiveType
    )->GLError{
        unsafe{
            GLCore.drawing.draw_elements_base_vertex_instanced(
                start,
                count,
                base_vertex,
                instances,
                index_type,
                mode
            );
            GLCore.get_error()
        }
    }

    /// Same as `Drawing::draw_elements_base_vertex_instanced`, but with a static index type.
    pub fn draw_elements_base_vertex_instanced_typed<T:AvailableIndexType>(
        start:isize,
        count:i32,
        base_vertex:i32,
        instances:i32,
        mode:PrimitiveType
    )->GLError{
        unsafe{
            GLCore.drawing.draw_elements_base_vertex_instanced_typed::<T>(
                start,
                count,
                base_vertex,
                instances,
                mode
            );
            GLCore.get_error()
        }
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
    pub fn draw_range_elements(
        start:isize,
        count:i32,
        range_start:u32,
        range_end:u32,
        index_type:IndexType,
        mode:PrimitiveType
    )->GLError{
        unsafe{
            GLCore.drawing.draw_range_elements(
                start,
                count,
                range_start,
                range_end,
                index_type,
                mode
            );
            GLCore.get_error()
        }
    }

    /// Same as `Drawing::draw_range_elements`, but with a static index type.
    pub fn draw_range_elements_typed<T:AvailableIndexType>(
        start:isize,
        count:i32,
        range_start:u32,
        range_end:u32,
        mode:PrimitiveType
    )->GLError{
        unsafe{
            GLCore.drawing.draw_range_elements_typed::<T>(
                start,
                count,
                range_start,
                range_end,
                mode
            );
            GLCore.get_error()
        }
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
    pub fn draw_range_elements_base_vertex(
        start:isize,
        count:i32,
        range_start:u32,
        range_end:u32,
        index_type:IndexType,
        base_vertex:i32,
        mode:PrimitiveType
    )->GLError{
        unsafe{
            GLCore.drawing.draw_range_elements_base_vertex(
                start,
                count,
                range_start,
                range_end,
                index_type,
                base_vertex,
                mode
            );
            GLCore.get_error()
        }
    }

    /// Same as `Drawing::draw_range_elements_base_vertex`, but with a static index type.
    pub fn draw_range_elements_base_vertex_typed<T:AvailableIndexType>(
        start:isize,
        count:i32,
        range_start:u32,
        range_end:u32,
        mode:PrimitiveType
    )->GLError{
        unsafe{
            GLCore.drawing.draw_range_elements_typed::<T>(
                start,
                count,
                range_start,
                range_end,
                mode
            );
            GLCore.get_error()
        }
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
    /// `GLError::InvalidValue` is generated if `start.len()` > `i32::MAX`.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if a non-zero buffer object name is bound to an enabled array
    /// and the buffer object's data store is currently mapped.
    pub fn multi_draw_arrays(
        start:&[i32],
        count:&[i32],
        mode:PrimitiveType
    )->GLError{
        unsafe{
            GLCore.drawing.multi_draw_arrays(
                start.as_ptr(),
                count.as_ptr(),
                start.len() as i32,
                mode
            );
            GLCore.get_error()
        }
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
    /// `GLError::InvalidValue` is generated if `start.len()` > `i32::MAX`.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if a non-zero buffer object name is bound to an enabled array or the element array
    /// and the buffer object's data store is currently mapped.
    pub fn multi_draw_elements(
        start:&[isize],
        count:&[i32],
        index_type:IndexType,
        mode:PrimitiveType
    )->GLError{
        unsafe{
            GLCore.drawing.multi_draw_elements(
                start.as_ptr(),
                count.as_ptr(),
                start.len() as i32,
                index_type,
                mode
            );
            GLCore.get_error()
        }
    }

    /// Same as `Drawing::multi_draw_elements`, but with a static index type.
    pub fn multi_draw_elements_typed<T:AvailableIndexType>(
        start:&[isize],
        count:&[i32],
        mode:PrimitiveType
    )->GLError{
        unsafe{
            GLCore.drawing.multi_draw_elements_typed::<T>(
                start.as_ptr(),
                count.as_ptr(),
                start.len() as i32,
                mode
            );
            GLCore.get_error()
        }
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
    /// `GLError::InvalidValue` is generated if `start.len()` > `i32::MAX`.
    /// 
    /// `GLError::InvalidOperation` is generated
    /// if a non-zero buffer object name is bound to an enabled array or the element array
    /// and the buffer object's data store is currently mapped.
    pub fn multi_draw_elements_base_vertex(
        start:&[isize],
        count:&[i32],
        base_vertex:&[i32],
        index_type:IndexType,
        mode:PrimitiveType
    )->GLError{
        unsafe{
            GLCore.drawing.multi_draw_elements_base_vertex(
                start.as_ptr(),
                count.as_ptr(),
                base_vertex.as_ptr(),
                start.len() as i32,
                index_type,
                mode
            );
            GLCore.get_error()
        }
    }

    /// Same as `Drawing::multi_draw_elements_base_vertex`, but with a static index type.
    pub fn multi_draw_elements_base_vertex_typed<T:AvailableIndexType>(
        start:&[isize],
        count:&[i32],
        base_vertex:&[i32],
        mode:PrimitiveType
    )->GLError{
        unsafe{
            GLCore.drawing.multi_draw_elements_base_vertex_typed::<T>(
                start.as_ptr(),
                count.as_ptr(),
                base_vertex.as_ptr(),
                start.len() as i32,
                mode
            );
            GLCore.get_error()
        }
    }
}