#![allow(unused_mut,dead_code)]

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
    WindowEvent,
    InnerWindowEvent,
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
    event_loop::{ControlFlow,EventLoop,EventLoopProxy,EventLoopClosed},
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
    (проверяется ControlFlow)
    4) LoopDestroyed
*/

/// Окно, включает в себя графические функции
/// и обработчик событий.
/// A window with graphic functions
/// and an event listener included.
/// 
/// #
/// 
/// Все события прописываются с помощь типажа `WindowPage`
/// и обработываются сразу же после их появления.
/// 
/// #
/// 
/// All the events are implemented with `WindowPage`
/// and handled immediately after emited.
/// 
pub struct Window{
    pub (crate) display:Display,
    pub (crate) graphics:Graphics2D,

    pub (crate) event_loop:EventLoop<InnerWindowEvent>,
    pub (crate) event_loop_proxy:EventLoopProxy<InnerWindowEvent>,

    #[cfg(feature="fps_counter")]
    pub (crate) frames_passed:u32,
    #[cfg(feature="fps_counter")]
    pub (crate) time:Instant,

    pub (crate) alpha_channel:f32,  // Для плавных
    pub (crate) smooth:f32,         // переходов

    #[cfg(feature="mouse_cursor_icon")]
    pub (crate) mouse_icon:MouseCursorIcon,
}


impl Window{
    //pub fn new_settings(settigs:WindowSettings)->Result<Window,DisplayCreationError>{}

    /// Создаёт окно. Принимает функцию для настройки.
    ///
    /// Creates the window. 
    pub fn new<F>(setting:F)->Result<Window,DisplayCreationError>
            where F:FnOnce(Vec<MonitorHandle>,&mut WindowSettings){
        let event_loop=EventLoop::<InnerWindowEvent>::with_user_event();
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
        event_loop:EventLoop<InnerWindowEvent>,
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

        let proxy=event_loop.create_proxy();

        Ok(Self{
            #[cfg(feature="mouse_cursor_icon")]
            mouse_icon:MouseCursorIcon::new(mouse_cursor_icon_path,mouse_cursor_icon_range,&display,&mut graphics),

            graphics,
            display,

            event_loop,
            event_loop_proxy:proxy,

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

    /// Запускает обработку событий с помощью данного замыкания.
    /// 
    /// Starts event handling with the given closure.
    pub fn run<F:FnMut(&mut Window,WindowEvent)>(&mut self,mut handler:F){
        let el=&mut self.event_loop as *mut EventLoop<InnerWindowEvent>;
        let event_loop=unsafe{&mut *el};

        #[cfg(not(feature="auto_hide"))]
        self.closure_event_listener(event_loop,&mut handler);

        #[cfg(feature="auto_hide")]
        loop{
            if self.closure_event_listener(event_loop,&mut handler){
                break
            }
            if self.closure_wait_until_focused(event_loop,&mut handler){
                break
            }
        }
    }

    /// Запускает данную страницу.
    /// 
    /// Starts the given page.
    pub fn run_page<P:WindowPage>(&mut self,page:&mut P){
        let el=&mut self.event_loop as *mut EventLoop<InnerWindowEvent>;
        let event_loop=unsafe{&mut *el};

        #[cfg(not(feature="auto_hide"))]
        self.paged_event_listener(event_loop,page);

        #[cfg(feature="auto_hide")]
        loop{
            if self.paged_event_listener(event_loop,page){
                break
            }
            if self.paged_wait_until_focused(event_loop,page){
                break
            }
        }
    }

    /// Останавливает обработку событий,
    /// отправляя событие для остановки,
    /// если она запущена.
    /// 
    /// Stops the event listener
    /// by sending the stopping event
    /// if it's running.
    pub fn stop_events(&self)->Result<(),EventLoopClosed<InnerWindowEvent>>{
        self.event_loop_proxy.send_event(InnerWindowEvent::Exit)
    }
}

/// Функции обработки событий.
/// 
/// Event handlers.
impl Window{
    fn closure_event_listener<F:FnMut(&mut Window,WindowEvent)>(&mut self,event_loop:&mut EventLoop<InnerWindowEvent>,mut handler:F)->bool{
        use WindowEvent::*;
        let mut close_flag=false;

        event_loop.run_return(|event,_,control_flow|{
            *control_flow=ControlFlow::Poll;

            let next_event=match event{
                Event::UserEvent(event)=>match event{
                    InnerWindowEvent::Exit=>{
                        *control_flow=ControlFlow::Exit;
                        close_flag=true;
                        return
                    }
                }
                // События окна
                Event::WindowEvent{event,..}=>{
                    match event{
                        // Закрытие окна
                        GWindowEvent::CloseRequested=>{
                            *control_flow=ControlFlow::Exit;
                            close_flag=true;
                            Exit
                        }

                        // Изменение размера окна
                        GWindowEvent::Resized(size)=>unsafe{
                            window_width=size.width as f32;
                            window_height=size.height as f32;
                            window_center=[window_width/2f32,window_height/2f32];

                            #[cfg(feature="mouse_cursor_icon")]
                            self.mouse_icon.update(&mut self.graphics);
                            return
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
                            *control_flow=ControlFlow::Exit;
                            self.display.gl_window().window().set_minimized(true); // Сворацивание окна
                            return
                        }
                        else{
                            Hide(false) // Передача события во внешнее управление
                        }

                        #[cfg(not(feature="auto_hide"))]
                        GWindowEvent::Focused(f)=>Focused(f),

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

            handler(self,next_event)
        });

        close_flag
    }

    #[cfg(feature="auto_hide")]
    fn closure_wait_until_focused<F:FnMut(&mut Window,WindowEvent)>(&mut self,event_loop:&mut EventLoop<InnerWindowEvent>,mut handler:F)->bool{
        use WindowEvent::*;
        let mut close_flag=false;

        event_loop.run_return(|event,_,control_flow|{
            *control_flow=ControlFlow::Wait;

            let next_event=match event{
                Event::UserEvent(event)=>match event{
                    InnerWindowEvent::Exit=>{
                        *control_flow=ControlFlow::Exit;
                        close_flag=true;
                        return
                    }
                }

                Event::WindowEvent{event,..}=>{
                    match event{
                        GWindowEvent::Resized(size)=>unsafe{
                            window_width=size.width as f32;
                            window_height=size.height as f32;
                            window_center=[window_width/2f32,window_height/2f32];

                            #[cfg(feature="mouse_cursor_icon")]
                            self.mouse_icon.update(&mut self.graphics);

                            Resized([size.width,size.height])
                        }

                        GWindowEvent::CloseRequested=>{ // Остановка цикла обработки событий,
                            *control_flow=ControlFlow::Exit;
                            close_flag=true;
                            Exit
                        }

                        // При получении фокуса
                        GWindowEvent::Focused(f)=>{
                            *control_flow=ControlFlow::Exit;
                            self.display.gl_window().window().set_minimized(false);
                            Focused(f)
                        }

                        _=>return
                    }
                }

                Event::Suspended=>Suspended,
                Event::Resumed=>Resumed,

                _=>return
            };

            handler(self,next_event)
        });

        close_flag
    }

    fn paged_event_listener<P:WindowPage>(&mut self,event_loop:&mut EventLoop<InnerWindowEvent>,page:&mut P)->bool{
        let mut close_flag=false;

        event_loop.run_return(|event,_,control_flow|{
            *control_flow=ControlFlow::Poll;
            match event{
                Event::UserEvent(event)=>match event{
                    InnerWindowEvent::Exit=>{
                        *control_flow=ControlFlow::Exit;
                        close_flag=true;
                        return
                    }
                }

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

                        GWindowEvent::DroppedFile(path)=>page.on_file_dropped(self,path),
                        GWindowEvent::HoveredFile(path)=>page.on_file_hovered(self,path),
                        GWindowEvent::HoveredFileCancelled=>page.on_file_hovered_canceled(self),

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
    fn paged_wait_until_focused<P:WindowPage>(&mut self,event_loop:&mut EventLoop<InnerWindowEvent>,page:&mut P)->bool{
        let mut close_flag=false;

        event_loop.run_return(|event,_,control_flow|{
            *control_flow=ControlFlow::Wait;

            match event{
                Event::UserEvent(event)=>match event{
                    InnerWindowEvent::Exit=>{
                        *control_flow=ControlFlow::Exit;
                        close_flag=true;
                        return
                    }
                }

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

    fn on_file_dropped(&mut self,window:&mut Window,path:PathBuf);
    fn on_file_hovered(&mut self,window:&mut Window,path:PathBuf);
    fn on_file_hovered_canceled(&mut self,window:&mut Window);
}