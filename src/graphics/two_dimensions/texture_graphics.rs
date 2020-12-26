use crate::{
    // statics
    window_center,
    // types
    Colour,
    // structs
    texture::Texture,
    graphics::InnerGraphicsSettings,
};

#[cfg(feature="colour_filter")]
use crate::graphics::ColourFilter;

use super::{
    TexturedObject2D,
    SimpleObject2D,
    TexturedVertex2D,
    DependentObject,
};

use glium::{
    // macroses
    uniform,
    // enums
    DrawError,
    index::PrimitiveType,
    // traits
    Surface,
    // structs
    Program,
    Display,
    Frame,
    DrawParameters,
    vertex::{Vertex,VertexFormat},
    buffer::{
        Buffer,
        BufferType,
        BufferMode,
    },
};

use std::mem::size_of;

/// Графическая основа для отрисовки текстур.
/// 
/// Размер буферов регулируется вручную при создании.
pub struct TextureGraphics{
    vertex_buffer:Buffer<[TexturedVertex2D]>,
    bindings:VertexFormat,
    vertex_buffer_active_edge:usize,
    vertex_buffer_edge:usize, // Сколько уже занято

    index_buffer:Buffer<[u8]>,
    index_buffer_active_edge:usize,
    index_buffer_edge:usize, // Сколько уже занято

    objects:Vec<TexturedObject2D>,
    textures:Vec<Texture>,
    draw:Program,
    draw_shift:Program,
    draw_rotate:Program,
}

impl TextureGraphics{
    pub fn new(display:&Display,settings:InnerGraphicsSettings,glsl:u16)->TextureGraphics{
        let (
            rotation,
            shift,
            vertex_shader,
            fragment_shader,
        )=if glsl==120{(
            include_str!("shaders/120/texture/rotation_vertex_shader.glsl"),
            include_str!("shaders/120/texture/shift_vertex_shader.glsl"),
            include_str!("shaders/120/texture/vertex_shader.glsl"),
            include_str!("shaders/120/texture/fragment_shader.glsl"),

        )}
        else{(
            include_str!("shaders/texture/rotation_vertex_shader.glsl"),
            include_str!("shaders/texture/shift_vertex_shader.glsl"),
            include_str!("shaders/texture/vertex_shader.glsl"),
            include_str!("shaders/texture/fragment_shader.glsl"),
        )};

        let vertex_buffer_size=settings.vertex_buffer_size*size_of::<TexturedVertex2D>();

        Self{
            vertex_buffer:Buffer::empty_unsized(
                display,
                BufferType::ArrayBuffer,
                vertex_buffer_size,
                BufferMode::Default
            ).unwrap(),

            bindings:TexturedVertex2D::build_bindings(),

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

            objects:Vec::<TexturedObject2D>::with_capacity(settings.object_buffer_size),
            textures:Vec::<Texture>::with_capacity(settings.object_buffer_size),

            draw:Program::from_source(display,vertex_shader,fragment_shader,None).unwrap(),
            draw_shift:Program::from_source(display,shift,fragment_shader,None).unwrap(),
            draw_rotate:Program::from_source(display,rotation,fragment_shader,None).unwrap(),
        }
    }

    /// Строит объект с нуля и выводит, игнорируя все области.
    /// Переписывает координаты с начала буфера [0..].
    pub fn draw<O,V,I>(
        &self,
        object:&O,
        texture:&Texture,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        draw_parameters:&DrawParameters,
        frame:&mut Frame,
    )->Result<(),DrawError>
        where
            O:DependentObject<
                TexturedVertex2D,
                u8,
                Vertices=V,
                Indices=I
            >,
            V:AsRef<[TexturedVertex2D]>,
            I:AsRef<[u8]>
    {
        // Вписывание вершин и подготовка к выводу
        let vertex_source=object.write_vertices(
            &self.vertex_buffer,
            &self.bindings
        ).expect("VertexBuffer: Not enough space");

        // Вписывание индексов и подготовка к выводу
        let indices_source=object.write_indices(&self.index_buffer)
                .expect("IndexBuffer: Not enough space");

        // Фильтрация цвета объекта
        let mut colour=object.colour();
        #[cfg(feature="colour_filter")]
        colour_filter.filter_colour(&mut colour);

        let uni=uniform!{
            texture2d:&texture.0,
            colour_filter:colour,
            window_center:unsafe{window_center}
        };

        frame.draw(
            vertex_source,
            indices_source,
            &self.draw,
            &uni,
            draw_parameters
        )
    }

    /// Строит объект с нуля и выводит, игнорируя все области.
    /// Переписывает координаты с начала буфера [0..].
    pub fn draw_shift<O,V,I>(
        &self,
        object:&O,
        texture:&Texture,
        shift:[f32;2],
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        draw_parameters:&DrawParameters,
        frame:&mut Frame,
    )->Result<(),DrawError>
        where
            O:DependentObject<
                TexturedVertex2D,
                u8,
                Vertices=V,
                Indices=I
            >,
            V:AsRef<[TexturedVertex2D]>,
            I:AsRef<[u8]>+
    {
        // Вписывание вершин и подготовка к выводу
        let vertex_source=object.write_vertices(
            &self.vertex_buffer,
            &self.bindings
        ).expect("VertexBuffer: Not enough space");

        // Вписывание индексов и подготовка к выводу
        let indices_source=object.write_indices(&self.index_buffer)
                .expect("IndexBuffer: Not enough space");

        // Фильтрация цвета объекта
        let mut colour=object.colour();
        #[cfg(feature="colour_filter")]
        colour_filter.filter_colour(&mut colour);

        let uni=uniform!{
            texture2d:&texture.0,
            colour_filter:colour,
            shift:shift,
            window_center:unsafe{window_center},
        };

        frame.draw(
            vertex_source,
            indices_source,
            &self.draw_shift,
            &uni,
            draw_parameters
        )
    }

    /// Строит объект с нуля и выводит под данным углом, игнорируя все области.
    /// Переписывает координаты с начала буфера [0..].
    pub fn draw_rotate<O,V,I>(
        &self,
        object:&O,
        texture:&Texture,
        rotation_center:[f32;2],
        angle:f32,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        draw_parameters:&DrawParameters,
        frame:&mut Frame,
    )->Result<(),DrawError>
        where
            O:DependentObject<
                TexturedVertex2D,
                u8,
                Vertices=V,
                Indices=I
            >,
            V:AsRef<[TexturedVertex2D]>,
            I:AsRef<[u8]>+
    {
        // Вписывание вершин и подготовка к выводу
        let vertex_source=object.write_vertices(
            &self.vertex_buffer,
            &self.bindings
        ).expect("VertexBuffer: Not enough space");

        // Фильтрация цвета объекта
        let mut colour=object.colour();
        #[cfg(feature="colour_filter")]
        colour_filter.filter_colour(&mut colour);

        // Вписывание индексов и подготовка к выводу
        let indices_source=object.write_indices(&self.index_buffer)
                .expect("IndexBuffer: Not enough space");

        let (sin,cos)=angle.sin_cos();

        let uni=uniform!{
            texture2d:&texture.0,
            cos:cos,
            sin:sin,
            rotation_center:rotation_center,
            window_center:unsafe{window_center},
            colour_filter:colour,
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


// Функции для добавления/удаления объектов
impl TextureGraphics{
    pub fn add_texture(&mut self,texture:Texture){
        self.textures.push(texture)
    }

    pub fn remove_last_texture(&mut self){
        self.textures.pop();
    }

    pub fn remove_all_textures(&mut self){
        self.textures.clear();
    }

    pub fn push_object<O,V,I>(
        &mut self,
        object:&O,
        texture:usize
    )->Option<usize>
        where
            O:DependentObject<
                TexturedVertex2D,
                u8,
                Vertices=V,
                Indices=I
            >,
            V:AsRef<[TexturedVertex2D]>,
            I:AsRef<[u8]>+
    {
        // Вершины
        let vertexesb=object.vertices();
        let vertexes=vertexesb.as_ref();

        let new_edge=self.vertex_buffer_edge+vertexes.len();
        let vertex_range=self.vertex_buffer_edge..new_edge;

        // Сдвиг границы
        self.vertex_buffer_edge=new_edge;

        let vertex_slice=self.vertex_buffer.slice(vertex_range.clone())?;
        vertex_slice.write(&vertexes);

        // Индексы
        let index_range=if let Some(indicesb)=object.indices(){
            let indices=indicesb.as_ref();

            let new_edge=self.index_buffer_edge+indices.len();
            let range=self.index_buffer_edge..new_edge;

            // Сдвиг границы
            self.index_buffer_edge=new_edge;

            let index_slice=self.index_buffer.slice(range.clone())?;
            index_slice.write(&indices);

            range
        }
        else{
            0..0
        };

        let len=self.objects.len();

        self.objects.push(TexturedObject2D{
            base:SimpleObject2D{
                vertex_buffer_range:vertex_range,
                index_buffer_range:index_range,

                colour:object.colour(),
                primitive_type:PrimitiveType::TriangleStrip,
            },
            texture:texture,
        });

        Some(len)
    }

    pub fn remove_last_object(&mut self){
        if let Some(object)=self.objects.pop(){
            // Сдвиг границы вершин
            let mut len=object.base.vertex_buffer_range.len();
            self.vertex_buffer_edge-=len;

            // Сдвиг границы индексов
            len=object.base.index_buffer_range.len();
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
impl TextureGraphics{
    pub fn get_object_colour(&mut self,index:usize)->&mut Colour{
        &mut self.objects[index].base.colour
    }

    pub fn get_object_texture(&mut self,index:usize)->&mut Texture{
        &mut self.textures[self.objects[index].texture]
    }

    pub fn set_object_colour(&mut self,index:usize,colour:Colour){
        self.objects[index].base.colour=colour
    }

    pub fn set_object_primitive_type(&mut self,index:usize,primitive_type:PrimitiveType){
        self.objects[index].base.primitive_type=primitive_type
    }

    // Если размер новых данных не соответсвует выделенному ранее размеру, то ПАНИКА!
    pub fn rewrite_object_vertices(&mut self,index:usize,vertices:&[TexturedVertex2D]){
        let object=&self.objects[index];

        let vertex_slice=self.vertex_buffer.slice(object.base.vertex_buffer_range.clone()).unwrap();
        vertex_slice.write(vertices);
    }

    // Если размер новых данных не соответсвует выделенному ранее размеру, то ПАНИКА!
    pub fn rewrite_object_indices(&mut self,index:usize,indices:&[u8]){
        let object=&self.objects[index];

        let index_slice=self.index_buffer.slice(object.base.index_buffer_range.clone()).unwrap();
        index_slice.write(&indices);
    }
}

/// Функции для рисования объектов.
impl TextureGraphics{
    pub fn draw_object(
        &self,
        index:usize,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        let object=&self.objects[index];

        let index_source=object.indices_source(&self.index_buffer);

        // Фильтрация цвета объекта
        let mut colour=object.base.colour;
        #[cfg(feature="colour_filter")]
        colour_filter.filter_colour(&mut colour);

        let uni=uniform!{
            texture2d:&self.textures[object.texture].0,
            colour_filter:colour,
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

    pub fn draw_shift_object(
        &self,
        index:usize,
        shift:[f32;2],
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        let object=&self.objects[index];

        let index_source=object.indices_source(&self.index_buffer);

        // Фильтрация цвета объекта
        let mut colour=object.base.colour;
        #[cfg(feature="colour_filter")]
        colour_filter.filter_colour(&mut colour);

        let uni=uniform!{
            texture2d:&self.textures[object.texture].0,
            colour_filter:colour,
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
        rotation_center:[f32;2],
        angle:f32,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        let object=&self.objects[index];

        let index_source=object.indices_source(&self.index_buffer);

        // Фильтрация цвета объекта
        let mut colour=object.base.colour;
        #[cfg(feature="colour_filter")]
        colour_filter.filter_colour(&mut colour);

        let (sin,cos)=angle.sin_cos();

        let uni=uniform!{
            texture2d:&self.textures[object.texture].0,
            cos:cos,
            sin:sin,
            rotation_center:rotation_center,
            window_center:unsafe{window_center},
            colour_filter:colour,
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