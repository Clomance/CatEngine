mod mono_windowing;

#[cfg(any(target_os="windows"))]
pub use mono_windowing::windows::{
    App,
    AppAttributes,
    Window,
    CursorIcon,
    Background,
    Fullscreen,
    Monitor,
    WindowAttributes,
    WindowClassAttributes,
    WindowInner,
    Event,
    EventLoop,
    LoopControl,
    MouseButton,
    ProcessEvent,
    WindowEvent,
    VirtualKeyCode,
    EventLoopAttributes,
    OpenGLRenderContextAttributes,
    EventInterval,
    WinError,
    WindowProcedure,
    EmptyHandler,
    quit,
};