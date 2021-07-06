mod mono_windowing;
#[cfg(target_os="windows")]
pub use mono_windowing::windows::{
    App,
    AppAttributes,
    Window,
    AppControl,
    CursorIcon,
    Background,
    Fullscreen,
    Monitor,
    WindowAttributes,
    WindowClassAttributes,
    WindowInner,
    Event,
    WindowEvent,
    VirtualKeyCode,
    EventLoopAttributes,
    OpenGLRenderContextAttributes,
    UpdateInterval,
    WinError,
    WindowProcedure,
    quit,
};