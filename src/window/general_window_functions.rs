use super::{
    Window
};

use crate::{
    Colour,
    graphics::{Graphics2D,Graphics,GraphicsSettings},
};

#[cfg(feature="mouse_cursor_icon")]
use super::mouse_cursor::MouseCursorIcon;

#[cfg(feature="fps_counter")]
use super::fps;

use super::{
    // statics
    window_width,
    window_height,
    window_center,
    mouse_cursor,
    // enums
    WindowSettings,
    WindowEvent,
    MouseButton,
    KeyboardButton,
};

use glium::{
    Display,
    Surface,
    Frame,
    Version,
    draw_parameters::{
        DrawParameters,
        Blend,
        BlendingFunction,
        LinearBlendingFactor,
        BackfaceCullingMode,
    },
    texture::RawImage2d,
    backend::glutin::DisplayCreationError
};

use glium::glutin::{
    ContextBuilder,
    NotCurrent,
    monitor::MonitorHandle,
    event_loop::{ControlFlow,EventLoop},
    event::{
        Event,
        WindowEvent as GWindowEvent,
        MouseButton as GMouseButton,
        ElementState,
    },
    window::{Fullscreen,WindowBuilder},
    platform::desktop::EventLoopExtDesktop,
    dpi::Size,
};

use image::{
    ImageFormat,
    ImageBuffer,
    DynamicImage
};

use std::{
    collections::VecDeque,
    path::{Path,PathBuf},
    ops::Range,
    time::{Instant,Duration},
};
impl Window{
    #[inline(always)]
    pub fn display(&self)->&Display{
        &self.display
    }

    /// Возвращает графическую основу.
    /// 
    /// Returns graphic base.
    #[inline(always)]
    pub fn graphics(&mut self)->&mut Graphics2D{
        &mut self.graphics
    }

    #[inline(always)]
    pub fn available_monitors(&self)->impl std::iter::Iterator<Item=MonitorHandle>{
        self.event_loop.available_monitors()
    }
}

impl Window{
    pub fn set_inner_size<S:Into<Size>>(&self,size:S){
        self.display.gl_window().window().set_inner_size(size)
    }

    pub fn set_min_inner_size<S:Into<Size>>(&self,size:Option<S>){
        self.display.gl_window().window().set_min_inner_size(size)
    }

    pub fn set_max_inner_size<S:Into<Size>>(&self,size:Option<S>){
        self.display.gl_window().window().set_max_inner_size(size)
    }

    pub fn set_title(&self,title:&str){
        self.display.gl_window().window().set_title(title)
    }

    pub fn set_visible(&self,visible:bool){
        self.display.gl_window().window().set_visible(visible)
    }

    pub fn set_resizable(&self,resizable:bool){
        self.display.gl_window().window().set_resizable(resizable)
    }


    pub fn choose_fullscreen_monitor(&self,monitor:usize)->Result<(),()>{
        if let Some(m)=self.available_monitors().nth(monitor){
            self.display.gl_window().window().set_fullscreen(Some(Fullscreen::Borderless(m)));
            Ok(())
        }
        else{
            Err(())
        }
    }


    pub fn set_fullscreen(&self,fullscreen:Option<Fullscreen>){
        self.display.gl_window().window().set_fullscreen(fullscreen)
    }

    /// Сворачивает окно.
    /// 
    /// Minimizes the window.
    #[inline(always)]
    pub fn set_minimized(&self,minimized:bool){
        self.display.gl_window().window().set_minimized(minimized)
    }

    /// Делает окно максимального размера.
    /// 
    /// Maximizes the window.
    #[inline(always)]
    pub fn set_maximized(&self,maximized:bool){
        self.display.gl_window().window().set_maximized(maximized)
    }

    pub fn set_decorations(&self,decorations:bool){
        self.display.gl_window().window().set_decorations(decorations)
    }

    pub fn set_always_on_top(&self,always_on_top:bool){
        self.display.gl_window().window().set_always_on_top(always_on_top)
    }

    #[inline(always)]
    pub fn set_cursor_visible(&mut self,visible:bool){
        #[cfg(feature="mouse_cursor_icon")]
        self.mouse_icon.set_visible(visible);

        #[cfg(not(feature="mouse_cursor_icon"))]
        self.display.gl_window().window().set_cursor_visible(visible);
    }

    #[cfg(feature="mouse_cursor_icon")]
    #[inline(always)]
    pub fn switch_cursor_visibility(&mut self){
        self.mouse_icon.switch_visibility()
    }
}

/// # Версии OpenGL. OpenGL versions.
impl Window{
    #[inline(always)]
    pub fn get_supported_glsl_version(&self)->Version{
        self.display.get_supported_glsl_version()
    }
    #[inline(always)]
    pub fn get_opengl_version(&self)->&Version{
        self.display.get_opengl_version()
    }
}


/// # feature = "alpha_smoothing"
#[cfg(feature="alpha_smoothing")]
impl Window{
    /// Sets alpha channel for smooth drawing.
    pub fn set_alpha(&mut self,alpha:f32){
        self.alpha_channel=alpha;
    }

    /// Sets smooth for smooth drawing.
    pub fn set_smooth(&mut self,smooth:f32){
        self.smooth=smooth
    }

    /// Sets smooth and zero alpha channel
    /// for smooth drawing.
    pub fn set_new_smooth(&mut self,smooth:f32){
        self.alpha_channel=0f32;
        self.smooth=smooth
    }
}

/// # Функции для рисования. Drawing functions.
impl Window{
    /// Даёт прямое управление над кадром.
    /// 
    /// Gives frame to raw drawing.
    pub fn draw_raw<F:FnOnce(&mut DrawParameters,&mut Frame)>(&self,f:F){
        let mut frame=self.display().draw();
        let mut draw_parameters=default_draw_parameters();
        f(&mut draw_parameters,&mut frame);
        frame.finish();
    }

    /// Выполняет замыкание (и рисует курсор, если `feature = "mouse_cursor_icon"`).
    /// 
    /// Executes the closure (and draws the mouse cursor if `feature = "mouse_cursor_icon"`).
    pub fn draw<F:FnOnce(&mut DrawParameters,&mut Graphics)>(&self,f:F){
        let mut draw_parameters=default_draw_parameters();

        let mut frame=self.display().draw();

        let mut g=Graphics::new(&self.graphics,&mut frame);

        f(&mut draw_parameters,&mut g);

        #[cfg(feature="mouse_cursor_icon")]
        self.mouse_icon.draw(&mut draw_parameters,&mut g);

        frame.finish();
    }

    /// Выполняет замыкание (и рисует курсор, если `feature = "mouse_cursor_icon"`).
    /// Выдаёт альфа-канал, возвращает его следующее значение.
    /// 
    /// Нужна для рисования с изменяющимся альфа-канала.
    /// 
    /// Executes closure (and draws the mouse cursor if `feature = "mouse_cursor_icon"`).
    /// Gives alpha channel, returns it's next value.
    /// 
    /// Needed for drawing with changing alpha channel.
    /// 
    /// feature = "alpha_smoothing"
    #[cfg(feature="alpha_smoothing")]
    pub fn draw_smooth<F:FnOnce(f32,&mut DrawParameters,&mut Graphics)>(&mut self,f:F)->f32{
        let mut draw_parameters=default_draw_parameters();

        let mut frame=self.display().draw();

        let mut g=Graphics::new(&mut self.graphics,&mut frame);

        f(self.alpha_channel,&mut draw_parameters,&mut g);

        #[cfg(feature="mouse_cursor_icon")]
        self.mouse_icon.draw(&mut draw_parameters,&mut g);

        frame.finish();

        self.alpha_channel+=self.smooth;
        self.alpha_channel
    }
}

/// # Дополнительные функции. Additional functions.
impl Window{
    /// Возвращает скриншот.
    /// 
    /// Returns a screenshot.
    pub fn screenshot(&self)->Option<DynamicImage>{
        // Копирование буфера окна
        let image:RawImage2d<u8>=match self.display.read_front_buffer(){
            Ok(t)=>t,
            Err(_)=>return Option::None
        };
        // Перевод в буфер изображения
        let image=match ImageBuffer::from_raw(image.width,image.height,image.data.into_owned()){
            Option::Some(i)=>i,
            Option::None=>return Option::None
        };
        // Перевод в изображение
        Some(DynamicImage::ImageRgba8(image).flipv())
    }
    /// Сохраняет скриншот в формате png.
    /// 
    /// Saves a screenshot in png format.
    pub fn save_screenshot<P:AsRef<Path>>(&self,path:P){
        // Копирование буфера окна
        let image:RawImage2d<u8>=match self.display.read_front_buffer(){
            Ok(t)=>t,
            Err(_)=>return
        };
        // Перевод в буфер изображения
        let image=match ImageBuffer::from_raw(image.width,image.height,image.data.into_owned()){
            Option::Some(i)=>i,
            Option::None=>return
        };
        // Перевод в изображение
        let image=DynamicImage::ImageRgba8(image).flipv();
        // Сохранение
        if let Err(_)=image.save_with_format(path,ImageFormat::Png){
            return
        }
    }
}

impl Window{
    #[cfg(feature="fps_counter")]
    pub (crate) fn count_fps(&mut self){
        self.frames_passed+=1;
        let current_time=Instant::now();
        let time_passed=current_time.duration_since(self.time);

        if Duration::from_secs(1)<time_passed{
            unsafe{
                fps=self.frames_passed;
            }
            self.frames_passed=0;
            self.time=current_time;
        }
    }
}

// Обычные параметры для рисования
fn default_draw_parameters<'a>()->DrawParameters<'a>{
    let mut draw_parameters=DrawParameters::default();

    draw_parameters.blend=Blend{
        color:BlendingFunction::Addition{
            source:LinearBlendingFactor::SourceAlpha,
            destination:LinearBlendingFactor::OneMinusSourceAlpha,
        },
        alpha:BlendingFunction::Addition{
            source:LinearBlendingFactor::One,
            destination:LinearBlendingFactor::One,
        },
        constant_value:(0.0,0.0,0.0,0.0),
    };

    draw_parameters.backface_culling=BackfaceCullingMode::CullingDisabled;

    draw_parameters
}