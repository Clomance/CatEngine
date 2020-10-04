use ttf_parser::{
    Face,
};

use std::{
    path::Path,
    fs::read,
};

struct OwnedFont{
    data:Vec<u8>,
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

pub struct FaceWrapper<'a>(Face<'a>);

impl<'a> FaceWrapper<'a>{
    pub fn glyph(&self){

    }
}

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
}