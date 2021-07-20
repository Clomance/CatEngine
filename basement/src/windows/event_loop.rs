use crate::event::ProcessEvent;

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
    // redraw_interval:i64,
}

impl EventLoop{
    pub fn new(attributes:EventLoopAttributes)->EventLoop{

        let mut frequency=0i64;
        unsafe{
            QueryPerformanceFrequency(transmute(&mut frequency));
        }

        let update_interval=match attributes.update_interval{
            EventInterval::Ticks(ticks)=>ticks as i64,
            EventInterval::EventsPerSecond(updates)=>{
                frequency/updates as i64
            }
            EventInterval::NanoSeconds(nanoseconds)=>{
                (nanoseconds as i64*frequency)/1_000_000_000i64
            }
        };

        // let redraw_interval=match attributes.redraw_interval{
        //     UpdateInterval::Ticks(ticks)=>ticks as i64,
        //     UpdateInterval::UpdatesPerSecond(redraws)=>{
        //         frequency/redraws as i64
        //     }
        //     UpdateInterval::NanoSeconds(nanoseconds)=>{
        //         (nanoseconds as i64*frequency)/1_000_000_000i64
        //     }
        // };

        Self{
            update_interval,
            // redraw_interval,
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
            QueryPerformanceCounter(transmute(&mut last_update));
            // let mut last_redraw=last_update;

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

                        // Текущее время в тактах
                        let mut current_ticks=0i64;
                        QueryPerformanceCounter(transmute(&mut current_ticks));
                        // проверка события обновления
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

                // Текущее время в тактах
                // let mut current_ticks=0i64;
                // QueryPerformanceCounter(transmute(&mut current_ticks));
                // // проверка события отрисовки
                // let ticks_passed=current_ticks-last_redraw;
                // if ticks_passed>=self.redraw_interval{
                //     if ticks_passed<self.redraw_interval<<1{
                //         last_redraw+=self.redraw_interval;
                //     }
                //     else{
                //         last_redraw=current_ticks;
                //     }

                //     f(Event::Redraw,&mut loop_control);
                // }
            }

            f(ProcessEvent::EventLoopBreak,&mut loop_control);
        }
    }
}


pub struct EventLoopAttributes{
    /// The default is `UpdateInteval::UpdatesPerSecond(50u32)`.
    pub update_interval:EventInterval,

    // pub redraw_interval:UpdateInterval,
}

impl EventLoopAttributes{
    pub fn new()->EventLoopAttributes{
        Self{
            update_interval:EventInterval::EventsPerSecond(50u32),
            // redraw_interval:UpdateInterval::UpdatesPerSecond(30u32),
        }
    }
}