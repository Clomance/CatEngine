use crate::basement::graphics::level0::Vertex;

use super::{
    // types
    ElementIndexType,
    ObjectIDType,
    FrameIDType,
    // consts
    frame_size,
};

mod heap_mem_system;
pub use heap_mem_system::{
    HeapSystem,
    HeapObject,
    HeapDrawType,
    HeapDrawableObject,
};

mod stack_mem_system;
pub use stack_mem_system::{
    StackSystem,
    StackObject,
    StackDrawType,
    StackDrawableObject,
};

pub struct ObjectAllocation<V:Vertex>{
    pub heap_system:HeapSystem<V>,
    pub stack_system:StackSystem<V>,
}

impl<V:Vertex> ObjectAllocation<V>{
    pub fn new(
        stack_vertices:ElementIndexType,
        stack_indices:i32,
        stack_objects:ObjectIDType,
        heap_vertex_frames:FrameIDType,
        heap_index_frames:FrameIDType,
        heap_objects:ObjectIDType
    )->ObjectAllocation<V>{
        // Проверка размера области буфера для "кучного" распределения: не больше, чем максимальное значения индекса
        let heap_vertex_buffer_size=if let Some(heap_vertex_buffer_size)=(heap_vertex_frames as ElementIndexType).checked_mul(frame_size as ElementIndexType){
            heap_vertex_buffer_size
        }
        else{
            panic!(
                "VertexBufferSizeOverflow. The maximal size is {}. Lessen the amount of frames (current is {}) to {}.",
                ElementIndexType::MAX,heap_vertex_frames,ElementIndexType::MAX/frame_size as ElementIndexType
            )
        };

        // Проверка размера области буфера для "кучного" распределения: не больше, чем максимальное значение указателя на индекс (пока взял i32)
        let heap_index_buffer_size=if let Some(heap_index_buffer_size)=(heap_index_frames as i32).checked_mul(frame_size as i32){
            heap_index_buffer_size
        }
        else{
            panic!(
                "IndexBufferSizeOverflow. The maximal size is {}. Lessen the amount of frames (current is {}) to {}.",
                i32::MAX,heap_index_frames,i32::MAX/frame_size as i32
            )
        };

        // Проверка размера всего буфера: не больше, чем максимальное значения индекса
        let vertex_buffer_size=if let Some(vertex_buffer_size)=heap_vertex_buffer_size.checked_add(stack_vertices){
            vertex_buffer_size as i32
        }
        else{
            panic!(
                "VertexBufferSizeOverflow. The maximal size is {}. Lessen the amount of 'stack' vertices (current is {}) to {} or the amount of frames.",
                ElementIndexType::MAX,stack_vertices,ElementIndexType::MAX-heap_vertex_buffer_size
            )
        };

        // Проверка размера всего буфера: не больше, чем максимальное значение указателя на индекс (пока взял i32)
        let index_buffer_size=if let Some(index_buffer_size)=(heap_index_buffer_size as i32).checked_add(stack_indices){
            index_buffer_size
        }
        else{
            panic!(
                "IndexBufferSizeOverflow. The maximal size is {}. Lessen the amount of 'stack' indices (current is {}) to {}.",
                i32::MAX,stack_indices,i32::MAX-heap_index_buffer_size
            )
        };

        let heap_system=HeapSystem::new(heap_vertex_frames,heap_index_frames,heap_objects);
        let stack_system=StackSystem::new(
            heap_vertex_buffer_size as i32,
            vertex_buffer_size as i32,
            heap_index_buffer_size,
            index_buffer_size,
            stack_objects
        );

        Self{
            heap_system,
            stack_system,
        }
    }
}