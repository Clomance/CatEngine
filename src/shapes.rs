use super::{
    Colour,
    graphics::{
        ShapeObject,
        SimpleVertex2D,
        PrimitiveType,
        ElementIndexType,
    },
};

#[cfg(feature="colour_filter")]
use super::graphics::ColourFilter;

#[derive(Clone)]
pub struct Quadrilateral{
    pub vertices:[SimpleVertex2D;4],
    pub colour:Colour,
}

impl Quadrilateral{
    pub fn new(vertices:[SimpleVertex2D;4],colour:Colour)->Quadrilateral{
        Self{
            vertices,
            colour
        }
    }
}

impl ShapeObject<SimpleVertex2D,ElementIndexType> for Quadrilateral{
    type Vertices=[SimpleVertex2D;4];
    type Indices=[ElementIndexType;0];

    fn vertices(&self)->[SimpleVertex2D;4]{
        self.vertices.clone()
    }

    fn indices(&self)->[ElementIndexType;0]{
        []
    }

    fn primitive_type(&self)->PrimitiveType{
        PrimitiveType::TriangleStrip
    }
}

#[derive(Clone)]
pub struct Rectangle{
    pub x1:f32,
    pub y1:f32,
    pub x2:f32,
    pub y2:f32,
    pub colour:Colour,
}

impl Rectangle{
    /// rect - [x1, y1, width, height]
    pub fn new(rect:[f32;4],colour:Colour)->Rectangle{
        Self{
            x1:rect[0],
            y1:rect[1],
            x2:rect[0]+rect[2],
            y2:rect[1]+rect[3],
            colour
        }
    }

    /// rect - [x1, y1, x2, y2]
    pub const fn raw(rect:[f32;4],colour:Colour)->Rectangle{
        Self{
            x1:rect[0],
            y1:rect[1],
            x2:rect[2],
            y2:rect[3],
            colour
        }
    }

    pub fn triangles(&self)->[SimpleVertex2D;6]{
        [
            SimpleVertex2D::new([self.x1,self.y1],self.colour),
            SimpleVertex2D::new([self.x1,self.y2],self.colour),
            SimpleVertex2D::new([self.x2,self.y1],self.colour),
            SimpleVertex2D::new([self.x1,self.y2],self.colour),
            SimpleVertex2D::new([self.x2,self.y1],self.colour),
            SimpleVertex2D::new([self.x2,self.y2],self.colour),
        ]
    }
}

impl ShapeObject<SimpleVertex2D,ElementIndexType> for Rectangle{
    type Vertices=[SimpleVertex2D;4];
    type Indices=[ElementIndexType;0];

    fn vertices(&self)->[SimpleVertex2D;4]{
        [
            SimpleVertex2D::new([self.x1,self.y1],self.colour),
            SimpleVertex2D::new([self.x1,self.y2],self.colour),
            SimpleVertex2D::new([self.x2,self.y1],self.colour),
            SimpleVertex2D::new([self.x2,self.y2],self.colour),
        ]
    }

    fn indices(&self)->[ElementIndexType;0]{
        []
    }

    fn primitive_type(&self)->PrimitiveType{
        PrimitiveType::TriangleStrip
    }
}

#[derive(Clone)]
pub struct RectangleBorder{
    pub x1:f32,
    pub y1:f32,
    pub x2:f32,
    pub y2:f32,
    pub colour:Colour,
}

impl RectangleBorder{
    /// rect - [x1, y1, x2, y2]
    pub const fn raw(rect:[f32;4],colour:Colour)->RectangleBorder{
        Self{
            x1:rect[0],
            y1:rect[1],
            x2:rect[2],
            y2:rect[3],
            colour
        }
    }

    /// Converts a rectanlge to border.
    pub fn from_rectangle(rect:Rectangle)->RectangleBorder{
        Self{
            x1:rect.x1,
            y1:rect.y1,
            x2:rect.x2,
            y2:rect.y2,
            colour:rect.colour
        }
    }

    pub fn rectangle_base(rect:Rectangle,colour:Colour)->RectangleBorder{
        Self{
            x1:rect.x1,
            y1:rect.y1,
            x2:rect.x2,
            y2:rect.y2,
            colour
        }
    }
}

impl ShapeObject<SimpleVertex2D,ElementIndexType> for RectangleBorder{
    type Vertices=[SimpleVertex2D;4];
    type Indices=[ElementIndexType;0];

    fn vertices(&self)->[SimpleVertex2D;4]{
        [
            SimpleVertex2D::new([self.x1,self.y1],self.colour),
            SimpleVertex2D::new([self.x1,self.y2],self.colour),
            SimpleVertex2D::new([self.x2,self.y2],self.colour),
            SimpleVertex2D::new([self.x2,self.y1],self.colour),
        ]
    }

    fn indices(&self)->[ElementIndexType;0]{
        []
    }

    fn primitive_type(&self)->PrimitiveType{
        PrimitiveType::LineLoop
    }
}

pub struct Line{
    pub x1:f32,
    pub y1:f32,
    pub x2:f32,
    pub y2:f32,
    pub colour:Colour,
}

impl Line{
    /// rect - [x1, y1, x2, y2]
    pub const fn new(rect:[f32;4],colour:Colour)->Line{
        Self{
            x1:rect[0],
            y1:rect[1],
            x2:rect[2],
            y2:rect[3],
            colour,
        }
    }

    pub fn position(&self)->[f32;4]{
        [
            self.x1,
            self.y1,
            self.x2,
            self.y2,
        ]
    }

    pub fn set_position(&mut self,[x1,y1,x2,y2]:[f32;4]){
        self.x1=x1;
        self.y1=y1;
        self.x2=x2;
        self.y2=y2;
    }

    pub fn shift_y(&mut self,dy:f32){
        self.y1+=dy;
        self.y2+=dy;
    }
}


impl <'o> ShapeObject<SimpleVertex2D,ElementIndexType> for Line{
    type Vertices=[SimpleVertex2D;2];
    type Indices=[ElementIndexType;0];

    fn vertices(&self)->[SimpleVertex2D;2]{
        [
            SimpleVertex2D::new([self.x1,self.y1],self.colour),
            SimpleVertex2D::new([self.x2,self.y2],self.colour)
        ]
    }

    fn indices(&self)->[ElementIndexType;0]{
        []
    }

    fn primitive_type(&self)->PrimitiveType{
        PrimitiveType::Lines
    }
}

// const ellipse_points:usize=15; // Количество точек для эллипса

// pub struct Circle{
//     pub x:f32,
//     pub y:f32,
//     pub radius:f32,
//     pub colour:Colour,
// }

// impl Circle{
//     /// circle - [x, y, radius]
//     pub const fn new(circle:[f32;3],colour:Colour)->Circle{
//         Self{
//             x:circle[0],
//             y:circle[1],
//             radius:circle[2],
//             colour
//         }
//     }
// }

// impl <'o> ShapeObject<SimpleVertex2D,ElementIndexType> for Circle{
//     type Vertices=Vec<SimpleVertex2D>;
//     type Indices=[ElementIndexType;0];

//     fn vertices(&self)->Vec<SimpleVertex2D>{
//         let r_x=self.radius;
//         let r_y=self.radius;

//         let c_x=self.x;
//         let c_y=self.y;

//         let mut shape=vec![SimpleVertex2D::new(position:[c_x,c_y]};4*ellipse_points+2];

//         let dx=r_x/ellipse_points as f32;
//         let mut x=dx;

//         for c in 1..ellipse_points{
//             let y=((r_x-x)*(r_x+x)).sqrt();
            
//             shape[c].position=[c_x+x,c_y+y];

//             shape[2*ellipse_points-c].position=[c_x+x,c_y-y];

//             shape[2*ellipse_points+c].position=[c_x-x,c_y-y];

//             shape[4*ellipse_points-c].position=[c_x-x,c_y+y];

//             x+=dx;
//         }

//         shape[1].position=[c_x,c_y+r_y];
//         shape[ellipse_points].position=[c_x+r_x,c_y];
//         shape[2*ellipse_points].position=[c_x,c_y-r_y];
//         shape[3*ellipse_points].position=[c_x-r_x,c_y];
//         shape[4*ellipse_points].position=[c_x,c_y+r_y];

//         shape
//     }

//     fn indices(&self)->[ElementIndexType;0]{
//         []
//     }

//     fn primitive_type(&self)->PrimitiveType{
//         PrimitiveType::TriangleFan
//     }
// }