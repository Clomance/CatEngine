use crate::{
    // types
    Colour,
    // structs
    graphics::{
        Graphics,
        TexturedVertex2D,
        DependentObject,
        PrimitiveType,
        ElementIndexType,
    },
};

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

    pub fn shift(&mut self,[dx,dy]:[f32;2]){
        self.x1+=dx;
        self.y1+=dy;
        self.x2+=dx;
        self.y2+=dy;
    }
}

impl<'o> DependentObject<TexturedVertex2D,ElementIndexType> for ImageBase{
    type Vertices=[TexturedVertex2D;4];
    type Indices=[ElementIndexType;0];

    fn vertices(&self)->[TexturedVertex2D;4]{
        [
            TexturedVertex2D::new([self.x1,self.y1],[0.0,1.0],self.colour_filter),
            TexturedVertex2D::new([self.x1,self.y2],[0.0,0.0],self.colour_filter),
            TexturedVertex2D::new([self.x2,self.y1],[1.0,1.0],self.colour_filter),
            TexturedVertex2D::new([self.x2,self.y2],[1.0,0.0],self.colour_filter)
        ]
    }

    fn indices(&self)->[ElementIndexType;0]{
        []
    }

    fn primitive_type(&self)->PrimitiveType{
        PrimitiveType::TriangleStrip
    }
}