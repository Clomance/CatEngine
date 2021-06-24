use crate::{
    Colour,
    text::{
        Scale,
        OutlinedGlyph,
        OutlineCurve,
        GlyphImageBuilder,
    },
};

use super::{
    TextVertex2D,
    DrawParameters,
};

use cat_engine_basement::graphics::{
    level0::{
        VertexArray,
        BufferUsage,
        TextureInternalFormat,
        TextureFilter,
        ImageDataFormat,
    },
    level1::{
        buffer::{
            VertexBuffer,
            IndexBuffer,
        },
        shader::{
            VertexShader,
            FragmentShader
        },
        texture::texture_2d::{
            Texture2D,
        },
    },
    level2::Program,
    gl::{
        // consts
        TEXTURE_2D,
        TEXTURE0,
        UNSIGNED_BYTE,
        TRIANGLE_STRIP,
        TRIANGLES,
        UNPACK_ALIGNMENT,
        // functions
        DrawArrays,
        DrawElements,
        Finish,
        ActiveTexture,
        Uniform4f,
        GetUniformLocation,
        Viewport,
        PixelStorei,
    },
};


use ttf_parser::{
    Rect,
    GlyphId,
    Face,
};
use ab_glyph_rasterizer::{
    Rasterizer,
    Point,
    point
};

use std::cell::UnsafeCell;

pub struct TextGraphicsAttributes{
    // The default is [512;2]
    pub glyph_texture_size:[u32;2]
}

impl TextGraphicsAttributes{
    pub fn new()->TextGraphicsAttributes{
        Self{
            glyph_texture_size:[512u32;2],
        }
    }
}

pub struct TextGraphics{
    vertex_buffer:VertexBuffer<TextVertex2D>,

    vertex_array:VertexArray<TextVertex2D>,

    /// For dynamic building glyphs
    glyph_image_builder:UnsafeCell<GlyphImageBuilder>,
    texture:Texture2D,
    texture_size:[f32;2],

    draw:Program,
}

impl TextGraphics{
    pub fn new(glyph_texture_size:[u32;2])->TextGraphics{
        let vertex_shader=VertexShader::new(&include_str!("shaders/text/vertex_shader.glsl")).unwrap();
        let fragment_shader=FragmentShader::new(&include_str!("shaders/text/fragment_shader.glsl")).unwrap();

        let program=Program::new(&vertex_shader,&fragment_shader).unwrap();
        program.bind_uniform_block("DrawParameters",0u32);

        let vertex_buffer=VertexBuffer::<TextVertex2D>::empty(4,BufferUsage::DynamicDraw);

        let texture=Texture2D::empty(
            TextureInternalFormat::R_U8,
            TextureFilter::Linear,
            TextureFilter::Linear,
            glyph_texture_size
        );
        

        Self{
            vertex_buffer,

            vertex_array:VertexArray::<TextVertex2D>::new(),

            glyph_image_builder:UnsafeCell::new(
                GlyphImageBuilder::new([
                    glyph_texture_size[0] as usize,
                    glyph_texture_size[1] as usize
                ])
            ),
            texture,
            texture_size:[
                glyph_texture_size[0] as f32,
                glyph_texture_size[1] as f32,
            ],

            draw:program,
        }
    }
}


impl TextGraphics{
    fn load_vertices(&self,vertices:&[TextVertex2D;4]){
        self.vertex_buffer.bind().write(0,vertices);
    }

    pub fn load_glyph_image(&self,size:[u32;2],image:&[u8]){
        unsafe{PixelStorei(UNPACK_ALIGNMENT,1)}
        self.texture.bind().write_image([0u32;2],size,ImageDataFormat::R_U8,image);
        unsafe{PixelStorei(UNPACK_ALIGNMENT,4)}
    }

    /// [offset_x,offset_y,width,height]
    pub fn build_glyph_image(&self,glyph_id:GlyphId,scale:Scale,font:&Face)->Option<([f32;4],&[u8])>{
        let glyph_image_builder=unsafe{&mut *self.glyph_image_builder.get()};

        // Setting parameters
        glyph_image_builder.set_scale(scale);

        // Building
        if let Some(bounding_box)=glyph_image_builder.build_image(glyph_id,font){
            Some((bounding_box,glyph_image_builder.image()))
        }
        else{
            None
        }
    }

    /// [offset_x,offset_y,width,height]
    pub fn load_glyph(&self,glyph_id:GlyphId,scale:Scale,font:&Face)->Option<[f32;4]>{
        if let Some((bounding_box,image))=self.build_glyph_image(glyph_id,scale,font){
            let width=bounding_box[2];
            let height=bounding_box[3];

            if width>self.texture_size[0] || height>self.texture_size[0]{
                panic!("Invalid glyph size")
            }

            self.load_glyph_image([width.ceil() as u32,height.ceil() as u32],image);

            Some(bounding_box)
        }
        else{
            None
        }
    }
}

impl TextGraphics{
    pub fn draw_loaded_glyph(
        &self,
        colour:Colour,
        position:[f32;2],
        draw_parameters:&DrawParameters
    ){
        // The size of the loaded glyph image
        let [width,height]=unsafe{(&mut *self.glyph_image_builder.get()).dimensions()};

        let [x1,y1,x2,y2]=[
            position[0],
            position[1],
            position[0]+width as f32,
            position[1]+height as f32,
        ];

        // The normalized size of the loaded glyph image
        let uwidth=width as f32/self.texture_size[0];
        let vheight=height as f32/self.texture_size[1];

        let vertices=&[
            TextVertex2D::new([x1,y1],[0f32,vheight]),
            TextVertex2D::new([x2,y1],[uwidth,vheight]),
            TextVertex2D::new([x1,y2],[0f32,0f32]),
            TextVertex2D::new([x2,y2],[uwidth,0f32]),
        ];

        self.load_vertices(vertices);
        self.draw.bind();
        self.vertex_array.bind();
        self.vertex_buffer.bind();
        self.texture.bind();

        let _=self.draw.set_uniform_value("glyph_colour",colour);
        draw_parameters.bind_uniform();

        unsafe{
            DrawArrays(TRIANGLE_STRIP,0,4);
        }
        VertexArray::<TextVertex2D>::unbind();
    }

    pub fn draw_glyph(
        &self,
        glyph_texture:&Texture2D,
        colour:Colour,
        [x,y]:[f32;2],
        [width,height]:[f32;2],
        draw_parameters:&DrawParameters
    ){
        let [x1,y1,x2,y2]=[
            x,
            y,
            x+width,
            y+height,
        ];

        let vertices=&[
            TextVertex2D::new([x1,y1],[0f32,1f32]),
            TextVertex2D::new([x2,y1],[1f32,1f32]),
            TextVertex2D::new([x1,y2],[0f32,0f32]),
            TextVertex2D::new([x2,y2],[1f32,0f32]),
        ];

        self.load_vertices(vertices);
        self.draw.bind();
        self.vertex_array.bind();
        self.vertex_buffer.bind();
        glyph_texture.bind();

        let _=self.draw.set_uniform_value("glyph_colour",colour);

        draw_parameters.bind_uniform();

        unsafe{
            DrawArrays(TRIANGLE_STRIP,0,4);
        }
        VertexArray::<TextVertex2D>::unbind();
    }
}