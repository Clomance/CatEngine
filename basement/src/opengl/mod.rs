//! This module provides OpenGL functions with different level of wrapping.

pub mod core;

pub mod buffer;

pub mod shader;

pub mod texture;

pub mod program;

pub mod vertex_array;

pub type ColourComponent=f32;
pub type Colour=[ColourComponent;4];