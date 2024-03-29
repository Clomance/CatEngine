use crate::{
    graphics::{
        // types
        ObjectIDType,
        ElementIndexType,
    },
};

use cat_engine_basement::graphics::{
    core::drawing::PrimitiveType,
    level0::Vertex,
    level1::{
        VertexBuffer,
        IndexBuffer,
    },
};

use std::marker::PhantomData;

pub struct StackObject{
    pub vertex_start:i32,
    pub vertex_count:i32,
    pub index_start:i32,
    pub index_count:i32,
    pub primitive_type:PrimitiveType,
}

impl StackObject{
    pub fn drawable(&self)->StackDrawableObject{
        let (draw_type,count)=if self.index_count==0{
            (
                StackDrawType::Vertices(self.vertex_start),
                self.vertex_count
            )
        }
        else{
            (
                StackDrawType::Indices(self.index_start),
                self.index_count
            )
        };

        StackDrawableObject{
            draw_type,
            count,
            primitive_type:self.primitive_type
        }
    }
}

#[derive(Debug)]
pub enum StackDrawType{
    Vertices(i32), // count
    Indices(i32), // start
}

pub struct StackDrawableObject{
    pub draw_type:StackDrawType,
    pub count:i32,
    pub primitive_type:PrimitiveType,
}

pub struct StackSystem<V:Vertex>{
    vertex_buffer_start:i32,
    vertex_buffer_ptr:i32,
    vertex_buffer_size:i32,

    index_buffer_start:i32,
    index_buffer_ptr:i32,
    index_buffer_size:i32,

    objects:Vec<StackObject>,
    marker:PhantomData<V>,
}

impl<V:Vertex> StackSystem<V>{
    pub fn new(
        vertex_buffer_start:i32,
        vertex_buffer_size:i32,
        index_buffer_start:i32,
        index_buffer_size:i32,
        objects:ObjectIDType,
    )->StackSystem<V>{
        Self{
            vertex_buffer_start,
            vertex_buffer_ptr:vertex_buffer_start,
            vertex_buffer_size,

            index_buffer_start,
            index_buffer_ptr:index_buffer_start,
            index_buffer_size,

            objects:Vec::with_capacity(objects as usize),
            marker:PhantomData,
        }
    }
}

impl<V:Vertex> StackSystem<V>{
    pub fn push_object_raw(
        &mut self,
        vertex_buffer:&VertexBuffer<V>,
        index_buffer:&IndexBuffer<ElementIndexType>,
        vertices:&[V],
        indices:&[ElementIndexType],
        primitive_type:PrimitiveType
    )->Option<ObjectIDType>{
        if vertices.len()==0{
            return None
        }
        // Количество вершин
        let vertex_count=vertices.len() as i32;
        // Количество индексов
        let index_count=indices.len() as i32;

        if self.vertex_buffer_ptr+vertex_count>self.vertex_buffer_size ||
                self.index_buffer_ptr+index_count>self.index_buffer_size{
            return None
        }

        // Индекс объекта
        let object_index=self.objects.len() as ObjectIDType;

        let object=StackObject{
            vertex_start:self.vertex_buffer_ptr,
            vertex_count,
            index_start:self.index_buffer_ptr,
            index_count,
            primitive_type
        };

        if index_count!=0{
            // Сдвиг всех индексов
            let offset_indices:Vec<ElementIndexType>=indices.iter().map(|&i|
                if i<vertex_count as ElementIndexType{
                    self.vertex_buffer_ptr as ElementIndexType+i
                }
                else{
                    panic!("IndexOutOfBounds. The index is {}. The amount of vertices is {}.",i,vertex_count)
                }
            ).collect();
            // Запись индексов
            index_buffer.write(self.index_buffer_ptr as isize,&offset_indices);
            self.index_buffer_ptr+=index_count;
        }

        // Запись вершин
        vertex_buffer.write(self.vertex_buffer_ptr as isize,vertices);
        self.vertex_buffer_ptr+=vertex_count;

        self.objects.push(object);

        Some(object_index)
    }

    pub fn pop_object(&mut self){
        if let Some(object)=self.objects.pop(){
            self.vertex_buffer_ptr-=object.vertex_count;
            self.index_buffer_ptr-=object.index_count;
        }
    }

    pub fn clear(&mut self){
        self.objects.clear();
        self.vertex_buffer_ptr=self.vertex_buffer_start;
        self.index_buffer_ptr=self.index_buffer_start;
    }
}

impl<V:Vertex> StackSystem<V>{
    pub fn write_object_vertices(
        &mut self,
        vertex_buffer:&VertexBuffer<V>,
        id:ObjectIDType,
        vertices:&[V]
    ){
        if let Some(object)=self.get_object(id){
            if object.vertex_count as usize==vertices.len(){
                vertex_buffer.write(object.vertex_start as isize,vertices).unwrap();
            }
        }
    }

    pub fn write_object_indices(
        &mut self,
        index_buffer:&IndexBuffer<ElementIndexType>,
        id:ObjectIDType,
        indices:&[ElementIndexType]
    ){
        if let Some(object)=self.get_object(id){
            if object.index_count as usize==indices.len(){
                index_buffer.write(object.vertex_start as isize,indices).unwrap();
            }
        }
    }
}

/// Default get functions.
#[cfg(not(feature="unsafe_release_stack_memmory_system"))]
impl <V:Vertex> StackSystem<V>{
    #[inline(always)]
    pub fn get_object(&self,id:ObjectIDType)->Option<&StackObject>{
        self.objects.get(id as usize)
    }

    pub fn get_drawable_object(&self,id:ObjectIDType)->Option<StackDrawableObject>{
        if let Some(object)=self.objects.get(id as usize){
            Some(object.drawable())
        }
        else{
            None
        }
    }
}

/// Removed some unnessesary checks.
#[cfg(feature="unsafe_release_stack_memmory_system")]
impl <V:Vertex> StackSystem<V>{
    #[inline(always)]
    pub fn get_object(&self,id:ObjectIDType)->&StackObject{
        self.objects.get_unchecked(id as usize)
    }

    #[inline(always)]
    pub fn get_mut_object(&self,id:ObjectIDType)->&mut StackObject{
        self.objects.get_mut_unchecked(id as usize)
    }

    pub fn get_drawable_object(&self,id:ObjectIDType)->StackDrawableObject{
        self.objects.get_unchecked(id as usize).drawable()
    }
}