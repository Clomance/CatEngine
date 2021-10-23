use super::device_context::DeviceContextHandle;

use core::{
    mem::{
        transmute,
        transmute_copy,
    },
    ptr::NonNull,
};

use winapi::{
    shared::windef::HGLRC,
    um::{
        wingdi::{
            wglCreateContext,
            wglMakeCurrent,
            wglDeleteContext,
        },
    }
};

/// A replacement for `HGLRC`.
/// Can be wraped with `Option` with null pointer optimization.
#[derive(Clone,Copy)]
#[repr(transparent)]
pub struct RenderContextHandle{
    inner:NonNull<HGLRC>,
}
implement_handle_wrapper!(RenderContextHandle,HGLRC);

/// Wraps the Windows API functions releative to a device context.
pub struct RenderContext;

impl RenderContext{
    pub const fn new()->RenderContext{
        Self
    }
}

impl RenderContext{
    /// Creates a new OpenGL rendering context, which is suitable for drawing on the device referenced by `context`.
    /// The rendering context has the same pixel format as the device context.
    /// 
    /// A rendering context is not the same as a device context.
    /// Set the pixel format of the device context before creating a rendering context.
    /// For more information on setting the device context's pixel format, see the `DeviceContext::set_pixel_format` function.
    /// 
    /// To use OpenGL, you create a rendering context,
    /// select it as a thread's current rendering context,
    /// and then call OpenGL functions.
    /// When you are finished with the rendering context,
    /// you dispose of it by calling the wglDeleteContext function.
    /// 
    /// If the function succeeds, the return value is a valid handle to an OpenGL rendering context.
    /// If the function fails, the return value is `None`.
    /// To get extended error information, call `WinCore::get_last_error`.
    #[inline(always)]
    pub unsafe fn wgl_create_context(&self,context:DeviceContextHandle)->Option<RenderContextHandle>{
        RenderContextHandle::from_raw(wglCreateContext(context.as_raw()))
    }

    /// Deletes a specified OpenGL rendering context.
    /// 
    /// It is an error to delete an OpenGL rendering context that is the current context of another thread.
    /// However, if a rendering context is the calling thread's current context,
    /// the `wglDeleteContext` function changes the rendering context to being not current before deleting it.
    /// 
    /// The function does not delete the device context associated
    /// with the OpenGL rendering context when you call the wglMakeCurrent function.
    /// After calling `wglDeleteContext`, you must call DeleteDC to delete the associated device context.
    /// 
    /// If the function succeeds, the return value is `true`.
    /// If the function fails, the return value is `false`.
    /// To get extended error information, call `WinCore::get_last_error`.
    #[inline(always)]
    pub unsafe fn wgl_delete_context(&self,render_context:RenderContextHandle)->bool{
        wglDeleteContext(render_context.as_raw())!=0
    }

    /// Makes a specified OpenGL rendering context the calling thread's current rendering context.
    /// All subsequent OpenGL calls made by the thread are drawn on the device identified by `context`.
    /// You can also use `DeviceContext::wgl_make_current` to change the calling thread's current rendering context so it's no longer current.
    /// 
    /// If `render_context` is `None`, the function makes the calling thread's current rendering context no longer current,
    /// and releases the device context that is used by the rendering context.
    /// In this case, `context` is ignored.
    /// 
    /// The hdc parameter must refer to a drawing surface supported by OpenGL.
    /// It need not be the same `hdc` that was passed to `wglCreateContext` when `hglrc` was created,
    /// but it must be on the same device and have the same pixel format.
    /// GDI transformation and clipping in hdc are not supported by the rendering context.
    /// The current rendering context uses the device context until the rendering context is no longer current.
    /// 
    /// Before switching to the new rendering context,
    /// OpenGL flushes any previous rendering context that was current to the calling thread.
    /// 
    /// A thread can have one current rendering context.
    /// A process can have multiple rendering contexts by means of multithreading.
    /// A thread must set a current rendering context before calling any OpenGL functions.
    /// Otherwise, all OpenGL calls are ignored.
    /// 
    /// A rendering context can be current to only one thread at a time.
    /// You cannot make a rendering context current to multiple threads.
    /// 
    /// An application can perform multithread drawing by making different rendering contexts current to different threads,
    /// supplying each thread with its own rendering context and device context.
    /// 
    /// If an error occurs, the function makes the thread's current rendering context not current before returning.
    /// 
    /// When the function succeeds, the return value is `true`;
    /// otherwise the return value is `false`.
    /// To get extended error information, call `WinCore::get_last_error`.
    #[inline(always)]
    pub unsafe fn wgl_make_current(&self,context:Option<DeviceContextHandle>,render_context:Option<RenderContextHandle>)->bool{
        wglMakeCurrent(DeviceContextHandle::to_raw(context),RenderContextHandle::to_raw(render_context))!=0
    }
}