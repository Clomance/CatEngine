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
    MouseButton,
    KeyboardButton,
    WindowEvent,
    InnerWindowEvent,
    // traits
    Window,
    WindowPage,
    // structs
    WindowBase
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

/// Окно, использует 'страницы' и замыкания для обработки событий.
/// A window usee 'pages' and closures to handle events.
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
pub struct PagedWindow{
    base:WindowBase<InnerWindowEvent>,

    event_loop_proxy:EventLoopProxy<InnerWindowEvent>,
    
    #[cfg(feature="auto_hide")]
    minimized:bool,
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
                if self.closure_event_listener(event_loop,&mut handler){
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
    pub fn run_page<P:WindowPage<'static,Window=PagedWindow>>(&mut self,page:&mut P){
        let el=&mut self.base.event_loop as *mut EventLoop<InnerWindowEvent>;
        let event_loop=unsafe{&mut *el};

        #[cfg(not(feature="auto_hide"))]
        self.paged_event_listener(event_loop,page);

        #[cfg(feature="auto_hide")]
        if self.minimized{
            loop{
                if self.paged_wait_until_focused(event_loop,page){
                    break
                }
                if self.paged_event_listener(event_loop,page){
                    break
                }
            }
        }
        else{
            loop{
                if self.paged_event_listener(event_loop,page){
                    break
                }
                if self.paged_wait_until_focused(event_loop,page){
                    break
                }
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
impl PagedWindow{
    fn closure_event_listener<F:FnMut(&mut PagedWindow,WindowEvent)>(&mut self,event_loop:&mut EventLoop<InnerWindowEvent>,mut handler:F)->bool{
        use WindowEvent::*;
        let mut close_flag=false;

        event_loop.run_return(|event,_,control_flow|{
            #[cfg(not(feature="lazy"))]{
                // Endless cycling checking events.
                *control_flow=ControlFlow::Poll;
            }
            
            #[cfg(feature="lazy")]{
                // Waiting for any event except redraw event.
                *control_flow=ControlFlow::Wait;
            }

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
                            *control_flow=ControlFlow::Exit;
                            self.minimized=true;
                            self.base.display.gl_window().window().set_minimized(true); // Сворацивание окна
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

            handler(self,next_event)
        });

        close_flag
    }

    #[cfg(feature="auto_hide")]
    fn closure_wait_until_focused<F:FnMut(&mut PagedWindow,WindowEvent)>(&mut self,event_loop:&mut EventLoop<InnerWindowEvent>,mut handler:F)->bool{
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
                         // Запрос на закрытие окна
                         // Остановка цикла обработки событий
                         GWindowEvent::CloseRequested=>{
                            *control_flow=ControlFlow::Exit;
                            close_flag=true;
                            Exit
                        }

                        GWindowEvent::Resized(size)=>unsafe{
                            window_width=size.width as f32;
                            window_height=size.height as f32;
                            window_center=[window_width/2f32,window_height/2f32];

                            #[cfg(feature="mouse_cursor_icon")]
                            self.base.mouse_icon.update(&mut self.base.graphics);

                            return
                        }

                        // При получении фокуса
                        GWindowEvent::Focused(f)=>{
                            *control_flow=ControlFlow::Exit;
                            self.minimized=false;
                            self.base.display.gl_window().window().set_minimized(false);



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

    fn paged_event_listener<P>(&mut self,event_loop:&mut EventLoop<InnerWindowEvent>,page:&mut P)->bool
            where P:WindowPage<'static,Window=PagedWindow>{
        let mut close_flag=false;

        event_loop.run_return(|event,_,control_flow|{
            #[cfg(not(feature="lazy"))]{
                // Endless cycling checking events.
                *control_flow=ControlFlow::Poll;
            }
            
            #[cfg(feature="lazy")]{
                // Waiting for any event except redraw event.
                *control_flow=ControlFlow::Wait;
            }

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
                            self.minimized=true;
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
    fn paged_wait_until_focused<P:WindowPage<'static,Window=PagedWindow>>(&mut self,event_loop:&mut EventLoop<InnerWindowEvent>,page:&mut P)->bool{
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
                            self.minimized=false;
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

impl Window for PagedWindow{
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
        event_loop:EventLoop<InnerWindowEvent>,
        initial_colour:Option<Colour>,
        
        #[cfg(feature="mouse_cursor_icon")]
        mouse_cursor_icon_settings:MouseCursorIconSettings<PathBuf>,
    )->Result<PagedWindow,DisplayCreationError>{

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

                    event_loop_proxy:proxy,

                    #[cfg(feature="auto_hide")]
                    minimized:false,

                })
            }
            Err(e)=>Err(e)
        }
    }
}