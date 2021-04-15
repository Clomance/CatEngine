use ab_glyph_rasterizer::{
    Point,
    point,
    Rasterizer,
};

use ttf_parser::{
    Rect,
    Face,
    GlyphId,
    OutlineBuilder,
};

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
    outline:Vec<OutlineCurve>,
}

impl OutlineCurveBuilder{
    pub fn new()->OutlineCurveBuilder{
        Self{
            last:point(0f32,0f32),
            last_move:None,
            outline:Vec::new(),
        }
    }

    pub fn with_capacity(capacity:usize)->OutlineCurveBuilder{
        Self{
            last:point(0f32,0f32),
            last_move:None,
            outline:Vec::with_capacity(capacity),
        }
    }

    pub fn outline_curves(self)->Vec<OutlineCurve>{
        self.outline
    }
}

impl OutlineBuilder for OutlineCurveBuilder{
    fn move_to(&mut self,x:f32,y:f32){
        self.last=point(x,y);
        self.last_move=Some(self.last);
    }

    fn line_to(&mut self,x1:f32,y1:f32){
        let p1=point(x1,y1);
        self.outline.push(OutlineCurve::Line(self.last,p1));
        self.last=p1;
    }

    fn quad_to(&mut self,x1:f32,y1:f32,x2:f32,y2:f32){
        let p1=point(x1,y1);
        let p2=point(x2,y2);
        self.outline.push(OutlineCurve::Quad(self.last,p1,p2));
        self.last=p2;
    }

    fn curve_to(&mut self,x1:f32,y1:f32,x2:f32,y2:f32,x3:f32,y3:f32){
        let p1=point(x1,y1);
        let p2=point(x2,y2);
        let p3=point(x3,y3);

        self.outline
            .push(OutlineCurve::Cubic(self.last, p1, p2, p3));
        self.last = p3;
    }

    fn close(&mut self){
        if let Some(m)=self.last_move{
            self.outline.push(OutlineCurve::Line(self.last,m));
        }
    }
}


/*
    if let Some(bounds)=face.outline_glyph(id,&mut outline_builder){
        // Сдвиг символа относительно глобальной рамки (global bounding box)
        let offset=[
            (bounds.x_min as f32*scale.horizontal),
            (bounds.y_min as f32*scale.vertical),
        ];

        // Размер символа, изображения
        let size=[
            (bounds.width() as f32*scale.horizontal).ceil(),
            (bounds.height() as f32*scale.vertical).ceil(),
        ];

        let glyph=OutlinedGlyph::raw(outline_builder.outline_curves(),[size[0] as u32,size[1] as u32],offset,scale);

        let width=size[0] as usize;
        let height=size[1] as u32;

        let len=width*height as usize;
        let mut image=Vec::with_capacity(len);

        glyph.draw(|_,a|{
            let gray=255f32*a;
            let byte=gray.round() as u8;
            image.push(byte);
        });

        let texture_2d=Texture2D::new(
            TextureInternalFormat::R8,
            TextureFilter::Linear,
            TextureFilter::Linear,
            [width as u32,height],
            ImageDataFormat::Red,
            ImageDataType::U8,
            &image
        );

        let advance_width=face.glyph_hor_advance(id).unwrap() as f32*scale.horizontal;

        let glyph=RawGlyph::<Texture2D>::raw(
            texture_2d,
            size,
            offset,
            advance_width,
        );

        Some(glyph)
    }
*/
pub struct GlyphImageBuilder{
    rasterizer:Rasterizer,
    image_buffer:Vec<u8>,
    offset:[f32;2],
    scale:Scale,
    last:Point,
    last_move:Option<Point>,
}

impl GlyphImageBuilder{
    pub fn new([width,height]:[usize;2])->GlyphImageBuilder{
        Self{
            rasterizer:Rasterizer::new(width,height),
            image_buffer:Vec::with_capacity(width*height),
            offset:[0f32;2],
            scale:Scale::new(1f32,1f32),
            last:point(0f32,0f32),
            last_move:None,
        }
    }

    pub fn set_scale(&mut self,scale:Scale){
        self.scale=scale
    }

    /// [offset_x,offset_y,width,height]
    pub fn build_image(&mut self,glyph_id:GlyphId,font:&Face)->Option<[f32;4]>{
        // Получение размера и положения глифа
        if let Some(bounding_box)=font.glyph_bounding_box(glyph_id){
            let width=(bounding_box.width() as f32*self.scale.horizontal).ceil();
            let height=(bounding_box.height() as f32*self.scale.vertical).ceil();

            // Не масштабируется, потому что вычитается из
            // начальных данных
            self.offset=[
                bounding_box.x_min as f32,
                bounding_box.y_min as f32,
            ];
            // Установка размера под новый глиф
            self.rasterizer.reset(width as usize,height as usize);
            // Отчистка буфера от старого изображения
            self.image_buffer.clear();

            // Построение очертания
            if let Some(_)=font.outline_glyph(glyph_id,self){
                // Перевод очертания в картинку
                let image_buffer=&mut self.image_buffer;
                self.rasterizer.for_each_pixel(|_,pixel|{
                    image_buffer.push((pixel*255f32).round() as u8);
                });

                Some([
                    self.offset[0]*self.scale.horizontal,
                    self.offset[1]*self.scale.vertical,
                    width,
                    height,
                ])
            }
            else{
                None
            }
        }
        else{
            None
        }
    }

    pub fn image(&self)->&[u8]{
        &self.image_buffer
    }

    pub fn dimensions(&self)->[usize;2]{
        let (width,height)=self.rasterizer.dimensions();
        [width,height]
    }
}

impl OutlineBuilder for GlyphImageBuilder{
    fn move_to(&mut self,x:f32,y:f32){
        self.last=point(
            self.scale.horizontal*(x-self.offset[0]),
            self.scale.vertical*(y-self.offset[1])
        );
        self.last_move=Some(self.last);
    }

    fn line_to(&mut self,x1:f32,y1:f32){
        let p1=point(
            self.scale.horizontal*(x1-self.offset[0]),
            self.scale.vertical*(y1-self.offset[1])
        );

        self.rasterizer.draw_line(self.last,p1);

        self.last=p1;
    }

    fn quad_to(&mut self,x1:f32,y1:f32,x2:f32,y2:f32){
        let p1=point(
            self.scale.horizontal*(x1-self.offset[0]),
            self.scale.vertical*(y1-self.offset[1])
        );
        let p2=point(
            self.scale.horizontal*(x2-self.offset[0]),
            self.scale.vertical*(y2-self.offset[1])
        );

        self.rasterizer.draw_quad(self.last,p1,p2);

        self.last=p2;
    }

    fn curve_to(&mut self,x1:f32,y1:f32,x2:f32,y2:f32,x3:f32,y3:f32){
        let p1=point(
            self.scale.horizontal*(x1-self.offset[0]),
            self.scale.vertical*(y1-self.offset[1])
        );
        let p2=point(
            self.scale.horizontal*(x2-self.offset[0]),
            self.scale.vertical*(y2-self.offset[1])
        );
        let p3=point(
            self.scale.horizontal*(x3-self.offset[0]),
            self.scale.vertical*(y3-self.offset[1])
        );

        self.rasterizer.draw_cubic(self.last,p1,p2,p3);

        self.last=p3;
    }

    fn close(&mut self){
        if let Some(m)=self.last_move{
            self.rasterizer.draw_line(self.last,m);
        }
    }
}