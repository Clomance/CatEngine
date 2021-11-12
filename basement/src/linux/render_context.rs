use super::{
    Window,
    Display,
};

use std::ptr::null_mut;

use x11::xlib::XVisualInfo;

use x11::glx::{
    GLXContext,

    glXCreateContext,
    glXMakeCurrent,
};

pub struct RenderContext{
    handle:GLXContext,
}

impl RenderContext{
    /// An OpenGL rendering context may be created by calling.
    /// 
    /// Calling glXCreateContext(dpy, visual, share list, direct) is equivalent
    /// to calling glXCreateNewContext(dpy, config, render type, share list, direct)
    /// where config is the GLXFBConfig identified by the GLX FBCONFIG ID attribute of visual.
    /// If visual’s GLX RGBA attribute is True then render type is taken as GLX RGBA TYPE, otherwise GLX COLOR INDEX TYPE.
    /// The GLXFBConfig identified by the GLX FBCONFIG ID attribute of visual is associated with the resulting
    /// context.
    /// 
    /// glXCreateContext can generate the following errors:
    /// GLXBadContext if share list is neither zero nor a valid GLX rendering context;
    /// BadValue if visual is not a valid X Visual or if GLX does not support it;
    /// BadMatch if share list defines an address space that cannot be shared with the newly created context or
    /// if share list was created on a different screen than the one referenced by visual;
    /// BadAlloc if the server does not have enough resources to allocate the new context.
    pub fn create(display:&Display,visual:*mut XVisualInfo,direct:bool)->RenderContext{// share_list:Option<&mut RenderContext>,
        unsafe{
            Self{
                handle:glXCreateContext(display.handle().as_raw(),visual,null_mut(),direct as i32)
            }
        }
    }

    /// To make a context current.
    /// 
    /// Calling glXMakeCurrent(dpy, draw, ctx) is equivalent to calling glXMakeContextCurrent(dpy, draw, draw, ctx).
    /// Note that draw will be used for both the draw and read drawable.
    /// If ctx and draw are not compatible then a BadMatch error will be generated.
    /// Some implementations may enforce a stricter rule and generate a BadMatch error
    /// if ctx and draw were not created with the same XVisualInfo.
    /// 
    /// If ctx is current to some other thread, then glXMakeCurrent will generate a BadAccess error.
    /// GLXBadContextState is generated if there is a current
    /// rendering context and its render mode is either GL FEEDBACK or GL SELECT.
    /// If ctx is not a valid GLX rendering context, GLXBadContext is generated.
    /// If draw is not a valid GLXPixmap or a valid Window, a GLXBadDrawable error is generated.
    /// If the previous context of the calling thread has unflushed commands, and the
    /// previous drawable is a window that is no longer valid, GLXBadCurrentWindow is generated.
    /// Finally, note that the ancillary buffers for draw need not be allocated
    /// until they are needed. A BadAlloc error will be generated if the server does not
    /// have enough resources to allocate the buffers.
    /// To release the current context without assigning a new one, use NULL for ctx
    /// and None for draw. If ctx is NULL and draw is not None, or if draw is None and
    /// ctx is not NULL, then a BadMatch error will be generated.
    pub fn make_current(&self,display:&Display,window:&Window)->i32{
        unsafe{
            glXMakeCurrent(display.handle().as_raw(),window.handle(),self.handle)
        }
    }
}