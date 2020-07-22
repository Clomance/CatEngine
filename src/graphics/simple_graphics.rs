use crate::{Colour,window_center};

use glium::{
    uniform,
    implement_vertex,
    Program,
    Display,
    Frame,
    DrawParameters,
    DrawError,
    index::{
        PrimitiveType,
        IndexBuffer,
        IndicesSource,
        IndexType,
    },
    Surface,
    vertex::{
        VerticesSource,
        Vertex,
        VertexFormat,
    },
    buffer::{
        Buffer,
        BufferType,
        BufferMode,
        BufferSlice,
        Content,
        BufferAnySlice,
    },
};

use core::ops::Range;

implement_vertex!(Vertex2D,position);
/// A vertex for simple objects.
#[derive(Copy,Clone)]
pub struct Vertex2D{
    pub position:[f32;2],
}

impl Vertex2D{
    pub const fn new(x:f32,y:f32)->Vertex2D{
        Self{
            position:[x,y]
        }
    }

    /// Переводит в формат OpenGL.
    /// 
    /// Converts to OpenGL format.
    pub fn convert(&mut self){
        unsafe{
            self.position[0]=self.position[0]/window_center[0]-1f32;
            self.position[1]=1f32-self.position[1]/window_center[1];
        }
    }

    pub fn center_radius(&mut self){
        unsafe{
            self.position[0]=self.position[0]-window_center[0];
            self.position[1]=window_center[1]-self.position[1];
        }
    }
}

pub struct PlainObject{
    vertex_buffer_range:Range<usize>,
    colour:Colour,

    index_buffer_range:Range<usize>,
    primitive_type:PrimitiveType,
}

impl PlainObject{
    pub fn vertex_source(&self){

    }
}

#[derive(Clone,Debug)]
pub struct SimpleGraphicsSettings{
    /// The default is 8.
    pub vertex_buffer_size:usize,

    /// The range from 0 to `offset` is for common drawing.
    /// The range after `offset` is for saving objects.
    /// 
    /// The default is 4.
    pub vertex_buffer_offset:usize,

    /// The default is 8.
    pub index_buffer_size:usize,

    /// The range from 0 to `offset` is for common drawing.
    /// The range after `offset` is for saving objects.
    /// 
    /// The default is 4.
    pub index_buffer_offset:usize,

    /// The default is 2.
    pub object_buffer_size:usize,
}

impl SimpleGraphicsSettings{
    pub const fn new()->SimpleGraphicsSettings{
        SimpleGraphicsSettings{
            vertex_buffer_size:8,
            vertex_buffer_offset:4,
            index_buffer_size:8,
            index_buffer_offset:4,
            object_buffer_size:2,
        }
    }
}

/// Графическая основа для простых одноцветных объектов.
/// Размер буферов регулируется вручную при создании.
pub struct SimpleGraphics{
    vertex_buffer:Buffer<[Vertex2D]>,
    bindings:VertexFormat,
    vertex_buffer_edge:usize, // Сколько уже занято

    index_buffer:Buffer<[u8]>,
    index_buffer_edge:usize, // Сколько уже занято

    plain_objects:Vec<PlainObject>,

    draw:Program,
    draw_shift:Program,
    draw_rotate:Program,
}

impl SimpleGraphics{
    pub fn new(display:&Display,settings:SimpleGraphicsSettings,glsl:u16)->SimpleGraphics{
        let (
            rotation,
            shift,
            vertex_shader,
            fragment_shader
        )=if glsl==120{(
            include_str!("shaders/120/simple/rotation_vertex_shader.glsl"),
            include_str!("shaders/120/simple/shift_vertex_shader.glsl"),
            include_str!("shaders/120/simple/vertex_shader.glsl"),
            include_str!("shaders/120/simple/fragment_shader.glsl"),
        )}
        else{(
            include_str!("shaders/simple/rotation_vertex_shader.glsl"),
            include_str!("shaders/simple/shift_vertex_shader.glsl"),
            include_str!("shaders/simple/vertex_shader.glsl"),
            include_str!("shaders/simple/fragment_shader.glsl"),
        )};

        let vertex_buffer_size=settings.vertex_buffer_size*8;

        Self{
            vertex_buffer:Buffer::empty_unsized(
                display,
                BufferType::ArrayBuffer,
                vertex_buffer_size,
                BufferMode::Default
            ).unwrap(),

            bindings:Vertex2D::build_bindings(),
            vertex_buffer_edge:settings.vertex_buffer_offset,

            index_buffer:Buffer::empty_unsized(
                display,
                BufferType::ArrayBuffer,
                settings.vertex_buffer_size,
                BufferMode::Default
            ).unwrap(),

            index_buffer_edge:settings.index_buffer_offset,

            plain_objects:Vec::<PlainObject>::with_capacity(settings.object_buffer_size),

            draw:Program::from_source(display,vertex_shader,fragment_shader,None).unwrap(),
            draw_shift:Program::from_source(display,shift,fragment_shader,None).unwrap(),
            draw_rotate:Program::from_source(display,rotation,fragment_shader,None).unwrap(),
        }
    }

    /// Вписывает в буфер вершин данные.
    pub fn write_vertex(&self,data:&[Vertex2D])->Option<BufferSlice<[Vertex2D]>>{
        let slice=self.vertex_buffer.slice(0..data.len())?;
        slice.write(data);
        Some(slice)
    }

    /// Вписывает в буфер индексов данные.
    pub fn write_indices(&self,data:&[u8])->Option<BufferSlice<[u8]>>{
        let slice=self.index_buffer.slice(0..data.len())?;
        slice.write(data);
        Some(slice)
    }

    pub fn verteces_source<'a>(&'a self,slice:&'a BufferSlice<[Vertex2D]>)->VerticesSource::<'a>{
        VerticesSource::VertexBuffer(slice.as_slice_any(),&self.bindings,false)
    }

    pub fn indices_source<'a>(&'a self,slice:Option<&'a BufferSlice<[u8]>>,primitive_type:PrimitiveType)->IndicesSource<'a>{
        match slice{
            Some(buffer)=>IndicesSource::IndexBuffer{
                buffer:buffer.as_slice_any(),
                data_type:IndexType::U8,
                primitives:primitive_type,
            },
            None=>IndicesSource::NoIndices{
                primitives:primitive_type,
            }
        }
    }

    pub fn draw<O:SimpleObject>(
        &self,
        object:&O,
        draw_parameters:&mut DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        let verteces=object.vertex_buffer();

        let vertex_slice=self.write_vertex(&verteces).unwrap();
        let vertex_source=self.verteces_source(&vertex_slice);

        let primitive_type=object.primitive_type();

        let indices=if let Some(indices)=object.indices(){
            self.write_indices(&indices)
        }
        else{
            None
        };

        let indices_source=self.indices_source(indices.as_ref(),primitive_type);

        let uni=uniform!{
            colour:object.colour(),
            window_center:unsafe{window_center}
        };

        frame.draw(vertex_source,indices_source,&self.draw,&uni,draw_parameters)
    }

    pub fn draw_shift<O:SimpleObject>(
        &self,
        object:&O,
        shift:[f32;2],
        draw_parameters:&mut DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        let points=object.vertex_buffer();

        let verteces=object.vertex_buffer();

        let vertex_slice=self.write_vertex(&verteces).unwrap();
        let vertex_source=self.verteces_source(&vertex_slice);

        let primitive_type=object.primitive_type();

        let indices=if let Some(indices)=object.indices(){
            self.write_indices(&indices)
        }
        else{
            None
        };
        let indices_source=self.indices_source(indices.as_ref(),primitive_type);

        let uni=uniform!{
            colour:object.colour(),
            shift:shift,
            window_center:unsafe{window_center}
        };

        frame.draw(vertex_source,indices_source,&self.draw_shift,&uni,draw_parameters)
    }

    pub fn draw_rotate<O:SimpleObject>(
        &self,
        object:&O,
        [x,y]:[f32;2],
        angle:f32,
        draw_parameters:&mut DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        let points=object.vertex_buffer();

        let verteces=object.vertex_buffer();

        let vertex_slice=self.write_vertex(&verteces).unwrap();
        let vertex_source=self.verteces_source(&vertex_slice);

        let primitive_type=object.primitive_type();

        let indices=if let Some(indices)=object.indices(){
            self.write_indices(&indices)
        }
        else{
            None
        };
        let indices_source=self.indices_source(indices.as_ref(),primitive_type);

        let (sin,cos)=angle.sin_cos();

        let uni=uniform!{
            cos:cos,
            sin:sin,
            rotation_center:unsafe{[x-window_center[0],window_center[1]-y]},
            window_center:unsafe{window_center},
            colour:object.colour(),
        };

        frame.draw(
            vertex_source,
            indices_source,
            &self.draw_rotate,
            &uni,
            draw_parameters
        )
    }
}

// Функции для работы с объектами
impl SimpleGraphics{
    // Добавляет объект в конец списка
    pub fn push_object<O:SimpleObject>(&mut self,object:&O)->Option<usize>{
        // Вершины
        let vertexes=object.vertex_buffer();

        let new_edge=self.vertex_buffer_edge+vertexes.len();
        let vertex_range=self.vertex_buffer_edge..new_edge;

        // Сдвиг границы
        self.vertex_buffer_edge=new_edge;

        let vertex_slice=self.vertex_buffer.slice(vertex_range.clone())?;
        vertex_slice.write(&vertexes);

        // Индексы
        let index_range=if let Some(indices)=object.indices(){
            let new_edge=self.index_buffer_edge+indices.len();
            let range=self.index_buffer_edge..new_edge;

            self.index_buffer_edge=new_edge;

            let index_slice=self.index_buffer.slice(range.clone())?;
            index_slice.write(&indices);

            range
        }
        else{
            0..0
        };

        let len=self.plain_objects.len();

        self.plain_objects.push(PlainObject{
            vertex_buffer_range:vertex_range,
            index_buffer_range:index_range,

            colour:object.colour(),
            primitive_type:object.primitive_type(),
        });

        Some(len)
    }

    pub fn pop_object(&mut self)->Option<PlainObject>{
        if let Some(object)=self.plain_objects.pop(){
            let len=object.vertex_buffer_range.len();
            self.vertex_buffer_edge-=len;
            Some(object)
        }
        else{
            None
        }
    }

    pub fn delete_last_object(&mut self){
        if let Some(object)=self.plain_objects.pop(){
            let len=object.vertex_buffer_range.len();
            self.vertex_buffer_edge-=len;
        }
    }

    pub fn clear_object_array(&mut self){
        self.vertex_buffer_edge=0;
        self.plain_objects.clear();
    }
}

/// Функции для рисования объектов.
impl SimpleGraphics{
    pub fn draw_object(
        &self,
        index:usize,
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        let object=&self.plain_objects[index];

        let vertex_slice=self.vertex_buffer.slice(object.vertex_buffer_range.clone()).unwrap();

        let index_source=if object.index_buffer_range.len()!=0{
            let slice=self.index_buffer.slice(object.index_buffer_range.clone()).unwrap();
            IndicesSource::IndexBuffer{
                buffer:slice.as_slice_any(),
                data_type:IndexType::U8,
                primitives:object.primitive_type,
            }
        }
        else{
            IndicesSource::NoIndices{primitives:object.primitive_type}
        };

        let uni=uniform!{
            colour:object.colour,
            window_center:unsafe{window_center},
        };

        let vertex_slice=self.verteces_source(&vertex_slice);

        frame.draw(
            vertex_slice,
            index_source,
            &self.draw,
            &uni,
            draw_parameters
        )
    }

    pub fn draw_all_objects(
        &self,
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        for object in &self.plain_objects{
            let vertex_slice=self.vertex_buffer.slice(object.vertex_buffer_range.clone()).unwrap();

            let index_source=if object.index_buffer_range.len()!=0{
                let slice=self.index_buffer.slice(object.index_buffer_range.clone()).unwrap();
                IndicesSource::IndexBuffer{
                    buffer:slice.as_slice_any(),
                    data_type:IndexType::U8,
                    primitives:object.primitive_type,
                }
            }
            else{
                IndicesSource::NoIndices{primitives:object.primitive_type}
            };

            let uni=uniform!{
                colour:object.colour,
                window_center:unsafe{window_center},
            };

            let vertex_slice=self.verteces_source(&vertex_slice);

            frame.draw(
                vertex_slice,
                index_source,
                &self.draw,
                &uni,
                draw_parameters
            )?;
        }

        Ok(())
    }

    pub fn draw_shift_object(
        &self,
        index:usize,
        shift:[f32;2],
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        let object=&self.plain_objects[index];

        let vertex_slice=self.vertex_buffer.slice(object.vertex_buffer_range.clone()).unwrap();

        let index_source=if object.index_buffer_range.len()!=0{
            let slice=self.index_buffer.slice(object.index_buffer_range.clone()).unwrap();
            IndicesSource::IndexBuffer{
                buffer:slice.as_slice_any(),
                data_type:IndexType::U8,
                primitives:object.primitive_type,
            }
        }
        else{
            IndicesSource::NoIndices{primitives:object.primitive_type}
        };

        let uni=uniform!{
            colour:object.colour,
            window_center:unsafe{window_center},
            shift:shift,
        };

        let vertex_slice=self.verteces_source(&vertex_slice);

        frame.draw(
            vertex_slice,
            index_source,
            &self.draw_shift,
            &uni,
            draw_parameters
        )
    }

    pub fn draw_rotate_object(
        &self,
        index:usize,
        [x,y]:[f32;2],
        angle:f32,
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        let object=&self.plain_objects[index];

        let vertex_slice=self.vertex_buffer.slice(object.vertex_buffer_range.clone()).unwrap();

        let index_source=if object.index_buffer_range.len()!=0{
            let slice=self.index_buffer.slice(object.index_buffer_range.clone()).unwrap();
            IndicesSource::IndexBuffer{
                buffer:slice.as_slice_any(),
                data_type:IndexType::U8,
                primitives:object.primitive_type,
            }
        }
        else{
            IndicesSource::NoIndices{primitives:object.primitive_type}
        };

        let (sin,cos)=angle.sin_cos();

        let uni=uniform!{
            cos:cos,
            sin:sin,
            rotation_center:unsafe{[x-window_center[0],window_center[1]-y]},
            window_center:unsafe{window_center},
            colour:object.colour,
        };

        let vertex_slice=self.verteces_source(&vertex_slice);

        frame.draw(
            vertex_slice,
            index_source,
            &self.draw_rotate,
            &uni,
            draw_parameters
        )
    }
}

/// Типаж для создания собственных простых одноцветных объектов.
/// 
/// Trait for creating plain objects.
pub trait SimpleObject{
    /// Цвет объекта.
    /// 
    /// An object's colour.
    fn colour(&self)->Colour;

    /// Точки объекта в оконных координатах.
    /// 
    /// Object's verteces in the window coordinate system.
    fn vertex_buffer(&self)->Vec<Vertex2D>;

    /// Индексы для построения объекта.
    /// 
    /// Indices to build an object.
    fn indices(&self)->Option<Vec<u8>>;

    fn primitive_type(&self)->PrimitiveType;

    fn write_indices(&self,){

    }
}