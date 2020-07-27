#[cfg(feature="simple_graphics")]
use crate::{
    // statics
    window_center,
    // types
    Colour,
    // structs
    graphics::Graphics,
};

use super::{
    InnerGraphicsSettings,
    SimpleObject2D,
    Vertex2D,
};

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
    },
};


/// Графическая основа для простых одноцветных объектов.
/// Размер буферов регулируется вручную при создании.
pub struct SimpleGraphics{
    vertex_buffer:Buffer<[Vertex2D]>,
    bindings:VertexFormat,
    vertex_buffer_active_edge:usize,
    vertex_buffer_edge:usize, // Сколько уже занято

    index_buffer:Buffer<[u8]>,
    index_buffer_active_edge:usize,
    index_buffer_edge:usize, // Сколько уже занято

    objects:Vec<SimpleObject2D>,

    draw:Program,
    draw_shift:Program,
    draw_rotate:Program,
}

impl SimpleGraphics{
    pub fn new(display:&Display,settings:InnerGraphicsSettings,glsl:u16)->SimpleGraphics{
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

            vertex_buffer_active_edge:settings.vertex_buffer_offset,
            vertex_buffer_edge:settings.vertex_buffer_offset,

            index_buffer:Buffer::empty_unsized(
                display,
                BufferType::ArrayBuffer,
                settings.vertex_buffer_size,
                BufferMode::Default
            ).unwrap(),

            index_buffer_active_edge:settings.index_buffer_offset,
            index_buffer_edge:settings.index_buffer_offset,

            objects:Vec::<SimpleObject2D>::with_capacity(settings.object_buffer_size),

            draw:Program::from_source(display,vertex_shader,fragment_shader,None).unwrap(),
            draw_shift:Program::from_source(display,shift,fragment_shader,None).unwrap(),
            draw_rotate:Program::from_source(display,rotation,fragment_shader,None).unwrap(),
        }
    }

    pub fn draw<O:SimpleObject>(
        &self,
        object:&O,
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        // Вписывание вершин и подготовка к выводу
        let vertex_source=object.write_vertices(
            &self.vertex_buffer,
            &self.bindings
        ).expect("VertexBuffer: Not enouth space");

        // Вписывание индексов и подготовка к выводу
        let indices_source=object.write_indices(&self.index_buffer)
                .expect("IndexBuffer: Not enouth space");

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
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        // Вписывание вершин и подготовка к выводу
        let vertex_source=object.write_vertices(
            &self.vertex_buffer,
            &self.bindings
        ).expect("VertexBuffer: Not enouth space");

        // Вписывание индексов и подготовка к выводу
        let indices_source=object.write_indices(&self.index_buffer)
                .expect("IndexBuffer: Not enouth space");

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
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        // Вписывание вершин и подготовка к выводу
        let vertex_source=object.write_vertices(
            &self.vertex_buffer,
            &self.bindings
        ).expect("VertexBuffer: Not enouth space");

        // Вписывание индексов и подготовка к выводу
        let indices_source=object.write_indices(&self.index_buffer)
                .expect("IndexBuffer: Not enouth space");

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

// Функции для добаления/удаления объектов
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

        let len=self.objects.len();

        self.objects.push(SimpleObject2D{
            vertex_buffer_range:vertex_range,
            index_buffer_range:index_range,

            colour:object.colour(),
            primitive_type:object.primitive_type(),
        });

        Some(len)
    }

    pub fn pop_object(&mut self)->Option<SimpleObject2D>{
        if let Some(object)=self.objects.pop(){
            let mut len=object.vertex_buffer_range.len();
            self.vertex_buffer_edge-=len;

            len=object.index_buffer_range.len();
            self.index_buffer_edge-=len;
            Some(object)
        }
        else{
            None
        }
    }

    pub fn delete_last_object(&mut self){
        if let Some(object)=self.objects.pop(){
            let mut len=object.vertex_buffer_range.len();
            self.vertex_buffer_edge-=len;

            len=object.index_buffer_range.len();
            self.index_buffer_edge-=len;
        }
    }

    pub fn clear_object_array(&mut self){
        self.vertex_buffer_edge=self.vertex_buffer_active_edge;
        self.index_buffer_edge=self.index_buffer_active_edge;
        self.objects.clear();
    }
}

// Редактирование объектов
impl SimpleGraphics{
    pub fn get_object_colour(&mut self,index:usize)->&mut Colour{
        &mut self.objects[index].colour
    }

    pub fn set_object_colour(&mut self,index:usize,colour:Colour){
        self.objects[index].colour=colour
    }

    pub fn set_object_primitive_type(&mut self,index:usize,primitive_type:PrimitiveType){
        self.objects[index].primitive_type=primitive_type
    }

    // Если размер новых данных не соответсвует выделенному ранее размеру, то ПАНИКА!
    pub fn rewrite_object_vertices(&mut self,index:usize,vertices:&[Vertex2D]){
        let object=&self.objects[index];

        let vertex_slice=self.vertex_buffer.slice(object.vertex_buffer_range.clone()).unwrap();
        vertex_slice.write(vertices);
    }

    // Если размер новых данных не соответсвует выделенному ранее размеру, то ПАНИКА!
    pub fn rewrite_object_indices(&mut self,index:usize,indices:&[u8]){
        let object=&self.objects[index];

        let index_slice=self.index_buffer.slice(object.index_buffer_range.clone()).unwrap();
        index_slice.write(&indices);
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
        let object=&self.objects[index];

        let index_source=object.indices_source(&self.index_buffer);

        let uni=uniform!{
            colour:object.colour,
            window_center:unsafe{window_center},
        };

        let vertex_slice=object.vertices_source(&self.vertex_buffer,&self.bindings);

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
        for object in &self.objects{
            let index_source=object.indices_source(&self.index_buffer);

            let uni=uniform!{
                colour:object.colour,
                window_center:unsafe{window_center},
            };

            let vertex_slice=object.vertices_source(&self.vertex_buffer,&self.bindings);

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
        let object=&self.objects[index];

        let index_source=object.indices_source(&self.index_buffer);

        let uni=uniform!{
            colour:object.colour,
            window_center:unsafe{window_center},
            shift:shift,
        };

        let vertex_slice=object.vertices_source(&self.vertex_buffer,&self.bindings);

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
        let object=&self.objects[index];

        let index_source=object.indices_source(&self.index_buffer);

        let (sin,cos)=angle.sin_cos();

        let uni=uniform!{
            cos:cos,
            sin:sin,
            rotation_center:unsafe{[x-window_center[0],window_center[1]-y]},
            window_center:unsafe{window_center},
            colour:object.colour,
        };

        let vertex_slice=object.vertices_source(&self.vertex_buffer,&self.bindings);

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
/// Trait for creating simple objects.
pub trait SimpleObject:Sized{
    /// Цвет объекта.
    /// 
    /// Object's colour.
    fn colour(&self)->Colour;

    /// Вершины объекта в оконных координатах.
    /// 
    /// Object's vertices in the window coordinate system.
    fn vertex_buffer(&self)->Vec<Vertex2D>;

    /// Индексы для построения объекта.
    /// 
    /// Indices to build the object.
    fn indices(&self)->Option<Vec<u8>>;

    fn primitive_type(&self)->PrimitiveType;

    /// Вписывает индексы в буфер индексов и возвращает `Some(IndicesSource)` для рисования
    /// или `None`, если недостаточно места.
    /// 
    /// Writes indices to the index buffer and return `Some(IndicesSource)` to draw
    /// or `None` if there is not enough space.
    fn write_indices<'a>(&self,index_buffer:&'a Buffer<[u8]>)->Option<IndicesSource<'a>>{
        Some(
            if let Some(indices)=self.indices(){
                let slice=match index_buffer.slice(0..indices.len()){
                    Some(slice)=>slice,
                    None=>return None,
                };
                slice.write(&indices);

                IndicesSource::IndexBuffer{
                    buffer:slice.as_slice_any(),
                    data_type:IndexType::U8,
                    primitives:self.primitive_type(),
                }
            }
            else{
                IndicesSource::NoIndices{
                    primitives:self.primitive_type(),
                }
            }
        )
    }

    /// Вписывает вершины в буфер индексов и возвращает `Some(IndicesSource)` для рисования
    /// или `None`, если недостаточно места.
    /// 
    /// Writes indices to the index buffer and return `Some(IndicesSource)` to draw
    /// or `None` if there is not enough space.
    fn write_vertices<'a>(
        &self,
        vertex_buffer:&'a Buffer<[Vertex2D]>,
        vertex_format:&'a VertexFormat,
    )->Option<VerticesSource<'a>>{
        let vertices=self.vertex_buffer();

        let slice=match vertex_buffer.slice(0..vertices.len()){
            Some(slice)=>slice,
            None=>return None,
        };

        slice.write(&vertices);

        Some(
            VerticesSource::VertexBuffer(
                slice.as_slice_any(),
                vertex_format,
                false
            )
        )
    }

    /// Рисует объект.
    /// 
    /// Draws the object.
    #[inline(always)]
    fn draw(&self,draw_parameters:&DrawParameters,graphics:&mut Graphics)->Result<(),DrawError>{
        graphics.draw_simple(self,draw_parameters)
    }

    /// Рисует сдвинутый объект.
    /// 
    /// Draws the shifted object.
    #[inline(always)]
    fn draw_shift(&self,shift:[f32;2],draw_parameters:&DrawParameters,graphics:&mut Graphics)->Result<(),DrawError>{
        graphics.draw_shift_simple(self,shift,draw_parameters)
    }

    /// Рисует повёрнутый объект.
    /// 
    /// Draws the rotated object.
    #[inline(always)]
    fn draw_rotate(&self,rotation_center:[f32;2],angle:f32,draw_parameters:&DrawParameters,graphics:&mut Graphics)->Result<(),DrawError>{
        graphics.draw_rotate_simple(self,rotation_center,angle,draw_parameters)
    }
}