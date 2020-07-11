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
/// A window with graphic functions and an event listener included.
/// 
/// #
/// 
/// Все события обрабатываются и добавляются в очередь внешней обработки (Window.events)
/// для работы с ними вне структуры окна.
/// 
/// #
/// 
/// All events are handled and added to the outer handling queue (Window.events)
/// to work with them outside of the window structure.
/// 
pub struct Window{
    pub (crate) display:Display,
    pub (crate) graphics:Graphics2D,

    pub (crate) event_loop:EventLoop<()>,
    pub (crate) events:VecDeque<WindowEvent>,

    #[cfg(feature="auto_hide")]
    pub (crate) events_handler:fn(&mut Self),

    #[cfg(feature="fps_counter")]
    pub (crate) frames_passed:u32,
    #[cfg(feature="fps_counter")]
    pub (crate) time:Instant,

    // #[cfg(feature="alpha_smoothing")]
    pub (crate) alpha_channel:f32,
    // #[cfg(feature="alpha_smoothing")]
    pub (crate) smooth:f32,

    #[cfg(feature="mouse_cursor_icon")]
    pub (crate) mouse_icon:MouseCursorIcon,
}

use WindowEvent::*;

impl Window{
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

    /// mouse_cursor_icon_path, mouse_cursor_icon_range - feature = "mouse_cursor_icon"
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
            events:VecDeque::with_capacity(32),

            #[cfg(feature="fps_counter")]
            frames_passed:0u32,
            #[cfg(feature="fps_counter")]
            time:Instant::now(),

            #[cfg(feature="auto_hide")]
            events_handler:Window::event_listener,

            alpha_channel:0f32,
            smooth:0f32,
        })
    }

    /// Возвращает следующее событие окна.
    /// 
    /// Блокирует поток, пока не получит следующее событие.
    /// 
    /// Returns next window event.
    /// 
    /// Blocks the thread until it receives the next event.
    pub fn next_event(&mut self)->Option<WindowEvent>{
        while self.events.is_empty(){
            #[cfg(feature="auto_hide")]
            (self.events_handler)(self); // Вызов функции обработки событий

            #[cfg(not(feature="auto_hide"))]
            self.event_listener(); // Вызов функции обработки событий
        }
        self.events.pop_front()
    }
}


//                         \\
//    ЛОКАЛЬНЫЕ ФУНКЦИИ    \\
//                         \\

/// Функции обработки событий.
/// 
/// Event handlers.
impl Window{
    /// Обычная функция обработки событий
    fn event_listener(&mut self){
        let el=&mut self.event_loop as *mut EventLoop<()>;
        let event_loop=unsafe{&mut *el};

        event_loop.run_return(|event,_,control_flow|{
            *control_flow=ControlFlow::Exit;
            let next_event=match event{
                // События окна
                Event::WindowEvent{event,..}=>{
                    match event{
                        // Закрытие окна
                        GWindowEvent::CloseRequested=>{
                            Exit
                        }

                        // Изменение размера окна
                        GWindowEvent::Resized(size)=>unsafe{
                            window_width=size.width as f32;
                            window_height=size.height as f32;
                            window_center=[window_width/2f32,window_height/2f32];

                            #[cfg(feature="mouse_cursor_icon")]
                            self.mouse_icon.update(&mut self.graphics);

                            Resized([size.width,size.height])
                        }

                        // Сдвиг окна
                        GWindowEvent::Moved(pos)=>Moved([pos.x,pos.y]),

                        // Сдвиг мыши (сдвиг за пределы окна игнорируется)
                        GWindowEvent::CursorMoved{position,..}=>unsafe{
                            let last_position=mouse_cursor.position();

                            let position=[position.x as f32,position.y as f32];

                            let dx=position[0]-last_position[0];
                            let dy=position[1]-last_position[1];

                            mouse_cursor.set_position(position);

                            MouseMovementDelta([dx,dy])
                        }

                        // Прокрутка колёсика мыши
                        GWindowEvent::MouseWheel{delta,..}=>MouseWheelScroll(delta),

                        // Обработка действий с кнопками мыши (только стандартные кнопки)
                        GWindowEvent::MouseInput{button,state,..}=>{
                            if state==ElementState::Pressed{
                                match button{
                                    GMouseButton::Left=>{
                                        #[cfg(feature="mouse_cursor_icon")]
                                        self.mouse_icon.pressed(&mut self.graphics);

                                        MousePressed(MouseButton::Left)
                                    }
                                    GMouseButton::Middle=>MousePressed(MouseButton::Middle),
                                    GMouseButton::Right=>MousePressed(MouseButton::Right),
                                    GMouseButton::Other(_)=>return
                                }
                            }
                            else{
                                match button{
                                    GMouseButton::Left=>{
                                        #[cfg(feature="mouse_cursor_icon")]
                                        self.mouse_icon.released(&mut self.graphics);

                                        MouseReleased(MouseButton::Left)
                                    }
                                    GMouseButton::Middle=>MouseReleased(MouseButton::Middle),
                                    GMouseButton::Right=>MouseReleased(MouseButton::Right),
                                    GMouseButton::Other(_)=>return
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
                                KeyboardPressed(key)
                            }
                            else{
                                KeyboardReleased(key)
                            }
                        }

                        // Получение вводимых букв
                        GWindowEvent::ReceivedCharacter(character)=>if character.is_ascii_control(){
                            return
                        }
                        else{
                            CharacterInput(character)
                        }

                        // При потере фокуса
                        #[cfg(feature="auto_hide")]
                        GWindowEvent::Focused(f)=>if !f{
                            self.lost_focus()
                        }
                        else{
                            WindowEvent::Hide(false) // Передача события во внешнее управление
                        }

                        #[cfg(not(feature="auto_hide"))]
                        GWindowEvent::Focused(f)=>WindowEvent::Focused(f),

                        GWindowEvent::DroppedFile(path)=>DroppedFile(path),
                        GWindowEvent::HoveredFile(path)=>HoveredFile(path),
                        GWindowEvent::HoveredFileCancelled=>HoveredFileCancelled,

                        _=>return // Игнорирование остальных событий
                    }
                }

                Event::Suspended=>Suspended,
                Event::Resumed=>Resumed,

                // Запрос на рендеринг
                Event::MainEventsCleared=>{
                    self.display.gl_window().window().request_redraw();
                    return
                }

                // Рендеринг
                Event::RedrawRequested(_)=>{
                    #[cfg(feature="fps_counter")]
                    self.count_fps();
                    Draw
                }

                _=>return  // Игнорирование остальных событий
            };

            self.events.push_back(next_event)
        });
    }

    /// Функция ожидания получения фокуса - перехватывает управление до получения окном фокуса
    #[cfg(feature="auto_hide")]
    fn wait_until_focused(&mut self){
        let el=&mut self.event_loop as *mut EventLoop<()>;
        let event_loop=unsafe{&mut *el};

        event_loop.run_return(|event,_,control_flow|{
            *control_flow=ControlFlow::Wait;

            let event=match event{
                Event::WindowEvent{event,..}=>{
                    match event{
                        GWindowEvent::Resized(size)=>unsafe{
                            window_width=size.width as f32;
                            window_height=size.height as f32;
                            window_center=[window_width/2f32,window_height/2f32];

                            #[cfg(feature="mouse_cursor_icon")]
                            self.mouse_icon.update(&mut self.graphics);

                            return
                        }

                        GWindowEvent::CloseRequested=>{ // Остановка цикла обработки событий,
                            *control_flow=ControlFlow::Exit;
                            Exit // Передача события во внешнее управление
                        }

                        // При получении фокуса
                        GWindowEvent::Focused(_)=>{
                            *control_flow=ControlFlow::Exit;
                            self.gained_focus()
                        }

                        _=>return
                    }
                }

                Event::Suspended=>{
                    *control_flow=ControlFlow::Exit;
                    Suspended
                }

                Event::Resumed=>{
                    *control_flow=ControlFlow::Exit;
                    Resumed
                }

                _=>return
            };
            self.events.push_back(event);
        })
    }
}

/// Функции внутренней обработки событий.
/// 
/// Inner event handling functions.
impl Window{
    #[cfg(feature="auto_hide")]
    fn lost_focus(&mut self)->WindowEvent{
        self.display.gl_window().window().set_minimized(true); // Сворацивание окна
        self.events_handler=Window::wait_until_focused; // Смена фукции обработки событий

        WindowEvent::Hide(true) // Передача события во внешнее управление
    }

    /// При получении фокуса
    #[cfg(feature="auto_hide")]
    fn gained_focus(&mut self)->WindowEvent{
        self.events_handler=Window::event_listener; // Смена фукции обработки событий
        self.display.gl_window().window().set_minimized(false);

        Hide(false) // Передача события во внешнее управление
    }
}