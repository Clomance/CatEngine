use super::{TexturedVertex2D,TextObject2D};

use crate::{
    // statics
    window_center,
    // types
    Colour,
    // structs
    text::{
        TextBase,
        GlyphCache,
        RawGlyph,
        OutlineCurveBuilder,
        OutlinedGlyph,
        TRect,
        Scale,
    },
    graphics::TextGraphicsSettings,
};

use glium::{
    uniform,
    // enums
    DrawError,
    // traits
    Surface,
    // structs
    Program,
    Frame,
    DrawParameters,
    VertexBuffer,
    Display,
    index::{NoIndices,PrimitiveType},

    vertex::{Vertex,VertexFormat,VerticesSource},

    texture::{
        RawImage2d,
        UncompressedFloatFormat,
        MipmapsOption,
        Texture2d,
        ClientFormat,
    },
    Rect,
};


use ttf_parser::{
    Face,
    GlyphId,
};

use std::borrow::Cow;

pub struct TextGraphics{
    vertex_buffer:VertexBuffer<TexturedVertex2D>,
    vertex_format:VertexFormat,

    image:Vec<u8>,
    glyph_texture:Texture2d,
    texture_size:[f32;2],

    objects:Vec<TextObject2D>,
    glyph_cache:Vec<GlyphCache>,

    draw:Program,
    draw_shift:Program,
    draw_rotate:Program,
}

impl TextGraphics{
    pub fn new(display:&Display,settings:TextGraphicsSettings,glsl:u16)->TextGraphics{
        let (common,shift,rotation,fragment)=if glsl==120{(
            include_str!("shaders/120/texture/vertex_shader.glsl"),
            include_str!("shaders/120/texture/shift_vertex_shader.glsl"),
            include_str!("shaders/120/texture/rotation_vertex_shader.glsl"),
            include_str!("shaders/120/text/fragment_shader.glsl")
        )}
        else{(
            include_str!("shaders/texture/vertex_shader.glsl"),
            include_str!("shaders/texture/shift_vertex_shader.glsl"),
            include_str!("shaders/texture/rotation_vertex_shader.glsl"),
            include_str!("shaders/text/fragment_shader.glsl")
        )};

        let texture_size=[
            settings.glyph_texture_size[0] as f32,
            settings.glyph_texture_size[1] as f32,
        ];

        Self{
            vertex_buffer:VertexBuffer::empty_dynamic(display,4).unwrap(),
            vertex_format:TexturedVertex2D::build_bindings(),

            image:Vec::with_capacity(100),
            glyph_texture:Texture2d::empty_with_format(
                display,
                UncompressedFloatFormat::U8,
                MipmapsOption::NoMipmap,
                20,
                5,
            ).unwrap(),
            texture_size,

            objects:Vec::with_capacity(10),
            glyph_cache:Vec::with_capacity(10),

            draw:Program::from_source(display,common,fragment,None).unwrap(),
            draw_shift:Program::from_source(display,shift,fragment,None).unwrap(),
            draw_rotate:Program::from_source(display,rotation,fragment,None).unwrap(),
        }
    }

    pub fn texture(&self)->&Texture2d{
        &self.glyph_texture
    }

    /// Вписывает вершины в локальный буфер.
    pub fn write_vertices(&self,[x,y,width,height]:[f32;4],[uvw,uvh]:[f32;2]){
        let [x1,y1,x2,y2]=[
            x,
            y,
            x+width,
            y+height
        ];

        let vertices=[
            TexturedVertex2D::new([x1,y1],[0.0,uvh]),
            TexturedVertex2D::new([x1,y2],[0.0,0.0]),
            TexturedVertex2D::new([x2,y1],[uvw,uvh]),
            TexturedVertex2D::new([x2,y2],[uvw,0.0])
        ];

        self.vertex_buffer.write(&vertices);
    }

    /// Записывает глиф в локальную тектстуру.
    pub fn write_glyph(&mut self,glyph:&OutlinedGlyph){
        self.image.clear();

        glyph.draw(|_,a|{
            let gray=255f32*a;
            let byte=gray.round() as u8;
            self.image.push(byte);
        });

        let size=[
            glyph.bounds.width as u32,
            glyph.bounds.height as u32
        ];

        let rect=Rect{
            left:0,
            bottom:0,
            width:size[0],
            height:size[1],
        };

        let raw_image=RawImage2d{
            data:Cow::Borrowed(&self.image),
            width:size[0],
            height:size[1],
            format:ClientFormat::U8,
        };

        self.glyph_texture.write(rect,raw_image);
    }

    /// Рисует глиф с загруженными вершинами.
    /// 
    /// Перед выводом нужно загрузить вершины
    /// с помощью функции `write_vertices`.
    pub fn draw_glyph(
        &self,
        glyph:&Texture2d,
        colour:Colour,
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        let vertices=VerticesSource::VertexBuffer(
            self.vertex_buffer.as_slice_any(),
            &self.vertex_format,
            false
        );

        let uni=uniform!{
            texture2d:glyph,
            colour:colour,
            window_center:unsafe{window_center}
        };

        frame.draw(
            vertices,
            NoIndices(PrimitiveType::TriangleStrip),
            &self.draw,
            &uni,
            draw_parameters
        )
    }



    // Рисует глиф с загруженными вершинами
    pub fn draw_shift_glyph(
        &self,
        glyph:&Texture2d,
        colour:Colour,
        shift:[f32;2],
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        let vertices=VerticesSource::VertexBuffer(
            self.vertex_buffer.as_slice_any(),
            &self.vertex_format,
            false
        );

        let uni=uniform!{
            texture2d:glyph,
            colour:colour,
            shift:shift,
            window_center:unsafe{window_center}
        };

        frame.draw(
            vertices,
            NoIndices(PrimitiveType::TriangleStrip),
            &self.draw_shift,
            &uni,
            draw_parameters
        )
    }


    // Рисует глиф с загруженными вершинами
    pub fn draw_rotate_glyph(
        &self,
        glyph:&Texture2d,
        colour:Colour,
        rotation_center:[f32;2],
        angle:f32,
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        let vertices=VerticesSource::VertexBuffer(
            self.vertex_buffer.as_slice_any(),
            &self.vertex_format,
            false
        );

        let (sin,cos)=angle.sin_cos();

        let uni=uniform!{
            texture2d:glyph,
            colour:colour,
            cos:cos,
            sin:sin,
            rotation_center:rotation_center,
            window_center:unsafe{window_center}
        };

        frame.draw(
            &self.vertex_buffer,
            NoIndices(PrimitiveType::TriangleStrip),
            &self.draw_rotate,
            &uni,
            draw_parameters
        )
    }
}

// Функции для работы с объектами
impl TextGraphics{
    pub fn push_glyph_cache(
        &mut self,
        glyph_cache:GlyphCache,
    )->Option<usize>{
        let index=self.objects.len();

        self.glyph_cache.push(glyph_cache);

        Some(index)
    }

    pub fn get_glyph_cache(&self,index:usize)->&GlyphCache{
        &self.glyph_cache[index]
    }

    pub fn push_object(
        &mut self,
        text:String,
        text_base:&TextBase,
        font:usize,
    )->Option<usize>{
        let object=TextObject2D{
            text,
            position:text_base.position,
            font_size:text_base.font_size,
            colour:text_base.colour,
            font,
        };

        let index=self.objects.len();

        self.objects.push(object);

        Some(index)
    }

    pub fn delete_last_object(&mut self){
        self.objects.pop();
    }

    pub fn draw_object(
        &self,
        index:usize,
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        let object=&self.objects[index];

        let mut position=object.position;

        let glyph_cache=&self.glyph_cache[object.font];

        for character in object.text.chars(){
            let glyph=if let Some(glyph)=glyph_cache.glyph(character){
                glyph
            }
            else{
                if character==' '{
                    position[0]+=glyph_cache.whitespace_advance(object.font_size);
                    continue
                }
                glyph_cache.undefined_glyph()
            };

            let glyph_frame=glyph.frame(object.font_size);

            let mut rect=glyph_frame.bounding_box(position);

            // Запись в буфер вершин
            self.write_vertices(rect,[1f32,1f32]);

            self.draw_glyph(
                glyph.texture(),
                object.colour,
                draw_parameters,
                frame
            )?;

            position[0]+=glyph_frame.advance;
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

        let mut position=object.position;

        let glyph_cache=&self.glyph_cache[object.font];

        for character in object.text.chars(){
            let glyph=if let Some(glyph)=glyph_cache.glyph(character){
                glyph
            }
            else{
                if character==' '{
                    position[0]+=glyph_cache.whitespace_advance(object.font_size);
                    continue
                }
                glyph_cache.undefined_glyph()
            };

            let glyph_frame=glyph.frame(object.font_size);

            let mut rect=glyph_frame.bounding_box(position);

            // Запись в буфер вершин
            self.write_vertices(rect,[1f32,1f32]);

            self.draw_shift_glyph(
                glyph.texture(),
                object.colour,
                shift,
                draw_parameters,
                frame
            )?;

            position[0]+=glyph_frame.advance;
        }

        Ok(())
    }

    pub fn draw_rotate_object(
        &self,
        index:usize,
        rotation_center:[f32;2],
        angle:f32,
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        let object=&self.objects[index];

        let mut position=object.position;

        let glyph_cache=&self.glyph_cache[object.font];

        for character in object.text.chars(){
            let glyph=if let Some(glyph)=glyph_cache.glyph(character){
                glyph
            }
            else{
                if character==' '{
                    position[0]+=glyph_cache.whitespace_advance(object.font_size);
                    continue
                }
                glyph_cache.undefined_glyph()
            };

            let glyph_frame=glyph.frame(object.font_size);

            let mut rect=glyph_frame.bounding_box(position);

            // Запись в буфер вершин
            self.write_vertices(rect,[1f32,1f32]);

            self.draw_rotate_glyph(
                glyph.texture(),
                object.colour,
                rotation_center,
                angle,
                draw_parameters,
                frame
            )?;

            position[0]+=glyph_frame.advance;
        }

        Ok(())
    }
}