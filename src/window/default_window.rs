use crate::{
    Colour,
    graphics::GraphicsSettings,
};

#[cfg(feature="mouse_cursor_icon")]
use super::MouseCursorIconSettings;

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
    InnerWindowEvent,
    PageState,
    // traits
    Window,
    // structs
    WindowBase,
    PagedWindow,
    DynamicWindow,
};

use glium::backend::glutin::DisplayCreationError;

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
    window::WindowBuilder,
    platform::desktop::EventLoopExtDesktop,
};

use std::{
    collections::VecDeque,
    path::PathBuf,
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
/// Все события обрабатываются и добавляются в очередь внешней обработки (DefaultWindow.events)
/// для работы с ними вне структуры окна.
/// 
/// #
/// 
/// All events are handled and added to the outer handling queue (DefaultWindow.events)
/// to work with them outside of the window structure.
pub struct DefaultWindow{
    pub (crate) base:WindowBase<InnerWindowEvent>,

    pub (crate) events:VecDeque<WindowEvent>,

    #[cfg(feature="auto_hide")]
    pub (crate) events_handler:fn(&mut Self),
}

use WindowEvent::*;

impl DefaultWindow{
    /// Создаёт окно.
    ///
    /// Creates the window.
    pub fn new<F>(setting:F)->Result<DefaultWindow,DisplayCreationError>
            where F:FnOnce(Vec<MonitorHandle>,&mut WindowSettings){

        Window::new::<F>(setting)
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

    /// Converts into `PagedWindow`.
    /// 
    /// Saves the 'auto_hide' feature state (the window hidden or not).
    pub fn into_paged_window(self)->PagedWindow{
        let proxy=self.base.event_loop.create_proxy();

        #[cfg(feature="auto_hide")]
        let minimized=if self.events_handler
            as *const fn(&mut DefaultWindow)
            ==
            DefaultWindow::event_listener
            as *const fn(&mut DefaultWindow)
        {
            true
        }
        else{
            false
        };

        PagedWindow{
            base:self.base,

            event_loop_proxy:proxy,

            #[cfg(feature="auto_hide")]
            minimized,
        }
    }

    /// Converts into `DynamicWindow`.
    /// 
    /// Ignores the 'auto_hide' feature state (the window hidden or not).
    pub fn into_dynamic_window<'a>(self)->DynamicWindow<'a>{
        let proxy=self.base.event_loop.create_proxy();

        DynamicWindow{
            base:self.base,

            event_loop_proxy:proxy,

            page:PageState::<'a>::SetNew(None),
        }
    }
}


//                         \\
//    ЛОКАЛЬНЫЕ ФУНКЦИИ    \\
//                         \\

/// Функции обработки событий.
/// 
/// Event handlers.
impl DefaultWindow{
    /// Обычная функция обработки событий
    pub (crate) fn event_listener(&mut self){
        let el=&mut self.base.event_loop as *mut EventLoop<InnerWindowEvent>;

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
                            self.base.mouse_icon.update(&mut self.base.graphics);

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
                                        self.base.mouse_icon.pressed(&mut self.base.graphics);

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
                                        self.base.mouse_icon.released(&mut self.base.graphics);

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
                    self.base.display.gl_window().window().request_redraw();
                    return
                }

                // Рендеринг
                Event::RedrawRequested(_)=>{
                    #[cfg(feature="fps_counter")]
                    self.base.count_fps();
                    Draw
                }

                _=>return  // Игнорирование остальных событий
            };

            self.events.push_back(next_event)
        });
    }

    /// Функция ожидания получения фокуса - перехватывает управление до получения окном фокуса
    #[cfg(feature="auto_hide")]
    pub (crate) fn wait_until_focused(&mut self){
        let el=&mut self.base.event_loop as *mut EventLoop<InnerWindowEvent>;

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
                            self.base.mouse_icon.update(&mut self.base.graphics);

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
impl DefaultWindow{
    #[cfg(feature="auto_hide")]
    fn lost_focus(&mut self)->WindowEvent{
        self.base.display.gl_window().window().set_minimized(true); // Сворацивание окна
        self.events_handler=DefaultWindow::wait_until_focused; // Смена фукции обработки событий

        WindowEvent::Hide(true) // Передача события во внешнее управление
    }

    /// При получении фокуса
    #[cfg(feature="auto_hide")]
    fn gained_focus(&mut self)->WindowEvent{
        self.events_handler=DefaultWindow::event_listener; // Смена фукции обработки событий
        self.base.display.gl_window().window().set_minimized(false);

        Hide(false) // Передача события во внешнее управление
    }
}

impl Window for DefaultWindow{
    type UserEvent=InnerWindowEvent;

    fn window_base(&self)->&WindowBase<Self::UserEvent>{
        &self.base
    }

    fn window_base_mut(&mut self)->&mut WindowBase<Self::UserEvent>{
        &mut self.base
    }

    fn raw(
        window_builder:WindowBuilder,
        context_builder:ContextBuilder<NotCurrent>,
        graphics_settings:GraphicsSettings,
        event_loop:EventLoop<Self::UserEvent>,
        initial_colour:Option<Colour>,

        #[cfg(feature="mouse_cursor_icon")]
        mouse_cursor_icon_settings:MouseCursorIconSettings<PathBuf>,
    )->Result<DefaultWindow,DisplayCreationError>{

        #[cfg(not(feature="mouse_cursor_icon"))]
        let base=WindowBase::raw(window_builder,
            context_builder,
            graphics_settings,
            event_loop,
            initial_colour
        );

        #[cfg(feature="mouse_cursor_icon")]
        let base=WindowBase::raw(window_builder,
            context_builder,
            graphics_settings,
            event_loop,
            initial_colour,
            mouse_cursor_icon_settings
        );

        match base{
            Ok(w)=>Ok(Self{
                base:w,

                events:VecDeque::with_capacity(32),

                #[cfg(feature="auto_hide")]
                events_handler:DefaultWindow::event_listener,
            }),
            Err(e)=>Err(e)
        }
    }
}