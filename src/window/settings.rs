use crate::{
    Colour,
    graphics::GraphicsSettings,
};

use glium::glutin::{
    ReleaseBehavior,
    NotCurrent,
    ContextBuilder,
    dpi::Size,
    window::{
        WindowBuilder,
        WindowAttributes,
        Fullscreen,
        Icon
    },
};

use std::{
    ops::Range,
    path::PathBuf
};

#[derive(Clone,Debug)]
#[allow(dead_code)]
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

    /// If true, only hardware-accelerated formats will be considered.
    /// If false, only software renderers.
    /// None means "don't care".
    /// 
    /// Default is Some(true).
    pub hardware_accelerated:Option<bool>,

    /// Minimum number of bits for the color buffer, excluding alpha.
    /// None means "don't care".
    /// 
    /// The default is Some(24).
    pub color_bits:Option<u8>,

    /// If true, the color buffer must be in a floating point format.
    /// Using floating points allows you to write values outside of the [0.0, 1.0] range.
    /// 
    /// Default is false.
    pub float_color_buffer:bool,

    /// Minimum number of bits for the alpha in the color buffer.
    /// None means "don't care".
    /// 
    /// The default is Some(8).
    pub alpha_bits:Option<u8>,

    /// Minimum number of bits for the depth buffer.
    /// None means "don't care".
    /// 
    /// The default value is Some(24).
    pub depth_bits:Option<u8>,

    /// Minimum number of stencil bits.
    /// None means "don't care".
    /// 
    /// The default value is Some(8).
    pub stencil_bits:Option<u8>,

    /// If true, only double-buffered formats will be considered.
    /// If false, only single-buffer formats.
    /// None means "don't care".
    /// 
    /// The default is Some(true).
    pub double_buffer:Option<bool>,

    /// Contains the minimum number of samples per pixel in the color, depth and stencil buffers.
    /// None means "don't care".
    /// A value of Some(0) indicates that multisampling must not be enabled.
    /// 
    /// Default is None.
    pub multisampling:Option<u16>,

    /// If true, only stereoscopic formats will be considered.
    /// If false, only non-stereoscopic formats.
    /// 
    /// The default is false.
    pub stereoscopy:bool,

    /// If true, only sRGB-capable formats will be considered.
    /// If false, don't care.
    /// 
    /// The default is true.
    pub srgb:bool,


    /// The behavior when changing the current context.
    /// 
    /// Default is Flush.
    pub release_behavior:ReleaseBehavior,



    //--Local graphics attributes--\\
    pub graphics_base_settings:GraphicsSettings,
}

#[allow(dead_code)]
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
            hardware_accelerated:Option::None,
            color_bits:Some(24),
            float_color_buffer:false,
            alpha_bits:Some(8),
            depth_bits:Some(24),
            stencil_bits:Some(8),
            double_buffer:Some(true),
            multisampling:None,
            stereoscopy:false,
            srgb:true,
            release_behavior:ReleaseBehavior::Flush,

            //--Local graphics attributes--\\
            graphics_base_settings:GraphicsSettings::new(),
        }
    }

    #[cfg(feature="mouse_cursor_icon")]
    pub (crate) fn devide<'a>(self)->(
        WindowBuilder,
        ContextBuilder<'a,NotCurrent>,
        GraphicsSettings,
        GeneralSettings,
        MouseCursorIconSettings<PathBuf>
    ){
        let mut window_builder=WindowBuilder::default();
        window_builder.window=self.window_attributes;

        let mut context_builder=ContextBuilder::new();
        context_builder.gl_attr.vsync=self.vsync;
        context_builder.gl_attr.debug=self.debug;

        context_builder.pf_reqs.hardware_accelerated=self.hardware_accelerated;
        context_builder.pf_reqs.color_bits=self.color_bits;
        context_builder.pf_reqs.float_color_buffer=self.float_color_buffer;
        context_builder.pf_reqs.alpha_bits=self.alpha_bits;
        context_builder.pf_reqs.depth_bits=self.depth_bits;
        context_builder.pf_reqs.stencil_bits=self.stencil_bits;
        context_builder.pf_reqs.double_buffer=self.double_buffer;
        context_builder.pf_reqs.multisampling=self.multisampling;
        context_builder.pf_reqs.stereoscopy=self.stereoscopy;
        context_builder.pf_reqs.srgb=self.srgb;
        context_builder.pf_reqs.release_behavior=self.release_behavior;

        let mouse_cursor_icon_settings=self.mouse_cursor_icon_settings;

        (
            window_builder,
            context_builder,
            self.graphics_base_settings,
            self.general,
            mouse_cursor_icon_settings,
        )
    }

    #[cfg(not(feature="mouse_cursor_icon"))]
    pub (crate) fn devide<'a>(self)->(
        WindowBuilder,
        ContextBuilder<'a,NotCurrent>,
        GraphicsSettings,
        GeneralSettings,
    ){
        let window_attributes=WindowAttributes{
            inner_size:self.inner_size,
            min_inner_size:self.min_inner_size,
            max_inner_size:self.max_inner_size,
            resizable:self.resizable,
            fullscreen:self.fullscreen,
            title:self.title,
            maximized:self.maximized,
            visible:self.visible,
            transparent:self.transparent,
            decorations:self.decorations,
            always_on_top:self.always_on_top,
            window_icon:self.window_icon,
        };

        let mut window_builder=WindowBuilder::default();
        window_builder.window=window_attributes;

        let mut context_builder=ContextBuilder::new();
        context_builder.gl_attr.vsync=self.vsync;
        context_builder.gl_attr.debug=self.debug;

        context_builder.pf_reqs.hardware_accelerated=self.hardware_accelerated;
        context_builder.pf_reqs.color_bits=self.color_bits;
        context_builder.pf_reqs.float_color_buffer=self.float_color_buffer;
        context_builder.pf_reqs.alpha_bits=self.alpha_bits;
        context_builder.pf_reqs.depth_bits=self.depth_bits;
        context_builder.pf_reqs.stencil_bits=self.stencil_bits;
        context_builder.pf_reqs.double_buffer=self.double_buffer;
        context_builder.pf_reqs.multisampling=self.multisampling;
        context_builder.pf_reqs.stereoscopy=self.stereoscopy;
        context_builder.pf_reqs.srgb=self.srgb;
        context_builder.pf_reqs.release_behavior=self.release_behavior;
        
        let graphics_settings=GraphicsSettings{
            #[cfg(feature="texture_graphics")]
            texture_vertex_buffer_size:self.texture_vertex_buffer_size,
            #[cfg(feature="simple_graphics")]
            simple_vertex_buffer_size:self.simple_vertex_buffer_size,
            #[cfg(feature="text_graphics")]
            text_vertex_buffer_size:self.text_vertex_buffer_size,
        };

        (
            window_builder,
            context_builder,
            graphics_settings,
            self.general,
        )
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