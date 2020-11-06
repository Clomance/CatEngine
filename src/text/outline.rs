use ab_glyph_rasterizer::{
    Point,
    point,
    Rasterizer,
};

use ttf_parser::OutlineBuilder;

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

// pub struct GlyphBuilder{
//     raster:Rasterizer,
//     scale:Scale,
//     offset:[f32;2],
// }