#![allow(unused_mut)]

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
        ModifiersState,
        MouseScrollDelta
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
    path::{Path,PathBuf},
    ops::Range,
    time::{Instant,Duration},
};

/*
    EventLoop - минимум четыре шага:
    1) NewEvent
    (Основные события)
    2) MainEventsCleared
    (RedrawRequested)
    3) RedrawEventsCleared
    4) LoopDestroyed
*/

/// Окно, включает в себя графические функции
/// и обработчик событий.
/// Window with graphic functions
/// and an event listener included.
/// 
/// #
/// 
/// Все события прописываются с помощь типажа `WindowPage`
/// и обработываются сразу же после их появления.
/// 
/// #
/// 
/// All events are implemented with `WindowPage`
/// and handled immediately after emited.
/// 
/// # feature = "mouse_cursor_icon"
/// 
/// Замена обычного курсора пользовательской картинкой.
/// 
/// Путь для картинки по умолчанию - `./mouse_cursor_icon.png`.
/// 
/// ImageBase для этой картинки добавляется в texture vertex buffer.
/// Область по умочанию 4..8.
/// 
/// #
/// 
/// Replaces the default mouse cursor with user's image.
/// 
/// The cursor points to the center of the image.
/// 
/// The default path to the image is `./mouse_cursor_icon.png`.
/// 
/// The ImageBase for the image binds to the texture vertex buffer.
/// The default range is 4..8.
/// 
/// # feature = "auto_hide"
/// 
/// При потере фокуса окно сворачивается,
/// и выполняется функция `on_focused`.
/// При получении фокуса окно возвращается в исходное состояние
/// (функция `on_focused` тоже выполняется).
/// 
/// #
/// 
/// The window gets minimized when loses focus and
/// the `on_focused` function is executed.
/// The window gets back when it gains focus (the `on_focused` function is executed, too).
/// 
/// # feature = "fps_counter"
/// 
/// Простой счётчик fps. Обновляется каждую секунду.
/// 
/// #
/// 
/// A simple fps counter. The value updates every second.
/// 
/// # feature = "paged_format"
/// 
/// Заменяет обычную структуру окна на эту.
/// 
/// Replaces the default window struct with this one.
pub struct Window{
    display:Display,
    graphics:Graphics2D,

    event_loop:EventLoop<()>,

    #[cfg(feature="fps_counter")]
    frames_passed:u32,
    #[cfg(feature="fps_counter")]
    time:Instant,

    alpha_channel:f32,  // Для плавных
    smooth:f32,         // переходов

    #[cfg(feature="mouse_cursor_icon")]
    mouse_icon:MouseCursorIcon,
}


impl Window{
    //pub fn new_settings(settigs:WindowSettings)->Result<Window,DisplayCreationError>{}

    /// Создаёт окно. Принимает функцию для настройки.
    ///
    /// Creates the window. 
    pub fn new<F>(setting:F)->Result<Window,DisplayCreationError>
            where F:FnOnce(Vec<MonitorHandle>,&mut WindowSettings){
        let event_loop=EventLoop::new();
        let monitors=event_loop.available_monitors().collect();

        let mut window_settings=WindowSettings::new();


        // Настройка
        setting(monitors,&mut window_settings);

        let initial_colour=window_settings.initial_colour;
        #[cfg(feature="mouse_cursor_icon")]
        let mouse_cursor_icon_path=window_settings.mouse_cursor_icon_path.clone();
        #[cfg(feature="mouse_cursor_icon")]
        let mouse_cursor_icon_range=window_settings.mouse_cursor_icon_range.clone();

        let (window_builder,context_builder,graphics_settings)
                =window_settings.devide();

        #[cfg(feature="mouse_cursor_icon")]
        let window=Window::raw(
            window_builder,
            context_builder,
            graphics_settings,
            event_loop,
            initial_colour,
            mouse_cursor_icon_path,
            mouse_cursor_icon_range,
        );

        #[cfg(not(feature="mouse_cursor_icon"))]
        let window=Window::raw::<PathBuf>(
            window_builder,
            context_builder,
            graphics_settings,
            event_loop,
            initial_colour,
        );

        window
    }

    /// mouse_cursor_icon_path - feature = "mouse_cursor_icon"
    /// mouse_cursor_icon_range - feature = "mouse_cursor_icon"
    pub fn raw<P:AsRef<Path>>(
        window_builder:WindowBuilder,
        context_builder:ContextBuilder<NotCurrent>,
        graphics_settings:GraphicsSettings,
        event_loop:EventLoop<()>,
        initial_colour:Option<Colour>,
        #[cfg(feature="mouse_cursor_icon")]
        mouse_cursor_icon_path:P,
        #[cfg(feature="mouse_cursor_icon")]
        mouse_cursor_icon_range:Range<usize>,
    )->Result<Window,DisplayCreationError>{
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

        let mut graphics=Graphics2D::new(&display,graphics_settings,glsl);

        Ok(Self{
            #[cfg(feature="mouse_cursor_icon")]
            mouse_icon:MouseCursorIcon::new(mouse_cursor_icon_path,mouse_cursor_icon_range,&display,&mut graphics),

            graphics,
            display,

            event_loop,

            #[cfg(feature="fps_counter")]
            frames_passed:0u32,
            #[cfg(feature="fps_counter")]
            time:Instant::now(),

            // #[cfg(feature="auto_hide")]
            // events_handler:Window::event_listener,

            alpha_channel:0f32,
            smooth:0f32,
        })
    }

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

    /// Запускает данную страницу.
    /// 
    /// Starts the given page.
    pub fn run<P:WindowPage>(&mut self,page:&mut P){
        let el=&mut self.event_loop as *mut EventLoop<()>;
        let event_loop=unsafe{&mut *el};

        #[cfg(not(feature="auto_hide"))]
        self.event_listener(event_loop,page);

        #[cfg(feature="auto_hide")]
        loop{
            if self.event_listener(event_loop,page){
                break
            }
            if self.wait_until_focused(event_loop,page){
                break
            }
        }
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

/// # Функции для сглаживания. Functions for smoothing.
impl Window{
    /// Set alpha channel for smooth drawing.
    pub fn set_alpha(&mut self,alpha:f32){
        self.alpha_channel=alpha;
    }

    /// Set smooth for smooth drawing.
    pub fn set_smooth(&mut self,smooth:f32){
        self.smooth=smooth
    }

    /// Set smooth and zero alpha channel
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
    /// Gives the frame to raw drawing.
    pub fn draw_raw<F:FnOnce(&mut DrawParameters,&mut Frame)>(&self,f:F){
        let mut frame=self.display().draw();
        let mut draw_parameters=default_draw_parameters();
        f(&mut draw_parameters,&mut frame);
        frame.finish();
    }

    /// Выполняет замыкание (и рисует курсор, если `feature="mouse_cursor_icon"`).
    /// 
    /// Executes the closure (and draws the mouse cursor if `feature="mouse_cursor_icon"`).
    pub fn draw<F:FnOnce(&mut DrawParameters,&mut Graphics)>(&self,f:F){
        let mut draw_parameters=default_draw_parameters();

        let mut frame=self.display().draw();

        let mut g=Graphics::new(&self.graphics,&mut frame);

        f(&mut draw_parameters,&mut g);

        #[cfg(feature="mouse_cursor_icon")]
        self.mouse_icon.draw(&mut draw_parameters,&mut g);

        frame.finish();
    }

    /// Выполняет замыкание (и рисует курсор, если `feature="mouse_cursor_icon"`).
    /// Выдаёт альфа-канал для рисования, возвращает следующее значение канала.
    /// 
    /// Нужна для плавных переходов или размытия с помощью альфа-канала.
    /// 
    /// Executes closure (and draws the mouse cursor if `feature="mouse_cursor_icon"`).
    /// Gives alpha channel for drawing, returns the next value of the channel.
    /// 
    /// Needed for smooth drawing or smoothing with alpha channel.
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

//                         \\
//    ЛОКАЛЬНЫЕ ФУНКЦИИ    \\
//                         \\

/// Функции обработки событий.
/// 
/// Event handlers.
impl Window{
    fn event_listener<P:WindowPage>(&mut self,event_loop:&mut EventLoop<()>,page:&mut P)->bool{
        let mut close_flag=false;

        event_loop.run_return(|event,_,control_flow|{
            *control_flow=ControlFlow::Wait;
            match event{
                // События окна
                Event::WindowEvent{event,..}=>{
                    match event{
                        // Закрытие окна
                        GWindowEvent::CloseRequested=>{
                            *control_flow=ControlFlow::Exit;
                            close_flag=true;
                            page.on_close_requested(self);
                        }

                        // Изменение размера окна
                        GWindowEvent::Resized(size)=>unsafe{
                            window_width=size.width as f32;
                            window_height=size.height as f32;
                            window_center=[window_width/2f32,window_height/2f32];

                            #[cfg(feature="mouse_cursor_icon")]
                            self.mouse_icon.update(&mut self.graphics);

                            page.on_window_resized(self,[size.width,size.height])
                        }

                        // Сдвиг окна
                        GWindowEvent::Moved(pos)=>page.on_window_moved(self,[pos.x,pos.y]),

                        // Сдвиг мыши (сдвиг за пределы окна игнорируется)
                        GWindowEvent::CursorMoved{position,..}=>unsafe{
                            let last_position=mouse_cursor.position();

                            let position=[position.x as f32,position.y as f32];

                            let dx=position[0]-last_position[0];
                            let dy=position[1]-last_position[1];

                            mouse_cursor.set_position(position);

                            page.on_mouse_moved(self,[dx,dy])
                        }

                        // Прокрутка колёсика мыши
                        GWindowEvent::MouseWheel{delta,..}=>page.on_mouse_scrolled(self,delta),

                        // Обработка действий с кнопками мыши (только стандартные кнопки)
                        GWindowEvent::MouseInput{button,state,..}=>{
                            if state==ElementState::Pressed{
                                match button{
                                    GMouseButton::Left=>{
                                        #[cfg(feature="mouse_cursor_icon")]
                                        self.mouse_icon.pressed(&mut self.graphics);

                                        page.on_mouse_pressed(self,MouseButton::Left)
                                    }
                                    GMouseButton::Middle=>page.on_mouse_pressed(self,MouseButton::Middle),
                                    GMouseButton::Right=>page.on_mouse_pressed(self,MouseButton::Right),
                                    GMouseButton::Other(_)=>{}
                                }
                            }
                            else{
                                match button{
                                    GMouseButton::Left=>{
                                        #[cfg(feature="mouse_cursor_icon")]
                                        self.mouse_icon.released(&mut self.graphics);

                                        page.on_mouse_released(self,MouseButton::Left)
                                    }
                                    GMouseButton::Middle=>page.on_mouse_released(self,MouseButton::Middle),
                                    GMouseButton::Right=>page.on_mouse_released(self,MouseButton::Right),
                                    GMouseButton::Other(_)=>{}
                                }
                            }
                        }

                        // Обработка действий с клавишами клавиатуры
                        GWindowEvent::KeyboardInput{input,..}=>{
                            let key=if let Some(key)=input.virtual_keycode{
                                unsafe{std::mem::transmute(key)}
                            }
                            else{
                                KeyboardButton::Unknown
                            };

                            if input.state==ElementState::Pressed{
                                page.on_keyboard_pressed(self,key)
                            }
                            else{
                                page.on_keyboard_released(self,key)
                            }
                        }

                        // Получение вводимых букв
                        GWindowEvent::ReceivedCharacter(character)=>if !character.is_ascii_control(){
                            page.on_character_recieved(self,character)
                        }

                        // При потере фокуса
                        #[cfg(feature="auto_hide")]
                        GWindowEvent::Focused(f)=>if !f{
                            *control_flow=ControlFlow::Exit;
                            self.display.gl_window().window().set_minimized(true); // Сворацивание окна
                            page.on_window_focused(self,f);
                        }

                        #[cfg(not(feature="auto_hide"))]
                        GWindowEvent::Focused(f)=>page.on_window_focused(self,f),

                        // GWindowEvent::DroppedFile(path)=>DroppedFile(path),
                        // GWindowEvent::HoveredFile(path)=>HoveredFile(path),
                        // GWindowEvent::HoveredFileCancelled=>HoveredFileCancelled,

                        _=>{} // Игнорирование остальных событий
                    }
                }

                Event::Suspended=>page.on_suspended(self),
                Event::Resumed=>page.on_resumed(self),

                // Запрос на рендеринг
                Event::MainEventsCleared=>{
                    self.display.gl_window().window().request_redraw();
                }

                // Рендеринг
                Event::RedrawRequested(_)=>{
                    #[cfg(feature="fps_counter")]
                    self.count_fps();

                    page.on_redraw_requested(self);
                }

                _=>{}
            }
        });

        close_flag
    }



    /// Функция ожидания получения фокуса - перехватывает управление до получения окном фокуса
    #[cfg(feature="auto_hide")]
    fn wait_until_focused<P:WindowPage>(&mut self,event_loop:&mut EventLoop<()>,page:&mut P)->bool{
        let mut close_flag=false;

        event_loop.run_return(|event,_,control_flow|{
            *control_flow=ControlFlow::Wait;

            match event{
                Event::WindowEvent{event,..}=>{
                    match event{
                        GWindowEvent::Resized(size)=>unsafe{
                            window_width=size.width as f32;
                            window_height=size.height as f32;
                            window_center=[window_width/2f32,window_height/2f32];

                            #[cfg(feature="mouse_cursor_icon")]
                            self.mouse_icon.update(&mut self.graphics);

                            page.on_window_resized(self,[size.width,size.height])
                        }

                        GWindowEvent::CloseRequested=>{ // Остановка цикла обработки событий,
                            *control_flow=ControlFlow::Exit;
                            close_flag=true;
                            page.on_close_requested(self)
                        }

                        // При получении фокуса
                        GWindowEvent::Focused(f)=>{
                            *control_flow=ControlFlow::Exit;
                            self.display.gl_window().window().set_minimized(false);
                            page.on_window_focused(self,f);
                        }

                        _=>return
                    }
                }

                Event::Suspended=>page.on_suspended(self),
                Event::Resumed=>page.on_resumed(self),

                _=>return
            }
        });

        close_flag
    }
}

/// Функции внутренней обработки событий.
/// 
/// Inner event handling functions.
impl Window{
    #[cfg(feature="fps_counter")]
    fn count_fps(&mut self){
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

/// Типаж для создания страниц окна.
/// Trait for implementing window pages.
pub trait WindowPage{
    fn on_close_requested(&mut self,window:&mut Window);
    fn on_redraw_requested(&mut self,window:&mut Window);

    fn on_mouse_pressed(&mut self,window:&mut Window,button:MouseButton);
    fn on_mouse_released(&mut self,window:&mut Window,button:MouseButton);
    fn on_mouse_scrolled(&mut self,window:&mut Window,scroll:MouseScrollDelta);
    fn on_mouse_moved(&mut self,window:&mut Window,position:[f32;2]);

    fn on_keyboard_pressed(&mut self,window:&mut Window,button:KeyboardButton);
    fn on_keyboard_released(&mut self,window:&mut Window,button:KeyboardButton);
    fn on_character_recieved(&mut self,window:&mut Window,character:char);

    fn on_window_resized(&mut self,window:&mut Window,new_size:[u32;2]);
    fn on_window_moved(&mut self,window:&mut Window,position:[i32;2]);

    fn on_window_focused(&mut self,window:&mut Window,focused:bool);

    fn on_suspended(&mut self,window:&mut Window);
    fn on_resumed(&mut self,window:&mut Window);
}