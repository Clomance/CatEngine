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

#[cfg(target_os="linux")]
pub mod linux;

#[cfg(target_os="windows")]
pub mod windows;

pub mod event;

pub use image;