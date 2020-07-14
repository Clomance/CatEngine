use crate::{
    Colour,
    graphics::GraphicsSettings,
};


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
    InnerWindowEvent,
    // traits
    Window,
    WindowPage,
    // structs
    WindowBase,
    MouseCursorIconSettings,
};

use glium::backend::glutin::DisplayCreationError;

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
    },
    window::WindowBuilder,
    platform::desktop::EventLoopExtDesktop,
};


use std::path::PathBuf;

enum PageState<'a>{
    SetNew(Option<&'a mut dyn WindowPage<'a,Window=DynamicWindow<'a>>>),
    TakeOld(Option<&'a mut dyn WindowPage<'a,Window=DynamicWindow<'a>>>),
}

/// Окно, использует 'страницы' как типажи-объекты.
/// A window uses 'pages' as trait-objects.
/// #
/// 
/// Все события прописываются с помощь типажа `WindowPage`
/// и обработываются сразу же после их появления.
/// 
/// Если страница не установлена, то все собития игнорируются.
/// #
/// 
/// All the events are implemented with `WindowPage`
/// and handled immediately after emited.
/// 
/// If no page is set, all the events are ignored.
pub struct DynamicWindow<'a>{
    base:WindowBase<InnerWindowEvent>,

    event_loop_proxy:EventLoopProxy<InnerWindowEvent>,

    page:PageState<'a>,
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
    pub fn set_page(&mut self,page:&'a mut dyn WindowPage<'a,Window=Self>){
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
    pub fn change_page(&mut self,page:&'a mut dyn WindowPage<'a,Window=Self>)->Option<&'a mut dyn WindowPage<'a,Window=DynamicWindow<'a>>>{
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
    pub fn take_old_page(&mut self)->Option<&'a mut dyn WindowPage<'a,Window=DynamicWindow<'a>>>{
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

        #[cfg(not(feature="auto_hide"))]
        self.event_listener(event_loop);

        #[cfg(feature="auto_hide")]
        loop{
            if self.event_listener(event_loop){
                break
            }

            if self.wait_until_focused(event_loop){
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

// Функции обработки событий.
//
// Event handlers.
impl<'a> DynamicWindow<'a>{
    fn event_listener(&mut self,event_loop:&mut EventLoop<InnerWindowEvent>)->bool{
        let mut close_flag=false;

        // Проверка, есть ли 'страница', чтобы установить,
        // и установка, если имеется
        let mut taken_page=if let PageState::SetNew(page)=&mut self.page{
            page.take()
        }
        else{
            None
        };

        event_loop.run_return(|event,_,control_flow|{
            #[cfg(not(feature="lazy"))]{
                // Endless cycling checking events.
                *control_flow=ControlFlow::Poll;
            }
            
            #[cfg(feature="lazy")]{
                // Waiting for any event except redraw event.
                *control_flow=ControlFlow::Wait;
            }

            // Проверка, есть ли 'страница', чтобы заменить текущую,
            // и замена, если есть такая
            if let PageState::SetNew(page)=&mut self.page{
                if let Some(_)=page{
                    let take_old=taken_page.take();
                    taken_page=page.take();
                    self.page=PageState::TakeOld(take_old);
                }
            }

            // Выбор текущей 'страницы' для обработки событий
            // либо их игнорирование, если нет 'страницы'
            let page=if let Some(page)=&mut taken_page{
                page
            }
            else{
                return
            };

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
                            self.base.mouse_icon.update(&mut self.base.graphics);

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
                                        self.base.mouse_icon.pressed(&mut self.base.graphics);

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
                                        self.base.mouse_icon.released(&mut self.base.graphics);

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
                            self.base.display.gl_window().window().set_minimized(true); // Сворацивание окна
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
    fn wait_until_focused(&mut self,event_loop:&mut EventLoop<InnerWindowEvent>)->bool{
        let mut close_flag=false;

        // Проверка, есть ли 'страница', чтобы установить,
        // и установка, если имеется
        let mut taken_page=if let PageState::SetNew(page)=&mut self.page{
            page.take()
        }
        else{
            None
        };

        event_loop.run_return(|event,_,control_flow|{
            *control_flow=ControlFlow::Wait;

            // Проверка, есть ли 'страница', чтобы заменить текущую,
            // и замена, если есть такая
            if let PageState::SetNew(page)=&mut self.page{
                if let Some(_)=page{
                    let take_old=taken_page.take();
                    taken_page=page.take();
                    self.page=PageState::TakeOld(take_old);
                }
            }

            let page=if let Some(page)=&mut taken_page{
                page
            }
            else{
                return
            };

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
                            self.base.mouse_icon.update(&mut self.base.graphics);

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
                            self.base.display.gl_window().window().set_minimized(false);

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
    type UserEvent=InnerWindowEvent;

    fn window_base_mut(&mut self)->&mut WindowBase<InnerWindowEvent>{
        &mut self.base
    }

    fn window_base(&self)->&WindowBase<InnerWindowEvent>{
        &self.base
    }

    fn raw(
        window_builder:WindowBuilder,
        context_builder:ContextBuilder<NotCurrent>,
        graphics_settings:GraphicsSettings,
        event_loop:EventLoop<InnerWindowEvent>,
        initial_colour:Option<Colour>,
        
        #[cfg(feature="mouse_cursor_icon")]
        mouse_cursor_icon_settings:MouseCursorIconSettings<PathBuf>,
    )->Result<DynamicWindow<'a>,DisplayCreationError>{

        #[cfg(not(feature="mouse_cursor_icon"))]
        let base=WindowBase::<InnerWindowEvent>::raw(window_builder,
            context_builder,
            graphics_settings,
            event_loop,
            initial_colour
        );

        #[cfg(feature="mouse_cursor_icon")]
        let base=WindowBase::<InnerWindowEvent>::raw(window_builder,
            context_builder,
            graphics_settings,
            event_loop,
            initial_colour,
            mouse_cursor_icon_settings
        );

        match base{
            Ok(w)=>{
                let proxy=w.event_loop.create_proxy();

                Ok(Self{
                    base:w,

                    page:PageState::SetNew(None),

                    event_loop_proxy:proxy,
                })
            }
            Err(e)=>Err(e)
        }
    }
}