use crate::windows::{
    Window,
    WindowReference,
};

mod events;
pub use events::{
    Event,
    WindowEvent,
    MouseButton,
    KeyboardButton,
    VirtualKeyCode,
};

use winapi::{
    shared::{
        windef::{
            HWND,
        },
        ntdef::LARGE_INTEGER
    },
    um::{
        winuser::{
            MSG,
            GetMessageW,
            PeekMessageW,
            SendMessageW,
            PostMessageW,
            TranslateMessage,
            DispatchMessageW,
            DestroyWindow,
            WM_QUIT,
            PM_REMOVE,
            WM_USER,
            SetTimer,
        },
        errhandlingapi::GetLastError,
        profileapi::{
            QueryPerformanceCounter,
            QueryPerformanceFrequency,
        },
    },
};

use std::{
    mem::{
        transmute,
        zeroed
    },
    ptr::null_mut,
    thread::{
        spawn,
        JoinHandle
    },
    sync::{
        Arc,
        Mutex,
        MutexGuard,
        LockResult,
        RwLock,
        TryLockResult,
    },
    panic::{
        UnwindSafe,
        catch_unwind,
    },
    collections::VecDeque,
    any::Any,
};

pub enum LoopControl{
    /// The loop is running with the defeault settings.
    Run,
    /// The loop will be closed and it's able to run again.
    Break,
    /// The loop will be closed and it won't be able to run again.
    /// 
    /// That means the end of application work.
    Close,
}

unsafe impl Sync for LoopControl{}
unsafe impl Send for LoopControl{}

pub enum UpdateInterval{
    Ticks(u32),
    UpdatesPerSecond(u32),
    NanoSeconds(u32),
}

#[derive(Debug,Clone,Copy)]
pub struct Ticks(pub u64);

impl Ticks{
    pub fn as_seconds(self)->u64{
        let mut frequency=0u64;
        unsafe{
            QueryPerformanceFrequency(transmute(&mut frequency));
        }
        self.0/frequency
    }

    pub fn as_nanoseconds(self)->u64{
        let mut frequency=0u64;
        unsafe{
            QueryPerformanceFrequency(transmute(&mut frequency));
        }
        (self.0*1_000_000_000u64)/frequency
    }
}

pub struct EventHandler{
    pub handler:Mutex<Box<dyn FnMut(Event,&mut LoopControl)>>,
    pub main_window:HWND,
}

impl EventHandler{
    pub fn new()->EventHandler{
        Self{
            handler:Mutex::new(Box::new(|_,_|{})),
            main_window:null_mut(),
        }
    }

    pub fn set_event_handler<F:FnMut(Event,&mut LoopControl)>(&self,f:F){
        unsafe{
            *self.handler.lock().unwrap()=transmute::<
                Box<dyn FnMut(Event,&mut LoopControl)>,
                Box<dyn FnMut(Event,&mut LoopControl)>
            >(Box::new(f));
        }
    }

    pub fn try_handle(&self,event:Event,mut loop_control:LoopControl)->Result<LoopControl,Box<dyn Any+Send>>{
        catch_unwind(||{
            if let TryLockResult::Ok(mut handler)=self.handler.try_lock(){
                handler(event,&mut loop_control)
            }
            loop_control
        })
    }
}

unsafe impl Sync for EventHandler{}
unsafe impl Send for EventHandler{}

// Mister Programmer, may I have some loops?
//      /\_____/\
//     /  o   o  \
//    ( ==  ^  == )
//     )         (
//    (           )
//   ( (  )   (  ) )
//  (__(__)___(__)__)

pub struct EventLoop{
    #[cfg(not(feature="own_event_handler"))]
    event_handler:Arc<EventHandler>,
    loop_control:Arc<Mutex<LoopControl>>,
    main_window:Option<HWND>,
    // in ticks
    update_interval:i64,
}

impl EventLoop{
    pub fn new(attributes:EventLoopAttributes)->EventLoop{
        let mut frequency=0i64;
        unsafe{
            QueryPerformanceFrequency(transmute(&mut frequency));
        }

        let update_interval=match attributes.update_interval{
            UpdateInterval::Ticks(ticks)=>ticks as i64,
            UpdateInterval::UpdatesPerSecond(updates)=>{
                frequency/updates as i64
            }
            UpdateInterval::NanoSeconds(nanoseconds)=>{
                (nanoseconds as i64*frequency)/1_000_000_000i64
            }
        };

        let loop_control=Arc::new(Mutex::new(LoopControl::Run));

        let event_handler=Arc::new(EventHandler::new());

        Self{
            #[cfg(not(feature="own_event_handler"))]
            event_handler,
            loop_control,
            main_window:None,
            update_interval,
        }
    }

    pub fn set_main_window(&mut self,window:Option<&Window>){
        if let Some(window)=window{
            self.main_window=Some(window.handle())
        }
        else{
            self.main_window=None
        }
    }

    #[cfg(not(feature="own_event_handler"))]
    pub fn get_handler(&self)->Arc<EventHandler>{
        self.event_handler.clone()
    }

    /// Runs an event loop without settings the handle function.
    /// 
    /// Запускает цикл событий без установки функции обработки.
    pub fn run<F:FnMut(Event,&mut LoopControl)>(&mut self,mut f:F){
        unsafe{
            // Сообщение
            let mut message:MSG=zeroed();

            // Флаг управления циклом
            let mut loop_control=LoopControl::Run;

            f(Event::EventLoopStart,&mut loop_control);
            if let LoopControl::Break=loop_control{
                f(Event::EventLoopClose,&mut loop_control);
                return
            }

            self.event_handler.set_event_handler(f);

            // Время последнего события обновления в тактах
            let mut last_update=0i64;
            QueryPerformanceCounter(transmute(&mut last_update));

            loop{
                // hWnd = window - получает сообщения этого окна
                // hWnd = null - получает сообщения и окна, и потока
                // hWnd = -1 - получает сообщение потока
                // filter = WM_TIMER..WM_TIMER - только сообщения таймера
                // filter = 0..0 - все сообщения
                // PM_REMOVE - при отклике сообщения удаление его из очереди
                if PeekMessageW(&mut message,null_mut(),0,0,PM_REMOVE)==1{
                    // Выход из окна
                    if message.message==WM_QUIT{
                        *self.loop_control.lock().unwrap()=LoopControl::Break;
                        // f(Event::WindowEvent(WindowEvent::Close),&mut loop_control);
                        break
                    }

                    // Трансляция сообщения (например, нажатия клавиш в сообщения ввода символов)
                    TranslateMessage(&message);
                    // Отправка сообщений в оконную функцию
                    DispatchMessageW(&message);
                }

                let mut current_ticks=0i64;
                QueryPerformanceCounter(transmute(&mut current_ticks));

                let tisks_passed=current_ticks-last_update;
                if tisks_passed>=self.update_interval{
                    if tisks_passed<self.update_interval<<1{
                        last_update+=self.update_interval;
                    }
                    else{
                        last_update=current_ticks;
                    }

                    if let Some(main_window)=self.main_window{
                        SendMessageW(main_window,WM_USER,0,tisks_passed as isize);
                    }
                    else{
                        if let LockResult::Ok(mut event_handler)=self.event_handler.handler.lock(){
                            let mut loop_control=LoopControl::Run;
                            event_handler(Event::Update(Ticks(tisks_passed as u64)),&mut loop_control);
                            if let LoopControl::Break=loop_control{
                                event_handler(Event::EventLoopClose,&mut loop_control);
                                return
                            }
                        }
                    }
                }
            }


            // f(Event::EventLoopClose,&mut loop_control);
        }
    }
}


pub struct EventLoopAttributes{
    /// The default is `UpdateInteval::UpdatesPerSecond(50u32)`.
    update_interval:UpdateInterval,
}

impl EventLoopAttributes{
    pub fn new()->EventLoopAttributes{
        Self{
            update_interval:UpdateInterval::UpdatesPerSecond(50u32),
        }
    }
}