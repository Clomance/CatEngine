use crate::graphics::GraphicsSettings;

use super::{
    // statics
    window_width,
    window_height,
    window_center,
    mouse_cursor,
    // enums
    KeyboardButton,
    WindowEvent,
    InnerWindowEvent,
    EventLoopState,
    // traits
    Window,
    WindowPage,
    // structs
    WindowBase,
    DefaultWindow,
    DynamicWindow,
    GeneralSettings,
    WindowSettings,
};

use glium::backend::glutin::DisplayCreationError;

use glium::glutin::{
    ContextBuilder,
    NotCurrent,
    monitor::MonitorHandle,
    event_loop::{ControlFlow,EventLoop,EventLoopClosed},
    event::{
        Event,
        WindowEvent as GWindowEvent,
        ElementState,
    },
    window::WindowBuilder,
    platform::desktop::EventLoopExtDesktop,
};

use std::collections::VecDeque;

/// Окно, которое использует "страницы" и замыкания для обработки событий.
/// A window that uses 'pages' and closures to handle the events.
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
pub struct PagedWindow{
    pub (crate) base:WindowBase,

    #[cfg(feature="auto_hide")]
    pub (crate) minimized:bool,
}


impl PagedWindow{
    /// Создаёт окно.
    /// 
    /// Creates the window.
    pub fn new<F>(setting:F)->Result<PagedWindow,DisplayCreationError>
            where F:FnOnce(Vec<MonitorHandle>,&mut WindowSettings){

        Window::new::<F>(setting)
    }

    /// Запускает обработку событий с помощью данного замыкания.
    /// 
    /// Starts event handling with the given closure.
    pub fn run<F:FnMut(&mut PagedWindow,WindowEvent)>(&mut self,mut handler:F){
        let el=&mut self.base.event_loop as *mut EventLoop<InnerWindowEvent>;
        let event_loop=unsafe{&mut *el};

        #[cfg(not(feature="auto_hide"))]
        self.closure_event_listener(event_loop,&mut handler);

        #[cfg(feature="auto_hide")]
        if self.minimized{
            loop{
                if self.closure_wait_until_focused(event_loop,&mut handler){
                    break
                }
                if  self.closure_event_listener(event_loop,&mut handler){
                    break
                }
            }
        }
        else{
            loop{
                if self.closure_event_listener(event_loop,&mut handler){
                    break
                }
                if self.closure_wait_until_focused(event_loop,&mut handler){
                    break
                }
            }
        }
    }

    /// Запускает данную страницу.
    /// 
    /// Starts the given page.
    pub fn run_page<O:PartialEq,P:WindowPage<'static,Window=PagedWindow,Output=O>>(&mut self,page:&mut P)->O{
        let el=&mut self.base.event_loop as *mut EventLoop<InnerWindowEvent>;
        let event_loop=unsafe{&mut *el};

        #[cfg(not(feature="auto_hide"))]
        if let EventLoopState::Closed(output)=self.paged_event_listener(event_loop,page){
            return output
        }
        else{
            panic!("No page output")
        }

        #[cfg(feature="auto_hide")]
        if self.minimized{
            loop{
                if let EventLoopState::Closed(output)=self.paged_wait_until_focused(event_loop,page){
                    return output
                }
                if let EventLoopState::Closed(output)=self.paged_event_listener(event_loop,page){
                    return output
                }
            }
        }
        else{
            loop{
                if let EventLoopState::Closed(output)=self.paged_event_listener(event_loop,page){
                    return output
                }
                if let EventLoopState::Closed(output)=self.paged_wait_until_focused(event_loop,page){
                    return output
                }
            }
        }
    }
    // Mister Programmer, may I have some loops?
    //      /\_____/\
    //     /  o   o  \
    //    ( ==  ^  == )
    //     )         (
    //    (           )
    //   ( (  )   (  ) )
    //  (__(__)___(__)__)


    /// Останавливает обработку событий,
    /// отправляя событие для остановки.
    /// 
    /// Возвращает `Err`, если обработчик уже остановлен.
    /// 
    /// Stops the event loop
    /// by sending the stopping event.
    /// 
    /// Returns `Err` if the loop is already stopped.
    pub fn stop_events(&self)->Result<(),EventLoopClosed<InnerWindowEvent>>{
        self.base.request_event_loop_close()
    }

    /// Переводит в `DefaultWindow`.
    /// 
    /// Сохраняет состояние окна при `feature = "auto_hide"` (свёрнутое или нет).
    /// 
    /// Converts into the `DefaultWindow`.
    /// 
    /// Saves the 'auto_hide' feature state (the window is hidden or not).
    pub fn into_default_window(self)->DefaultWindow{
        #[cfg(feature="auto_hide")]
        let _fn=if self.minimized{
            DefaultWindow::wait_until_focused
        }
        else{
            DefaultWindow::event_listener
        };

        DefaultWindow{
            base:self.base,

            events:VecDeque::with_capacity(32),

            #[cfg(feature="auto_hide")]
            events_handler:_fn,
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

    #[cfg(feature="auto_hide")]
    #[inline(always)]
    fn on_window_hidden(&mut self){
        self.minimized=true;
    }

    #[cfg(feature="auto_hide")]
    #[inline(always)]
    fn on_window_unhidden(&mut self){
        self.minimized=false;
    }
}

/// Функции обработки событий.
/// 
/// Event handlers.
impl PagedWindow{
    fn closure_event_listener<F:FnMut(&mut PagedWindow,WindowEvent)>(
        &mut self,
        event_loop:&mut EventLoop<InnerWindowEvent>,
        mut handler:F
    )->bool{
        use WindowEvent::*;

        let mut close_flag=false;

        event_loop.run_return(|event,_,control_flow|{
            #[cfg(not(feature="lazy"))]{
                self.base.update_check();
                // Endless cycling checking events
                *control_flow=ControlFlow::Poll;
            }
            
            #[cfg(feature="lazy")]{
                // Waiting for any event except redraw event
                *control_flow=ControlFlow::Wait;
            }

            let next_event=match event{
                Event::UserEvent(event)=>match event{
                    InnerWindowEvent::EventLoopCloseRequested=>{
                        *control_flow=ControlFlow::Exit;
                        close_flag=true;
                        return
                    }

                    #[cfg(not(feature="lazy"))]
                    InnerWindowEvent::Update=>{
                        self.base.next_update+=self.base.update_interval;
                        WindowEvent::Update
                    }

                    #[cfg(feature="lazy")]
                    _=>return
                }
                // События окна
                Event::WindowEvent{event,..}=>{
                    match event{
                        // Закрытие окна
                        GWindowEvent::CloseRequested=>{
                            *control_flow=ControlFlow::Exit;
                            close_flag=true;
                            CloseRequested
                        }

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

                        // При потере фокуса
                        #[cfg(feature="auto_hide")]
                        GWindowEvent::Focused(f) if !f=>{
                            *control_flow=ControlFlow::Exit;
                            self.minimized=true;
                            self.base.display.gl_window().window().set_minimized(true); // Сворацивание окна
                            return
                        }

                        #[cfg(not(feature="auto_hide"))]
                        GWindowEvent::Focused(f)=>Focused(f),

                        GWindowEvent::ModifiersChanged(modifier)=>ModifiersChanged(modifier),

                        #[cfg(feature="file_drop")]
                        GWindowEvent::DroppedFile(path)=>DroppedFile(path),
                        #[cfg(feature="file_drop")]
                        GWindowEvent::HoveredFile(path)=>HoveredFile(path),
                        #[cfg(feature="file_drop")]
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
                    RedrawRequested
                }

                Event::LoopDestroyed=>WindowEvent::EventLoopClosed,

                _=>return  // Игнорирование остальных событий
            };

            handler(self,next_event)
        });

        close_flag
    }

    #[cfg(feature="auto_hide")]
    fn closure_wait_until_focused<F:FnMut(&mut PagedWindow,WindowEvent)>(
        &mut self,
        event_loop:&mut EventLoop<InnerWindowEvent>,
        mut handler:F
    )->bool{
        use WindowEvent::*;
        let mut close_flag=false;

        event_loop.run_return(|event,_,control_flow|{
            *control_flow=ControlFlow::Wait;

            let next_event=match event{
                Event::UserEvent(event)=>match event{
                    InnerWindowEvent::EventLoopCloseRequested=>{
                        *control_flow=ControlFlow::Exit;
                        close_flag=true;
                        return
                    }
                    _=>return
                }

                Event::WindowEvent{event,..}=>{
                    match event{
                        // Запрос на закрытие окна
                        // Остановка цикла обработки событий
                        GWindowEvent::CloseRequested=>{
                            *control_flow=ControlFlow::Exit;
                            close_flag=true;
                            CloseRequested
                        }

                        GWindowEvent::Resized(size)=>window_resized!(size,self),

                        // При получении фокуса
                        GWindowEvent::Focused(f)=>{
                            *control_flow=ControlFlow::Exit;
                            self.minimized=false;
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

    pub (crate) fn paged_event_listener<O:PartialEq,P:WindowPage<'static,Window=PagedWindow,Output=O>>(
        &mut self,
        event_loop:&mut EventLoop<InnerWindowEvent>,
        page:&mut P
    )->EventLoopState<O>{
        let mut state=EventLoopState::<O>::Running;

        event_loop.run_return(|event,_,control_flow|{
            paged_event_listener!(self,event,control_flow,page,state);
        });

        state
    }


    /// Функция ожидания получения фокуса - перехватывает управление до получения окном фокуса
    #[cfg(feature="auto_hide")]
    pub (crate) fn paged_wait_until_focused<O:PartialEq,P:WindowPage<'static,Window=PagedWindow,Output=O>>(
        &mut self,
        event_loop:&mut EventLoop<InnerWindowEvent>,
        page:&mut P
    )->EventLoopState<O>{
        let mut state=EventLoopState::<O>::Running;

        event_loop.run_return(|event,_,control_flow|{
            paged_wait_until_focused!(self,event,control_flow,page,state);
        });

        state
    }
}


impl Window for PagedWindow{
    #[inline(always)]
    fn window_base(&self)->&WindowBase{
        &self.base
    }

    #[inline(always)]
    fn window_base_mut(&mut self)->&mut WindowBase{
        &mut self.base
    }

    fn raw(
        window_builder:WindowBuilder,
        context_builder:ContextBuilder<NotCurrent>,
        graphics_settings:GraphicsSettings,
        event_loop:EventLoop<InnerWindowEvent>,
        general_settings:GeneralSettings,
    )->Result<PagedWindow,DisplayCreationError>{

        let base=WindowBase::raw(window_builder,
            context_builder,
            graphics_settings,
            event_loop,
            general_settings,
        );

        match base{
            Ok(w)=>{
                Ok(Self{
                    base:w,


                    #[cfg(feature="auto_hide")]
                    minimized:false,

                })
            }
            Err(e)=>Err(e)
        }
    }
}