use ab_glyph_rasterizer::{
    Rasterizer,
    Point,
    point,
};

use ttf_parser::OutlineBuilder;

#[derive(Copy,Clone,Debug)]
pub struct Rect{
    pub x:f32,
    pub y:f32,
    pub width:f32,
    pub height:f32,
}

impl Rect {
    pub fn new(x:f32,y:f32,width:f32,height:f32)->Rect{
        Self{
            x,
            y,
            width,
            height
        }
    }
}

#[derive(Clone,Copy,Debug)]
pub struct Scale{
    pub horizontal:f32,
    pub vertical:f32,
}

impl Scale{
    pub fn new(h:f32,v:f32)->Scale{
        Self{
            horizontal:h,
            vertical:v
        }
    }
}

#[derive(Clone,Debug)]
pub struct OutlinedGlyph{
    pub bounds:Rect,
    pub scale:Scale,
    curves:Vec<OutlineCurve>,
}

impl OutlinedGlyph{
    #[inline]
    pub fn new(curves:Vec<OutlineCurve>,bounds:Rect,scale:Scale)->Self{
        Self{
            bounds,
            scale,
            curves,
        }
    }

    pub fn draw<O:FnMut(usize,f32)>(&self,mut o:O){
        let scale_up=|&Point{x,y}|point(
            (x*self.scale.horizontal)-self.bounds.x,
            (y*self.scale.vertical)-self.bounds.y,
        );

        self.curves.iter().fold(
            Rasterizer::new(
                self.bounds.width as usize,
                self.bounds.height as usize
            ),
            |mut rasterizer,curve|match curve{
                OutlineCurve::Line(p0, p1)=>{
                    rasterizer.draw_line(scale_up(p0),scale_up(p1));
                    rasterizer
                }
                OutlineCurve::Quad(p0,p1,p2)=>{
                    rasterizer.draw_quad(
                        scale_up(p0),
                        scale_up(p1),
                        scale_up(p2),
                    );
                    rasterizer
                }
                OutlineCurve::Cubic(p0,p1,p2,p3)=>{
                    rasterizer.draw_cubic(
                        scale_up(p0),
                        scale_up(p1),
                        scale_up(p2),
                        scale_up(p3),
                    );
                    rasterizer
                }
            }
        )
        .for_each_pixel(|c,f|{
            o(c,f)
        });
    }
}


/// Glyph outline primitives.
#[derive(Clone,Debug)]
pub enum OutlineCurve{
    /// Straight line from `.0` to `.1`.
    Line(Point,Point),
    /// Quadratic Bézier curve from `.0` to `.2` using `.1` as the control.
    Quad(Point,Point,Point),
    /// Cubic Bézier curve from `.0` to `.3` using `.1` as the control at the beginning of the
    /// curve and `.2` at the end of the curve.
    Cubic(Point,Point,Point,Point),
}


#[derive(Debug,Default)]
pub (crate) struct OutlineCurveBuilder{
    last:Point,
    last_move:Option<Point>,
    pub outline:Vec<OutlineCurve>,
}

impl OutlineBuilder for OutlineCurveBuilder{
    fn move_to(&mut self, x: f32, y: f32) {
        self.last = point(x, y);
        self.last_move = Some(self.last);
    }

    fn line_to(&mut self, x1: f32, y1: f32) {
        let p1 = point(x1, y1);
        self.outline.push(OutlineCurve::Line(self.last, p1));
        self.last = p1;
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        let p1 = point(x1, y1);
        let p2 = point(x2, y2);
        self.outline.push(OutlineCurve::Quad(self.last, p1, p2));
        self.last = p2;
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        let p1 = point(x1, y1);
        let p2 = point(x2, y2);
        let p3 = point(x3, y3);

        self.outline
            .push(OutlineCurve::Cubic(self.last, p1, p2, p3));
        self.last = p3;
    }

    fn close(&mut self) {
        if let Some(m) = self.last_move {
            self.outline.push(OutlineCurve::Line(self.last, m));
        }
    }
}