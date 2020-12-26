use super::{
    graphics::{
        Graphics,
        DependentObject,
        Vertex2D
    },
    Colour,
};

#[cfg(feature="colour_filter")]
use super::graphics::ColourFilter;

use glium::{
    DrawError,
    DrawParameters,
    index::PrimitiveType,
};

#[derive(Clone)]
pub struct Quadrilateral{
    pub vertices:[Vertex2D;4],
    pub colour:Colour,
}

impl Quadrilateral{
    pub fn new(vertices:[Vertex2D;4],colour:Colour)->Quadrilateral{
        Self{
            vertices,
            colour
        }
    }

    pub fn draw(
        &self,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        graphics.draw_simple(
            self,
            #[cfg(feature="colour_filter")]colour_filter,
        )
    }

    pub fn draw_rotate(
        &self,
        rotation_center:[f32;2],
        angle:f32,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        graphics.draw_rotate_simple(
            self,
            rotation_center,
            angle,
            #[cfg(feature="colour_filter")]colour_filter,
        )
    }

    pub fn draw_shift(
        &self,
        shift:[f32;2],
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        graphics.draw_shift_simple(
            self,
            shift,
            #[cfg(feature="colour_filter")]colour_filter,
        )
    }
}

impl DependentObject<Vertex2D,u8> for Quadrilateral{
    type Vertices=[Vertex2D;4];
    type Indices=[u8;0];

    fn colour(&self)->Colour{
        self.colour
    }

    fn vertices(&self)->[Vertex2D;4]{
        self.vertices
    }

    fn indices(&self)->Option<[u8;0]>{
        None
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

    pub fn draw(
        &self,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        graphics.draw_simple(
            self,
            #[cfg(feature="colour_filter")]colour_filter,
        )
    }

    pub fn draw_rotate(
        &self,
        rotation_center:[f32;2],
        angle:f32,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        graphics.draw_rotate_simple(
            self,
            rotation_center,
            angle,
            #[cfg(feature="colour_filter")]colour_filter,
        )
    }

    pub fn draw_shift(
        &self,
        shift:[f32;2],
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        graphics.draw_shift_simple(
            self,
            shift,
            #[cfg(feature="colour_filter")]colour_filter,
        )
    }
}

impl DependentObject<Vertex2D,u8> for Rectangle{
    type Vertices=[Vertex2D;4];
    type Indices=[u8;0];

    fn colour(&self)->Colour{
        self.colour
    }

    fn vertices(&self)->[Vertex2D;4]{
        [
            Vertex2D::new(self.x1,self.y1),
            Vertex2D::new(self.x1,self.y2),
            Vertex2D::new(self.x2,self.y1),
            Vertex2D::new(self.x2,self.y2),
        ]
    }

    fn indices(&self)->Option<[u8;0]>{
        None
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
    pub width:f32,
    pub colour:Colour,
}

impl RectangleBorder{
    /// rect - [x1, y1, x2, y2]
    pub const fn raw(rect:[f32;4],width:f32,colour:Colour)->RectangleBorder{
        Self{
            x1:rect[0],
            y1:rect[1],
            x2:rect[2],
            y2:rect[3],
            width,
            colour
        }
    }

    /// Converts a rectanlge to border.
    pub fn from_rectangle(rect:Rectangle,width:f32)->RectangleBorder{
        Self{
            x1:rect.x1,
            y1:rect.y1,
            x2:rect.x2,
            y2:rect.y2,
            width,
            colour:rect.colour
        }
    }

    pub fn rectangle_base(rect:Rectangle,width:f32,colour:Colour)->RectangleBorder{
        Self{
            x1:rect.x1,
            y1:rect.y1,
            x2:rect.x2,
            y2:rect.y2,
            width,
            colour
        }
    }

    pub fn draw(
        &self,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        graphics.draw_parameters.line_width=Some(self.width);
        graphics.draw_simple(
            self,
            #[cfg(feature="colour_filter")]colour_filter,
        )
    }

    pub fn draw_rotate(
        &self,
        rotation_center:[f32;2],
        angle:f32,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        graphics.draw_parameters.line_width=Some(self.width);
        graphics.draw_rotate_simple(
            self,
            rotation_center,
            angle,
            #[cfg(feature="colour_filter")]colour_filter,
        )
    }

    pub fn draw_shift(
        &self,
        shift:[f32;2],
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        graphics.draw_parameters.line_width=Some(self.width);
        graphics.draw_shift_simple(
            self,
            shift,
            #[cfg(feature="colour_filter")]colour_filter,
        )
    }
}

impl DependentObject<Vertex2D,u8> for RectangleBorder{
    type Vertices=[Vertex2D;4];
    type Indices=[u8;0];

    fn colour(&self)->Colour{
        self.colour
    }

    fn vertices(&self)->[Vertex2D;4]{
        [
            Vertex2D::new(self.x1,self.y1),
            Vertex2D::new(self.x1,self.y2),
            Vertex2D::new(self.x2,self.y2),
            Vertex2D::new(self.x2,self.y1),
        ]
    }

    fn indices(&self)->Option<[u8;0]>{
        None
    }

    fn primitive_type(&self)->PrimitiveType{
        PrimitiveType::LineLoop
    }
}

#[derive(Clone)]
pub struct RectangleWithBorder{
    pub rect:Rectangle,
    pub border_width:f32,
    pub border_colour:Colour,
}

impl RectangleWithBorder{
    /// rect - [x1, y1, width, height]
    pub fn new(rect:[f32;4],colour:Colour)->RectangleWithBorder{
        Self{
            rect:Rectangle::new(rect,colour),
            border_width:1f32,
            border_colour:colour,
        }
    }

    /// rect - [x1, y1, x2, y2]
    pub const fn raw(rect:[f32;4],colour:Colour,width:f32,border_colour:Colour)->RectangleWithBorder{
        Self{
            rect:Rectangle::raw(rect,colour),
            border_width:width,
            border_colour,
        }
    }

    pub const fn border(mut self,width:f32,colour:Colour)->RectangleWithBorder{
        self.border_width=width;
        self.border_colour=colour;
        self
    }

    pub fn draw(
        &self,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        self.rect.draw(
            #[cfg(feature="colour_filter")]colour_filter,
            graphics
        )?;
        let border=RectangleBorder::rectangle_base(self.rect.clone(),self.border_width,self.border_colour);
        border.draw(
            #[cfg(feature="colour_filter")]colour_filter,
            graphics
        )
    }

    pub fn draw_rotate(
        &self,
        rotation_center:[f32;2],
        angle:f32,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        self.rect.draw_rotate(
            rotation_center,
            angle,
            #[cfg(feature="colour_filter")]colour_filter,
            graphics
        )?;
        let border=RectangleBorder::rectangle_base(self.rect.clone(),self.border_width,self.border_colour);
        border.draw_rotate(
            rotation_center,
            angle,
            #[cfg(feature="colour_filter")]colour_filter,
            graphics
        )
    }

    pub fn draw_shift(
        &self,
        shift:[f32;2],
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        self.rect.draw_shift(
            shift,
            #[cfg(feature="colour_filter")]colour_filter,
            graphics
        )?;
        let border=RectangleBorder::rectangle_base(self.rect.clone(),self.border_width,self.border_colour);
        border.draw_shift(
            shift,
            #[cfg(feature="colour_filter")]colour_filter,
            graphics
        )
    }
}

pub struct Line{
    pub x1:f32,
    pub y1:f32,
    pub x2:f32,
    pub y2:f32,
    pub radius:f32,
    pub colour:Colour,
}

impl Line{
    /// rect - [x1, y1, x2, y2]
    pub const fn new(rect:[f32;4],radius:f32,colour:Colour)->Line{
        Self{
            x1:rect[0],
            y1:rect[1],
            x2:rect[2],
            y2:rect[3],
            radius,
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

    pub fn draw(
        &self,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        graphics.draw_parameters.line_width=Some(self.radius);
        graphics.draw_simple(
            self,
            #[cfg(feature="colour_filter")]colour_filter,
        )
    }

    pub fn draw_rotate(
        &self,
        rotation_center:[f32;2],
        angle:f32,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        graphics.draw_parameters.line_width=Some(self.radius);
        graphics.draw_rotate_simple(
            self,
            rotation_center,
            angle,
            #[cfg(feature="colour_filter")]colour_filter,
        )
    }

    pub fn draw_shift(
        &self,
        shift:[f32;2],
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        graphics.draw_parameters.line_width=Some(self.radius);
        graphics.draw_shift_simple(
            self,
            shift,
            #[cfg(feature="colour_filter")]colour_filter,
        )
    }
}


impl <'o> DependentObject<Vertex2D,u8> for Line{
    type Vertices=[Vertex2D;2];
    type Indices=[u8;0];

    fn colour(&self)->Colour{
        self.colour
    }

    fn vertices(&self)->[Vertex2D;2]{
        [
            Vertex2D::new(self.x1,self.y1),
            Vertex2D::new(self.x2,self.y2)
        ]
    }

    fn indices(&self)->Option<[u8;0]>{
        None
    }

    fn primitive_type(&self)->PrimitiveType{
        PrimitiveType::LinesList
    }
}

const ellipse_points:usize=15; // Количество точек для эллипса

pub struct Circle{
    pub x:f32,
    pub y:f32,
    pub radius:f32,
    pub colour:Colour,
}

impl Circle{
    /// circle - [x, y, radius]
    pub const fn new(circle:[f32;3],colour:Colour)->Circle{
        Self{
            x:circle[0],
            y:circle[1],
            radius:circle[2],
            colour
        }
    }

    pub fn draw(
        &self,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        graphics.draw_simple(
            self,
            #[cfg(feature="colour_filter")]colour_filter,
        )
    }

    pub fn draw_rotate(
        &self,
        rotation_center:[f32;2],
        angle:f32,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        graphics.draw_rotate_simple(
            self,
            rotation_center,
            angle,
            #[cfg(feature="colour_filter")]colour_filter,
        )
    }

    pub fn draw_shift(
        &self,
        shift:[f32;2],
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        graphics.draw_shift_simple(
            self,
            shift,
            #[cfg(feature="colour_filter")]colour_filter,
        )
    }
}

impl <'o> DependentObject<Vertex2D,u8> for Circle{
    type Vertices=Vec<Vertex2D>;
    type Indices=[u8;0];

    fn colour(&self)->Colour{
        self.colour
    }

    fn vertices(&self)->Vec<Vertex2D>{
        let r_x=self.radius;
        let r_y=self.radius;

        let c_x=self.x;
        let c_y=self.y;

        let mut shape=vec![Vertex2D{position:[c_x,c_y]};4*ellipse_points+2];

        let dx=r_x/ellipse_points as f32;
        let mut x=dx;

        for c in 1..ellipse_points{
            let y=((r_x-x)*(r_x+x)).sqrt();
            
            shape[c].position=[c_x+x,c_y+y];

            shape[2*ellipse_points-c].position=[c_x+x,c_y-y];

            shape[2*ellipse_points+c].position=[c_x-x,c_y-y];

            shape[4*ellipse_points-c].position=[c_x-x,c_y+y];

            x+=dx;
        }

        shape[1].position=[c_x,c_y+r_y];
        shape[ellipse_points].position=[c_x+r_x,c_y];
        shape[2*ellipse_points].position=[c_x,c_y-r_y];
        shape[3*ellipse_points].position=[c_x-r_x,c_y];
        shape[4*ellipse_points].position=[c_x,c_y+r_y];

        shape
    }

    fn indices(&self)->Option<[u8;0]>{
        None
    }

    fn primitive_type(&self)->PrimitiveType{
        PrimitiveType::TriangleFan
    }
}