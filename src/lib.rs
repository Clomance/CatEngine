#![allow(non_upper_case_globals,unused_must_use,unused_imports,dead_code)]

//! # 2D графический движок с поддержкой аудио. A 2D graphics engine with audio support.
//! 
//! Использует OpenGL 2.0 и выше.
//! 
//! Текст рисуется поточечно. Так что важно указать правильный размер буфера.
//! 
//! #
//! 
//! Uses OpenGL 2.0 and above.
//! 
//! Text is drawn pointwise. It's important to set correct size of the text graphics buffer.
//! 
//! #
//! 
//! 
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


#[cfg(feature="audio")]
pub mod audio;

#[cfg(feature="text_graphics")]
pub mod text;

#[cfg(feature="texture_graphics")]
pub mod image;

pub mod graphics;

mod window;
pub use window::*;

/// Геометрические фигуры. Geometric shapes.
/// feature = "simple_graphics"
#[cfg(feature="simple_graphics")]
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