use crate::Colour;

use super::GraphicsSettings;

#[cfg(feature="simple_graphics")]
mod simple_graphics;
#[cfg(feature="simple_graphics")]
pub (crate) use simple_graphics::SimpleGraphics;
#[cfg(feature="simple_graphics")]
pub use simple_graphics::{
    SimpleObject,
    Vertex2D,
    SimpleGraphicsSettings,
};

#[cfg(feature="texture_graphics")]
mod texture_graphics;
#[cfg(feature="texture_graphics")]
pub (crate) use texture_graphics::{TextureGraphics,TexturedVertex};
#[cfg(feature="texture_graphics")]
use crate::image::{ImageBase,Texture};

#[cfg(feature="text_graphics")]
mod text_graphics;
#[cfg(feature="text_graphics")]
pub (crate) use text_graphics::TextGraphics;

use glium::{
    // enums
    DrawError,
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
    pub (crate) texture:TextureGraphics,
    #[cfg(feature="simple_graphics")]
    pub (crate) simple:SimpleGraphics,
    #[cfg(feature="text_graphics")]
    pub (crate) text:TextGraphics,
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
    pub (crate) fn draw_range_image(
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
    pub (crate) fn draw_shift_range_image(
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
    pub (crate) fn draw_rotate_range_image(
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
#[cfg(feature="simple_graphics")]
impl Graphics2D{
    /// Добавляет простой объект в массив.
    /// 
    /// Adds the simple object to the array.
    #[inline(always)]
    pub fn add_simple_object<O:SimpleObject>(&mut self,object:&O)->Option<usize>{
        self.simple.push_object(object)
    }

    /// Удаляет последний простой объект в массиве,
    /// если такой есть.
    /// 
    /// Deletes the last simple object of the array
    /// if there is any.
    #[inline(always)]
    pub fn delete_last_simple_object(&mut self){
        self.simple.delete_last_object();
    }

    /// Отчищает массив объектов.
    /// 
    /// Clears the object array.
    #[inline(always)]
    pub fn clear_simple_object_array(&mut self){
        self.simple.clear_object_array()
    }
}