//! # Основы работы с изображениями. Image basics.
//! 
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

#[cfg(feature="colour_filter")]
use super::graphics::ColourFilter;

mod image_object;
pub use image_object::ImageObject;

mod texture;
pub use texture::{Texture,TextureCreationResult};

use glium::{
    Surface,
    DrawError,
    index::PrimitiveType,
};

/// Основа для изображений. An image base.
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
    pub fn new([x,y,width,height]:[f32;4],colour_filter:Colour)->ImageBase{
        Self{
            x1:x,
            y1:y,
            x2:x+width,
            y2:y+height,
            colour_filter,
        }
    }

    pub fn set_rect(&mut self,[x,y,width,height]:[f32;4]){
        self.x1=x;
        self.y1=y;
        self.x2=x+width;
        self.y2=y+height;
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
    /// Draws an image.
    #[cfg(feature="texture_graphics")]
    #[inline(always)]
    pub fn draw<S:Surface>(
        &self,
        texture:&Texture,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        graphics:&mut Graphics<S>
    )->Result<(),DrawError>{
        graphics.draw_texture(
            self,
            texture,
            #[cfg(feature="colour_filter")]colour_filter
        )
    }

    /// Рисует сдвинутое изображение.
    /// 
    /// Draws the shifted image.
    /// 
    /// shift - [dx, dy]
    #[cfg(feature="texture_graphics")]
    #[inline(always)]
    pub fn draw_shift<S:Surface>(
        &self,
        texture:&Texture,
        shift:[f32;2],
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        graphics:&mut Graphics<S>
    )->Result<(),DrawError>{
        graphics.draw_shift_texture(
            self,
            texture,
            shift,
            #[cfg(feature="colour_filter")]colour_filter
        )
    }

    /// Рисует изображение под углом.
    /// 
    /// Draws the rotated image.
    /// 
    /// rotation_center - [x, y],
    /// angle - radians
    #[cfg(feature="texture_graphics")]
    #[inline(always)]
    pub fn draw_rotate<S:Surface>(
        &self,
        texture:&Texture,
        rotation_center:[f32;2],
        angle:f32,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        graphics:&mut Graphics<S>
    )->Result<(),DrawError>{
        graphics.draw_rotate_texture(
            self,
            texture,
            rotation_center,
            angle,
            #[cfg(feature="colour_filter")]colour_filter
        )
    }
}

impl<'o> DependentObject<TexturedVertex2D,u8> for ImageBase{
    type Vertices=[TexturedVertex2D;4];
    type Indices=[u8;1];

    fn colour(&self)->Colour{
        self.colour_filter
    }

    fn vertices(&self)->Self::Vertices{
        [
            TexturedVertex2D::new([self.x1,self.y1],[0.0,1.0]),
            TexturedVertex2D::new([self.x1,self.y2],[0.0,0.0]),
            TexturedVertex2D::new([self.x2,self.y1],[1.0,1.0]),
            TexturedVertex2D::new([self.x2,self.y2],[1.0,0.0])
        ]
    }

    fn indices(&self)->Option<Self::Indices>{
        None
    }

    fn primitive_type(&self)->PrimitiveType{
        PrimitiveType::TriangleStrip
    }
}