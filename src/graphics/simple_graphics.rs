use super::{
    // types
    FrameIDType,
    ObjectIDType,
    ElementIndexType,
    // consts
    frame_size,
    // structs
    SimpleVertex2D,
    HeapObject,
    StackObject,
    ObjectAllocation,
    DrawParameters,
    // enums
    HeapDrawType,
    StackDrawType,
};

use cat_engine_basement::graphics::{
    GLCore,
    core::{
        drawing::PrimitiveType,
        buffer::BufferUsage,
    },
    level0::VertexArray,
    level1::{
        VertexBuffer,
        IndexBuffer,
        VertexShader,
        FragmentShader
    },
    level2::Program,
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

        let program=Program::new(&vertex_shader,&fragment_shader).unwrap();

        let heap_vertex_buffer_size=heap_vertex_frames as ElementIndexType*frame_size as ElementIndexType;

        let heap_index_buffer_size=heap_index_frames as ElementIndexType*frame_size as ElementIndexType;

        let vertex_buffer_size=heap_vertex_buffer_size+stack_vertices;
        let index_buffer_size=heap_index_buffer_size+stack_indices as ElementIndexType;

        let vertex_buffer=VertexBuffer::empty(vertex_buffer_size as isize,BufferUsage::DynamicDraw).unwrap();
        let index_buffer=IndexBuffer::empty(index_buffer_size as isize,BufferUsage::DynamicDraw).unwrap();
        let vertex_array=VertexArray::new(vertex_buffer.as_raw());

        Self{
            vertex_buffer,
            index_buffer,
            vertex_array,

            object_allocation:ObjectAllocation::new(
                stack_vertices,
                stack_indices,
                stack_objects,
                heap_vertex_frames,
                heap_index_frames,
                heap_objects,
            ),

            draw:program,
        }
    }
}

/// Heap.
impl SimpleGraphics{
    pub fn add_object_raw(
        &mut self,
        vertices:&[SimpleVertex2D],
        indices:&[ElementIndexType],
        primitive_type:PrimitiveType
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

            self.vertex_array.bind();
            self.index_buffer.bind();
            self.vertex_buffer.bind();

            let _=self.draw.set_uniform_value("viewport",draw_parameters.viewport());

            let _=self.draw.set_uniform_value("draw_mode",draw_parameters.flag());

            if let Some(shift)=draw_parameters.shift(){
                let _=self.draw.set_uniform_value("vertex_shift",shift);
            }

            if let Some(rotation)=draw_parameters.rotation(){
                let _=self.draw.set_uniform_value("vertex_rotation",rotation);
            }

            match object.draw_type{
                HeapDrawType::Vertices(first)=>unsafe{
                    GLCore.drawing.multi_draw_arrays(&first,&object.count,object.primitive_type)
                }

                HeapDrawType::Indices(indices)=>unsafe{
                    GLCore.drawing.multi_draw_elements_typed::<ElementIndexType>(
                        &indices,
                        &object.count,
                        object.primitive_type
                    )
                }
            }

            self.vertex_array.unbind();
        }
    }
}

/// Stack.
impl SimpleGraphics{
    pub fn push_object_raw(
        &mut self,
        vertices:&[SimpleVertex2D],
        indices:&[ElementIndexType],
        primitive_type:PrimitiveType
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

            self.vertex_array.bind();
            self.vertex_buffer.bind();
            self.index_buffer.bind();

            let _=self.draw.set_uniform_value("viewport",draw_parameters.viewport());

            let _=self.draw.set_uniform_value("draw_mode",draw_parameters.flag());

            if let Some(shift)=draw_parameters.shift(){
                let _=self.draw.set_uniform_value("vertex_shift",shift);
            }

            if let Some(rotation)=draw_parameters.rotation(){
                let _=self.draw.set_uniform_value("vertex_rotation",rotation);
            }


            match object.draw_type{
                StackDrawType::Vertices(first)=>unsafe{
                    GLCore.drawing.draw_arrays(first,object.count,object.primitive_type)
                }

                StackDrawType::Indices(first)=>unsafe{
                    GLCore.drawing.draw_elements_typed::<ElementIndexType>(
                        first,
                        object.count,
                        object.primitive_type
                    )
                }
            }

            self.vertex_array.unbind();
        }
    }
}