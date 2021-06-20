#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    unused_must_use,
    unused_mut,
    unused_macros,
    unused_imports,
    dead_code
)]

pub mod graphics;

pub mod app;

#[cfg(feature="fps_counter")]
pub use app::fps;

#[cfg(feature="ups_counter")]
pub use app::ups;

#[cfg(feature="texture_graphics")]
pub mod texture;

#[cfg(feature="text_graphics")]
pub mod text;

pub use image;

#[cfg(feature="simple_graphics")]
pub mod shapes;

/// RGBA - [f32; 4]
pub type Colour=[f32;4];

#[cfg(feature="audio")]
pub use cat_audio as audio;