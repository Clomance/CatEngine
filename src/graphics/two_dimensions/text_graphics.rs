use super::{TexturedVertex2D,TextObject2D};

use crate::{
    // statics
    window_center,
    // types
    Colour,
    // structs
    text::TextBase,
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

use rusttype::{
    PositionedGlyph,
    Font,
    Scale,
    Point,
};

use std::borrow::Cow;

pub struct TextGraphics{
    vertex_buffer:VertexBuffer<TexturedVertex2D>,
    image:Vec<u8>, // Здесь рисуется символ
    texture:Texture2d, // Изображение символа записывается сюда, для рендеринга
    texture_size:[f32;2],

    objects:Vec<TextObject2D>,
    fonts:Vec<Font<'static>>,

    draw:Program,
    draw_shift:Program,
    draw_rotate:Program,
}

impl TextGraphics{
    pub fn new(display:&Display,[width,height]:[u32;2],glsl:u16)->TextGraphics{
        let (common,shift,rotation,fragment)=if glsl==120{(
            include_str!("shaders/120/texture/vertex_shader.glsl"),
            include_str!("shaders/120/texture/shift_vertex_shader.glsl"),
            include_str!("shaders/120/texture/rotation_vertex_shader.glsl"),
            include_str!("shaders/120/text/fragment_shader.glsl")
        )}
        else{(
            include_str!("shaders/texture/vertex_shader.glsl"),
            include_str!("shaders/texture/shift_vertex_shader.glsl"),
            include_str!("shaders/texture/rotation_vertex_shader.glsl"),
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

            objects:Vec::with_capacity(10),
            fonts:Vec::with_capacity(10),

            draw:Program::from_source(display,common,fragment,None).unwrap(),
            draw_shift:Program::from_source(display,shift,fragment,None).unwrap(),
            draw_rotate:Program::from_source(display,rotation,fragment,None).unwrap(),
        }
    }

    pub fn write_glyph(
        &mut self,
        glyph:PositionedGlyph,
    )->Option<()>{
        if let Some(bounding_box)=glyph.pixel_bounding_box(){
            let width=bounding_box.width() as usize;
            let height=bounding_box.height() as u32;

            // Из-за неудобной системы построения глифа
            // приходится добавлять всякие конструкции
            // (переворот картинки для openGL)
            {
                let mut len=width*height as usize;
                unsafe{self.image.set_len(len)};

                let mut c=len-width;
                glyph.draw(|_,_,a|{
                    let gray=255f32*a;
                    self.image[c]=gray.round() as u8;
                    c+=1;
                    if c==len && len!=width{
                        len-=width;
                        c-=2*width;
                    }
                });
            }

            let width=width as u32;

            let rect=Rect{
                left:0,
                bottom:0,
                width,
                height,
            };

            let raw_image=RawImage2d{
                data:Cow::Borrowed(&self.image),
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


            let x1=bounding_box.min.x as f32;
            let y1=bounding_box.min.y as f32;
            let x2=bounding_box.max.x as f32;
            let y2=bounding_box.max.y as f32;

            let vertices=[
                TexturedVertex2D::new([x1,y1],[0.0,uheight]),
                TexturedVertex2D::new([x1,y2],[0.0,0.0]),
                TexturedVertex2D::new([x2,y1],[uwidth,uheight]),
                TexturedVertex2D::new([x2,y2],[uwidth,0.0])
            ];

            slice.write(&vertices);

            Some(())
        }
        else{
            None
        }
    }

    pub fn draw_glyph(
        &mut self,
        glyph:PositionedGlyph,
        colour:Colour,
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        if let Some(_)=self.write_glyph(glyph){
            let uni=uniform!{
                texture2d:&self.texture,
                colour:colour,
                window_center:unsafe{window_center}
            };

            frame.draw(
                &self.vertex_buffer,
                NoIndices(PrimitiveType::TriangleStrip),
                &self.draw,
                &uni,
                draw_parameters
            )?
        }

        Ok(())
    }

    pub fn draw_shift_glyph(
        &mut self,
        glyph:PositionedGlyph,
        colour:Colour,
        shift:[f32;2],
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        if let Some(_)=self.write_glyph(glyph){
            let uni=uniform!{
                texture2d:&self.texture,
                colour:colour,
                shift:shift,
                window_center:unsafe{window_center}
            };

            frame.draw(
                &self.vertex_buffer,
                NoIndices(PrimitiveType::TriangleStrip),
                &self.draw_shift,
                &uni,
                draw_parameters
            )?
        }

        Ok(())
    }

    pub fn draw_rotate_glyph(
        &mut self,
        glyph:PositionedGlyph,
        colour:Colour,
        rotation_center:[f32;2],
        angle:f32,
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        if let Some(_)=self.write_glyph(glyph){
            let (sin,cos)=angle.sin_cos();

            let uni=uniform!{
                texture2d:&self.texture,
                colour:colour,
                cos:cos,
                sin:sin,
                rotation_center:rotation_center,
                window_center:unsafe{window_center}
            };

            frame.draw(
                &self.vertex_buffer,
                NoIndices(PrimitiveType::TriangleStrip),
                &self.draw_shift,
                &uni,
                draw_parameters
            )?
        }

        Ok(())
    }
}

// Функции для работы с объектами
impl TextGraphics{
    pub fn push_font(
        &mut self,
        font:Font<'static>,
    )->Option<usize>{
        let index=self.objects.len();

        self.fonts.push(font);

        Some(index)
    }

    pub fn get_font(&self,index:usize)->&Font<'static>{
        &self.fonts[index]
    }

    pub fn push_object(
        &mut self,
        text:String,
        text_base:&TextBase,
        font:usize,
    )->Option<usize>{
        let object=TextObject2D{
            text,
            position:text_base.position,
            font_size:text_base.font_size,
            colour:text_base.colour,
            font,
        };

        let index=self.objects.len();

        self.objects.push(object);

        Some(index)
    }

    pub fn delete_last_object(&mut self){
        self.objects.pop();
    }

    pub fn draw_object(
        &mut self,
        index:usize,
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        // Убрать этот ужас отсюда)
        let object=unsafe{&*(&self.objects[index] as *const TextObject2D)};

        let scale=Scale::uniform(object.font_size);
        // позиция для вывода символа
        let mut point=Point{
            x:object.position[0],
            y:object.position[1]
        };

        let mut width_offset; // сдвиг для следующего символа

        for character in object.text.chars(){
            // Получение символа
            let scaled_glyph=self.fonts[object.font].glyph(character).scaled(scale);

            width_offset=scaled_glyph.h_metrics().advance_width;

            // установка положения символа
            let glyph=scaled_glyph.positioned(point);

            self.draw_glyph(glyph,object.colour,draw_parameters,frame)?;

            point.x+=width_offset;
        }

        Ok(())
    }

    pub fn draw_shift_object(
        &mut self,
        index:usize,
        shift:[f32;2],
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        // Убрать этот ужас отсюда)
        let object=unsafe{&*(&self.objects[index] as *const TextObject2D)};

        let scale=Scale::uniform(object.font_size);
        // позиция для вывода символа
        let mut point=Point{
            x:object.position[0],
            y:object.position[1]
        };

        let mut width_offset; // сдвиг для следующего символа

        for character in object.text.chars(){
            // Получение символа
            let scaled_glyph=self.fonts[object.font].glyph(character).scaled(scale);

            width_offset=scaled_glyph.h_metrics().advance_width;

            // установка положения символа
            let glyph=scaled_glyph.positioned(point);

            self.draw_shift_glyph(glyph,object.colour,shift,draw_parameters,frame)?;

            point.x+=width_offset;
        }

        Ok(())
    }

    pub fn draw_rotate_object(
        &mut self,
        index:usize,
        rotation_center:[f32;2],
        angle:f32,
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        // Убрать этот ужас отсюда)
        let object=unsafe{&*(&self.objects[index] as *const TextObject2D)};

        let scale=Scale::uniform(object.font_size);
        // позиция для вывода символа
        let mut point=Point{
            x:object.position[0],
            y:object.position[1]
        };

        let mut width_offset; // сдвиг для следующего символа

        for character in object.text.chars(){
            // Получение символа
            let scaled_glyph=self.fonts[object.font].glyph(character).scaled(scale);

            width_offset=scaled_glyph.h_metrics().advance_width;

            // установка положения символа
            let glyph=scaled_glyph.positioned(point);

            self.draw_rotate_glyph(glyph,object.colour,rotation_center,angle,draw_parameters,frame)?;

            point.x+=width_offset;
        }

        Ok(())
    }
}