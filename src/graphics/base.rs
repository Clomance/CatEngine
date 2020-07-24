use crate::Colour;

#[cfg(feature="text_graphics")]
use crate::text::Character;

#[cfg(feature="texture_graphics")]
use crate::image::{
    ImageBase,
    Texture,
};

#[cfg(feature="simple_graphics")]
use super::two_dimensions::SimpleObject;

use super::two_dimensions::Graphics2D;

#[cfg(feature="3D")]
use super::three_dimensions::Graphics3D;

use glium::{
    Frame,
    DrawParameters,
    DrawError,
    Surface,
};

/// Простой интерфейс для связи кадра и графических функций.
/// Simple interface to connect graphics fuctions to the frame.
pub struct Graphics<'graphics,'frame>{
    graphics2d:&'graphics Graphics2D,

    #[cfg(feature="3D")]
    graphics3d:&'graphics Graphics3D,

    frame:&'frame mut Frame,
}

impl<'graphics,'frame> Graphics<'graphics,'frame>{
    #[inline(always)]
    pub (crate) fn new(
        graphics2d:&'graphics Graphics2D,
        #[cfg(feature="3D")]graphics3d:&'graphics Graphics3D,
        frame:&'frame mut Frame
    )->Graphics<'graphics,'frame>{
        Self{
            graphics2d,

            #[cfg(feature="3D")]
            graphics3d,

            frame
        }
    }

    /// Возвращает ссылку на кадр.
    /// 
    /// Returns the reference to the frame.
    #[inline(always)]
    pub fn frame(&mut self)->&mut Frame{
        self.frame
    }

    /// Заполняет окно данным цветом.
    /// 
    /// Fills the window with the given colour.
    #[inline(always)]
    pub fn clear_colour(&mut self,[r,g,b,a]:[f32;4]){
        self.frame.clear_color(r,g,b,a);
    }

    /// Рисует один символ.
    /// 
    /// Draws one character.
    #[inline(always)]
    #[cfg(feature="text_graphics")]
    pub fn draw_character(
        &mut self,
        colour:Colour,
        character:&Character,
        draw_parameters:&mut DrawParameters
    )->Result<(),DrawError>{
        self.graphics2d.text.draw_character(character,colour,draw_parameters,self.frame)
    }

    /// Рисует изображение на основе `ImageBase`.
    /// 
    /// Draws the image based on `ImageBase`.
    #[inline(always)] 
    #[cfg(feature="texture_graphics")]
    pub fn draw_image(
        &mut self,
        image_base:&ImageBase,
        texture:&Texture,
        draw_parameters:&mut DrawParameters
    )->Result<(),DrawError>{
        self.graphics2d.texture.draw_image(image_base,texture,draw_parameters,self.frame)
    }

    /// Рисует изображение на основе `ImageBase`.
    /// 
    /// Draws the image based on `ImageBase`.
    #[inline(always)] 
    #[cfg(feature="texture_graphics")]
    pub fn draw_shift_image(
        &mut self,
        image_base:&ImageBase,
        texture:&Texture,
        shift:[f32;2],
        draw_parameters:&mut DrawParameters
    )->Result<(),DrawError>{
        self.graphics2d.texture.draw_shift_image(image_base,texture,shift,draw_parameters,self.frame)
    }

    /// Рисует изображение на основе `ImageBase` c поворотом в 'angle' градусов.
    /// 
    /// Draws the image based on `ImageBase` rotated `angle` degrees.
    #[inline(always)]
    #[cfg(feature="texture_graphics")]
    pub fn draw_rotate_image(
        &mut self,
        image_base:&ImageBase,
        texture:&Texture,
        rotation_center:[f32;2],
        angle:f32,
        draw_parameters:&mut DrawParameters
    )->Result<(),DrawError>{
        self.graphics2d.texture.draw_rotate_image(
            image_base,
            texture,
            rotation_center,
            angle,
            self.frame,
            draw_parameters
        )
    }
}

/// # Функции для работы с областями. Functions to work with ranges.
impl<'graphics,'frame> Graphics<'graphics,'frame>{
    /// Рисует изображение на основе данных из области.
    /// 
    /// Draws the image based on data from the range.
    #[inline(always)]
    #[cfg(feature="texture_graphics")]
    pub fn draw_range_image(
        &mut self,
        index:usize,
        texture:&Texture,
        colour_filter:Colour,
        draw_parameters:&mut DrawParameters
    )->Result<(),DrawError>{
        self.graphics2d.draw_range_image(
            index,
            texture,
            colour_filter,
            draw_parameters,
            self.frame,
        )
    }

    /// Рисует сдвинутое изображение на основе данных из области.
    /// 
    /// Draws shifted the image based on data from the range.
    #[inline(always)]
    #[cfg(feature="texture_graphics")]
    pub fn draw_shift_range_image(
        &mut self,
        index:usize,
        texture:&Texture,
        colour_filter:Colour,
        shift:[f32;2],
        draw_parameters:&mut DrawParameters
    )->Result<(),DrawError>{
        self.graphics2d.draw_shift_range_image(
            index,
            texture,
            colour_filter,
            shift,
            draw_parameters,
            self.frame
        )
    }

    /// Рисует изображение с поворотом в 'angle' градусов на основе
    /// данных из области.
    /// 
    /// Draws the image based on data from the range rotated `angle` degrees.
    /// 
    /// rotation_center - [x, y]
    /// angle - radians
    #[inline(always)]
    #[cfg(feature="texture_graphics")]
    pub fn draw_rotate_range_image(
        &mut self,
        index:usize,
        texture:&Texture,
        colour_filter:Colour,
        rotation_center:[f32;2],
        angle:f32,
        draw_parameters:&mut DrawParameters
    )->Result<(),DrawError>{
        self.graphics2d.draw_rotate_range_image(
            index,
            texture,
            colour_filter,
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
    pub fn draw_simple<O:SimpleObject>(
        &mut self,
        object:&O,
        draw_parameters:&DrawParameters
    )->Result<(),DrawError>{
        self.graphics2d.simple.draw(object,draw_parameters,self.frame)
    }

    /// Рисует сдвинутый простой объект.
    /// 
    /// Draws shifted simple object.
    #[inline(always)] 
    pub fn draw_shift_simple<O:SimpleObject>(
        &mut self,
        object:&O,
        shift:[f32;2],
        draw_parameters:&DrawParameters
    )->Result<(),DrawError>{
        self.graphics2d.simple.draw_shift(object,shift,draw_parameters,self.frame)
    }

    /// Рисует повёрнутый простой объект.
    /// 
    /// Draws the rotated simple object.
    #[inline(always)]
    pub fn draw_rotate_simple<O:SimpleObject>(
        &mut self,object:&O,
        rotation_center:[f32;2],
        angle:f32,
        draw_parameters:&DrawParameters
    )->Result<(),DrawError>{
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