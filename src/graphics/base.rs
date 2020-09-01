use crate::Colour;

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

use super::{DependentObject,ObjectType,DrawType};

use glium::{
    Frame,
    DrawParameters,
    DrawError,
    Surface,
};

#[cfg(feature="text_graphics")]
use rusttype::{PositionedGlyph,Font};





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
        self.frame.clear_color(r,g,b,a)
    }

    /// Рисует сохранённый объект.
    /// 
    /// Draws a saved object.
    #[cfg(any(feature="simple_graphics",feature="texture_graphics",feature="text_graphics"))]
    pub fn draw_object(
        &mut self,
        index:usize,
        object_type:ObjectType,
        draw_type:DrawType,
        draw_parameters:&DrawParameters
    )->Result<(),DrawError>{
        match object_type{
            #[cfg(feature="simple_graphics")]
            ObjectType::Simple=>{
                match draw_type{
                    DrawType::Common=>self.draw_simple_object(
                        index,
                        &draw_parameters
                    ),

                    DrawType::Shifting(shift)=>self.draw_shift_simple_object(
                        index,
                        shift,
                        &draw_parameters
                    ),

                    DrawType::Rotating((angle,position))=>self.draw_rotate_simple_object(
                        index,
                        position,
                        angle,
                        &draw_parameters
                    ),
                }
            }

            #[cfg(feature="texture_graphics")]
            ObjectType::Textured=>{
                match draw_type{
                    DrawType::Common=>self.draw_textured_object(
                        index,
                        &draw_parameters
                    ),

                    DrawType::Shifting(shift)=>self.draw_shift_textured_object(
                        index,
                        shift,
                        &draw_parameters
                    ),

                    DrawType::Rotating((angle,position))=>self.draw_rotate_textured_object(
                        index,
                        position,
                        angle,
                        &draw_parameters
                    )
                }
            }

            #[cfg(feature="text_graphics")]
            ObjectType::Text=>{
                match draw_type{
                    DrawType::Common=>self.draw_text_object(
                        index,
                        &draw_parameters
                    ),

                    DrawType::Shifting(shift)=>self.draw_shift_text_object(
                        index,
                        shift,
                        &draw_parameters
                    ),

                    DrawType::Rotating((angle,position))=>self.draw_rotate_text_object(
                        index,
                        position,
                        angle,
                        &draw_parameters
                    ),
                }
            }

            #[cfg(not(all(feature="simple_graphics",feature="texture_graphics",feature="text_graphics")))]
            _=>Ok(())
        }
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
        self.graphics2d.simple.draw(
            object,
            draw_parameters,
            self.frame
        )
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
        self.graphics2d.simple.draw_shift(
            object,
            shift,
            draw_parameters,
            self.frame
        )
    }

    /// Рисует повёрнутый простой объект.
    /// 
    /// Draws the rotated simple object.
    #[inline(always)]
    pub fn draw_rotate_simple<'o,O,V,I>(
        &mut self,
        object:&'o O,
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
    /// Draws the simple object.
    #[inline(always)]
    pub fn draw_simple_general<'o,O,V,I>(
        &mut self,
        object:&'o O,
        draw_type:DrawType,
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
        match draw_type{
            DrawType::Common=>self.graphics2d.simple.draw(
                object,
                draw_parameters,
                self.frame
            ),

            DrawType::Shifting(shift)=>self.graphics2d.simple.draw_shift(
                object,
                shift,
                draw_parameters,
                self.frame
            ),

            DrawType::Rotating((angle,position))=>self.graphics2d.simple.draw_rotate(
                object,
                position,
                angle,
                draw_parameters,
                self.frame
            ),
        }
    }
}



/// # Функции для отрисовки изображений. Image rendering functions.
#[cfg(feature="texture_graphics")]
impl<'graphics,'frame> Graphics<'graphics,'frame>{
    /// Рисует изображение.
    /// 
    /// Draws an image.
    #[inline(always)]
    pub fn draw_image(
        &mut self,
        image_base:&ImageBase,
        texture:&Texture,
        draw_parameters:&DrawParameters
    )->Result<(),DrawError>{
        self.graphics2d.texture.draw(image_base,texture,draw_parameters,self.frame)
    }

    /// Рисует сдвинутое изображение.
    /// 
    /// Draws a shifted image.
    #[inline(always)]
    pub fn draw_shift_image(
        &mut self,
        image_base:&ImageBase,
        texture:&Texture,
        shift:[f32;2],
        draw_parameters:&DrawParameters
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
    /// Draws a rotated image.
    /// 
    /// angle - radians
    #[inline(always)]
    pub fn draw_rotate_image<'o>(
        &mut self,
        image_base:&'o ImageBase,
        texture:&Texture,
        rotation_center:[f32;2],
        angle:f32,
        draw_parameters:&DrawParameters
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

    /// Рисует изображение.
    /// 
    /// Draws an image.
    #[inline(always)]
    pub fn draw_image_general(
        &mut self,
        image_base:&ImageBase,
        texture:&Texture,
        draw_type:DrawType,
        draw_parameters:&DrawParameters
    )->Result<(),DrawError>{
        match draw_type{
            DrawType::Common=>self.graphics2d.texture.draw(
                image_base,
                texture,
                draw_parameters,
                self.frame
            ),

            DrawType::Shifting(shift)=>self.graphics2d.texture.draw_shift(
                image_base,
                texture,
                shift,
                draw_parameters,
                self.frame
            ),

            DrawType::Rotating((angle,position))=>self.graphics2d.texture.draw_rotate(
                image_base,
                texture,
                position,
                angle,
                draw_parameters,
                self.frame
            )
        }
    }
}



/// # Фукнции для отрисовка символов. Rendering text functions.
#[cfg(feature="text_graphics")]
impl<'graphics,'frame> Graphics<'graphics,'frame>{
    /// Выводит уже готовый символ.
    /// 
    /// Draws an already built glyph.
    #[inline(always)]
    pub fn draw_glyph(
        &mut self,
        glyph:PositionedGlyph,
        colour:Colour,
        draw_parameters:&DrawParameters,
    )->Result<(),DrawError>{
        self.graphics2d.text.draw_glyph(
            glyph,
            colour,
            draw_parameters,
            self.frame
        )
    }

    /// Выводит сдвинутый, уже готовый символ.
    /// 
    /// Draws a shifted, already built glyph.
    #[inline(always)]
    pub fn draw_shift_glyph(
        &mut self,
        glyph:PositionedGlyph,
        colour:Colour,
        shift:[f32;2],
        draw_parameters:&DrawParameters,
    )->Result<(),DrawError>{
        self.graphics2d.text.draw_shift_glyph(
            glyph,
            colour,
            shift,
            draw_parameters,
            self.frame
        )
    }

    /// Выводит повёрнутый, уже готовый символ.
    /// 
    /// Draws a rotated, already built glyph.
    #[inline(always)]
    pub fn draw_rotate_glyph(
        &mut self,
        glyph:PositionedGlyph,
        colour:Colour,
        rotation_center:[f32;2],
        angle:f32,
        draw_parameters:&DrawParameters,
    )->Result<(),DrawError>{
        self.graphics2d.text.draw_rotate_glyph(
            glyph,
            colour,
            rotation_center,
            angle,
            draw_parameters,
            self.frame
        )
    }

    /// Выводит уже готовый символ.
    /// 
    /// Draws a already built glyph.
    pub fn draw_glyph_general(
        &mut self,
        glyph:PositionedGlyph,
        colour:Colour,
        draw_type:DrawType,
        draw_parameters:&DrawParameters,
    )->Result<(),DrawError>{
        match draw_type{
            DrawType::Common=>self.graphics2d.text.draw_glyph(
                glyph,colour,
                draw_parameters,
                self.frame
            ),

            DrawType::Shifting(shift)=>self.graphics2d.text.draw_shift_glyph(
                glyph,
                colour,
                shift,
                draw_parameters,
                self.frame
            ),

            DrawType::Rotating((angle,position))=>self.graphics2d.text.draw_rotate_glyph(
                glyph,
                colour,
                position,
                angle,
                draw_parameters,
                self.frame
            ),
        }
    }
}





/// # Функции для работы с сохранёнными простыми объектами. Functions to work with saved simple objects.
#[cfg(feature="simple_graphics")]
impl<'graphics,'frame> Graphics<'graphics,'frame>{
    /// Рисует простой объект.
    /// 
    /// Draws the saved simple object.
    #[inline(always)]
    pub fn draw_simple_object(
        &mut self,
        index:usize,
        draw_parameters:&DrawParameters
    )->Result<(),DrawError>{
        self.graphics2d.simple.draw_object(
            index,
            draw_parameters,
            self.frame
        )
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

    /// Рисует простой объект.
    /// 
    /// Draws the saved simple object.
    #[inline(always)]
    pub fn draw_simple_object_general(
        &mut self,
        index:usize,
        draw_type:DrawType,
        draw_parameters:&DrawParameters
    )->Result<(),DrawError>{
        match draw_type{
            DrawType::Common=>self.draw_simple_object(
                index,
                &draw_parameters
            ),

            DrawType::Shifting(shift)=>self.draw_shift_simple_object(
                index,
                shift,
                &draw_parameters
            ),

            DrawType::Rotating((angle,position))=>self.draw_rotate_simple_object(
                index,
                position,
                angle,
                &draw_parameters
            ),
        }
    }
}



/// # Функции для работы с сохранёнными текстурными объектами. Functions to work with saved textured objects.
#[cfg(feature="texture_graphics")]
impl<'graphics,'frame> Graphics<'graphics,'frame>{
    /// Рисует сохранённый текстурный объект.
    /// 
    /// Draws the saved textured object.
    #[inline(always)]
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

    /// Рисует сдвинутый сохранённый текстурный объект.
    /// 
    /// Draws the shifted saved textured object.
    #[inline(always)]
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

    /// Рисует сохранённый текстурный объект.
    /// 
    /// Draws the saved textured object.
    #[inline(always)]
    pub fn draw_textured_object_general(
        &mut self,
        index:usize,
        draw_type:DrawType,
        draw_parameters:&DrawParameters
    )->Result<(),DrawError>{
        match draw_type{
            DrawType::Common=>self.draw_textured_object(
                index,
                &draw_parameters
            ),

            DrawType::Shifting(shift)=>self.draw_shift_textured_object(
                index,
                shift,
                &draw_parameters
            ),

            DrawType::Rotating((angle,position))=>self.draw_rotate_textured_object(
                index,
                position,
                angle,
                &draw_parameters
            )
        }
    }
}



/// # Функции для работы с сохранёнными текстовыми объектами. Functions to work with saved text objects.
#[cfg(feature="text_graphics")]
impl<'graphics,'frame> Graphics<'graphics,'frame>{
    /// Рисует сохранённый текстовой объект.
    /// 
    /// Draws the saved text object.
    #[inline(always)]
    pub fn draw_text_object(
        &mut self,
        index:usize,
        draw_parameters:&DrawParameters
    )->Result<(),DrawError>{
        self.graphics2d.text.draw_object(
            index,
            draw_parameters,
            self.frame,
        )
    }

    /// Рисует сохранённый текстовой объект.
    /// 
    /// Draws the saved text object.
    #[inline(always)]
    pub fn draw_shift_text_object(
        &mut self,
        index:usize,
        shift:[f32;2],
        draw_parameters:&DrawParameters
    )->Result<(),DrawError>{
        self.graphics2d.text.draw_shift_object(
            index,
            shift,
            draw_parameters,
            self.frame,
        )
    }

    /// Рисует сохранённый текстовой объект.
    /// 
    /// Draws the saved text object.
    #[inline(always)]
    pub fn draw_rotate_text_object(
        &mut self,
        index:usize,
        rotation_center:[f32;2],
        angle:f32,
        draw_parameters:&DrawParameters
    )->Result<(),DrawError>{
        self.graphics2d.text.draw_rotate_object(
            index,
            rotation_center,
            angle,
            draw_parameters,
            self.frame,
        )
    }

    /// Рисует сохранённый текстовой объект.
    /// 
    /// Draws the saved text object.
    pub fn draw_text_object_general(
        &mut self,
        index:usize,
        draw_type:DrawType,
        draw_parameters:&DrawParameters
    )->Result<(),DrawError>{
        match draw_type{
            DrawType::Common=>self.draw_text_object(
                index,
                &draw_parameters
            ),

            DrawType::Shifting(shift)=>self.draw_shift_text_object(
                index,
                shift,
                &draw_parameters
            ),

            DrawType::Rotating((angle,position))=>self.draw_rotate_text_object(
                index,
                position,
                angle,
                &draw_parameters
            ),
        }
    }
}