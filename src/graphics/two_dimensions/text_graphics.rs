use super::TexturedVertex2D;

use crate::{
    // statics
    window_center,
    // types
    Colour,
};

use glium::{
    uniform,
    // enums
    DrawError,
    // traits
    Surface,
    // structs
    Program,
    Frame,
    DrawParameters,
    VertexBuffer,
    Display,
    index::{NoIndices,PrimitiveType},

    texture::{
        RawImage2d,
        UncompressedFloatFormat,
        MipmapsOption,
        Texture2d,
        ClientFormat,
    },
    Rect,
};

use rusttype::PositionedGlyph;

pub struct TextGraphics{
    vertex_buffer:VertexBuffer<TexturedVertex2D>,
    image:Vec<u8>, // Здесь рисуется символ
    texture:Texture2d, // Изображение символа записывается сюда, для рендеринга
    texture_size:[f32;2],
    program:Program,
}

impl TextGraphics{
    pub fn new(display:&Display,[width,height]:[u32;2],glsl:u16)->TextGraphics{
        let (vertex,fragment)=if glsl==120{(
            include_str!("shaders/120/text/vertex_shader.glsl"),
            include_str!("shaders/120/text/fragment_shader.glsl")
        )}
        else{(
            include_str!("shaders/text/vertex_shader.glsl"),
            include_str!("shaders/text/fragment_shader.glsl")
        )};
        Self{
            vertex_buffer:VertexBuffer::empty_dynamic(display,4).unwrap(),
            image:Vec::<u8>::with_capacity((width*height) as usize),
            texture:Texture2d::empty_with_format(
                display,
                UncompressedFloatFormat::U8,
                MipmapsOption::NoMipmap,
                width,
                height
            ).unwrap(),
            texture_size:[width as f32,height as f32],
            program:Program::from_source(display,vertex,fragment,None).unwrap()
        }
    }

    pub fn draw_glyph(
        &mut self,
        glyph:PositionedGlyph,
        colour:Colour,
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        if let Some(bounding_box)=glyph.pixel_bounding_box(){
            let x1=bounding_box.min.x as f32;
            let y1=bounding_box.min.y as f32;
            let x2=bounding_box.max.x as f32;
            let y2=bounding_box.max.y as f32;

            let width=bounding_box.width() as u32;
            let height=bounding_box.height() as u32;

            glyph.draw(|_,_,a|{
                let gray=255f32*a;

                self.image.push(gray.round() as u8);
            });

            let rect=Rect{
                left:0,
                bottom:0,
                width,
                height,
            };

            let raw_image=RawImage2d{
                data:self.image
                    .chunks(width as usize)
                    .rev()
                    .flat_map(|row|row.iter())
                    .map(|p|p.clone())
                    .collect(),
                width,
                height,
                format:ClientFormat::U8,
            };

            self.texture.write(rect,raw_image);

            self.image.clear();
            let slice=self.vertex_buffer.slice(0..4).unwrap();

            // размер изображения символа
            let uwidth=width as f32/self.texture_size[0];
            let uheight=height as f32/self.texture_size[1];


            let vertices=[
                TexturedVertex2D::new([x1,y1],[0.0,uheight]),
                TexturedVertex2D::new([x1,y2],[0.0,0.0]),
                TexturedVertex2D::new([x2,y1],[uwidth,uheight]),
                TexturedVertex2D::new([x2,y2],[uwidth,0.0])
            ];

            slice.write(&vertices);

            let uni=uniform!{
                texture2d:&self.texture,
                colour:colour,
                window_center:unsafe{window_center}
            };

            frame.draw(
                slice,
                NoIndices(PrimitiveType::TriangleStrip),
                &self.program,
                &uni,
                draw_parameters
            )?
        }

        Ok(())
    }
}