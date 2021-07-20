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

/// The dark side of the engine.
pub use cat_engine_basement as basement;

pub mod graphics;

pub mod app;

#[cfg(feature="texture_graphics")]
pub mod texture;

#[cfg(feature="text_graphics")]
pub mod text;

pub use cat_engine_basement::image;

#[cfg(feature="simple_graphics")]
pub mod shapes;

/// RGBA - [f32; 4]
pub type Colour=[f32;4];

#[cfg(feature="audio")]
pub use cat_audio as audio;