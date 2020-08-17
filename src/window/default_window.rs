use crate::graphics::GraphicsSettings;

#[cfg(feature="mouse_cursor_icon")]
use super::MouseCursorIconSettings;

use super::{
    // statics
    window_width,
    window_height,
    window_center,
    mouse_cursor,
    // enums
    WindowEvent,
    KeyboardButton,
    InnerWindowEvent,
    // traits
    Window,
    // structs
    WindowBase,
    PagedWindow,
    DynamicWindow,
    WindowSettings,
    GeneralSettings,
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
        MouseButton,
        ElementState,
    },
    window::WindowBuilder,
    platform::desktop::EventLoopExtDesktop,
};

use std::{
    collections::VecDeque,
    path::PathBuf,
    time::Instant,
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

/// Окно с внешним обработчиком событий.
/// A window with an outer event hanlder.
/// 
/// #
/// 
/// Все события обрабатываются и добавляются в очередь внешней обработки
/// для работы с ними вне структуры окна.
/// 
/// #
/// 
/// All the events are handled and added to the outer handling queue
/// to work with them outside of the window structure.
pub struct DefaultWindow{
    pub (crate) base:WindowBase,

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
    /// Returns the next window event.
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

    /// Переводит в `PagedWindow`.
    /// 
    /// Сохраняет состояние окна при `feature = "auto_hide"` (свёрнутое или нет).
    /// 
    /// Converts into the `PagedWindow`.
    /// 
    /// Saves the 'auto_hide' feature state (the window hidden or not).
    pub fn into_paged_window(self)->PagedWindow{
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

            #[cfg(feature="auto_hide")]
            minimized,
        }
    }

    /// Переводит в `DynamicWindow`.
    /// 
    /// Не учитывает состояние окна при `feature = "auto_hide"` (свёрнутое или нет).
    /// 
    /// Converts into the `DynamicWindow`.
    /// 
    /// Ignores the 'auto_hide' feature state (the window hidden or not).
    pub fn into_dynamic_window<'a,O:PartialEq>(self)->DynamicWindow<'a,O>{
        DynamicWindow{
            base:self.base,

            page_ref:std::ptr::null_mut(),

            last_page:None,
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
            #[cfg(not(feature="lazy"))]{
                let now=Instant::now();
                if self.base.next_update<now{
                    self.base.event_loop_proxy
                            .send_event(InnerWindowEvent::Update)
                                    .expect("Dead event loop");

                    self.base.next_update+=self.base.update_interval;
                }
            }

            *control_flow=ControlFlow::Exit;

            let next_event=match event{
                Event::UserEvent(event)=>match event{
                    #[cfg(not(feature="lazy"))]
                    InnerWindowEvent::Update=>{
                        self.base.next_update+=self.base.update_interval;
                        WindowEvent::Update
                    }
                    _=>return
                }

                // События окна
                Event::WindowEvent{event,..}=>match event{
                    // Запрос на закрытие окна
                    GWindowEvent::CloseRequested=>CloseRequested,

                    // Изменение размера окна
                    GWindowEvent::Resized(size)=>window_resized!(size,self),

                    // Сдвиг окна
                    GWindowEvent::Moved(pos)=>Moved([pos.x,pos.y]),

                    // Сдвиг мыши (сдвиг за пределы окна игнорируется)
                    GWindowEvent::CursorMoved{position,..}=>cursor_moved!(position),

                    // Прокрутка колёсика мыши
                    GWindowEvent::MouseWheel{delta,..}=>MouseWheelScroll(delta),

                    // Обработка действий с кнопками мыши
                    GWindowEvent::MouseInput{button,state,..}=>mouse_input!(button,state,self),

                    // Обработка действий с клавишами клавиатуры
                    GWindowEvent::KeyboardInput{input,..}=>keyboard_input!(input),

                    // Получение вводимых букв
                    GWindowEvent::ReceivedCharacter(character)
                            if !character.is_ascii_control()=>CharacterInput(character),

                    // Сворачивание при потере фокуса
                    #[cfg(feature="auto_hide")]
                    GWindowEvent::Focused(f) if !f=>self.lost_focus(),

                    // Потеря фокуса
                    #[cfg(not(feature="auto_hide"))]
                    GWindowEvent::Focused(f)=>Focused(f),

                    GWindowEvent::ModifiersChanged(modifier)=>ModifiersChanged(modifier),

                    // Файл перенесён в окно
                    #[cfg(feature="file_drop")]
                    GWindowEvent::DroppedFile(path)=>DroppedFile(path),
                    #[cfg(feature="file_drop")]
                    GWindowEvent::HoveredFile(path)=>HoveredFile(path),
                    #[cfg(feature="file_drop")]
                    GWindowEvent::HoveredFileCancelled=>HoveredFileCancelled,

                    _=>return // Игнорирование остальных событий
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
                    RedrawRequested
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
                        GWindowEvent::CloseRequested=>{ // Остановка цикла обработки событий,
                            *control_flow=ControlFlow::Exit;
                            CloseRequested
                        }

                        GWindowEvent::Resized(size)=>{
                            *control_flow=ControlFlow::Exit;
                            window_resized!(size,self)
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

        Focused(false)
    }

    /// При получении фокуса
    #[cfg(feature="auto_hide")]
    fn gained_focus(&mut self)->WindowEvent{
        self.events_handler=DefaultWindow::event_listener; // Смена фукции обработки событий

        self.base.display.gl_window().window().set_minimized(false); // Разворацивание окна

        Focused(true)
    }
}

impl Window for DefaultWindow{
    fn window_base(&self)->&WindowBase{
        &self.base
    }

    fn window_base_mut(&mut self)->&mut WindowBase{
        &mut self.base
    }

    fn raw(
        window_builder:WindowBuilder,
        context_builder:ContextBuilder<NotCurrent>,
        graphics_settings:GraphicsSettings,
        event_loop:EventLoop<InnerWindowEvent>,
        general_settings:GeneralSettings,

        #[cfg(feature="mouse_cursor_icon")]
        mouse_cursor_icon_settings:MouseCursorIconSettings<PathBuf>,
    )->Result<DefaultWindow,DisplayCreationError>{
        let base=WindowBase::raw(window_builder,
            context_builder,
            graphics_settings,
            event_loop,
            general_settings,
            #[cfg(feature="mouse_cursor_icon")]
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