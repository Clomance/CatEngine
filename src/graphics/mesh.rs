use cat_engine_basement::{
    opengl::{
        core::{
            vertex_array::{
                VertexArray as VertexArrayFunctions,
            },
            drawing::{
                Drawing,
                PrimitiveType,
                AvailableIndexType,
            },
        },
        buffer::{
            IndexBuffer,
            VertexBuffer,
            BufferUsage,
        },
        vertex_array::VertexArray,
    },
    support::storage::DynamicStorage,
};

use std::{
    fmt::Debug,
    ops::{
        Add,
        SubAssign
    },
};

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum MeshError{
    NoVertices,
    NotEnoughSpaceAlloc,
    WrongBounds,
    OutOfBounds,
    VertexBufferOverflow,
    IndexBufferOverflow,
}

pub trait Vertex:Copy{
    fn attribute_pointer();
}

#[derive(Clone)]
pub struct VertexInfo{
    pub vertex_buffer_id:u32,
    pub vertex_buffer_start:i32,
    pub vertex_buffer_count:i32,

    pub index_buffer_id:u32,
    pub index_buffer_start:i32,
    pub index_buffer_count:i32,

    pub active_index:usize,
}

pub struct RenderData<'a,V:Vertex,I:AvailableIndexType>{
    mesh:&'a mut BufferedMesh<V,I>,
    object:usize,

    vertex_start:usize,
    vertex_count:usize,

    index_start:usize,
    index_count:usize
}

impl<
    'a,
    V:Vertex+Debug,
    I:AvailableIndexType+Add<Output=I>+SubAssign+Debug
> RenderData<'a,V,I>
    where <I as TryFrom<usize>>::Error:Debug
{
    pub (crate) fn new(
        mesh:&'a mut BufferedMesh<V,I>,
        object:usize,
        vertex_start:usize,
        vertex_count:usize,
        index_start:usize,
        index_count:usize
    )->RenderData<'a,V,I>{
        Self{
            mesh,
            object,
            vertex_start,
            vertex_count,
            index_start,
            index_count
        }
    }

    pub fn read_vertices(&self)->&[V]{
        self.mesh.read_vertices(self.vertex_start,self.vertex_count).unwrap()
    }

    pub fn read_indices(&self)->&[I]{
        self.mesh.read_indices(self.index_start,self.index_count).unwrap()
    }

    pub fn write_vertices(&mut self,start:usize,data:&[V])->Result<(),MeshError>{
        if start+data.len()>self.vertex_count{
            return Err(MeshError::OutOfBounds)
        }
        self.mesh.write_vertices(self.vertex_start+start,data);
        Ok(())
    }

    pub fn write_indices(&mut self,start:usize,data:&[I])->Result<(),MeshError>{
        if start+data.len()>self.index_count{
            return Err(MeshError::OutOfBounds)
        }
        self.mesh.write_indices(self.vertex_start,self.index_start+start,data);
        Ok(())
    }

    pub fn set_index_render_bounds(&mut self,start:usize,count:usize)->Result<(),MeshError>{
        if start+count>self.index_count{
            return Err(MeshError::OutOfBounds)
        }

        self.mesh.set_index_render_bounds(self.object,self.index_start+start,count);

        Ok(())
    }

    pub fn remove_index_render_bounds(&mut self){
        self.mesh.set_index_render_bounds(self.object,self.index_start,self.index_count)
    }
}

pub struct ObjectAttributes<'v,'i,V:'v,I:'i>{
    vertices:&'v [V],
    vertices_allocate_count:usize,
    indices:&'i [I],
    indices_allocate_count:usize,
    active_range:[usize;2]
}

impl<'v,'i,V:'v,I:'i> ObjectAttributes<'v,'i,V,I>{
    pub const fn new(
        vertices:&'v [V],
        vertices_allocate_count:usize,
        indices:&'i [I],
        indices_allocate_count:usize,
        active_range:[usize;2]
    )->ObjectAttributes<'v,'i,V,I>{
        Self{
            vertices,
            vertices_allocate_count,
            indices,
            indices_allocate_count,
            active_range
        }
    }

    pub fn vertices_allocate(&self)->usize{
        self.vertices_allocate_count
    }

    pub fn vertices(&self)->Option<&[V]>{
        if self.vertices.is_empty(){
            None
        }
        else{
            Some(self.vertices)
        }
    }

    pub fn indices_allocate(&self)->usize{
        self.indices_allocate_count
    }

    pub fn indices(&self)->Option<&[I]>{
        if self.indices.is_empty(){
            None
        }
        else{
            Some(self.indices)
        }
    }
}

pub struct MeshAttributes{
    pub vertex_buffer_capacity:usize,
    pub index_buffer_capacity:usize,
    pub object_storage_capacity:usize,

    pub primitive_type:PrimitiveType,
}

impl MeshAttributes{
    pub const fn new(primitive_type:PrimitiveType)->MeshAttributes{
        Self{
            vertex_buffer_capacity:1024,
            index_buffer_capacity:1024,
            object_storage_capacity:10,

            primitive_type
        }
    }
}

impl Default for MeshAttributes{
    fn default()->MeshAttributes{
        MeshAttributes::new(PrimitiveType::Triangles)
    }
}

pub (crate) struct BufferedMesh<V:Vertex,I:AvailableIndexType>{
    vertex_buffer:VertexBuffer<V>,
    vertex_buffer_local:Vec<V>,
    vertex_buffer_changed:bool,

    pub vertex_array:VertexArray,

    index_buffer:IndexBuffer<I>,
    index_buffer_local:Vec<I>,
    index_buffer_changed:bool,

    primitive_type:PrimitiveType,

    starts:Vec<isize>,
    counts:Vec<i32>,
    ids:Vec<usize>,

    objects:DynamicStorage<VertexInfo>
}

impl<V:Vertex,I:AvailableIndexType> BufferedMesh<V,I>{
    pub fn new(attributes:MeshAttributes)->BufferedMesh<V,I>{
        let vertex_array=VertexArray::new();
        vertex_array.bind();

        let vertex_buffer=VertexBuffer::empty(attributes.vertex_buffer_capacity,BufferUsage::DynamicDraw);
        let index_buffer=IndexBuffer::empty(attributes.index_buffer_capacity,BufferUsage::DynamicDraw);

        V::attribute_pointer();

        unsafe{
            VertexArrayFunctions::bind(0);
        }

        Self{
            vertex_buffer,
            vertex_buffer_local:Vec::with_capacity(attributes.vertex_buffer_capacity),
            vertex_buffer_changed:false,

            vertex_array,

            index_buffer,
            index_buffer_local:Vec::with_capacity(attributes.index_buffer_capacity),
            index_buffer_changed:false,

            primitive_type:attributes.primitive_type,

            starts:Vec::new(),
            counts:Vec::new(),
            ids:Vec::new(),

            objects:DynamicStorage::with_capacity(attributes.object_storage_capacity),
        }
    }

    pub fn read_vertices(&self,start:usize,count:usize)->Option<&[V]>{
        self.vertex_buffer_local.get(start..start+count)
    }

    pub fn read_indices(&self,start:usize,count:usize)->Option<&[I]>{
        self.index_buffer_local.get(start..start+count)
    }

    pub fn flush_vertices(&mut self){
        if self.vertex_buffer_changed{
            self.vertex_buffer.write(0,&self.vertex_buffer_local[..]).unwrap();
            self.vertex_buffer_changed=false;
        }
    }

    pub fn flush_indices(&mut self){
        if self.index_buffer_changed{
            self.index_buffer.write(0,&self.index_buffer_local[..]).unwrap();
            self.index_buffer_changed=false;
        }
    }

    pub fn write_vertices(&mut self,start:usize,data:&[V]){
        let end=start+data.len();
        for vertex in self.vertex_buffer_local[start..end].iter_mut().zip(data.iter()){
            *vertex.0=*vertex.1
        }
        self.vertex_buffer_changed=true;
    }

    pub fn remove_vertices(&mut self,start:usize,count:usize){
        for _ in 0..count{
            self.vertex_buffer_local.remove(start);
        }
        self.vertex_buffer_changed=true;
    }

    pub fn remove_indices(&mut self,start:usize,count:usize){
        for _ in 0..count{
            self.index_buffer_local.remove(start);
        }
        self.index_buffer_changed=true;
    }

    pub fn set_index_render_bounds(&mut self,object:usize,start:usize,count:usize){
        let info=&self.objects.get(object).unwrap();

        self.starts[info.active_index as usize]=(start*std::mem::size_of::<I>()) as isize;
        self.counts[info.active_index as usize]=count as i32;
    }

    pub fn is_empty(&self)->bool{
        self.ids.is_empty()
    }

    pub fn get_render_data(&self,object:usize)->Option<&VertexInfo>{
        self.objects.get(object)
    }

    pub fn draw(&self){
        let size=self.starts.len() as i32;
        unsafe{
            Drawing::multi_draw_elements_typed::<I>(self.starts.as_ptr(),self.counts.as_ptr(),size,self.primitive_type)
        }
    }
}
    
impl<
    V:Vertex+Debug,
    I:AvailableIndexType+Add<Output=I>+SubAssign+Debug
> BufferedMesh<V,I>
    where <I as TryFrom<usize>>::Error:Debug
{
    /// Добавляет вершины и индексы объекта в конец буфера и включает его в список рендиринга.
    pub fn push_object(&mut self,object:ObjectAttributes<V,I>)->Result<usize,MeshError>{
        // Определение границы массива вершин
        let vertices_start=self.vertex_buffer_local.len();
        let mut vertex_count=0;

        let vertex_allocate=object.vertices_allocate();
        let vertices_end=vertices_start+vertex_allocate;

        // Проверка границ
        if vertices_end>self.vertex_buffer_local.capacity(){
            return Err(MeshError::VertexBufferOverflow)
        }

        // Запись в локольный буфер
        unsafe{self.vertex_buffer_local.set_len(vertices_end)}

        if let Some(vertices)=object.vertices(){
            if vertex_allocate<vertices.len(){
                unsafe{self.vertex_buffer_local.set_len(vertices_start)}
                return Err(MeshError::NotEnoughSpaceAlloc)
            }
            vertex_count=vertices.len();
            self.write_vertices(vertices_start,vertices);
        }

        if vertex_allocate==0 && vertex_count==0{
            unsafe{self.vertex_buffer_local.set_len(vertices_start)}
            return Err(MeshError::NoVertices)
        }


        let indices_start=self.index_buffer_local.len();

        let index_count;
        let mut index_allocate=object.indices_allocate();

        // Запись индексов, если они представлены
        if let Some(indices)=object.indices(){
            index_count=indices.len();

            let indices_end=indices_start+index_allocate;

            if indices_end>self.index_buffer_local.capacity(){
                unsafe{self.vertex_buffer_local.set_len(vertices_start)}
                return Err(MeshError::IndexBufferOverflow)
            }

            unsafe{self.index_buffer_local.set_len(indices_end)}

            self.write_indices(vertices_start,indices_start,indices);
        }
        // Создание индексов, если они не представлены
        else{
            if index_allocate==0{
                if vertex_count==0{
                    index_allocate=vertex_allocate;
                }
                else{
                    index_allocate=vertex_count;
                }

                index_count=index_allocate;
            }
            else{
                index_count=vertex_count
            }

            let indices_end=indices_start+index_allocate;

            if indices_end>self.index_buffer_local.capacity(){
                unsafe{self.vertex_buffer_local.set_len(vertices_start)}
                return Err(MeshError::IndexBufferOverflow)
            }

            unsafe{self.index_buffer_local.set_len(indices_end)}
            // определяем индексы и вписываем в локальный буфер
            for index in self.index_buffer_local[indices_start..indices_start+index_count].iter_mut().enumerate(){
                *index.1=I::try_from(vertices_start+index.0).unwrap()
            }
        }



        if object.active_range[0]>index_count || object.active_range[0]+object.active_range[1]>index_count{
            unsafe{self.vertex_buffer_local.set_len(vertices_start)}
            unsafe{self.index_buffer_local.set_len(indices_start)}

            return Err(MeshError::WrongBounds)
        }


        // Установка флагов изменения в буферах вершин и индексов
        self.vertex_buffer_changed=true;
        self.index_buffer_changed=true;

        // Добавление в список рендеринга
        let id=self.ids.len();

        let indices_start=indices_start+object.active_range[0];

        self.starts.push((indices_start*std::mem::size_of::<I>()) as isize);
        self.counts.push(index_count as i32);

        // Сохранение данных о вершинах, индексах и положении в спике рендеринга
        let info=VertexInfo{
            vertex_buffer_id:self.vertex_buffer.id(),
            vertex_buffer_start:vertices_start as i32,
            vertex_buffer_count:vertex_allocate as i32,

            index_buffer_id:self.index_buffer.id(),
            index_buffer_start:indices_start as i32,
            index_buffer_count:index_allocate as i32,

            active_index:id,
        };

        let id=self.objects.add(info);
        self.ids.push(id);

        Ok(id)
    }

    pub fn remove_object(&mut self,index:usize){
        let info=self.objects.remove(index).unwrap();

        let vertex_count=info.vertex_buffer_count;
        let index_count=info.index_buffer_count;

        self.starts.remove(info.active_index);
        self.counts.remove(info.active_index);
        self.ids.remove(info.active_index);

        self.remove_vertices(info.vertex_buffer_start as usize,vertex_count as usize);

        self.remove_indices(info.index_buffer_start as usize,index_count as usize);
        for index in &mut self.index_buffer_local[info.index_buffer_start as usize..]{
            *index-=I::try_from(vertex_count as usize).unwrap()
        }

        let mut c=info.active_index;

        while let Some(&id)=self.ids.get(c){
            self.starts[c]-=index_count as isize*std::mem::size_of::<I>() as isize;

            let object=self.objects.get_mut(id).unwrap();

            object.active_index-=1;

            object.index_buffer_start-=index_count;

            object.vertex_buffer_start-=vertex_count;

            c+=1;
        }
    }

    pub fn write_indices(&mut self,vertex_offset:usize,start:usize,data:&[I]){
        let end=start+data.len();
        for index in self.index_buffer_local[start..end].iter_mut().zip(data.iter()){
            *index.0=*index.1+I::try_from(vertex_offset).unwrap()
        }
        self.index_buffer_changed=true;
    }
}