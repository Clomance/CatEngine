use crate::{
    Colour,
    graphics::{Graphics2D,Graphics,GraphicsSettings},
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
    WindowEvent,
    InnerWindowEvent,
    // traits
    Window,
    WindowPage,
    // structs
    WindowBase,
    MouseCursorIconSettings,
};

use glium::{
    Display,
    Surface,
    Frame,
    draw_parameters::DrawParameters,
    backend::glutin::DisplayCreationError,
    SwapBuffersError
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
        MouseScrollDelta,
    },
    window::WindowBuilder,
    platform::desktop::EventLoopExtDesktop,
};

use image::DynamicImage;

use std::path::{Path,PathBuf};

struct DefaultPage;

static mut Default_page:DefaultPage=DefaultPage;

impl<'a> WindowPage<'a> for DefaultPage{
    type Window=DynamicWindow<'a>;

    fn on_close_requested(&mut self,_window:&mut DynamicWindow<'a>){}
    fn on_redraw_requested(&mut self,window:&mut DynamicWindow<'a>){
        window.draw(|_,g|{
            g.clear_colour([1.0;4]);
        }).unwrap();
    }
    fn on_mouse_pressed(&mut self,_:&mut DynamicWindow<'a>,_:MouseButton){}
    fn on_mouse_released(&mut self,_:&mut DynamicWindow<'a>,_:MouseButton){}
    fn on_mouse_moved(&mut self,_:&mut DynamicWindow<'a>,_:[f32;2]){}
    fn on_mouse_scrolled(&mut self,_:&mut DynamicWindow<'a>,_:MouseScrollDelta){}
    fn on_keyboard_pressed(&mut self,_:&mut DynamicWindow<'a>,_:KeyboardButton){}
    fn on_keyboard_released(&mut self,_:&mut DynamicWindow<'a>,_:KeyboardButton){}
    fn on_character_recieved(&mut self,_:&mut DynamicWindow<'a>,_:char){}
    fn on_window_resized(&mut self,_:&mut DynamicWindow<'a>,_:[u32;2]){}
    fn on_suspended(&mut self,_:&mut DynamicWindow<'a>){}
    fn on_resumed(&mut self,_:&mut DynamicWindow<'a>){}
    fn on_window_moved(&mut self,_:&mut DynamicWindow<'a>,_:[i32;2]){}
    fn on_window_focused(&mut self,_:&mut DynamicWindow<'a>,_:bool){}
    fn on_file_dropped(&mut self,_:&mut DynamicWindow<'a>,_:PathBuf){}
    fn on_file_hovered(&mut self,_:&mut DynamicWindow<'a>,_:PathBuf){}
    fn on_file_hovered_canceled(&mut self,_:&mut DynamicWindow<'a>){}
}

/// Окно, заменяет собой обычное окно.
/// A window replaces the default one. feature = "paged_format"
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
pub struct DynamicWindow<'a>{
    base:WindowBase<InnerWindowEvent>,

    event_loop_proxy:EventLoopProxy<InnerWindowEvent>,

    page:Option<&'a mut dyn WindowPage<'a,Window=Self>>,
}


impl<'a> DynamicWindow<'a>{
    /// Создаёт окно. Принимает функцию для настройки.
    ///
    /// Creates the window.
    pub fn new<F>(setting:F)->Result<DynamicWindow<'a>,DisplayCreationError>
            where F:FnOnce(Vec<MonitorHandle>,&mut WindowSettings){

        Window::new(setting)
    }


    pub fn set_page(&mut self,page:&'a mut dyn WindowPage<'a,Window=Self>){
        self.page=Some(page);
    }


    /// Запускает текущую страницу.
    /// 
    /// Starts the current page.
    pub fn run(self){
        let el=&mut self.base.event_loop as *mut EventLoop<InnerWindowEvent>;
        let event_loop=unsafe{&mut *el};

        #[cfg(not(feature="auto_hide"))]
        self.event_listener(event_loop);

        loop{
            #[cfg(feature="auto_hide")]
            if self.wait_until_focused(event_loop){
                break
            }
            if self.event_listener(event_loop){
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
                            self.page.on_close_requested(&mut self.base);
                        }

                        // Изменение размера окна
                        GWindowEvent::Resized(size)=>unsafe{
                            window_width=size.width as f32;
                            window_height=size.height as f32;
                            window_center=[window_width/2f32,window_height/2f32];

                            #[cfg(feature="mouse_cursor_icon")]
                            self.base.mouse_icon.update(&mut self.base.graphics);

                            self.page.on_window_resized(&mut self.base,[size.width,size.height])
                        }

                        // Сдвиг окна
                        GWindowEvent::Moved(pos)=>self.page.on_window_moved(&mut self.base,[pos.x,pos.y]),

                        // Сдвиг мыши (сдвиг за пределы окна игнорируется)
                        GWindowEvent::CursorMoved{position,..}=>unsafe{
                            let last_position=mouse_cursor.position();

                            let position=[position.x as f32,position.y as f32];

                            let dx=position[0]-last_position[0];
                            let dy=position[1]-last_position[1];

                            mouse_cursor.set_position(position);

                            self.page.on_mouse_moved(&mut self.base,[dx,dy])
                        }

                        // Прокрутка колёсика мыши
                        GWindowEvent::MouseWheel{delta,..}=>self.page.on_mouse_scrolled(&mut self.base,delta),

                        // Обработка действий с кнопками мыши (только стандартные кнопки)
                        GWindowEvent::MouseInput{button,state,..}=>{
                            if state==ElementState::Pressed{
                                match button{
                                    GMouseButton::Left=>{
                                        #[cfg(feature="mouse_cursor_icon")]
                                        self.base.mouse_icon.pressed(&mut self.base.graphics);

                                        self.page.on_mouse_pressed(&mut self.base,MouseButton::Left)
                                    }
                                    GMouseButton::Middle=>self.page.on_mouse_pressed(&mut self.base,MouseButton::Middle),
                                    GMouseButton::Right=>self.page.on_mouse_pressed(&mut self.base,MouseButton::Right),
                                    GMouseButton::Other(_)=>{}
                                }
                            }
                            else{
                                match button{
                                    GMouseButton::Left=>{
                                        #[cfg(feature="mouse_cursor_icon")]
                                        self.base.mouse_icon.released(&mut self.base.graphics);

                                        self.page.on_mouse_released(&mut self.base,MouseButton::Left)
                                    }
                                    GMouseButton::Middle=>self.page.on_mouse_released(&mut self.base,MouseButton::Middle),
                                    GMouseButton::Right=>self.page.on_mouse_released(&mut self.base,MouseButton::Right),
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
                                self.page.on_keyboard_pressed(&mut self.base,key)
                            }
                            else{
                                self.page.on_keyboard_released(&mut self.base,key)
                            }
                        }

                        // Получение вводимых букв
                        GWindowEvent::ReceivedCharacter(character)=>if !character.is_ascii_control(){
                            self.page.on_character_recieved(&mut self.base,character)
                        }

                        // При потере фокуса
                        #[cfg(feature="auto_hide")]
                        GWindowEvent::Focused(f)=>if !f{
                            *control_flow=ControlFlow::Exit;
                            self.minimized=true;
                            self.base.display.gl_window().window().set_minimized(true); // Сворацивание окна
                            self.page.on_window_focused(&mut self.base,f);
                        }

                        #[cfg(not(feature="auto_hide"))]
                        GWindowEvent::Focused(f)=>page.on_window_focused(&mut self.base,f),

                        GWindowEvent::DroppedFile(path)=>self.page.on_file_dropped(&mut self.base,path),
                        GWindowEvent::HoveredFile(path)=>self.page.on_file_hovered(&mut self.base,path),
                        GWindowEvent::HoveredFileCancelled=>self.page.on_file_hovered_canceled(&mut self.base),

                        _=>{} // Игнорирование остальных событий
                    }
                }

                Event::Suspended=>self.page.on_suspended(&mut self.base),
                Event::Resumed=>self.page.on_resumed(&mut self.base),

                // Запрос на рендеринг
                Event::MainEventsCleared=>{
                    self.base.display.gl_window().window().request_redraw();
                }

                // Рендеринг
                Event::RedrawRequested(_)=>{
                    #[cfg(feature="fps_counter")]
                    self.base.count_fps();

                    self.page.on_redraw_requested(&mut self.base);
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

                            self.page.on_window_resized(&mut self.base,[size.width,size.height])
                        }

                        GWindowEvent::CloseRequested=>{ // Остановка цикла обработки событий,
                            *control_flow=ControlFlow::Exit;
                            close_flag=true;
                            self.page.on_close_requested(&mut self.base)
                        }

                        // При получении фокуса
                        GWindowEvent::Focused(f)=>{
                            *control_flow=ControlFlow::Exit;
                            self.base.display.gl_window().window().set_minimized(false);
                            self.minimized=false;
                            self.page.on_window_focused(&mut self.base,f);
                        }

                        _=>return
                    }
                }

                Event::Suspended=>self.page.on_suspended(&mut self.base),
                Event::Resumed=>self.page.on_resumed(&mut self.base),

                _=>return
            }
        });

        close_flag
    }
}

impl<'a> Window for DynamicWindow<'a>{
    type UserEvent=InnerWindowEvent;

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

                    minimized:false,
                    page:unsafe{&mut Default_page},

                    event_loop_proxy:proxy,
                })
            }
            Err(e)=>Err(e)
        }
    }

    #[inline(always)]
    fn display(&self)->&Display{
        &self.base.display
    }

    #[inline(always)]
    fn graphics(&mut self)->&mut Graphics2D{
        &mut self.base.graphics
    }




    #[inline(always)]
    fn draw_raw<F:FnOnce(&mut DrawParameters,&mut Frame)>(&self,f:F)->Result<(),SwapBuffersError>{
        self.base.draw_raw(f)
    }

    #[inline(always)]
    fn draw<F:FnOnce(&mut DrawParameters,&mut Graphics)>(&self,f:F)->Result<(),SwapBuffersError>{
        self.base.draw(f)
    }




    #[cfg(feature="alpha_smoothing")]
    #[inline(always)]
    fn set_alpha(&mut self,alpha:f32){
        self.base.set_alpha(alpha)
    }

    #[cfg(feature="alpha_smoothing")]
    #[inline(always)]
    fn set_smooth(&mut self,smooth:f32){
        self.base.set_smooth(smooth)
    }

    #[cfg(feature="alpha_smoothing")]
    #[inline(always)]
    fn set_new_smooth(&mut self,smooth:f32){
        self.base.set_new_smooth(smooth)
    }

    #[cfg(feature="alpha_smoothing")]
    #[inline(always)]
    fn draw_smooth<F:FnOnce(f32,&mut DrawParameters,&mut Graphics)>(&mut self,f:F)->Result<f32,SwapBuffersError>{
        self.base.draw_smooth(f)
    }




    #[inline(always)]
    fn screenshot(&self)->Option<DynamicImage>{
        self.base.screenshot()
    }

    #[inline(always)]
    fn save_screenshot<P:AsRef<Path>>(&self,path:P){
        self.base.save_screenshot(path)
    }
}