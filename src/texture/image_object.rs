use crate::{
    Colour,
    graphics::{DependentObject,TexturedVertex2D},
};

use glium::index::{NoIndices, PrimitiveType};

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
pub struct ImageObject{
    pub x1:f32,
    pub y1:f32,
    pub x2:f32,
    pub y2:f32,

    pub u1:f32,
    pub v1:f32,
    pub u2:f32,
    pub v2:f32,

    pub colour_filter:[f32;4],
}

impl ImageObject{
    pub fn new([x,y,width,height]:[f32;4],[u,v,uwidth,vheight]:[f32;4],colour_filter:[f32;4])->ImageObject{
        Self{
            x1:x,
            y1:y,
            x2:x+width,
            y2:y+height,

            u1:u,
            v1:v,
            u2:u+uwidth,
            v2:v+vheight,

            colour_filter,
        }
    }
}

impl<'o> DependentObject<'o,TexturedVertex2D,u8> for ImageObject{
    type Vertices=[TexturedVertex2D;4];
    type Indices=[u8;1];

    fn colour(&self)->Colour{
        self.colour_filter
    }

    fn vertices(&self)->Self::Vertices{
        [
            TexturedVertex2D::new([self.x1,self.y1],[self.u1,self.v2]),
            TexturedVertex2D::new([self.x1,self.y2],[self.u1,self.v1]),
            TexturedVertex2D::new([self.x2,self.y1],[self.u2,self.v2]),
            TexturedVertex2D::new([self.x2,self.y2],[self.u2,self.v1])
        ]
    }

    fn indices(&self)->Option<Self::Indices>{
        None
    }

    fn primitive_type(&self)->PrimitiveType{
        PrimitiveType::TriangleStrip
    }
}