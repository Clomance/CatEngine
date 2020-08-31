//! # Основы работы с изображениями. Image basics.

use super::{
    // types
    Colour,
    // structs
    graphics::{
        Graphics,
        TexturedVertex2D,
        DependentObject,
    },
};

mod texture;
pub use texture::{Texture,TextureCreationResult};

use glium::{
    DrawError,
    draw_parameters::DrawParameters,
    index::PrimitiveType,
};

/// Основа для изображений. Image base.
/// 
/// Прямоугольник с вершинами: (x1, y1), (x1, y2), (x2, y1), (x2, y2).
/// 
/// Цветовой фильтр - [red, green, blue, alpha].
/// Цвет = цвет * фильтр.
/// 
/// #
/// 
/// A rectangle with vertices: (x1, y1), (x1, y2), (x2, y1), (x2, y2).
/// 
/// Colour filter - [red, green, blue, alpha].
/// Colour = colour * filter.
#[derive(Clone)]
pub struct ImageBase{
    pub x1:f32,
    pub y1:f32,
    pub x2:f32,
    pub y2:f32,
    pub colour_filter:Colour,
}

impl ImageBase{
    /// rect - [x, y, width, height]
    pub fn new(colour_filter:Colour,rect:[f32;4])->ImageBase{
        Self{
            x1:rect[0],
            y1:rect[1],
            x2:rect[0]+rect[2],
            y2:rect[1]+rect[3],
            colour_filter,
        }
    }

    /// rect - [x, y, width, height]
    pub fn set_rect(&mut self,rect:[f32;4]){
        self.x1=rect[0];
        self.y1=rect[1];
        self.x2=rect[0]+rect[2];
        self.y2=rect[1]+rect[3];
    }

    /// Сдвигает координаты.
    ///
    /// Shifts coordinates.
    pub fn shift(&mut self,[dx,dy]:[f32;2]){
        self.x1+=dx;
        self.y1+=dy;
        self.x2+=dx;
        self.y2+=dy;
    }

    /// Рисует изображение.
    /// 
    /// Draws the image.
    #[cfg(feature="texture_graphics")]
    #[inline(always)]
    pub fn draw(
        &self,
        texture:&Texture,
        draw_parameters:&mut DrawParameters,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        graphics.draw_image(self,texture,draw_parameters)
    }

    /// Рисует сдвинутое изображение.
    /// 
    /// Draws the shifted image.
    /// 
    /// shift - [dx, dy]
    #[cfg(feature="texture_graphics")]
    #[inline(always)] 
    pub fn draw_shift(
        &self,
        texture:&Texture,
        shift:[f32;2],
        draw_parameters:&mut DrawParameters,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        graphics.draw_shift_image(self,texture,shift,draw_parameters)
    }

    /// Рисует изображение под углом.
    /// 
    /// Draws the rotated image.
    /// 
    /// rotation_center - [x, y],
    /// angle - radians
    #[cfg(feature="texture_graphics")]
    #[inline(always)]
    pub fn draw_rotate(
        &self,
        texture:&Texture,
        rotation_center:[f32;2],
        angle:f32,
        draw_parameters:&mut DrawParameters,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        graphics.draw_rotate_image(self,texture,rotation_center,angle,draw_parameters)
    }
}

impl<'o> DependentObject<'o,TexturedVertex2D,u8> for ImageBase{
    type Vertices=[TexturedVertex2D;4];
    type Indices=[u8;1];

    /// Цвет объекта.
    /// 
    /// Object's colour.
    fn colour(&self)->Colour{
        self.colour_filter
    }

    /// Вершины объекта.
    /// 
    /// Object's vertices.
    fn vertices(&self)->Self::Vertices{
        [
            TexturedVertex2D::new([self.x1,self.y1],[0.0,1.0]),
            TexturedVertex2D::new([self.x1,self.y2],[0.0,0.0]),
            TexturedVertex2D::new([self.x2,self.y1],[1.0,1.0]),
            TexturedVertex2D::new([self.x2,self.y2],[1.0,0.0])
        ]
    }

    /// Индексы для построения объекта.
    /// 
    /// Indices to build the object.
    fn indices(&self)->Option<Self::Indices>{
        None
    }

    fn primitive_type(&self)->PrimitiveType{
        PrimitiveType::TriangleStrip
    }
}