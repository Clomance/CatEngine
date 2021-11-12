use super::{
    Display,
};

use x11::{
    xlib::{
        XCreateWindow,
        XDefaultRootWindow,
        InputOutput,
        CWColormap,
        CWEventMask,
        XSetWindowAttributes,
        AllocNone,
        ExposureMask,
        KeyPressMask,
        XDestroyWindow,
        XCreateColormap,
        XMapWindow
    },
    glx::{
        glXChooseVisual,
        glXSwapBuffers,

        GLX_RGBA,
        GLX_DEPTH_SIZE,
        GLX_DOUBLEBUFFER
    },
};

use std::{
    ptr::null_mut,
    mem::{
        transmute,
        zeroed,
    },
};

pub struct Window{
    handle:u64,
}

impl Window {
    pub fn new(display:&Display)->Window{
        // Lots of scary unsafes down there.
        unsafe{
            let display_handle=display.handle().as_raw();
            let root_window=XDefaultRootWindow(display_handle);

            let mut visual_attributes=[GLX_RGBA,GLX_DEPTH_SIZE,0,GLX_DOUBLEBUFFER,0];
            let visual=glXChooseVisual(display_handle,0,&mut visual_attributes[0] as *mut _);

            let colourmap=XCreateColormap(display_handle,root_window,(*visual).visual,AllocNone);
            if colourmap==0{
                panic!("XCreateColormap failed.")
            }
            let mut set_window_attributes:XSetWindowAttributes=zeroed();
            set_window_attributes.colormap=colourmap;
            set_window_attributes.event_mask=ExposureMask|KeyPressMask;

            if visual.is_null(){
                panic!("Cannot create visual info.")
            }

            let window=XCreateWindow(
                display_handle,
                root_window,
                10,10,200,200,
                1,
                (*visual).depth,
                InputOutput as u32,
                (*visual).visual,
                CWColormap|CWEventMask,
                &mut set_window_attributes as *mut _
            );
            if window==0{
                panic!("Failed to create a window.")
            }

            XMapWindow(display_handle,window);

            Self{
                handle:window,
            }
        }
    }

    pub fn swap_buffers(&self,display:&Display){
        unsafe{
            glXSwapBuffers(display.handle().as_raw(),self.handle)
        }
    }

    pub fn handle(&self)->u64{
        self.handle
    }
}

pub struct WindowAttributes{
        /// The window name and title.
        pub name:String,

        /// The window size.
        /// 
        /// The default is `None`.
        pub size:Option<[i32;2]>,
    
        /// The window position.
        pub position:Option<[i32;2]>,
    
        /// The default is `true`.
        pub visible:bool,
}