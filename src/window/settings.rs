use crate::{
    Colour,
    graphics::GraphicsSettings,
};

use glium::glutin::{
    window::WindowAttributes,
    PixelFormatRequirements,
};

#[derive(Clone,Debug)]
pub struct WindowSettings{
    //--General attributes--\\
    pub general:GeneralSettings,



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
        Self{
            //--General attributes--\\
            general:GeneralSettings::new(),

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

    /// The amount of the update events per second.
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