use cat_engine_basement::{
    opengl::{
        core::{
            Core as GLCore,
            PixelStoreParameter
        },
        texture::{
            Texture,
            TextureTarget,
            Texture3DAllocateTarget,
            Texture3DWriteTarget,
            TextureParameterTarget,
            TextureMagFilter,
            TextureMinFilter,
            TextureParameter,
            TextureInternalFormat,
            PixelType,
            PixelFormat,
        },
    }
};

use ab_glyph_rasterizer::{
    Point,
    point,
    Rasterizer,
};

use ttf_parser::{
    GlyphId,
    OutlineBuilder,
};

pub use ttf_parser::{
    Face,
    FaceParsingError
};

use std::{
    path::Path,
    fs::read,
    io::Error,
    ptr::null_mut,
};

use std::collections::HashMap;



#[derive(Debug)]
pub enum FontError{
    IOError(Error),
    ParseError(FaceParsingError)
}



pub struct Scale{
    horizontal:f32,
    vertical:f32,
}

impl Scale{
    pub fn new(horizontal:f32,vertical:f32)->Scale{
        Self{
            horizontal,
            vertical
        }
    }
}



pub struct FontOwner{
    /// Данные.
    data:Vec<u8>,
    /// Ссылка на данные,
    /// которая предоставляет методы для работы с ними.
    face:Face<'static>,
}

impl FontOwner{
    pub fn load<P:AsRef<Path>>(path:P)->Result<FontOwner,FontError>{
        let data=match read(path){
            Ok(data)=>data,
            Err(e)=>return Err(FontError::IOError(e)),
        };

        let slice:&'static [u8]=unsafe{
            std::slice::from_raw_parts(data.as_ptr(),data.len())
        };

        let face=match Face::from_slice(slice,0){
            Ok(face)=>face,
            Err(e)=>return Err(FontError::ParseError(e)),
        };

        Ok(
            Self{
                data,
                face,
            }
        )
    }

    pub fn parse(data:Vec<u8>)->Result<FontOwner,FaceParsingError>{
        let slice:&'static [u8]=unsafe{
            std::slice::from_raw_parts(data.as_ptr(),data.len())
        };

        let face=match Face::from_slice(slice,0){
            Ok(face)=>face,
            Err(e)=>return Err(e),
        };

        Ok(Self{
            data,
            face,
        })
    }

    pub fn data(&self)->&Vec<u8>{
        &self.data
    }

    pub fn face(&self)->&Face{
        &self.face
    }
}



pub struct Glyph{
    pub texture:i32,
    pub size:[f32;2],
    pub bearing:[f32;2],
    pub horizontal_advance:f32,
}



pub struct GlyphCache{
    global_size:[f32;2],
    global_offset:[f32;2],
    whitespace_advance:f32,
    glyphs:HashMap<char,Glyph>,
    map:Texture,
}

impl GlyphCache{
    pub fn new(alphabet:&str,font_size:f32,font:&FontOwner)->GlyphCache{
        let mut glyphs=HashMap::with_capacity(alphabet.len());

        let global_bounding_box=font.face().global_bounding_box();
        let global_size=[global_bounding_box.width() as usize,global_bounding_box.height() as usize];

        let scale=font_size/global_size[1] as f32;

        let mut global_size=[
            global_size[0] as f32*scale,
            global_size[1] as f32*scale,
        ];

        global_size[0]=global_size[0].ceil();
        global_size[1]=global_size[1].ceil();

        let global_offset=[
            global_bounding_box.x_min as f32,
            global_bounding_box.y_min as f32
        ];
        let mut global_offset=[
            global_offset[0] as f32*scale,
            global_offset[1] as f32*scale,
        ];
        global_offset[0]=global_offset[0].floor();
        global_offset[1]=global_offset[1].floor();

        let mut builder=GlyphImageBuilder::new([global_size[0] as usize,global_size[1] as usize]);
        builder.set_scale(Scale::new(scale,scale));
        builder.set_offset(global_offset);

        unsafe{GLCore::set_pixel_storei(PixelStoreParameter::UNPACK_ALIGNMENT,1)}

        let texture=Texture::new();
        texture.bind(TextureTarget::Texture2DArray);
        Texture::allocate_3d(
            Texture3DAllocateTarget::Texture2DArray,
            0,
            TextureInternalFormat::R8,
            global_size[0] as i32,
            global_size[1] as i32,
            alphabet.len() as i32,
            PixelFormat::RED,
            PixelType::U8,
            null_mut::<u8>()
        );

        Texture::set_parameteri(
            TextureParameterTarget::Texture2DArray,
            TextureParameter::MinFilter,
            TextureMinFilter::Linear as i32
        );

        Texture::set_parameteri(
            TextureParameterTarget::Texture2DArray,
            TextureParameter::MagFilter,
            TextureMagFilter::Linear as i32
        );

        for (layer,character) in alphabet.chars().enumerate(){
            if let Some(glyph_id)=font.face().glyph_index(character){
                if let Some(bounding_box)=builder.build_image(glyph_id,font.face()){
                    let horizontal_advance=if let Some(advance)=font.face().glyph_hor_advance(glyph_id){
                        advance as f32*scale
                    }
                    else{
                        0f32
                    };

                    Texture::write_3d(
                        Texture3DWriteTarget::Texture2DArray,
                        0,
                        0,
                        0,
                        layer as i32,
                        global_size[0] as i32,
                        global_size[1] as i32,
                        1,
                        PixelFormat::RED,
                        PixelType::U8,
                        unsafe{builder.image_buffer.get_unchecked(0)}
                    );

                    let glyph=Glyph{
                        texture:layer as i32,
                        size:[bounding_box[2],bounding_box[3]],
                        bearing:[bounding_box[0],bounding_box[1]],
                        horizontal_advance
                    };

                    glyphs.insert(character,glyph);
                }
            }
        }

        unsafe{GLCore::set_pixel_storei(PixelStoreParameter::UNPACK_ALIGNMENT,4)}

        let whitespace_advance=if let Some(id)=font.face().glyph_index(' '){
            if let Some(advance)=font.face().glyph_hor_advance(id){
                advance as f32*scale
            }
            else{
                global_size[0]*scale
            }
        }
        else{
            global_size[0]*scale
        };

        Self{
            global_size,
            global_offset,
            whitespace_advance,
            glyphs,
            map:texture
        }
    }

    pub fn len(&self)->usize{
        self.glyphs.len()
    }

    pub fn global_size(&self)->[f32;2]{
        self.global_size
    }

    pub fn global_offset(&self)->[f32;2]{
        self.global_offset
    }

    pub fn whitespace_advance(&self)->f32{
        self.whitespace_advance
    }

    pub fn glyph(&self,character:&char)->Option<&Glyph>{
        self.glyphs.get(character)
    }

    pub (crate) fn bind(&self){
        self.map.bind(TextureTarget::Texture2DArray)
    }
}



struct GlyphImageBuilder{
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

    pub fn set_offset(&mut self,offset:[f32;2]){
        self.offset=offset
    }

    /// [offset_x,offset_y,width,height]
    pub fn build_image(&mut self,glyph_id:GlyphId,font:&Face)->Option<[f32;4]>{
        self.rasterizer.clear();
        self.last=point(0f32,0f32);
        self.last_move=None;
        // Отчистка буфера от старого изображения
        self.image_buffer.clear();

        // Построение очертания
        if let Some(bounding_box)=font.outline_glyph(glyph_id,self){
            // Перевод очертания в картинку
            let image_buffer=&mut self.image_buffer;
            self.rasterizer.for_each_pixel(|_,pixel|{
                image_buffer.push((pixel*255f32).round() as u8);
            });

            let width=(bounding_box.width() as f32*self.scale.horizontal).ceil();
            let height=(bounding_box.height() as f32*self.scale.vertical).ceil();

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


}

impl OutlineBuilder for GlyphImageBuilder{
    fn move_to(&mut self,x:f32,y:f32){
        self.last=point(
            self.scale.horizontal*(x)-self.offset[0],
            self.scale.vertical*(y)-self.offset[1]
        );
        self.last_move=Some(self.last);
    }

    fn line_to(&mut self,x1:f32,y1:f32){
        let p1=point(
            self.scale.horizontal*(x1)-self.offset[0],
            self.scale.vertical*(y1)-self.offset[1]
        );

        self.rasterizer.draw_line(self.last,p1);

        self.last=p1;
    }

    fn quad_to(&mut self,x1:f32,y1:f32,x2:f32,y2:f32){
        let p1=point(
            self.scale.horizontal*(x1)-self.offset[0],
            self.scale.vertical*(y1)-self.offset[1]
        );
        let p2=point(
            self.scale.horizontal*(x2)-self.offset[0],
            self.scale.vertical*(y2)-self.offset[1]
        );

        self.rasterizer.draw_quad(self.last,p1,p2);

        self.last=p2;
    }

    fn curve_to(&mut self,x1:f32,y1:f32,x2:f32,y2:f32,x3:f32,y3:f32){
        let p1=point(
            self.scale.horizontal*(x1)-self.offset[0],
            self.scale.vertical*(y1)-self.offset[1]
        );
        let p2=point(
            self.scale.horizontal*(x2)-self.offset[0],
            self.scale.vertical*(y2)-self.offset[1]
        );
        let p3=point(
            self.scale.horizontal*(x3)-self.offset[0],
            self.scale.vertical*(y3)-self.offset[1]
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

/// An iteratable glyph cache builder.
pub struct GlyphCacheBuilder{
    font:FontOwner,
    inner:UnsafeGlyphCacheBuilder
}

impl GlyphCacheBuilder{
    pub fn new(capacity:usize,font:FontOwner,font_size:f32)->GlyphCacheBuilder{
        let inner=UnsafeGlyphCacheBuilder::new(capacity,&font,font_size);
        Self{
            font,
            inner
        }
    }

    /// Adds a character to a glyph cache.
    /// 
    /// If the cache already contains the character, does nothing.
    pub fn add_glyph(&mut self,character:char){
        unsafe{
            self.inner.add_glyph(&self.font,character)
        }
    }

    pub fn finish(self)->(GlyphCache,FontOwner){
        (self.inner.finish(),self.font)
    }
}



pub struct UnsafeGlyphCacheBuilder{
    font_scale:f32,
    glyph_cache:GlyphCache,
    builder:GlyphImageBuilder
}

impl UnsafeGlyphCacheBuilder{
    pub fn new(capacity:usize,font:&FontOwner,font_size:f32)->UnsafeGlyphCacheBuilder{
        let global_bounding_box=font.face().global_bounding_box();
        let global_size=[global_bounding_box.width() as usize,global_bounding_box.height() as usize];

        let scale=font_size/global_size[1] as f32;

        let mut global_size=[
            global_size[0] as f32*scale,
            global_size[1] as f32*scale,
        ];

        global_size[0]=global_size[0].ceil();
        global_size[1]=global_size[1].ceil();

        let global_offset=[
            global_bounding_box.x_min as f32,
            global_bounding_box.y_min as f32
        ];
        let mut global_offset=[
            global_offset[0] as f32*scale,
            global_offset[1] as f32*scale,
        ];
        global_offset[0]=global_offset[0].floor();
        global_offset[1]=global_offset[1].floor();

        let mut builder=GlyphImageBuilder::new([global_size[0] as usize,global_size[1] as usize]);
        builder.set_scale(Scale::new(scale,scale));
        builder.set_offset(global_offset);

        let texture=Texture::new();
        texture.bind(TextureTarget::Texture2DArray);
        Texture::allocate_3d(
            Texture3DAllocateTarget::Texture2DArray,
            0,
            TextureInternalFormat::R8,
            global_size[0] as i32,
            global_size[1] as i32,
            capacity as i32,
            PixelFormat::RED,
            PixelType::U8,
            null_mut::<u8>()
        );

        Texture::set_parameteri(
            TextureParameterTarget::Texture2DArray,
            TextureParameter::MinFilter,
            TextureMinFilter::Linear as i32
        );

        Texture::set_parameteri(
            TextureParameterTarget::Texture2DArray,
            TextureParameter::MagFilter,
            TextureMagFilter::Linear as i32
        );

        let whitespace_advance=if let Some(id)=font.face().glyph_index(' '){
            if let Some(advance)=font.face().glyph_hor_advance(id){
                advance as f32*scale
            }
            else{
                global_size[0]*scale
            }
        }
        else{
            global_size[0]*scale
        };

        Self{
            font_scale:scale,
            glyph_cache:GlyphCache{
                global_size,
                global_offset,
                whitespace_advance,
                glyphs:HashMap::with_capacity(capacity),
                map:texture
            },
            builder,
        }
    }

    /// Adds a character to a glyph cache.
    /// 
    /// If the cache already contains the character, does nothing.
    /// 
    /// Use the same font as used when creating the builder, the other font may cause undefined behavior.
    pub unsafe fn add_glyph(&mut self,font:&FontOwner,character:char){
        if self.glyph_cache.glyphs.contains_key(&character){
            return
        }

        if let Some(glyph_id)=font.face().glyph_index(character){
            if let Some(bounding_box)=self.builder.build_image(glyph_id,font.face()){
                let horizontal_advance=if let Some(advance)=font.face().glyph_hor_advance(glyph_id){
                    advance as f32*self.font_scale
                }
                else{
                    0f32
                };

                let layer=self.glyph_cache.len() as i32;

                GLCore::set_pixel_storei(PixelStoreParameter::UNPACK_ALIGNMENT,1);

                Texture::write_3d(
                    Texture3DWriteTarget::Texture2DArray,
                    0,
                    0,
                    0,
                    layer,
                    self.glyph_cache.global_size[0] as i32,
                    self.glyph_cache.global_size[1] as i32,
                    1,
                    PixelFormat::RED,
                    PixelType::U8,
                    self.builder.image_buffer.get_unchecked(0)
                );

                GLCore::set_pixel_storei(PixelStoreParameter::UNPACK_ALIGNMENT,4);

                let glyph=Glyph{
                    texture:layer as i32,
                    size:[bounding_box[2],bounding_box[3]],
                    bearing:[bounding_box[0],bounding_box[1]],
                    horizontal_advance
                };

                self.glyph_cache.glyphs.insert(character,glyph);
            }
        }
    }

    pub fn finish(self)->GlyphCache{
        self.glyph_cache
    }
}