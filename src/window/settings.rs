use crate::{
    Colour,
    graphics::GraphicsSettings,
};

use glium::glutin::{
    window::WindowAttributes,
    PixelFormatRequirements,
};

use std::{
    ops::Range,
    path::PathBuf
};

#[derive(Clone,Debug)]
pub struct WindowSettings{
    //--General attributes--\\
    pub general:GeneralSettings,

    /// feature = "mouse_cursor_icon"
    #[cfg(feature="mouse_cursor_icon")]
    pub mouse_cursor_icon_settings:MouseCursorIconSettings<PathBuf>,



    //--Window attributes--\\
    pub window_attributes:WindowAttributes,



    //--OpenGL attributes--\\

    /// Whether to enable the debug flag of the context.
    /// 
    /// Debug contexts are usually slower but give better error reporting.
    /// 
    /// The default is false.
    pub debug:bool,

    /// Whether to use vsync.
    /// If vsync is enabled, calling swap_buffers will block until the screen refreshes.
    /// This is typically used to prevent screen tearing.
    /// 
    /// The default is false.
    pub vsync:bool,



    //--Pixel format requirements--\\
    pub pixel_fmt_req:PixelFormatRequirements,



    //--Local graphics attributes--\\
    pub graphics_base_settings:GraphicsSettings,
}

impl WindowSettings{
    /// Default settings.
    pub fn new()->WindowSettings{

        #[cfg(feature="mouse_cursor_icon")]
        let mut path=PathBuf::new();
        #[cfg(feature="mouse_cursor_icon")]
        path.push("./mouse_cursor_icon.png");

        Self{
            //--General attributes--\\
            general:GeneralSettings::new(),

            #[cfg(feature="mouse_cursor_icon")]
            mouse_cursor_icon_settings:MouseCursorIconSettings::<PathBuf>::new(),

            //--Window attributes--\\
            window_attributes:WindowAttributes::default(),

            //--OpenGL attributes--\\
            debug:false,
            vsync:false,

            //--Pixel format requirements--\\
            pixel_fmt_req:PixelFormatRequirements::default(),

            //--Local graphics attributes--\\
            graphics_base_settings:GraphicsSettings::new(),
        }
    }
}

#[derive(Clone,Debug)]
pub struct GeneralSettings{
    /// Whether the window should be filled with given colour upon creation.
    /// 
    /// The default is None.
    pub initial_colour:Option<Colour>,

    /// The amount of update events per second.
    /// 
    /// The default is 50.
    pub updates_per_second:u32,
}

impl GeneralSettings{
    pub fn new()->GeneralSettings{
        Self{
            initial_colour:None,
            updates_per_second:50u32,
        }
    }
}

use std::path::Path;

#[derive(Clone,Debug)]
pub struct MouseCursorIconSettings<P:AsRef<Path>>{
    /// The icon size.
    /// 
    /// The default is [30f32;2].
    pub size:[f32;2],

    /// The icon position = mouse cursor position + shift
    /// 
    /// The default is [-15f32;2].
    pub shift:[f32;2],

    /// The path to the icon.
    /// 
    /// The default is `./mouse_cursor_icon.png`.
    pub path:P,

    /// The range of the texture vertex buffer
    /// to save icon vertexes.
    /// 
    /// The default is 4..8.
    pub range:Range<usize>,
}

impl MouseCursorIconSettings<PathBuf>{
    pub fn new()->MouseCursorIconSettings<PathBuf>{
        let mut path=PathBuf::new();
        path.push("./mouse_cursor_icon.png");

        Self{
            size:[30f32;2],
            shift:[-15f32;2],
            path:path,
            range:4..8
        }
    }
}