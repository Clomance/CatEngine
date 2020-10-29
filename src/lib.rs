#![allow(non_upper_case_globals,unused_must_use,unused_imports,
    dead_code,unused_mut,unused_variables,unused_macros)]

//! # 2D графический движок с поддержкой аудио. A 2D graphics engine with audio support.
//! 
//! "Фичи" по умолчанию - `simple_graphics`, `texture_graphics`, `text_graphics`.
//! 
//! The defealt features are `simple_graphics`, `texture_graphics`, `text_graphics`.
//! 
//! ```
//! use cat_engine::{
//!     DefaultWindow,
//!     Window,
//!     WindowEvent
//! };
//! 
//! fn main(){
//!     // Default settings
//!     let mut window=Window::new(|_,_|{}).unwrap();
//! 
//!     while let Some(event)=window.next_event(){
//!         match event{
//!             WindowEvent::Exit=>break,
//!             WindowEvent::Draw=>{
//!                 window.draw(|_,g|{
//!                     g.clear_colour([1.0,0.0,0.0,0.0]);
//!                 }).unwrap();
//!             }
//!             _=>{}
//!         }
//!     }
//! }
//! ```
//! 
//! 
//! 
//! Modifying the window.
//! ```
//! let wi=window.display().gl_window();
//! let w=wi.window();
//! w.set_minimized(false);
//! w.set_visible(false);
//! ```

// re-exports
pub use glium::{
    self,
    glutin::event::{MouseButton,ModifiersState,MouseScrollDelta},
};

// re-exports
pub use image;

mod support;

#[cfg(feature="audio")]
pub mod audio;

#[cfg(feature="text_graphics")]
pub mod text;

pub mod texture;

pub mod graphics;

mod window;
pub use window::*;

//      Caution      \\
// Under construction \\
mod app;

/// Геометрические фигуры. Geometric shapes.
/// `feature = "simple_graphics"`, `default_features`
#[cfg(all(feature="simple_graphics"))]
pub mod shapes;

//    _.---.._             _.---...__
// .-'   /\   \          .'  /\     /
// `.   (  )   \        /   (  )   /
//   `.  \/   .'\      /`.   \/  .'
//     ``---''   )    (   ``---''
//             .';.--.;`.
//           .' /_...._\ `.
//         .'   `.a  a.'   `.
//        (        \/        )
//         `.___..-'`-..___.'
//            \          /
//             `-.____.-'
//      Henlo, Mister Programmer.
// If you think there are to many cats, you are mistaken.
// There can't be too many cats.
// Cats are cute.
// I'd better write a good documentation rather than paste cats.
// But......................... Cats are more important!


/// RGBA - [f32; 4]
pub type Colour=[f32;4];

/// Возвращает прямоугольник размера окна.
/// Returns a window sized rectangle.
/// [0, 0, width, height]
pub fn window_rect()->[f32;4]{
    unsafe{[
        0f32,
        0f32,
        window_width,
        window_height,
    ]}
}