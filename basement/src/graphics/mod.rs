pub mod core;
use self::core::GraphicsCore;

pub mod level0;

pub mod level1;

pub mod level2;

pub type ColourComponent=f32;
pub type Colour=[ColourComponent;4];

pub static mut GCore:GraphicsCore=GraphicsCore::new();