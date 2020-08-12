use crate::Colour;

#[cfg(feature="text_graphics")]
use crate::text::TextBase;

use crate::texture::{
    ImageBase,
    Texture,
};

use super::two_dimensions::{
    Vertex2D,
    Graphics2D
};

#[cfg(feature="3D")]
use super::three_dimensions::Graphics3D;

use super::DependentObject;

use glium::{
    Frame,
    DrawParameters,
    DrawError,
    Surface,
};

#[cfg(feature="text_graphics")]
use rusttype::{
    Font,
    Scale,
    Point,
    PositionedGlyph,
};

/// Простой интерфейс для связи кадра и графических функций.
/// Simple interface to connect graphics fuctions to the frame.
pub struct Graphics<'graphics,'frame>{
    pub graphics2d:&'graphics mut Graphics2D,

    /// feature = "3D" (not implemented)
    #[cfg(feature="3D")]
    pub graphics3d:&'graphics mut Graphics3D,

    pub frame:&'frame mut Frame,
}

impl<'graphics,'frame> Graphics<'graphics,'frame>{
    #[inline(always)]
    pub (crate) fn new(
        graphics2d:&'graphics mut Graphics2D,
        #[cfg(feature="3D")]graphics3d:&'graphics mut Graphics3D,
        frame:&'frame mut Frame
    )->Graphics<'graphics,'frame>{
        Self{
            graphics2d,

            #[cfg(feature="3D")]
            graphics3d,

            frame
        }
    }

    /// Заполняет окно данным цветом.
    /// 
    /// Fills the window with the given colour.
    #[inline(always)]
    pub fn clear_colour(&mut self,[r,g,b,a]:[f32;4]){
        self.frame.clear_color(r,g,b,a);
    }

    pub fn draw_glyph(
        &mut self,
        glyph:PositionedGlyph,
        colour:Colour,
        draw_parameters:&DrawParameters,
    )->Result<(),DrawError>{
        self.graphics2d.text.draw_glyph(glyph,colour,draw_parameters,self.frame)
    }

    pub fn draw_char(
        &mut self,
        character:char,
        base:&TextBase,
        font:&Font,
        draw_parameters:&DrawParameters,
    )->Result<(),DrawError>{
        let scale=Scale::uniform(base.font_size);
        let point=Point{
            x:base.position[0],
            y:base.position[1]
        };
        // Получение символа
        let glyph=font.glyph(character).scaled(scale).positioned(point);

        self.graphics2d.text.draw_glyph(glyph,base.colour,draw_parameters,self.frame)
    }

    /// Выводит строку.
    pub fn draw_str(
        &mut self,
        s:&str,
        base:&TextBase,
        font:&Font,
        draw_parameters:&DrawParameters,
    )->Result<(),DrawError>{
        let scale=Scale::uniform(base.font_size);
        // позиция для вывода символа
        let mut point=Point{
            x:base.position[0],
            y:base.position[1]
        };

        let mut width_offset; // сдвиг для следующего символа

        for character in s.chars(){
            // Получение символа
            let scaled_glyph=font.glyph(character).scaled(scale);

            width_offset=scaled_glyph.h_metrics().advance_width;

            let glyph=scaled_glyph.positioned(point);

            self.graphics2d.text.draw_glyph(glyph,base.colour,draw_parameters,self.frame)?;

            point.x+=width_offset;
        }

        Ok(())
    }

    /// Рисует изображение.
    /// 
    /// Draws the image.
    #[inline(always)]
    #[cfg(feature="texture_graphics")]
    pub fn draw_texture(
        &mut self,
        image_base:&ImageBase,
        texture:&Texture,
        draw_parameters:&mut DrawParameters
    )->Result<(),DrawError>{
        self.graphics2d.texture.draw(image_base,texture,draw_parameters,self.frame)
    }

    /// Рисует сдвинутое изображение.
    /// 
    /// Draws the shifted image.
    #[inline(always)]
    #[cfg(feature="texture_graphics")]
    pub fn draw_shift_image(
        &mut self,
        image_base:&ImageBase,
        texture:&Texture,
        shift:[f32;2],
        draw_parameters:&mut DrawParameters
    )->Result<(),DrawError>{
        self.graphics2d.texture.draw_shift(
            image_base,
            texture,
            shift,
            draw_parameters,
            self.frame
        )
    }

    /// Рисует повёрнутое изображение.
    /// 
    /// Draws the rotated image.
    /// 
    /// angle - radians
    #[inline(always)]
    #[cfg(feature="texture_graphics")]
    pub fn draw_rotate_image<'o>(
        &mut self,
        image_base:&'o ImageBase,
        texture:&Texture,
        rotation_center:[f32;2],
        angle:f32,
        draw_parameters:&mut DrawParameters
    )->Result<(),DrawError>{
        self.graphics2d.texture.draw_rotate(
            image_base,
            texture,
            rotation_center,
            angle,
            draw_parameters,
            self.frame,
        )
    }
}

/// # Функции для работы с текстурными объектами. Functions to work with textured objects.
impl<'graphics,'frame> Graphics<'graphics,'frame>{
    /// Рисует сохранённый текстурный объект.
    /// 
    /// Draws the saved textured object.
    #[inline(always)]
    #[cfg(feature="texture_graphics")]
    pub fn draw_textured_object(
        &mut self,
        index:usize,
        draw_parameters:&DrawParameters
    )->Result<(),DrawError>{
        self.graphics2d.texture.draw_object(
            index,
            draw_parameters,
            self.frame,
        )
    }

    /// Рисует все сохранённые текстурные объекты.
    /// 
    /// Draws all the saved textured objects.
    #[inline(always)]
    #[cfg(feature="texture_graphics")]
    pub fn draw_all_textured_objects(
        &mut self,
        draw_parameters:&DrawParameters
    )->Result<(),DrawError>{
        self.graphics2d.texture.draw_all_objects(
            draw_parameters,
            self.frame,
        )
    }

    /// Рисует сдвинутый сохранённый текстурный объект.
    /// 
    /// Draws the shifted saved textured object.
    #[inline(always)]
    #[cfg(feature="texture_graphics")]
    pub fn draw_shift_textured_object(
        &mut self,
        index:usize,
        shift:[f32;2],
        draw_parameters:&DrawParameters
    )->Result<(),DrawError>{
        self.graphics2d.texture.draw_shift_object(
            index,
            shift,
            draw_parameters,
            self.frame
        )
    }

    /// Рисует повёрнутый сохранённый текстурный объект.
    /// 
    /// Draws the rotated saved textured object.
    /// 
    /// rotation_center - [x, y]
    /// angle - radians
    #[inline(always)]
    #[cfg(feature="texture_graphics")]
    pub fn draw_rotate_textured_object(
        &mut self,
        index:usize,
        rotation_center:[f32;2],
        angle:f32,
        draw_parameters:&DrawParameters
    )->Result<(),DrawError>{
        self.graphics2d.texture.draw_rotate_object(
            index,
            rotation_center,
            angle,
            draw_parameters,
            self.frame
        )
    }
}

/// # Функции для работы с простыми объектами. Functions to work with simple objects.
#[cfg(feature="simple_graphics")]
impl<'graphics,'frame> Graphics<'graphics,'frame>{
    /// Рисует простой объект.
    /// 
    /// Draws the simple object.
    #[inline(always)]
    pub fn draw_simple<'o,O,V,I>(
        &mut self,
        object:&'o O,
        draw_parameters:&DrawParameters
    )->Result<(),DrawError>
    where
            O:DependentObject<
                'o,
                Vertex2D,
                u8,
                Vertices=V,
                Indices=I
            >,
            V:AsRef<[Vertex2D]>+'o,
            I:AsRef<[u8]>+'o
    {
        self.graphics2d.simple.draw(object,draw_parameters,self.frame)
    }

    /// Рисует сдвинутый простой объект.
    /// 
    /// Draws shifted simple object.
    #[inline(always)] 
    pub fn draw_shift_simple<'o,O,V,I>(
        &mut self,
        object:&'o O,
        shift:[f32;2],
        draw_parameters:&DrawParameters
    )->Result<(),DrawError>
        where
            O:DependentObject<
                'o,
                Vertex2D,
                u8,
                Vertices=V,
                Indices=I
            >,
            V:AsRef<[Vertex2D]>+'o,
            I:AsRef<[u8]>+'o
    {
        self.graphics2d.simple.draw_shift(object,shift,draw_parameters,self.frame)
    }

    /// Рисует повёрнутый простой объект.
    /// 
    /// Draws the rotated simple object.
    #[inline(always)]
    pub fn draw_rotate_simple<'o,O,V,I>(
        &mut self,object:&'o O,
        rotation_center:[f32;2],
        angle:f32,
        draw_parameters:&DrawParameters
    )->Result<(),DrawError>
        where
            O:DependentObject<
                'o,
                Vertex2D,
                u8,
                Vertices=V,
                Indices=I
            >,
            V:AsRef<[Vertex2D]>+'o,
            I:AsRef<[u8]>+'o
    {
        self.graphics2d.simple.draw_rotate(
            object,
            rotation_center,
            angle,
            draw_parameters,
            self.frame
        )
    }

    /// Рисует простой объект.
    /// 
    /// Draws the saved simple object.
    #[inline(always)]
    pub fn draw_simple_object(
        &mut self,
        index:usize,
        draw_parameters:&DrawParameters
    )->Result<(),DrawError>{
        self.graphics2d.simple.draw_object(index,draw_parameters,self.frame)
    }

    /// Рисует сдвинутый простой объект.
    /// 
    /// Draws the shifted saved simple object.
    #[inline(always)]
    pub fn draw_shift_simple_object(
        &mut self,
        index:usize,
        shift:[f32;2],
        draw_parameters:&DrawParameters
    )->Result<(),DrawError>{
        self.graphics2d.simple.draw_shift_object(
            index,
            shift,
            draw_parameters,
            self.frame
        )
    }

    /// Рисует повёрнутый простой объект.
    /// 
    /// Draws the rotated saved simple object.
    #[inline(always)]
    pub fn draw_rotate_simple_object(
        &mut self,
        index:usize,
        rotation_center:[f32;2],
        angle:f32,
        draw_parameters:&DrawParameters
    )->Result<(),DrawError>{
        self.graphics2d.simple.draw_rotate_object(
            index,
            rotation_center,
            angle,
            draw_parameters,
            self.frame
        )
    }
}