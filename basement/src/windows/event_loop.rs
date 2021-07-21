use crate::{
    windows::Window,
    event::ProcessEvent
};

use winapi::um::{
    winuser::{
        MSG,
        GetMessageW,
        PeekMessageW,
        TranslateMessage,
        DispatchMessageW,

        PM_REMOVE,

        WM_QUIT,
    },
    profileapi::{
        QueryPerformanceCounter,
        QueryPerformanceFrequency,
    },
};

use std::{
    mem::{
        transmute,
        zeroed
    },
    ptr::null_mut,
};

#[derive(Clone,Debug)]
pub enum LoopControl{
    /// The loop is running with the defeault settings.
    Run,

    /// Updates are disabled.
    Lazy,

    /// The loop will be closed.
    Break,
}

unsafe impl Sync for LoopControl{}
unsafe impl Send for LoopControl{}

pub enum EventInterval{
    Ticks(u32),
    EventsPerSecond(u32),
    NanoSeconds(u32),
}

impl EventInterval{
    pub fn into_ticks(self)->i64{
        let mut frequency=0i64;
        unsafe{
            QueryPerformanceFrequency(transmute(&mut frequency));
        }

        match self{
            EventInterval::Ticks(ticks)=>ticks as i64,
            EventInterval::EventsPerSecond(updates)=>{
                if updates==0{
                    0i64
                }
                else{
                    frequency/updates as i64
                }
            }
            EventInterval::NanoSeconds(nanoseconds)=>{
                (nanoseconds as i64*frequency)/1_000_000_000i64
            }
        }
    }
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

// Mister Programmer, may I have some loops?
//      /\_____/\
//     /  o   o  \
//    ( ==  ^  == )
//     )         (
//    (           )
//   ( (  )   (  ) )
//  (__(__)___(__)__)

pub struct EventLoop{
    // в тактах
    update_interval:i64,
    redraw_request_interval:i64,
}

impl EventLoop{
    pub fn new(attributes:EventLoopAttributes)->EventLoop{

        let update_interval=attributes.update_interval.into_ticks();

        let redraw_request_interval=attributes.redraw_request_interval.into_ticks();

        Self{
            update_interval,
            redraw_request_interval,
        }
    }

    /// Runs an event loop.
    /// 
    /// Запускает цикл событий.
    pub fn run<F:FnMut(ProcessEvent,&mut LoopControl)>(&mut self,mut f:F){
        unsafe{
            let mut message:MSG=zeroed();

            // Флаг управления циклом
            let mut loop_control=LoopControl::Run;

            // Время последнего события обновления в тактах
            let mut last_update=0i64;
            let mut current_ticks=0i64;
            QueryPerformanceCounter(transmute(&mut last_update));
            let mut last_redraw_request=last_update;

            // Начальное событие
            f(ProcessEvent::EventLoopStart,&mut loop_control);

            loop{
                match loop_control{
                    LoopControl::Run=>{
                        if PeekMessageW(&mut message,null_mut(),0,0,PM_REMOVE)==1{
                            match message.message{
                                WM_QUIT=>{
                                    f(ProcessEvent::Quit,&mut loop_control);
                                    break
                                },
                                _=>{
                                    TranslateMessage(&message);
                                    DispatchMessageW(&message);
                                },
                            }
                        }

                        // если интервал не нулевой (событие включёно)
                        if self.update_interval!=0{
                            // Текущее время в тактах
                            QueryPerformanceCounter(transmute(&mut current_ticks));
                            // проверка интервала события обновления
                            let ticks_passed=current_ticks-last_update;
                            if ticks_passed>=self.update_interval{
                                if ticks_passed<self.update_interval<<1{
                                    last_update+=self.update_interval;
                                }
                                else{
                                    last_update=current_ticks;
                                }

                                f(ProcessEvent::Update(Ticks(ticks_passed as u64)),&mut loop_control);
                            }
                        }
                    },

                    LoopControl::Lazy=>{
                        match GetMessageW(&mut message,null_mut(),0,0){
                            -1=>break,
                            0=>{
                                f(ProcessEvent::Quit,&mut loop_control);
                                break
                            },

                            _=>match message.message{
                                _=>{
                                    TranslateMessage(&message);
                                    DispatchMessageW(&message);
                                },
                            }
                        }
                    }

                    LoopControl::Break=>break,
                }

                if !message.hwnd.is_null(){
                    // если интервал не нулевой (событие включёно)
                    if self.redraw_request_interval!=0{
                        // текущее время в тактах
                        QueryPerformanceCounter(transmute(&mut current_ticks));
                        // проверка интервала события отрисовки
                        let ticks_passed=current_ticks-last_redraw_request;
                        if ticks_passed>=self.redraw_request_interval{
                            if ticks_passed<self.redraw_request_interval<<1{
                                last_redraw_request+=self.redraw_request_interval;
                            }
                            else{
                                last_redraw_request=current_ticks;
                            }

                            let window:&Window=transmute(&message.hwnd);
                            window.redraw();
                        }
                    }
                }
            }

            // Завершение цикла
            f(ProcessEvent::EventLoopBreak,&mut loop_control);
        }
    }
}

impl EventLoop{
    pub fn set_update_interval(&mut self,interval:EventInterval){
        self.update_interval=interval.into_ticks()
    }

    pub fn set_redraw_request_interval(&mut self,interval:EventInterval){
        self.redraw_request_interval=interval.into_ticks()
    }
}


pub struct EventLoopAttributes{
    /// The default is `EventInteval::UpdatesPerSecond(50u32)`.
    pub update_interval:EventInterval,

    /// The default is `EventInteval::UpdatesPerSecond(0u32)`
    /// (disabled).
    pub redraw_request_interval:EventInterval,
}

impl EventLoopAttributes{
    pub fn new()->EventLoopAttributes{
        Self{
            update_interval:EventInterval::EventsPerSecond(50u32),
            redraw_request_interval:EventInterval::EventsPerSecond(0u32),
        }
    }
}