use crate::graphics::{
    Graphics,
    GraphicsSettings,
    two_dimensions::Graphics2D,
};

#[cfg(feature="3D")]
use crate::graphics::three_dimensions::Graphics3D;

#[cfg(feature="fps_counter")]
use super::fps;

#[cfg(feature="ups_counter")]
use super::ups;

use super::{
    // statics
    window_width,
    window_height,
    window_center,
    // enums
    InnerWindowEvent,
    // structs
    GeneralSettings,
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
    SwapBuffersError,
};

use glium::glutin::{
    ContextBuilder,
    NotCurrent,
    event_loop::{EventLoop,EventLoopClosed,EventLoopProxy},
    window::WindowBuilder,
};

use image::{
    ImageFormat,
    ImageBuffer,
    DynamicImage
};

use std::{
    path::Path,
    time::{Instant,Duration}
};

/// Основа для окон для создания более сложных окон.
/// A window base for creating more complex windows.
pub struct WindowBase{
    /// A window with a GL context.
    pub display:Display,

    /// An event loop.
    pub event_loop:EventLoop<InnerWindowEvent>,

    /// Used to send custom events to the event loop.
    pub event_loop_proxy:EventLoopProxy<InnerWindowEvent>,

    /// feature != "lazy"
    #[cfg(not(feature="lazy"))]
    pub update_interval:Duration,
    /// feature != "lazy"
    #[cfg(not(feature="lazy"))]
    pub next_update:Instant,

    /// feature = "fps_counter"
    #[cfg(feature="fps_counter")]
    pub frames_passed:u32,
    /// feature = "ups_counter"
    #[cfg(feature="ups_counter")]
    pub updates_passed:u32,
    /// feature = "fps_counter"
    #[cfg(any(feature="fps_counter",feature="ups_counter"))]
    pub time:Instant,
}

impl WindowBase{
    pub fn raw(
        window_builder:WindowBuilder,
        context_builder:ContextBuilder<NotCurrent>,
        graphics_settings:GraphicsSettings,
        event_loop:EventLoop<InnerWindowEvent>,
        general_settings:GeneralSettings,
    )->Result<(WindowBase,Graphics2D),DisplayCreationError>{
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

        if let Some([r,g,b,a])=general_settings.initial_colour{
            let mut frame=display.draw();   //
            frame.clear_color(r,g,b,a);     // Заполнение окна
            frame.finish().unwrap();        //
        }

        let graphics2d=Graphics2D::new(&display,graphics_settings,glsl);

        let proxy=event_loop.create_proxy();

        #[cfg(not(feature="lazy"))]
        let update_interval=Duration::from_secs(1).checked_div(general_settings.updates_per_second).expect("UPD = 0");
        // #[cfg(not(feature="lazy"))]
        // println!("{:?}",update_interval);

        Ok((
            Self{
                display,

                event_loop,
                event_loop_proxy:proxy,

                #[cfg(not(feature="lazy"))]
                update_interval,
                #[cfg(not(feature="lazy"))]
                next_update:Instant::now(),

                #[cfg(feature="fps_counter")]
                frames_passed:0u32,
                #[cfg(feature="ups_counter")]
                updates_passed:0u32,
                #[cfg(any(feature="fps_counter",feature="ups_counter"))]
                time:Instant::now(),
            },
            graphics2d,
        ))
    }

    /// Останавливает цикл событий,
    /// отправляя событие для остановки.
    /// 
    /// Возвращает `Err`, если цикл уже остановлен.
    /// 
    /// Stops the event loop
    /// by sending the stopping event.
    /// 
    /// Returns `Err` if the loop has been already stopped.
    #[inline(always)]
    pub fn request_event_loop_close(&self)->Result<(),EventLoopClosed<InnerWindowEvent>>{
        self.event_loop_proxy.send_event(InnerWindowEvent::EventLoopCloseRequested)
    }
}

/// Функции для рисования. Drawing functions.
impl WindowBase{
    /// Выполняет замыкание.
    /// 
    /// Executes the closure.
    pub fn draw<F:FnOnce(&mut Graphics<Frame>)>(&self,graphics_base:&Graphics2D,f:F)->Result<(),SwapBuffersError>{
        let mut draw_parameters=default_draw_parameters();

        let mut frame=self.display.draw();

        let mut g=Graphics::new(
            graphics_base,
            draw_parameters,
            &mut frame
        );

        f(&mut g);

        frame.finish()
    }
}

/// # Дополнительные функции. Additional functions.
impl WindowBase{
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


//                     \\
//  ЛОКАЛЬНЫЕ ФУНКЦИИ  \\
//                     \\
impl WindowBase{
    #[cfg(feature="fps_counter")]
    pub (crate) fn count_fps(&mut self){
        self.frames_passed+=1;
    }

    #[cfg(feature="ups_counter")]
    pub (crate) fn count_ups(&mut self){
        self.updates_passed+=1;
    }

    #[cfg(not(feature="lazy"))]
    pub (crate) fn update_check(&mut self){
        let now=Instant::now();
        if self.next_update<=now{
            self.event_loop_proxy
                    .send_event(InnerWindowEvent::Update)
                            .expect("Dead event loop");

            self.next_update+=self.update_interval;
        }
    }

    #[cfg(any(feature="fps_counter",feature="ups_counter"))]
    pub fn check_counters(&mut self){
        let current_time=Instant::now();
        let time_passed=current_time.duration_since(self.time);

        if Duration::from_secs(1)<time_passed{
            #[cfg(feature="fps_counter")]
            unsafe{
                fps=self.frames_passed;
                self.frames_passed=0;
            }
            #[cfg(feature="ups_counter")]
            unsafe{
                ups=self.updates_passed;
                self.updates_passed=0;
            }

            self.time=current_time;
        }
    }
}


pub fn default_draw_parameters<'a>()->DrawParameters<'a>{
    DrawParameters{
        blend:Blend{
            color:BlendingFunction::Addition{
                source:LinearBlendingFactor::SourceAlpha,
                destination:LinearBlendingFactor::OneMinusSourceAlpha,
            },
            alpha:BlendingFunction::Addition{
                source:LinearBlendingFactor::One,
                destination:LinearBlendingFactor::One,
            },
            constant_value:(1f32,1f32,1f32,1f32),
        },

        backface_culling:BackfaceCullingMode::CullingDisabled,

        ..DrawParameters::default()
    }
}