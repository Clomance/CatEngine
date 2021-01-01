use crate::{
    // statics
    window_center,
    // types
    Colour,
    // structs
    text::{
        OutlinedGlyph,
        Scale,
        CachedFont,
        RawGlyphCache,
    },
    graphics::TextGraphicsSettings,
};

#[cfg(feature="colour_filter")]
use crate::graphics::ColourFilter;

use super::{TexturedVertex2D,TextObject2D};

use glium::{
    uniform,
    // enums
    DrawError,
    // traits
    Surface,
    // structs
    Program,
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

use std::{
    cell::UnsafeCell,
    borrow::Cow
};

pub struct TextGraphics{
    vertex_buffer:VertexBuffer<TexturedVertex2D>,
    vertex_format:VertexFormat,

    image:UnsafeCell<Vec<u8>>,
    glyph_texture:Texture2d,
    texture_size:[f32;2],

    objects:Vec<TextObject2D>,
    cached_font:Vec<CachedFont>,

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

            image:UnsafeCell::new(Vec::with_capacity((settings.glyph_texture_size[0]*settings.glyph_texture_size[1]) as usize)),
            glyph_texture:Texture2d::empty_with_format(
                display,
                UncompressedFloatFormat::U8,
                MipmapsOption::NoMipmap,
                settings.glyph_texture_size[0],
                settings.glyph_texture_size[1],
            ).unwrap(),
            texture_size,

            objects:Vec::with_capacity(10),
            cached_font:Vec::with_capacity(10),

            draw:Program::from_source(display,common,fragment,None).unwrap(),
            draw_shift:Program::from_source(display,shift,fragment,None).unwrap(),
            draw_rotate:Program::from_source(display,rotation,fragment,None).unwrap(),
        }
    }

    /// Локальная тектстура.
    pub fn texture(&self)->&Texture2d{
        &self.glyph_texture
    }

    pub fn texture_size(&self)->[f32;2]{
        self.texture_size
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
    pub fn write_glyph(&self,glyph:&OutlinedGlyph){
        let image:&mut Vec<u8>=unsafe{&mut *self.image.get()};
        image.clear();

        // Запись изображения
        glyph.draw(|_,a|{
            let gray=255f32*a;
            let byte=gray.round() as u8;
            image.push(byte);
        });

        let size=glyph.size();

        let rect=Rect{
            left:0,
            bottom:0,
            width:size[0],
            height:size[1],
        };

        let raw_image=RawImage2d{
            data:Cow::Borrowed(unsafe{&*self.image.get()}),
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
    pub fn draw_glyph<S:Surface>(
        &self,
        glyph:&Texture2d,
        mut colour:Colour,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        draw_parameters:&DrawParameters,
        frame:&mut S,
    )->Result<(),DrawError>{
        let vertices=VerticesSource::VertexBuffer(
            self.vertex_buffer.as_slice_any(),
            &self.vertex_format,
            false
        );

        // Фильтрация цвета объекта
        #[cfg(feature="colour_filter")]
        colour_filter.filter_colour(&mut colour);

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



    /// Рисует глиф с загруженными вершинами.
    pub fn draw_shift_glyph<S:Surface>(
        &self,
        glyph:&Texture2d,
        mut colour:Colour,
        shift:[f32;2],
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        draw_parameters:&DrawParameters,
        frame:&mut S
    )->Result<(),DrawError>{
        let vertices=VerticesSource::VertexBuffer(
            self.vertex_buffer.as_slice_any(),
            &self.vertex_format,
            false
        );

        // Фильтрация цвета объекта
        #[cfg(feature="colour_filter")]
        colour_filter.filter_colour(&mut colour);

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


    /// Рисует глиф с загруженными вершинами.
    pub fn draw_rotate_glyph<S:Surface>(
        &self,
        glyph:&Texture2d,
        mut colour:Colour,
        rotation_center:[f32;2],
        angle:f32,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        draw_parameters:&DrawParameters,
        frame:&mut S
    )->Result<(),DrawError>{
        let (sin,cos)=angle.sin_cos();

        // Фильтрация цвета объекта
        #[cfg(feature="colour_filter")]
        colour_filter.filter_colour(&mut colour);

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

/// Функции для работы с объектами.
impl TextGraphics{
    /// Добавляет шрифт.
    pub fn push_font(&mut self,cached_font:CachedFont)->usize{
        let index=self.objects.len();
        self.cached_font.push(cached_font);
        index
    }

    #[inline(always)]
    pub fn remove_last_font(&mut self){
        self.cached_font.pop();
    }

    #[inline(always)]
    pub fn remove_all_fonts(&mut self){
        self.cached_font.clear();
    }

    /// Возвращает шрифт.
    #[inline(always)]
    pub fn get_font(&self,index:usize)->&CachedFont{
        &self.cached_font[index]
    }

    /// Добавляет объект в конец массива.
    /// 
    /// Возвращает индекс объекта, если он добавлен.
    pub fn push_object(
        &mut self,
        text:String,
        position:[f32;2],
        scale:Scale,
        colour:Colour,
        font:usize,
    )->Option<usize>{
        let object=TextObject2D{
            text,
            position:position,
            scale,
            colour:colour,
            font,
        };

        let index=self.objects.len();

        // Если есть место в массиве,
        // добавляет объект
        if index<self.objects.capacity(){
            self.objects.push(object);
            Some(index)
        }
        else{
            None
        }
    }

    /// Удаляет последний объект.
    #[inline(always)]
    pub fn remove_last_object(&mut self){
        self.objects.pop();
    }

    #[inline(always)]
    pub fn clear_object_array(&mut self){
        self.objects.clear();
    }
}

/// Функции редактирования объектов.
impl TextGraphics{
    #[inline(always)]
    pub fn object_text(&mut self,index:usize)->&mut String{
        &mut self.objects[index].text
    }

    #[inline(always)]
    pub fn object_colour(&mut self,index:usize)->&mut Colour{
        &mut self.objects[index].colour
    }

    #[inline(always)]
    pub fn object_font(&mut self,index:usize)->&mut usize{
        &mut self.objects[index].font
    }

    #[inline(always)]
    pub fn object_scale(&mut self,index:usize)->&mut Scale{
        &mut self.objects[index].scale
    }

    #[inline(always)]
    pub fn object_position(&mut self,index:usize)->&mut [f32;2]{
        &mut self.objects[index].position
    }
}

/// Функции для отрисовки объектов.
impl TextGraphics{
    /// Выводит сохранённый объект.
    pub fn draw_object<S:Surface>(
        &self,
        index:usize,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        draw_parameters:&DrawParameters,
        frame:&mut S
    )->Result<(),DrawError>{
        let object=&self.objects[index];

        let mut position=object.position;

        let font=&self.cached_font[object.font];

        for character in object.text.chars(){
            let glyph=if let Some(glyph)=font.scaled_glyph(character,object.scale){
                glyph
            }
            else{
                if character==' '{
                    position[0]+=font.whitespace_advance_width(object.scale.horizontal);
                    continue
                }
                font.scaled_undefined_glyph(object.scale)
            };

            let rect=glyph.positioned_bounding_box(position);

            // Запись в буфер вершин
            self.write_vertices(rect,[1f32,1f32]);

            self.draw_glyph(
                glyph.data(), // текстура в данном случае
                object.colour,
                #[cfg(feature="colour_filter")]colour_filter,
                draw_parameters,
                frame
            )?;

            position[0]+=glyph.advance_width();
        }
        Ok(())
    }

    /// Выводит сдвинутый сохранённый объект.
    pub fn draw_shift_object<S:Surface>(
        &self,
        index:usize,
        shift:[f32;2],
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        draw_parameters:&DrawParameters,
        frame:&mut S
    )->Result<(),DrawError>{
        let object=&self.objects[index];

        let mut position=object.position;

        let font=&self.cached_font[object.font];

        for character in object.text.chars(){
            let glyph=if let Some(glyph)=font.scaled_glyph(character,object.scale){
                glyph
            }
            else{
                if character==' '{
                    position[0]+=font.whitespace_advance_width(object.scale.horizontal);
                    continue
                }
                font.scaled_undefined_glyph(object.scale)
            };

            let rect=glyph.positioned_bounding_box(position);

            // Запись в буфер вершин
            self.write_vertices(rect,[1f32,1f32]);

            self.draw_shift_glyph(
                glyph.data(), // текстура в данном случае
                object.colour,
                shift,
                #[cfg(feature="colour_filter")]colour_filter,
                draw_parameters,
                frame
            )?;

            position[0]+=glyph.advance_width();
        }

        Ok(())
    }

    /// Выводит повёрнутый сохранённый объект.
    pub fn draw_rotate_object<S:Surface>(
        &self,
        index:usize,
        rotation_center:[f32;2],
        angle:f32,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        draw_parameters:&DrawParameters,
        frame:&mut S
    )->Result<(),DrawError>{
        let object=&self.objects[index];

        let mut position=object.position;

        let font=&self.cached_font[object.font];

        for character in object.text.chars(){
            let glyph=if let Some(glyph)=font.scaled_glyph(character,object.scale){
                glyph
            }
            else{
                if character==' '{
                    position[0]+=font.whitespace_advance_width(object.scale.horizontal);
                    continue
                }
                font.scaled_undefined_glyph(object.scale)
            };

            let rect=glyph.positioned_bounding_box(position);

            // Запись в буфер вершин
            self.write_vertices(rect,[1f32,1f32]);

            self.draw_rotate_glyph(
                glyph.data(), // текстура в данном случае
                object.colour,
                rotation_center,
                angle,
                #[cfg(feature="colour_filter")]colour_filter,
                draw_parameters,
                frame
            )?;

            position[0]+=glyph.advance_width();
        }

        Ok(())
    }
}