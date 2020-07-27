#![cfg(feature="texture_graphics")]
use crate::{
    // statics
    window_center,
    // types
    Colour,
    // structs
    image::{ImageBase,Texture},
};

use super::{
    InnerGraphicsSettings,
    TexturedObject2D,
    SimpleObject2D,
    TexturedVertex2D,
};

use glium::{
    // macroses
    implement_vertex,
    uniform,
    // enums
    DrawError,
    // structs
    Program,
    Display,
    Frame,
    DrawParameters,
    index::{
        PrimitiveType, // enum
        NoIndices,
    },
    Surface, // trait

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

use image::RgbaImage;

use std::mem::size_of;

/// Графическая основа для отрисовки текстур (изображений).
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
    draw:Program,
    pub (crate) draw_shift:Program,
    draw_rotate:Program,
}

impl TextureGraphics{
    pub fn new(display:&Display,settings:InnerGraphicsSettings,glsl:u16)->TextureGraphics{
        let (
            rotation,
            shift,
            vertex_shader,
            fragment_shader
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

            draw:Program::from_source(display,vertex_shader,fragment_shader,None).unwrap(),
            draw_shift:Program::from_source(display,shift,fragment_shader,None).unwrap(),
            draw_rotate:Program::from_source(display,rotation,fragment_shader,None).unwrap(),
        }
    }

    fn write_vertices<'a>(
        &'a self,
        vertices:&[TexturedVertex2D],
    )->Option<VerticesSource<'a>>{
        let slice=match self.vertex_buffer.slice(0..vertices.len()){
            Some(slice)=>slice,
            None=>return None,
        };

        slice.write(&vertices);

        Some(
            VerticesSource::VertexBuffer(
                slice.as_slice_any(),
                &self.bindings,
                false
            )
        )
    }

    /// Строит объект с нуля и выводит, игнорируя все области.
    /// Переписывает координаты с начала буфера [0..4].
    pub fn draw_image(
        &self,
        image_base:&ImageBase,
        texture:&Texture,
        draw_parameters:&DrawParameters,
        frame:&mut Frame,
    )->Result<(),DrawError>{
        let indices=NoIndices(PrimitiveType::TriangleStrip);

        let slice=self.write_vertices(&image_base.vertex_buffer()).unwrap();

        let uni=uniform!{
            tex:&texture.0,
            colour_filter:image_base.colour_filter,
            window_center:unsafe{window_center},
        };

        frame.draw(
            slice,
            indices,
            &self.draw,
            &uni,
            draw_parameters
        )
    }

    /// Строит объект с нуля и выводит, игнорируя все области.
    /// Переписывает координаты с начала буфера [0..4].
    pub fn draw_shift_image(
        &self,
        image_base:&ImageBase,
        texture:&Texture,
        shift:[f32;2],
        draw_parameters:&DrawParameters,
        frame:&mut Frame,
    )->Result<(),DrawError>{
        let indices=NoIndices(PrimitiveType::TriangleStrip);

        let slice=self.write_vertices(&image_base.vertex_buffer()).unwrap();

        frame.draw(
            slice,
            indices,
            &self.draw_shift,
            &uniform!{
                tex:&texture.0,
                colour_filter:image_base.colour_filter,
                shift:shift,
                window_center:unsafe{window_center},
            },
            draw_parameters
        )
    }

    /// Строит объект с нуля и выводит под данным углом, игнорируя все области.
    /// Переписывает координаты с начала буфера [0..4].
    pub fn draw_rotate_image(
        &self,
        image_base:&ImageBase,
        texture:&Texture,
        [x,y]:[f32;2],
        angle:f32,
        frame:&mut Frame,
        draw_parameters:&DrawParameters
    )->Result<(),DrawError>{
        let indices=NoIndices(PrimitiveType::TriangleStrip);

        let (sin,cos)=angle.sin_cos();

        let uni=uniform!{
            tex:&texture.0,
            cos:cos,
            sin:sin,
            rotation_center:unsafe{[x-window_center[0],window_center[1]-y]},
            window_center:unsafe{window_center},
            colour_filter:image_base.colour_filter,
        };

        let slice=self.write_vertices(&image_base.vertex_buffer()).unwrap();

        frame.draw(
            slice,
            indices,
            &self.draw_rotate,
            &uni,
            draw_parameters
        )
    }
}


// Функции для добавления/удаления объектов
impl TextureGraphics{
    pub fn push_object(&mut self,image_base:&ImageBase,texture:Texture)->Option<usize>{
        // Вершины
        let vertexes=image_base.vertex_buffer();

        let new_edge=self.vertex_buffer_edge+vertexes.len();
        let vertex_range=self.vertex_buffer_edge..new_edge;

        // Сдвиг границы
        self.vertex_buffer_edge=new_edge;

        let vertex_slice=self.vertex_buffer.slice(vertex_range.clone())?;
        vertex_slice.write(&vertexes);

        let len=self.objects.len();

        self.objects.push(TexturedObject2D{
            base:SimpleObject2D{
                vertex_buffer_range:vertex_range,
                index_buffer_range:0..0,

                colour:image_base.colour_filter,
                primitive_type:PrimitiveType::TriangleStrip,
            },
            texture:texture,
        });

        Some(len)
    }

    pub fn pop_object(&mut self)->Option<TexturedObject2D>{
        if let Some(object)=self.objects.pop(){
            let mut len=object.base.vertex_buffer_range.len();
            self.vertex_buffer_edge-=len;

            len=object.base.index_buffer_range.len();
            self.index_buffer_edge-=len;
            Some(object)
        }
        else{
            None
        }
    }

    pub fn delete_last_object(&mut self){
        if let Some(object)=self.objects.pop(){
            let mut len=object.base.vertex_buffer_range.len();
            self.vertex_buffer_edge-=len;

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
        &mut self.objects[index].texture
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
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        let object=&self.objects[index];

        let index_source=object.indices_source(&self.index_buffer);

        let uni=uniform!{
            tex:&object.texture.0,
            colour_filter:object.base.colour,
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
                tex:&object.texture.0,
                colour_filter:object.base.colour,
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
            tex:&object.texture.0,
            colour_filter:object.base.colour,
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
            tex:&object.texture.0,
            cos:cos,
            sin:sin,
            rotation_center:unsafe{[x-window_center[0],window_center[1]-y]},
            window_center:unsafe{window_center},
            colour_filter:object.base.colour,
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

    pub fn update_object_image(&mut self,index:usize,image:&RgbaImage){
        self.objects[index].texture.update(image)
    }
}