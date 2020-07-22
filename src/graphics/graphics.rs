use crate::Colour;

#[cfg(feature="texture_graphics")]
use crate::image::{ImageBase,Texture};
#[cfg(feature="texture_graphics")]
use super::TextureGraphics;

#[cfg(feature="simple_graphics")]
use super::{SimpleGraphics,SimpleObject,SimpleGraphicsSettings};

#[cfg(feature="text_graphics")]
use crate::text::Character;
#[cfg(feature="text_graphics")]
use super::TextGraphics;

use glium::{
    // enums
    DrawError,
    // traits
    Surface,
    // structs
    Frame,
    DrawParameters,
    Display,
    index::{
        PrimitiveType, // enum
        NoIndices,
    },
};

use core::ops::Range;

/// Настройки графических основ.
/// 
/// Settings for graphic basics.
#[derive(Clone,Debug)]
pub struct GraphicsSettings{
    /// The default is 8.
    /// 
    /// feature = "texture_graphics"
    #[cfg(feature="texture_graphics")]
    pub texture_vertex_buffer_size:usize,

    /// feature = "simple_graphics"
    #[cfg(feature="simple_graphics")]
    pub simple:SimpleGraphicsSettings,

    /// The default is 2000.
    /// 
    /// feature = "text_graphics"
    #[cfg(feature="text_graphics")]
    pub text_vertex_buffer_size:usize,
}

impl GraphicsSettings{
    pub const fn new()->GraphicsSettings{
        Self{
            #[cfg(feature="texture_graphics")]
            texture_vertex_buffer_size:8usize,
            #[cfg(feature="simple_graphics")]
            simple:SimpleGraphicsSettings::new(),
            #[cfg(feature="text_graphics")]
            text_vertex_buffer_size:2000usize,
        }
    }
}



/// Графическая основа. A graphics base.
/// 
/// # Области. Ranges.
/// 
/// Вы можете выбрать область в буфере вершин и сохранить туда вершины объектов.
/// Это помогает ускорить процесс отрисовки неменяющихся объектов.
/// 
/// You can choose a range of the vertex buffer and save there vertexes of objects.
/// It speeds up drawing unchanging objects.
pub struct Graphics2D{
    #[cfg(feature="texture_graphics")]
    texture:TextureGraphics,
    #[cfg(feature="simple_graphics")]
    simple:SimpleGraphics,
    #[cfg(feature="text_graphics")]
    text:TextGraphics,
}

impl Graphics2D{
    pub (crate) fn new(window:&Display,settings:GraphicsSettings,glsl:u16)->Graphics2D{
        Self{
            #[cfg(feature="texture_graphics")]
            texture:TextureGraphics::new(window,settings.texture_vertex_buffer_size,glsl),
            #[cfg(feature="simple_graphics")]
            simple:SimpleGraphics::new(window,settings.simple,glsl),
            #[cfg(feature="text_graphics")]
            text:TextGraphics::new(window,settings.text_vertex_buffer_size,glsl),
        }
    }

    /// Сохраняет координаты картинки в выбранной области в буфере.
    /// Возращает номер области, если она не выходит за границы буфера.
    /// 
    /// Только для невращающихся изображений.
    /// 
    /// Для вывода изображения из этой области используется функция 'draw_range_image'.
    /// 
    /// Saves vertexes of the image to the given range of the vertex buffer.
    /// Returns the index of the range.
    /// 
    /// Only for non-rotating images.
    /// 
    /// Use 'draw_range_image' for drawing.
    #[cfg(feature="texture_graphics")]
    pub fn bind_image(&mut self,range:Range<usize>,image_base:ImageBase)->Option<usize>{
        let data=image_base.vertex_buffer();
        self.texture.bind_range(range,&data)
    }

    /// Сохраняет координаты картинки в выбранной области в буфере.
    /// Возращает номер области, если она не выходит за границы буфера.
    /// 
    /// Только для вращающихся изображений.
    /// 
    /// Для вывода изображения из этой области используется функция 'draw_rotate_range_image'.
    /// 
    /// Saves vertexes of the image to the given range of the vertex buffer.
    /// Returns the index of the range.
    /// 
    /// Only for rotating images.
    /// 
    /// Use 'draw_rotate_range_image' for drawing.
    #[cfg(feature="texture_graphics")]
    pub fn bind_rotating_image(&mut self,range:Range<usize>,image_base:ImageBase)->Option<usize>{
        let data=image_base.rotation_vertex_buffer();
        self.texture.bind_range(range,&data)
    }

    /// Обновляет значения области массива для текстур.
    /// 
    /// Только для невращающихся изображений.
    /// 
    /// Rewrites the range with new ImageBase.
    /// 
    /// Only for non-rotating images.
    #[cfg(feature="texture_graphics")]
    pub fn rewrite_range_image(&mut self,range:usize,image_base:ImageBase)->Option<()>{
        let data=image_base.vertex_buffer();
        self.texture.rewrite_range(range,&data)
    }

    /// Обновляет значения области массива для текстур.
    /// 
    /// Только для вращающихся изображений.
    /// 
    /// Rewrites the range with new ImageBase.
    /// 
    /// Only for rotating images.
    #[cfg(feature="texture_graphics")]
    pub fn rewrite_range_rotating_image(&mut self,range:usize,image_base:ImageBase)->Option<()>{
        let data=image_base.rotation_vertex_buffer();
        self.texture.rewrite_range(range,&data)
    }


    /// Удаляет и возращает последюю область из массива областей текстур.
    /// 
    /// Removes the last range from the range buffer of textures.
    #[inline(always)]
    #[cfg(feature="texture_graphics")]
    pub fn pop_texture(&mut self)->Option<Range<usize>>{
        self.texture.pop_range()
    }


    /// Удаляет область из массива областей текстур.
    /// 
    /// Removes the range from the range buffer of textures.
    #[inline(always)]
    #[cfg(feature="texture_graphics")]
    pub fn unbind_texture(&mut self,index:usize){
        self.texture.unbind(index)
    }


    #[cfg(feature="texture_graphics")]
    fn draw_range_image(
        &self,
        index:usize,
        texture:&Texture,
        colour_filter:Colour,
        draw_parameters:&mut DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        let indices=NoIndices(PrimitiveType::TriangleStrip);
        self.texture.draw_range(
            index,
            texture,
            colour_filter,
            indices,
            draw_parameters,
            frame
        )
    }

    #[cfg(feature="texture_graphics")]
    fn draw_shift_range_image(
        &self,
        index:usize,
        texture:&Texture,
        colour_filter:Colour,
        shift:[f32;2],
        draw_parameters:&mut DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        let indices=NoIndices(PrimitiveType::TriangleStrip);
        self.texture.draw_shift_range(
            index,
            texture,
            colour_filter,
            shift,
            indices,
            draw_parameters,
            frame
        )
    }

    #[cfg(feature="texture_graphics")]
    fn draw_rotate_range_image(
        &self,
        index:usize,
        texture:&Texture,
        colour_filter:Colour,
        rotation_center:[f32;2],
        angle:f32,
        draw_parameters:&mut DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        let indices=NoIndices(PrimitiveType::TriangleStrip);
        self.texture.draw_rotate_range(
            index,
            texture,
            colour_filter,
            rotation_center,
            angle,
            indices,
            draw_parameters,
            frame
        )
    }
}

/// # Функции для работы с объектами. Functions to work with objects.
impl Graphics2D{
    #[cfg(feature="simple_graphics")]
    #[inline(always)]
    pub fn add_plain_object<O:SimpleObject>(&mut self,object:&O)->Option<usize>{
        self.simple.push_object(object)
    }

    #[cfg(feature="simple_graphics")]
    #[inline(always)]
    pub fn clear_plain_object_array(&mut self){
        self.simple.clear_object_array()
    }
}

/// Простой интерфейс для связи кадра и графических функций.
/// Simple interface to connect graphic fuctions to the frame.
pub struct Graphics<'graphics,'frame>{
    graphics:&'graphics Graphics2D,
    frame:&'frame mut Frame,
}

impl<'graphics,'frame> Graphics<'graphics,'frame>{
    #[inline(always)]
    pub (crate) fn new(graphics:&'graphics Graphics2D,frame:&'frame mut Frame)->Graphics<'graphics,'frame>{
        Self{
            graphics,
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

    #[inline(always)]
    pub fn clear_colour(&mut self,colour:[f32;4]){
        self.frame.clear_color(colour[0],colour[1],colour[2],colour[3]);
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
        self.graphics.text.draw_character(character,colour,draw_parameters,self.frame)
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
        self.graphics.texture.draw_image(image_base,texture,draw_parameters,self.frame)
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
        self.graphics.texture.draw_shift_image(image_base,texture,shift,draw_parameters,self.frame)
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
        self.graphics.texture.draw_rotate_image(
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
        self.graphics.draw_range_image(
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
        self.graphics.draw_shift_range_image(
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
        self.graphics.draw_rotate_range_image(
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
        draw_parameters:&mut DrawParameters
    )->Result<(),DrawError>{
        self.graphics.simple.draw(object,draw_parameters,self.frame)
    }

    /// Рисует сдвинутый простой объект.
    /// 
    /// Draws shifted simple object.
    #[inline(always)] 
    pub fn draw_shift_simple<O:SimpleObject>(
        &mut self,
        object:&O,
        shift:[f32;2],
        draw_parameters:&mut DrawParameters
    )->Result<(),DrawError>{
        self.graphics.simple.draw_shift(object,shift,draw_parameters,self.frame)
    }

    /// Рисует повёрнутый простой объект.
    /// 
    /// Draws the rotated simple object.
    #[inline(always)]
    pub fn draw_rotate_simple<O:SimpleObject>(
        &mut self,object:&O,
        rotation_center:[f32;2],
        angle:f32,
        draw_parameters:&mut DrawParameters
    )->Result<(),DrawError>{
        self.graphics.simple.draw_rotate(
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
    pub fn draw_plain_object(
        &mut self,
        index:usize,
        draw_parameters:&DrawParameters
    )->Result<(),DrawError>{
        self.graphics.simple.draw_object(index,draw_parameters,self.frame)
    }

    /// Рисует сдвинутый простой объект.
    /// 
    /// Draws the shifted simple object.
    #[inline(always)]
    pub fn draw_shift_plain_object(
        &mut self,
        index:usize,
        shift:[f32;2],
        draw_parameters:&DrawParameters
    )->Result<(),DrawError>{
        self.graphics.simple.draw_shift_object(
            index,
            shift,
            draw_parameters,
            self.frame
        )
    }

    /// Рисует повёрнутый простой объект.
    /// 
    /// Draws the rotated simple object.
    #[inline(always)]
    pub fn draw_rotate_plain_object(
        &mut self,
        index:usize,
        rotation_center:[f32;2],
        angle:f32,
        draw_parameters:&DrawParameters
    )->Result<(),DrawError>{
        self.graphics.simple.draw_rotate_object(
            index,
            rotation_center,
            angle,
            draw_parameters,
            self.frame
        )
    }
}