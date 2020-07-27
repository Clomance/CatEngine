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
    KeyboardButton,
    InnerWindowEvent,
    // traits
    Window,
    WindowPage,
    // structs
    WindowBase,
    WindowSettings,
    GeneralSettings,
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
        MouseButton,
        ElementState,
    },
    window::WindowBuilder,
    platform::desktop::EventLoopExtDesktop,
};


use std::{
    path::PathBuf,
    time::Instant,
};

/// Ссылка на типаж-объект для `DynamicWindow`.
/// A trait object reference for a `DynamicWindow`.
pub type PageRef<'a>=&'a mut dyn WindowPage<'a,Window=DynamicWindow<'a>,Output=()>;

pub enum PageState<'a>{
    SetNew(Option<PageRef<'a>>),
    TakeOld(Option<PageRef<'a>>),
}

/// Окно, которое использует "страницы" как типажи-объекты для обработки событий.
/// A window that uses 'pages' as trait objects to handle the events.
/// 
/// #
/// 
/// Все события прописываются с помощь типажа `WindowPage`
/// и обработываются сразу же после их появления.
/// 
/// Если страница не установлена, то все события игнорируются.
/// 
/// #
/// 
/// All the events are implemented with `WindowPage`
/// and handled immediately after emited.
/// 
/// If no page is set, all the events are ignored.
pub struct DynamicWindow<'a>{
    pub (crate) base:WindowBase,

    pub (crate) page:PageState<'a>,
}


impl<'a> DynamicWindow<'a>{
    /// Создаёт окно.
    ///
    /// Creates the window.
    pub fn new<F>(setting:F)->Result<DynamicWindow<'a>,DisplayCreationError>
            where F:FnOnce(Vec<MonitorHandle>,&mut WindowSettings){

        Window::new(setting)
    }

    /// Устанавливает страницу.
    /// 
    /// Управление переходит ей со следующего события.
    /// 
    /// Sets the page.
    /// 
    /// It starts to operate with the next event.
    pub fn set_page(&mut self,page:PageRef<'a>){
        if let PageState::SetNew(page)=&mut self.page{
            page.take()
        }
        else{
            None
        };
        self.page=PageState::SetNew(Some(page));
    }

    /// Устанавливает страницу.
    /// 
    /// Управление переходит ей со следующего события.
    /// 
    /// Возвращает прошлую страницу.
    /// 
    /// Sets the page.
    /// 
    /// It starts to operate with the next event.
    /// 
    /// Returns the last page.
    pub fn change_page(&mut self,page:PageRef<'a>)->Option<PageRef<'a>>{
        let old=if let PageState::SetNew(page)=&mut self.page{
            page.take()
        }
        else{
            None
        };
        self.page=PageState::SetNew(Some(page));
        old
    }

    /// Возвращает прошлую страницу.
    /// 
    /// Returns the last page.
    pub fn take_old_page(&mut self)->Option<PageRef<'a>>{
        if let PageState::TakeOld(page)=&mut self.page{
            page.take()
        }
        else{
            None
        }
    }

    /// Запускает текущую страницу.
    /// 
    /// Starts the current page.
    pub fn run(mut self){
        let el=&mut self.base.event_loop as *mut EventLoop<InnerWindowEvent>;
        let event_loop=unsafe{&mut *el};

        // Проверка, есть ли 'страница', чтобы установить,
        // и установка, если имеется
        let mut taken_page=if let PageState::SetNew(page)=&mut self.page{
            page.take()
        }
        else{
            None
        };

        #[cfg(not(feature="auto_hide"))]
        self.event_listener(event_loop,&mut taken_page);

        #[cfg(feature="auto_hide")]
        loop{
            if self.event_listener(event_loop,&mut taken_page){
                break
            }

            if self.wait_until_focused(event_loop,&mut taken_page){
                break
            }
        }
    }


    /// Останавливает обработку событий,
    /// отправляя событие для остановки.
    /// 
    /// Возвращает `Err`, если обработчик уже остановлен.
    /// 
    /// Stops the event listener
    /// by sending the stopping event.
    /// 
    /// Returns `Err` if the listener has been already stopped.
    pub fn stop_events(&self)->Result<(),EventLoopClosed<InnerWindowEvent>>{
        self.base.request_event_loop_close()
    }
}

// Функции обработки событий.
//
// Event handlers.
impl<'a> DynamicWindow<'a>{
    fn event_listener(
        &mut self,
        event_loop:&mut EventLoop<InnerWindowEvent>,
        taken_page:&mut Option<PageRef<'a>>
    )->bool{
        let mut close_flag=false;

        event_loop.run_return(|event,_,control_flow|{
            #[cfg(not(feature="lazy"))]{
                let now=Instant::now();
                if self.base.next_update<now{
                    self.base.event_loop_proxy
                            .send_event(InnerWindowEvent::Update)
                                    .expect("Dead event loop");

                    self.base.next_update+=self.base.update_interval;
                }
                // Endless cycling checking events
                *control_flow=ControlFlow::Poll;
            }
            
            #[cfg(feature="lazy")]{
                // Waiting for any event except redraw event
                *control_flow=ControlFlow::Wait;
            }

            // Проверка, есть ли 'страница', чтобы заменить текущую,
            // и замена, если есть такая
            if let PageState::SetNew(page)=&mut self.page{
                if let Some(_)=page{
                    let take_old=taken_page.take();
                    *taken_page=page.take();
                    self.page=PageState::TakeOld(take_old);
                }
            }

            // Выбор текущей 'страницы' для обработки событий
            // либо их игнорирование, если нет 'страницы'
            let page=if let Some(page)=taken_page{
                page
            }
            else{
                return
            };

            match event{
                Event::UserEvent(event)=>match event{
                    // Прерывает цикл
                    InnerWindowEvent::EventLoopCloseRequested=>{
                        *control_flow=ControlFlow::Exit;
                        close_flag=true;
                        return
                    }

                    #[cfg(not(feature="lazy"))]
                    InnerWindowEvent::Update=>{
                        self.base.next_update+=self.base.update_interval;
                        page.on_update_requested(self);
                    }

                    #[cfg(feature="lazy")]
                    _=>{}
                }

                // События окна
                Event::WindowEvent{event,..}=>{
                    match event{
                        // Закрытие окна
                        GWindowEvent::CloseRequested=>{
                            *control_flow=ControlFlow::Exit;
                            close_flag=true;
                            page.on_window_close_requested(self);
                        }

                        // Изменение размера окна
                        GWindowEvent::Resized(size)=>window_resized!(size,page,self),

                        // Сдвиг окна
                        GWindowEvent::Moved(pos)=>page.on_window_moved(self,[pos.x,pos.y]),

                        // Сдвиг мыши (сдвиг за пределы окна игнорируется)
                        GWindowEvent::CursorMoved{position,..}=>cursor_moved!(position,page,self),

                        // Прокрутка колёсика мыши
                        GWindowEvent::MouseWheel{delta,..}=>page.on_mouse_scrolled(self,delta),

                        // Обработка действий с кнопками мыши
                        GWindowEvent::MouseInput{button,state,..}=>mouse_input!(button,state,page,self),

                        // Обработка действий с клавишами клавиатуры
                        GWindowEvent::KeyboardInput{input,..}=>keyboard_input!(input,page,self),

                        // Получение вводимых символов
                        GWindowEvent::ReceivedCharacter(character)
                                if !character.is_ascii_control()=>page.on_character_recieved(self,character),

                        // При потере фокуса
                        #[cfg(feature="auto_hide")]
                        GWindowEvent::Focused(f)=>if !f{
                            *control_flow=ControlFlow::Exit;
                            self.base.display.gl_window().window().set_minimized(true); // Сворацивание окна
                            page.on_window_focused(self,f);
                        }

                        #[cfg(not(feature="auto_hide"))]
                        GWindowEvent::Focused(f)=>page.on_window_focused(self,f),

                        GWindowEvent::ModifiersChanged(modifier)=>page.on_modifiers_changed(self,modifier),

                        #[cfg(feature="file_drop")]
                        GWindowEvent::DroppedFile(path)=>page.on_file_dropped(self,path),
                        #[cfg(feature="file_drop")]
                        GWindowEvent::HoveredFile(path)=>page.on_file_hovered(self,path),
                        #[cfg(feature="file_drop")]
                        GWindowEvent::HoveredFileCancelled=>page.on_file_hovered_canceled(self),

                        _=>{} // Игнорирование остальных событий
                    }
                }

                Event::Suspended=>page.on_suspended(self),
                Event::Resumed=>page.on_resumed(self),

                // Запрос на рендеринг
                Event::MainEventsCleared=>{
                    self.base.display.gl_window().window().request_redraw();
                }

                // Рендеринг
                Event::RedrawRequested(_)=>{
                    #[cfg(feature="fps_counter")]
                    self.base.count_fps();

                    page.on_redraw_requested(self);
                }

                _=>{}
            }
        });

        close_flag
    }



    /// Функция ожидания получения фокуса - перехватывает управление до получения окном фокуса
    #[cfg(feature="auto_hide")]
    fn wait_until_focused(
        &mut self,
        event_loop:&mut EventLoop<InnerWindowEvent>,
        taken_page:&mut Option<PageRef<'a>>
    )->bool{
        let mut close_flag=false;

        event_loop.run_return(|event,_,control_flow|{
            *control_flow=ControlFlow::Wait;

            // Проверка, есть ли 'страница', чтобы заменить текущую,
            // и замена, если есть такая
            if let PageState::SetNew(page)=&mut self.page{
                if let Some(_)=page{
                    let take_old=taken_page.take();
                    *taken_page=page.take();
                    self.page=PageState::TakeOld(take_old);
                }
            }

            let page=if let Some(page)=taken_page{
                page
            }
            else{
                return
            };

            match event{
                Event::UserEvent(event)=>match event{
                    // Прерывает ожидание и выходит из цикла
                    InnerWindowEvent::EventLoopCloseRequested=>{
                        *control_flow=ControlFlow::Exit;
                        close_flag=true;
                        return
                    }
                    _=>{}
                }

                Event::WindowEvent{event,..}=>{
                    match event{
                        GWindowEvent::CloseRequested=>{ // Остановка цикла обработки событий,
                            *control_flow=ControlFlow::Exit;
                            close_flag=true;
                            page.on_window_close_requested(self)
                        }

                        GWindowEvent::Resized(size)=>window_resized!(size,page,self),

                        // При получении фокуса
                        GWindowEvent::Focused(f)=>{
                            *control_flow=ControlFlow::Exit;
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

impl<'a> Window for DynamicWindow<'a>{
    fn window_base_mut(&mut self)->&mut WindowBase{
        &mut self.base
    }

    fn window_base(&self)->&WindowBase{
        &self.base
    }

    fn raw(
        window_builder:WindowBuilder,
        context_builder:ContextBuilder<NotCurrent>,
        graphics_settings:GraphicsSettings,
        event_loop:EventLoop<InnerWindowEvent>,
        general_settings:GeneralSettings,
        
        #[cfg(feature="mouse_cursor_icon")]
        mouse_cursor_icon_settings:MouseCursorIconSettings<PathBuf>,
    )->Result<DynamicWindow<'a>,DisplayCreationError>{

        let base=WindowBase::raw(window_builder,
            context_builder,
            graphics_settings,
            event_loop,
            general_settings,
            #[cfg(feature="mouse_cursor_icon")]
            mouse_cursor_icon_settings
        );

        match base{
            Ok(w)=>{
                Ok(Self{
                    base:w,

                    page:PageState::SetNew(None),
                })
            }
            Err(e)=>Err(e)
        }
    }
}