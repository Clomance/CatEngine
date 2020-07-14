use crate::{
    Colour,
    graphics::{Graphics2D,Graphics,GraphicsSettings},
};

#[cfg(feature="mouse_cursor_icon")]
use super::mouse_cursor::MouseCursorIcon;
#[cfg(feature="mouse_cursor_icon")]
use super::MouseCursorIconSettings;

#[cfg(feature="fps_counter")]
use super::fps;

use super::{
    // statics
    window_width,
    window_height,
    window_center,
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
    backend::glutin::DisplayCreationError,
    SwapBuffersError
};

use glium::glutin::{
    ContextBuilder,
    NotCurrent,
    event_loop::EventLoop,
    window::WindowBuilder,
};

use image::{
    ImageFormat,
    ImageBuffer,
    DynamicImage
};

#[cfg(feature="fps_counter")]
use std::time::{Instant,Duration};

use std::path::Path;

#[cfg(feature="mouse_cursor_icon")]
use std::path::PathBuf;

/// Окно, включает в себя графические функции.
/// A window with graphic functions included.
pub struct WindowBase<E:'static>{
    pub display:Display,
    pub graphics:Graphics2D,
    pub event_loop:EventLoop<E>,

    #[cfg(feature="fps_counter")]
    pub frames_passed:u32,
    #[cfg(feature="fps_counter")]
    pub time:Instant,

    #[cfg(feature="alpha_smoothing")]
    pub alpha_channel:f32,
    #[cfg(feature="alpha_smoothing")]
    pub smooth:f32,

    #[cfg(feature="mouse_cursor_icon")]
    pub mouse_icon:MouseCursorIcon,
}

impl<E:'static> WindowBase<E>{
    pub fn raw(
        window_builder:WindowBuilder,
        context_builder:ContextBuilder<NotCurrent>,
        graphics_settings:GraphicsSettings,
        event_loop:EventLoop<E>,
        initial_colour:Option<Colour>,

        #[cfg(feature="mouse_cursor_icon")]
        mouse_cursor_icon_settings:MouseCursorIconSettings<PathBuf>,
    )->Result<WindowBase<E>,DisplayCreationError>{
        // Создание окна и привязывание графической библиотеки
        let display=Display::new(window_builder,context_builder,&event_loop)?;

        let size=display.gl_window().window().inner_size();
        unsafe{
            window_width=size.width as f32;
            window_height=size.height as f32;
            window_center=[window_width/2f32,window_height/2f32];
        }

        // Опреление поддерживаемой версии GLSL
        let Version(..,m,l)=display.get_supported_glsl_version();
        let glsl=match m{
            1 if l<3 =>{
                120
            }
            _=>{
                140
            }
        };

        if let Some([r,g,b,a])=initial_colour{
            let mut frame=display.draw();   //
            frame.clear_color(r,g,b,a);     // Заполнение окна
            frame.finish().unwrap();        //
        }

        // Отлючение курсора системы
        // Замена его собственным
        #[cfg(feature="mouse_cursor_icon")]
        display.gl_window().window().set_cursor_visible(false);

        #[cfg(feature="mouse_cursor_icon")]
        let mut graphics=Graphics2D::new(&display,graphics_settings,glsl);

        #[cfg(not(feature="mouse_cursor_icon"))]
        let graphics=Graphics2D::new(&display,graphics_settings,glsl);

        Ok(Self{
            #[cfg(feature="mouse_cursor_icon")]
            mouse_icon:MouseCursorIcon::new(mouse_cursor_icon_settings,&display,&mut graphics),

            graphics,
            display,

            event_loop,

            #[cfg(feature="fps_counter")]
            frames_passed:0u32,
            #[cfg(feature="fps_counter")]
            time:Instant::now(),

            #[cfg(feature="alpha_smoothing")]
            alpha_channel:0f32,
            #[cfg(feature="alpha_smoothing")]
            smooth:0f32,
        })
    }
}

/// Функции для рисования. Drawing functions.
impl<E:'static> WindowBase<E>{
    /// Даёт прямое управление над кадром.
    /// 
    /// Gives frame to raw drawing.
    pub fn draw_raw<F:FnOnce(&mut DrawParameters,&mut Frame)>(&self,f:F)->Result<(),SwapBuffersError>{
        let mut frame=self.display.draw();
        let mut draw_parameters=default_draw_parameters();
        f(&mut draw_parameters,&mut frame);
        frame.finish()
    }

    /// Выполняет замыкание (и рисует курсор, если `feature = "mouse_cursor_icon"`).
    /// 
    /// Executes the closure (and draws the mouse cursor if `feature = "mouse_cursor_icon"`).
    pub fn draw<F:FnOnce(&mut DrawParameters,&mut Graphics)>(&self,f:F)->Result<(),SwapBuffersError>{
        let mut draw_parameters=default_draw_parameters();

        let mut frame=self.display.draw();

        let mut g=Graphics::new(&self.graphics,&mut frame);

        f(&mut draw_parameters,&mut g);

        #[cfg(feature="mouse_cursor_icon")]
        self.mouse_icon.draw(&mut draw_parameters,&mut g);

        frame.finish()
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
    pub fn draw_smooth<F:FnOnce(f32,&mut DrawParameters,&mut Graphics)>(&mut self,f:F)->Result<f32,SwapBuffersError>{
        let mut draw_parameters=default_draw_parameters();

        let mut frame=self.display.draw();

        let mut g=Graphics::new(&mut self.graphics,&mut frame);

        f(self.alpha_channel,&mut draw_parameters,&mut g);

        #[cfg(feature="mouse_cursor_icon")]
        self.mouse_icon.draw(&mut draw_parameters,&mut g);

        self.alpha_channel+=self.smooth;
        
        match frame.finish(){
            Ok(())=>Ok(self.alpha_channel),
            Err(e)=>Err(e),
        }
    }
}

/// # Дополнительные функции. Additional functions.
impl<E:'static> WindowBase<E>{
    /// feature = "mouse_cursor_icon
    #[cfg(feature="mouse_cursor_icon")]
    #[inline(always)]
    pub fn set_user_cursor_visible(&mut self,visible:bool){
        self.mouse_icon.set_visible(visible);
    }

    /// feature = "mouse_cursor_icon
    #[cfg(feature="mouse_cursor_icon")]
    #[inline(always)]
    pub fn switch_cursor_visibility(&mut self){
        self.mouse_icon.switch_visibility()
    }

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


/// # feature = "alpha_smoothing"
#[cfg(feature="alpha_smoothing")]
impl<E:'static> WindowBase<E>{
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

//                     \\
//  ЛОКАЛЬНЫЕ ФУНКЦИИ  \\
//                     \\
impl<E:'static> WindowBase<E>{
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