use crate::{
    graphics::{
        // types
        FrameIDType,
        ObjectIDType,
        ElementIndexType,
        // consts
        frame_size,
        minimal_frames,
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

use std::{
    mem::size_of,
    ffi::c_void,
    marker::PhantomData,
};

pub struct HeapObject{
    /// Номера блоков с вершинами
    pub vertex_frames:Vec<FrameIDType>,
    /// Размер последнего блока (если ноль, то он полный)
    pub last_vertex_frame_size:u8,
    /// Номера блоков с индексами
    pub index_frames:Vec<FrameIDType>,
    /// Размер последнего блока (если ноль, то он полный)
    pub last_index_frame_size:u8,
    pub primitive_type:PrimitiveType,
}

impl HeapObject{
    pub fn drawable(&self)->HeapDrawableObject{
        let (draw_type,count)=if self.index_frames.len()==0{
            let mut start=Vec::with_capacity(self.vertex_frames.len());
            let mut count=Vec::with_capacity(self.vertex_frames.len());

            for &frame_id in &self.vertex_frames{
                let start_index=frame_id*frame_size as FrameIDType;
                start.push(start_index as i32);
                count.push(frame_size as i32);
            }

            // Если последний блок вершин неполный
            if self.last_vertex_frame_size!=0{
                // Установка размера последнего блока вершин
                if let Some(last)=count.last_mut(){
                    *last=self.last_vertex_frame_size as i32;
                }
            }

            (
                HeapDrawType::Vertices(start),
                count
            )
        }
        else{
            let mut start=Vec::with_capacity(self.index_frames.len());
            let mut count=Vec::with_capacity(self.index_frames.len());

            for &frame_id in &self.index_frames{
                let start_index=frame_id*frame_size as FrameIDType;
                start.push(start_index as isize*size_of::<ElementIndexType>() as isize);
                count.push(frame_size as i32);
            }

            // Если последний блок индексов неполный
            if self.last_index_frame_size!=0{
                // Установка размера последнего блока индексов
                if let Some(last)=count.last_mut(){
                    *last=self.last_index_frame_size as i32;
                }
            }

            (
                HeapDrawType::Indices(start),
                count
            )
        };

        HeapDrawableObject{
            draw_type,
            count,
            primitive_type:self.primitive_type
        }
    }
}

#[derive(Debug)]
pub enum HeapDrawType{
    Vertices(Vec<i32>), // count
    Indices(Vec<isize>), // start
}

pub struct HeapDrawableObject{
    pub draw_type:HeapDrawType,
    pub count:Vec<i32>,
    pub primitive_type:PrimitiveType,
}

pub struct HeapSystem<V:Vertex>{
    free_vertex_frames:Vec<FrameIDType>,
    free_index_frames:Vec<FrameIDType>,

    objects:Vec<HeapObject>,
    free_objects:Vec<ObjectIDType>,

    marker:PhantomData<V>,
}

impl<V:Vertex> HeapSystem<V>{
    /// Один блок равен трём элементам.
    pub fn new(
        vertex_frames:FrameIDType,
        index_frames:FrameIDType,
        objects:ObjectIDType
    )->HeapSystem<V>{
        // Свободные блоки вершин
        let mut free_vertex_frames=Vec::with_capacity(vertex_frames as usize);
        for c in 0..vertex_frames{
            free_vertex_frames.push(c)
        }
        // Свободные блоки индексов
        let mut free_index_frames=Vec::with_capacity(index_frames as usize);
        for c in 0..index_frames{
            free_index_frames.push(c)
        }
        // Объекты
        let mut object_array=Vec::with_capacity(objects as usize);
        for _ in 0..objects{
            let object=HeapObject{
                vertex_frames:Vec::with_capacity(minimal_frames),
                last_vertex_frame_size:0u8,
                index_frames:Vec::with_capacity(minimal_frames),
                last_index_frame_size:0u8,
                primitive_type:PrimitiveType::Points,
            };
            object_array.push(object)
        }
        // Свободные объекты
        let mut free_objects=Vec::with_capacity(objects as usize);
        for c in 0..objects{
            free_objects.push(c)
        }

        Self{
            free_vertex_frames,
            free_index_frames,

            objects:object_array,
            free_objects,

            marker:PhantomData,
        }
    }
}

impl<V:Vertex> HeapSystem<V>{
    pub fn add_object(
        &mut self,
        vertex_buffer:&VertexBuffer<V>,
        index_buffer:&IndexBuffer<ElementIndexType>,
        vertices:&[V],
        indices:&[ElementIndexType],
        primitive_type:PrimitiveType
    )->Option<ObjectIDType>{
        if let Some(object_id)=self.free_objects.pop(){
            // Количество блоков для вершин и индексов
            let vertex_frames=(vertices.len()+frame_size-1)/frame_size;
            let index_frames=(indices.len()+frame_size-1)/frame_size;
            // Проверка: есть ли нужное количество блоков для вершин и индексов
            if vertex_frames>self.free_vertex_frames.len() || index_frames>self.free_index_frames.len(){
                self.free_objects.push(object_id);
                None
            }
            else{
                // Получение ссылки на объект
                let object=unsafe{self.objects.get_unchecked_mut(object_id as usize)};
                // Установка размера последних блоков
                object.last_vertex_frame_size=(vertices.len()%frame_size) as u8;
                object.last_index_frame_size=(indices.len()%frame_size) as u8;
                // Установка типа отрисовки
                object.primitive_type=primitive_type;

                // Проверка наличия места для блоков вершин
                if object.vertex_frames.capacity()<vertex_frames{
                    // Увеличение максимального количества блоков вершин
                    object.vertex_frames.reserve(vertex_frames-object.vertex_frames.capacity());
                }
                // Проверка наличия места для блоков индексов
                if object.index_frames.capacity()<index_frames{
                    // Увеличение максимального количества блоков индексов
                    object.index_frames.reserve(index_frames-object.index_frames.capacity());
                }

                // Загрузка вершин
                {
                    for c in 0..vertex_frames-1{
                        // Добавление блока в объект
                        let vertex_frame_id=self.free_vertex_frames.pop().unwrap();
                        object.vertex_frames.push(vertex_frame_id);
                        // Запись в буфер вершин
                        let offset=vertex_frame_id as usize*frame_size;
                        vertex_buffer.write(offset as isize,&vertices[c*frame_size..(c+1)*frame_size])
                    }
                    { // Последний блок вершин
                        let vertex_frame_id=self.free_vertex_frames.pop().unwrap();
                        object.vertex_frames.push(vertex_frame_id);
                        // Запись в буфер вершин
                        let offset=vertex_frame_id as usize*frame_size;
                        let c=vertex_frames-1;
                        vertex_buffer.write(offset as isize,&vertices[c*frame_size..])
                    }
                }

                // Загрузка индексов
                if index_frames!=0{
                    // Временный блок для новых индексов
                    let mut new_indices:[ElementIndexType;frame_size]=[0;frame_size];
                    
                    for c in 0..index_frames-1{
                        // Добавление блока в объект
                        let index_frame_id=self.free_index_frames.pop().unwrap();
                        object.index_frames.push(index_frame_id);

                        // Распределение индексов для текущего положения вершин
                        let frame_slice=&indices[c*frame_size..(c+1)*frame_size];
                        for d in 0..frame_size{
                            // Проверка выхода за пределы массива вершин
                            if frame_slice[d]>=vertices.len() as ElementIndexType{
                                panic!("No such vertex");
                            }
                            // Номер блока вершин в объекте
                            let object_vertex_frame_id=frame_slice[d] as usize/frame_size;
                            // Номер блока вершин в буфере
                            if let Some(vertex_frame_id)=object.vertex_frames.get(object_vertex_frame_id){
                                // Новый индекс
                                new_indices[d]=vertex_frame_id*frame_size as ElementIndexType+frame_slice[d]%frame_size as u16;
                            }
                            else{
                                panic!("No such vertex");
                            }
                        }
                        // Запись в буфер вершин
                        let offset=index_frame_id as usize*frame_size;
                        index_buffer.write(offset as isize,&new_indices)
                    }
                    // Последний блок индексов
                    let c=index_frames-1;
                    // Добавление блока в объект
                    let index_frame_id=self.free_index_frames.pop().unwrap();
                    object.index_frames.push(index_frame_id);
                    // Распределение индексов для текущего положения вершин
                    let frame_slice=&indices[c*frame_size..];
                    for d in 0..frame_slice.len(){
                        // Проверка выхода за пределы массива вершин
                        if frame_slice[d]>=vertices.len() as ElementIndexType{
                            panic!("No such vertex");
                        }
                        // Номер блока вершин в объекте
                        let object_vertex_frame_id=frame_slice[d] as usize/frame_size;
                        // Номер блока вершин в буфере
                        if let Some(vertex_frame_id)=object.vertex_frames.get(object_vertex_frame_id){
                            // Новый индекс
                            new_indices[d]=vertex_frame_id*frame_size as ElementIndexType+frame_slice[d]%frame_size as u16;
                        }
                        else{
                            panic!("No such vertex");
                        }
                    }
                    // Запись в буфер вершин
                    let offset=index_frame_id as usize*frame_size;
                    index_buffer.write(offset as isize,&new_indices)
                }

                Some(object_id)
            }
        }
        else{
            None
        }
    }

    /// Removes an object.
    /// 
    /// It's not actually removes it, just clears it's data.
    pub fn remove_object(&mut self,id:ObjectIDType){
        if let Some(object)=self.objects.get_mut(id as usize){
            if object.vertex_frames.len()!=0{
                self.free_objects.push(id);
                // Освобождение блоков вершин и добавление их в очередь
                while let Some(vertex_frame_id)=object.vertex_frames.pop(){
                    self.free_vertex_frames.push(vertex_frame_id);
                }
                // Освобождение блоков индексов и добавление их в очередь
                while let Some(index_frame_id)=object.index_frames.pop(){
                    self.free_index_frames.push(index_frame_id);
                }
            }
        }
    }

    pub fn get_object(&self,id:ObjectIDType)->Option<&HeapObject>{
        self.objects.get(id as usize)
    }

    pub fn get_drawable_object(&self,id:ObjectIDType)->Option<HeapDrawableObject>{
        if let Some(object)=self.objects.get(id as usize){
            Some(object.drawable())
        }
        else{
            None
        }
    }
}

impl<V:Vertex> HeapSystem<V>{
    pub fn write_object_vertices(
        &mut self,
        vertex_buffer:&VertexBuffer<V>,
        id:ObjectIDType,
        vertices:&[V]
    ){
        if let Some(object)=self.get_object(id){
            // Количество вершин объекта
            let object_vertices_len=if object.last_vertex_frame_size==0{
                object.vertex_frames.len()*frame_size
            }
            else{
                object.last_vertex_frame_size as usize+(object.vertex_frames.len()-1)*frame_size
            };

            // Проверка количества вписываемых вершин
            if vertices.len()!=object_vertices_len{
                return
            }

            for c in 0..object.vertex_frames.len()-1{
                let offset=object.vertex_frames[c] as usize*frame_size;
                let vertices_slice=&vertices[c*frame_size..(c+1)*frame_size];

                vertex_buffer.write(offset as isize,vertices_slice)
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
            // Количество индексов объекта
            let object_indices_len=if object.last_index_frame_size==0{
                object.index_frames.len()*frame_size
            }
            else{
                object.last_index_frame_size as usize+(object.index_frames.len()-1)*frame_size
            };

            // Проверка количества вписываемых индексов
            if indices.len()!=object_indices_len{
                return
            }

            for c in 0..object.index_frames.len()-1{
                let offset=object.index_frames[c] as usize*frame_size;
                let indices_slice=&indices[c*frame_size..(c+1)*frame_size];

                index_buffer.write(offset as isize,indices_slice)
            }
        }
    }
}

/// Removed some unnessesary checks.
/// feature="unsafe_release_heap_memmory_system"
#[cfg(feature="unsafe_release_heap_memmory_system")]
impl HeapSystem{
    /// Removed index checks.
    pub fn get_object(&self,id:ObjectIDType)->&HeapObject{
        self.objects.get_unchecked(id as usize)
    }

    /// Removed index checks.
    pub fn get_drawable_object(&self,id:ObjectIDType)->HeapDrawableObject{
        self.objects.get_unchecked(id as usize).drawable()
    }
}