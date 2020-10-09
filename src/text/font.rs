use super::{
    OutlineCurveBuilder,
    Glyph,
};

use ttf_parser::{
    Face,
    GlyphId,
};

use std::{
    path::Path,
    fs::read,
};

/// Хранит данные шрифта
/// Должен быть несдвигаемым в памяти,
/// чтобы не сломать ссылку
struct OwnedFont{
    // Данные
    data:Vec<u8>,
    // Ссылка на данные, которая предоставляет методы для работы с ними
    face:Option<Face<'static>>,
}

impl OwnedFont{
    fn load<P:AsRef<Path>>(path:P)->Option<OwnedFont>{
        let data=match read(path){
            Ok(data)=>data,
            Err(_)=>return None,
        };

        let mut font=Self{
            data,
            face:None,
        };

        let slice:&'static [u8]=unsafe{
            std::slice::from_raw_parts(font.data.as_ptr(),font.data.len())
        };

        let face=Face::from_slice(slice,0).unwrap();

        font.face=Some(face);

        Some(font)
    }
}

// Ссылка на данные шрифта
// /ᐠ｡ꞈ｡ᐟ\
/// Обёртка позволяющая работать со шрифтом.
/// 
/// A wrapper that provides methods to work with fonts.
pub struct FaceWrapper<'a>(pub Face<'a>);

impl<'a> FaceWrapper<'a>{
    /// Строит глиф для данного символа.
    /// 
    /// Builds a glyph for the given character.
    pub fn glyph(&self,character:char)->Option<Glyph>{
        // Поиск глифа
        if let Some(glyph_id)=self.0.glyph_index(character){
            let mut outline_builder=OutlineCurveBuilder::default();
            // Получение точек для построения глифа
            if let Some(bounding_box)=self.0.outline_glyph(glyph_id,&mut outline_builder){
                // Высота для выравнивания
                let global_height=self.0.ascender() as f32;

                let glyph_size=[
                    bounding_box.width() as f32,
                    bounding_box.height() as f32,
                ];

                let glyph_offset=[
                    bounding_box.x_min as f32,
                    bounding_box.y_min as f32,
                ];

                // Мастабирование под размер шрифта (относительно самого высокого символа)
                let scale=glyph_size[1]/global_height;

                // Ширина до следующего символа
                let advance_width=self.0.glyph_hor_advance(glyph_id).unwrap() as f32;

                let glyph=Glyph::new(
                    glyph_offset,
                    glyph_size,
                    advance_width,
                    scale,
                    outline_builder.outline
                );

                Some(glyph)
            }
            else{
                None
            }
        }
        else{
            None
        }
    }

    /// Строит глиф для неопределённого символа.
    /// 
    /// Builds a glyph for the undefined character.
    pub fn undefined_glyph(&self)->Glyph{
        let glyph_id=GlyphId(0);

        let mut outline_builder=OutlineCurveBuilder::default();

        if let Some(bounding_box)=self.0.outline_glyph(glyph_id,&mut outline_builder){
            // Высота для выравнивания
            let global_height=self.0.ascender() as f32;

            let glyph_size=[
                bounding_box.width() as f32,
                bounding_box.height() as f32,
            ];

            let glyph_offset=[
                bounding_box.x_min as f32,
                bounding_box.y_min as f32,
            ];

            // Мастабирование под размер шрифта (относительно самого высокого символа)
            let scale=glyph_size[1]/global_height;

            // Ширина до следующего символа
            let advance_width=self.0.glyph_hor_advance(glyph_id).unwrap() as f32;

            Glyph::new(
                glyph_offset,
                glyph_size,
                advance_width,
                scale,
                outline_builder.outline
            )
        }
        else{
            // unreachable... maybe :)
            panic!("No undefined glyph");
        }
    }

    /// Возвращает ширину пробела.
    /// 
    /// Returns the whitespace advance.
    pub fn whitespace_advance(&self,font_size:f32)->f32{
        // TODO: доделать
        font_size 
    }
}

/// Хранилище для шрифта.
/// 
/// A font owner.
pub struct Font{
    font:Box<OwnedFont>,
}

impl Font{
    pub fn load<P:AsRef<Path>>(path:P)->Option<Font>{
        let font=OwnedFont::load(path)?;

        Some(Self{
            font:Box::new(font),
        })
    }

    pub fn face(&self)->&Face{
        self.font.as_ref().face.as_ref().unwrap()
    }

    pub fn face_wrapper<'a>(&'a self)->FaceWrapper<'a>{
        FaceWrapper(self.font.as_ref().face.clone().unwrap())
    }
}