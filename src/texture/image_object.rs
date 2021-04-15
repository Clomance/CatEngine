use crate::{
    Colour,
    graphics::{
        DependentObject,
        TexturedVertex2D,
        PrimitiveType,
        ElementIndexType,
    },
};

#[derive(Clone)]
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

    pub fn raw([x1,y1,x2,y2]:[f32;4],[u1,v1,u2,v2]:[f32;4],colour_filter:[f32;4])->ImageObject{
        Self{
            x1,
            y1,
            x2,
            y2,

            u1,
            v1,
            u2,
            v2,

            colour_filter,
        }
    }

    pub fn raw_uv([x,y,width,height]:[f32;4],[u1,v1,u2,v2]:[f32;4],colour_filter:[f32;4])->ImageObject{
        Self{
            x1:x,
            y1:y,
            x2:x+width,
            y2:y+height,

            u1,
            v1,
            u2,
            v2,

            colour_filter,
        }
    }

    pub fn set_rect(&mut self,[x,y,width,height]:[f32;4]){
        self.x1=x;
        self.y1=y;
        self.x2=x+width;
        self.y2=y+height;
    }

    pub fn set_rect_uv(&mut self,[u,v,uwidth,vheight]:[f32;4]){
        self.u1=u;
        self.v1=v;
        self.u2=u+uwidth;
        self.v2=v+vheight;
    }

    pub fn set_raw_uv(&mut self,[u1,v1,u2,v2]:[f32;4]){
        self.u1=u1;
        self.v1=v1;
        self.u2=u2;
        self.v2=v2;
    }

    pub fn set_new(&mut self,[x,y,width,height]:[f32;4],[u,v,uwidth,vheight]:[f32;4],colour_filter:[f32;4]){
        self.x1=x;
        self.y1=y;
        self.x2=x+width;
        self.y2=y+height;

        self.u1=u;
        self.v1=v;
        self.u2=u+uwidth;
        self.v2=v+vheight;

        self.colour_filter=colour_filter
    }

    pub fn set_new_raw_uv(&mut self,[x,y,width,height]:[f32;4],[u1,v1,u2,v2]:[f32;4],colour_filter:[f32;4]){
        self.x1=x;
        self.y1=y;
        self.x2=x+width;
        self.y2=y+height;

        self.u1=u1;
        self.v1=v1;
        self.u2=u2;
        self.v2=v2;

        self.colour_filter=colour_filter
    }
}

impl DependentObject<TexturedVertex2D,ElementIndexType> for ImageObject{
    type Vertices=[TexturedVertex2D;4];
    type Indices=[ElementIndexType;0];

    fn vertices(&self)->Self::Vertices{
        [
            TexturedVertex2D::new([self.x1,self.y1],[self.u1,self.v2],self.colour_filter),
            TexturedVertex2D::new([self.x1,self.y2],[self.u1,self.v1],self.colour_filter),
            TexturedVertex2D::new([self.x2,self.y1],[self.u2,self.v2],self.colour_filter),
            TexturedVertex2D::new([self.x2,self.y2],[self.u2,self.v1],self.colour_filter)
        ]
    }

    fn indices(&self)->[ElementIndexType;0]{
        []
    }

    fn primitive_type(&self)->PrimitiveType{
        PrimitiveType::TriangleStrip
    }
}