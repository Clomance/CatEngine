use crate::{
    window_center,
};

use super::{
    // types
    FrameIDType,
    ObjectIDType,
    ElementIndexType,
    // consts
    frame_size,
    minimal_frames,
    // structs
    SimpleVertex2D,
    HeapSystem,
    StackSystem,
    HeapObject,
    StackObject,
    HeapDrawableObject,
    StackDrawableObject,
    ObjectAllocation,
    DrawParameters,
    // enums
    HeapDrawType,
    StackDrawType,
    DrawMode,
};

use cat_engine_basement::graphics::{
    level0::{
        VertexArray,
        BufferUsage,
    },
    level1::{
        buffer::{
            VertexBuffer,
            IndexBuffer,
        },
        shader::{
            VertexShader,
            FragmentShader
        }
    },
    level2::Program,
    gl::{
        // constants
        UNSIGNED_BYTE,
        UNSIGNED_SHORT,
        // functions
        DrawArrays,
        DrawElements,
        MultiDrawElements,
        MultiDrawArrays,
        Uniform2f,
    },
};

use std::{
    mem::size_of,
    ffi::c_void
};

pub struct SimpleGraphics{
    vertex_buffer:VertexBuffer<SimpleVertex2D>,
    index_buffer:IndexBuffer<ElementIndexType>,
    vertex_array:VertexArray<SimpleVertex2D>,

    object_allocation:ObjectAllocation<SimpleVertex2D>,
    draw:Program,
}

impl SimpleGraphics{
    pub fn new(
        stack_vertices:ElementIndexType,
        stack_indices:i32,
        stack_objects:ObjectIDType,
        heap_vertex_frames:FrameIDType,
        heap_index_frames:FrameIDType,
        heap_objects:ObjectIDType
    )->SimpleGraphics{
        let vertex_shader=VertexShader::new(&include_str!("shaders/simple/vertex_shader.glsl")).unwrap();
        let fragment_shader=FragmentShader::new(&include_str!("shaders/simple/fragment_shader.glsl")).unwrap();

        let heap_vertex_buffer_size=heap_vertex_frames as ElementIndexType*frame_size as ElementIndexType;

        let heap_index_buffer_size=heap_index_frames as ElementIndexType*frame_size as ElementIndexType;

        let vertex_buffer_size=heap_vertex_buffer_size+stack_vertices;
        let index_buffer_size=heap_index_buffer_size+stack_indices as ElementIndexType;

        Self{
            vertex_buffer:VertexBuffer::empty(vertex_buffer_size as usize,BufferUsage::DynamicDraw),
            index_buffer:IndexBuffer::empty(index_buffer_size as usize,BufferUsage::DynamicDraw),
            vertex_array:VertexArray::new(),

            object_allocation:ObjectAllocation::new(
                stack_vertices,
                stack_indices,
                stack_objects,
                heap_vertex_frames,
                heap_index_frames,
                heap_objects,
            ),

            draw:Program::new(&vertex_shader,&fragment_shader).unwrap(),
        }
    }
}

/// Heap.
impl SimpleGraphics{
    pub fn add_object_raw(
        &mut self,
        vertices:&[SimpleVertex2D],
        indices:&[ElementIndexType],
        primitive_type:u32
    )->Option<ObjectIDType>{
        self.object_allocation.heap_system.add_object(
            &self.vertex_buffer,
            &self.index_buffer,
            vertices,
            indices,
            primitive_type
        )
    }

    /// Removes an object.
    /// 
    /// It's not actually removes it, just clears it's data.
    pub fn remove_object(&mut self,index:ObjectIDType){
        self.object_allocation.heap_system.remove_object(index)
    }

    pub fn get_heap_object(&self,id:ObjectIDType)->Option<&HeapObject>{
        self.object_allocation.heap_system.get_object(id)
    }

    pub fn write_heap_object_vertices(&mut self,id:ObjectIDType,vertices:&[SimpleVertex2D]){
        self.object_allocation.heap_system.write_object_vertices(&self.vertex_buffer,id,vertices)
    }

    pub fn write_heap_object_indices(&mut self,id:ObjectIDType,indices:&[ElementIndexType]){
        self.object_allocation.heap_system.write_object_indices(&self.index_buffer,id,indices)
    }

    pub fn draw_heap_object(&self,index:ObjectIDType,draw_parameters:&DrawParameters){
        if let Some(object)=self.object_allocation.heap_system.get_drawable_object(index){
            self.draw.bind();

            let _=self.draw.set_uniform_value("window_half_size",unsafe{window_center});

            let _=self.draw.set_uniform_value("draw_mode",draw_parameters.flag());

            if let Some(shift)=draw_parameters.shift(){
                let _=self.draw.set_uniform_value("vertex_shift",shift);
            }

            if let Some(rotation)=draw_parameters.rotation(){
                let _=self.draw.set_uniform_value("vertex_rotation",rotation);
            }

            self.vertex_array.bind();
            self.index_buffer.bind();
            self.vertex_buffer.bind();

            match object.draw_type{
                HeapDrawType::Vertices(first)=>unsafe{
                    let count=object.count.as_ptr();
                    let draw_count=first.len() as i32;
                    MultiDrawArrays(object.primitive_type,first.as_ptr(),count,draw_count)
                }
                HeapDrawType::Indices(indices)=>unsafe{
                    let count=object.count.as_ptr();
                    let draw_count=indices.len() as i32;
                    MultiDrawElements(object.primitive_type,count,UNSIGNED_SHORT,indices.as_ptr(),draw_count);
                }
            }

            VertexArray::<SimpleVertex2D>::unbind();
        }
    }
}

/// Stack.
impl SimpleGraphics{
    pub fn push_object_raw(
        &mut self,
        vertices:&[SimpleVertex2D],
        indices:&[ElementIndexType],
        primitive_type:u32
    )->Option<ObjectIDType>{
        self.object_allocation.stack_system.push_object_raw(
            &self.vertex_buffer,
            &self.index_buffer,
            vertices,
            indices,
            primitive_type
        )
    }

    pub fn get_stack_object(&self,id:ObjectIDType)->Option<&StackObject>{
        self.object_allocation.stack_system.get_object(id)
    }

    pub fn pop_object(&mut self){
        self.object_allocation.stack_system.pop_object()
    }

    pub fn clear_stack(&mut self){
        self.object_allocation.stack_system.clear()
    }

    pub fn write_stack_object_vertices(&mut self,id:ObjectIDType,vertices:&[SimpleVertex2D]){
        self.object_allocation.stack_system.write_object_vertices(&self.vertex_buffer,id,vertices)
    }

    pub fn write_stack_object_indices(&mut self,id:ObjectIDType,indices:&[ElementIndexType]){
        self.object_allocation.stack_system.write_object_indices(&self.index_buffer,id,indices)
    }

    pub fn draw_stack_object(&self,index:ObjectIDType,draw_parameters:&DrawParameters){
        if let Some(object)=&self.object_allocation.stack_system.get_drawable_object(index){
            self.draw.bind();

            let _=self.draw.set_uniform_value("window_half_size",unsafe{window_center});

            let _=self.draw.set_uniform_value("draw_mode",draw_parameters.flag());

            if let Some(shift)=draw_parameters.shift(){
                let _=self.draw.set_uniform_value("vertex_shift",shift);
            }

            if let Some(rotation)=draw_parameters.rotation(){
                let _=self.draw.set_uniform_value("vertex_rotation",rotation);
            }

            self.vertex_array.bind();
            self.vertex_buffer.bind();
            self.index_buffer.bind();

            match object.draw_type{
                StackDrawType::Vertices(first)=>unsafe{
                    DrawArrays(object.primitive_type,first,object.count);
                }

                StackDrawType::Indices(first)=>unsafe{
                    DrawElements(object.primitive_type,object.count,UNSIGNED_SHORT,first);
                }
            }

            VertexArray::<SimpleVertex2D>::unbind();
        }
    }
}