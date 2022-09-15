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

pub use cat_engine_basement as basement;

pub mod window{
    pub use cat_engine_basement::winapi::{
        window::{
            Window,
            WindowAttributes,

            WindowClass,
            WindowClassAttributes,

            Fullscreen,
            CursorIcon,
            SystemCursor,
            Background,
            WindowBackgroundSystemColour,

            WindowClassStyle,
            WindowClassStyles,

            WindowStyle,
            WindowStyles,

            ExtendedWindowStyle,
            ExtendedWindowStyles,
        },

        MouseButton,
        VirtualKeyCode,

        Monitor,
        EventLoopAttributes,
        OpenGLRenderContextAttributes,
    };

    pub type WinError=cat_engine_basement::winapi::Error;
}

pub use cat_audio as audio;

mod app;
pub use app::{
    App,
    AppAttributes,
};

pub mod system;

pub mod object;

pub mod graphics;

pub mod text;

pub mod texture;