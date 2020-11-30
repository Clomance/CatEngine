use crate::graphics::{GraphicsSettings,Graphics2D};

use super::{
    // statics
    window_width,
    window_height,
    window_center,
    mouse_cursor,
    // enums
    KeyboardButton,
    InnerWindowEvent,
    EventLoopState,
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
        ElementState,
    },
    window::WindowBuilder,
    platform::desktop::EventLoopExtDesktop,
};


/// Ссылка на типаж-объект для `DynamicWindow`.
/// A trait object reference for a `DynamicWindow`.
pub type PageRef<'a,O>=&'a mut dyn WindowPage<'a,Window=DynamicWindow<'a,O>,Output=O>;

/// Окно, которое использует "страницы" как типажи-объекты для обработки событий.
/// A window that uses 'pages' as trait objects to handle the events.
/// 
/// #
/// 
/// Все события прописываются с помощь типажа `WindowPage`
/// и обработываются сразу же после их появления.
/// 
/// "Страницы" можно переключать на ходу.
/// Смена "страницы" до запуска цикла вызывает панику.
/// 
/// #
/// 
/// All the events are implemented with `WindowPage`
/// and handled immediately after emited.
/// 
/// A 'page' may be switched while the event loop is running.
/// Panics if a 'page' is changed before starting the cycle.
pub struct DynamicWindow<'a,O:PartialEq>{
    pub (crate) base:WindowBase,

    pub (crate) page_ref:*mut PageRef<'a,O>,

    pub (crate) last_page:Option<PageRef<'a,O>>,
}


impl<'a,O:PartialEq> DynamicWindow<'a,O>{
    /// Создаёт окно.
    ///
    /// Creates the window.
    pub fn new<F>(setting:F)->Result<(DynamicWindow<'a,O>,Graphics2D),DisplayCreationError>
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
    pub fn set_page_raw(&mut self,page:PageRef<'a,O>){
        unsafe{
            *self.page_ref=page
        }
    }

    /// Устанавливает новую страницу, сохраняя прошлую.
    /// 
    /// Управление переходит ей со следующего события.
    /// 
    /// Sets a new page saving the last one.
    /// 
    /// It starts to operate with the next event.
    pub fn set_page(&mut self,page:PageRef<'a,O>){
        unsafe{
            self.last_page=Some(*self.page_ref);
            *self.page_ref=page
        }
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
    pub fn change_page(&mut self,page:PageRef<'a,O>)->Option<PageRef<'a,O>>{
        unsafe{
            self.last_page=Some(*self.page_ref);
            *self.page_ref=page;
            self.last_page.take()
        }
    }

    /// Возвращает прошлую страницу.
    /// 
    /// Returns the last page.
    pub fn take_old_page(&mut self)->Option<PageRef<'a,O>>{
        self.last_page.take()
    }

    /// Запускает обработчик событий с данной страницей.
    /// 
    /// Starts the event loop with the given page.
    pub fn run(mut self,page:PageRef<'a,O>)->O{
        let el=&mut self.base.event_loop as *mut EventLoop<InnerWindowEvent>;
        let event_loop=unsafe{&mut *el};

        let mut current_page=page;

        self.page_ref=&mut current_page as *mut PageRef<O>;

        #[cfg(not(feature="auto_hide"))]
        if let EventLoopState::Closed(output)=self.event_listener(event_loop,&mut current_page){
            return output
        }
        else{
            panic!("No page output")
        }

        #[cfg(feature="auto_hide")]
        loop{
            if let EventLoopState::Closed(output)=self.event_listener(event_loop,&mut current_page){
                return output
            }

            if let EventLoopState::Closed(output)=self.wait_until_focused(event_loop,&mut current_page){
                return output
            }
        }
    }


    /// Останавливает обработку событий,
    /// отправляя событие для остановки.
    /// 
    /// Возвращает `Err`, если обработчик уже остановлен.
    /// 
    /// Stops the event loop
    /// by sending the stopping event.
    /// 
    /// Returns `Err` if the loop is already stopped.
    #[inline(always)]
    pub fn stop_events(&self)->Result<(),EventLoopClosed<InnerWindowEvent>>{
        self.base.request_event_loop_close()
    }

    #[cfg(feature="auto_hide")]
    #[inline(always)]
    fn on_window_hidden(&mut self){
        
    }

    #[cfg(feature="auto_hide")]
    #[inline(always)]
    fn on_window_unhidden(&mut self){
        
    }
}

// Функции обработки событий.
//
// Event handlers.
impl<'a,O:PartialEq> DynamicWindow<'a,O>{
    fn event_listener(
        &mut self,
        event_loop:&mut EventLoop<InnerWindowEvent>,
        current_page:&mut PageRef<'a,O>,
    )->EventLoopState<O>{
        let mut state=EventLoopState::<O>::Running;

        event_loop.run_return(|event,_,control_flow|{
            paged_event_listener!(self,event,control_flow,current_page,state);
        });

        state
    }



    /// Функция ожидания получения фокуса - перехватывает управление до получения окном фокуса
    #[cfg(feature="auto_hide")]
    fn wait_until_focused(
        &mut self,
        event_loop:&mut EventLoop<InnerWindowEvent>,
        current_page:&mut PageRef<'a,O>
    )->EventLoopState<O>{
        let mut state=EventLoopState::<O>::Running;
        
        event_loop.run_return(|event,_,control_flow|{
            paged_wait_until_focused!(self,event,control_flow,current_page,state);
        });

        state
    }
}

impl<'a,O:PartialEq> Window for DynamicWindow<'a,O>{
    #[inline(always)]
    fn window_base_mut(&mut self)->&mut WindowBase{
        &mut self.base
    }

    #[inline(always)]
    fn window_base(&self)->&WindowBase{
        &self.base
    }

    fn raw(
        window_builder:WindowBuilder,
        context_builder:ContextBuilder<NotCurrent>,
        graphics_settings:GraphicsSettings,
        event_loop:EventLoop<InnerWindowEvent>,
        general_settings:GeneralSettings,
    )->Result<(DynamicWindow<'a,O>,Graphics2D),DisplayCreationError>{

        let base=WindowBase::raw(window_builder,
            context_builder,
            graphics_settings,
            event_loop,
            general_settings,
        );

        match base{
            Ok((w,g))=>{
                Ok((
                    Self{
                        base:w,

                        page_ref:std::ptr::null_mut(),

                        last_page:None,
                    },
                    g
                ))
            }
            Err(e)=>Err(e)
        }
    }
}