#[cfg(target_os="windows")]
mod windows;
#[cfg(target_os="windows")]
pub use windows::{
    App,
    AppAttributes,
    AppControl,
    CursorIcon,
    Background,
    Fullscreen,
    WindowAttributes,
    WindowClassAttributes,
    Event,
    WindowEvent,
    VirtualKeyCode,
    EventLoopAttributes,
    OpenGLRenderContextAttributes,
    UpdateInterval,
};