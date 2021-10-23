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

#![cfg_attr(feature="nightly-docs",feature(doc_cfg))]

/// Some useful structures and functions.
#[cfg(feature="support")]
pub mod support;

#[cfg(any(feature="opengl"))]
pub mod graphics;

#[cfg_attr(feature = "nightly-docs", doc(cfg(target_os="windows")))]
#[cfg(all(target_os="windows",feature="windowing"))]
pub mod windows;

#[cfg(any(feature="opengl",feature="windowing"))]
pub use image;