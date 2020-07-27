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

    /// Рисует изображение.
    /// 
    /// Draws the image.
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
        self.graphics2d.texture.draw_shift_image(image_base,texture,shift,draw_parameters,self.frame)
    }

    /// Рисует повёрнутое изображение.
    /// 
    /// Draws the rotated image.
    /// 
    /// angle - radians
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