use super::{
    Display,
};

use std::{
    mem::{
        transmute,
        zeroed
    },
    ptr::null_mut,
    time::{
        Instant,
        Duration,
    },
};

use x11::xlib::{
    XNextEvent,
    XEvent,

    MotionNotify,
    ButtonPress,
    ButtonRelease,
    ColormapNotify,
    EnterNotify,
    Expose,
    GraphicsExpose,
    NoExpose,
    FocusIn,
    FocusOut,
    KeymapNotify,
    KeyPress,
    KeyRelease,
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

/// Represents interval between events or an event rate.
/// 
/// The event is disabled if an interval or rate is `0`.
// #[derive(Clone,Copy,Debug)]
// pub enum EventInterval{
//     Ticks(u32),
//     EventsPerSecond(u32),
//     NanoSeconds(u32),
//     Seconds(u32),
// }

// impl EventInterval{
//     pub fn into_ticks(self)->i64{
//         let mut frequency=0i64;

//         match self{
//             EventInterval::Ticks(ticks)=>ticks as i64,
//             EventInterval::EventsPerSecond(updates)=>{
//                 if updates==0{
//                     0i64
//                 }
//                 else{
//                     frequency/updates as i64
//                 }
//             }
//             EventInterval::NanoSeconds(nanoseconds)=>{
//                 (nanoseconds as i64*frequency)/1_000_000_000i64
//             }
//             EventInterval::Seconds(nanoseconds)=>{
//                 nanoseconds as i64*frequency
//             }
//         }
//     }
// }

// #[derive(Debug,Clone,Copy)]
// pub struct Ticks(pub u64);

// impl Ticks{
//     pub fn as_seconds(self)->u64{
//         let mut frequency=0u64;
//         unsafe{
//             QueryPerformanceFrequency(transmute(&mut frequency));
//         }
//         self.0/frequency
//     }

//     pub fn as_nanoseconds(self)->u64{
//         let mut frequency=0u64;
//         unsafe{
//             QueryPerformanceFrequency(transmute(&mut frequency));
//         }
//         (self.0*1_000_000_000u64)/frequency
//     }
// }

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
    // update_interval:i64,
    // redraw_request_interval:i64,
}

impl EventLoop{
//     pub fn new(attributes:EventLoopAttributes)->EventLoop{

//         let update_interval=attributes.update_interval.into_ticks();

//         let redraw_request_interval=attributes.redraw_request_interval.into_ticks();

//         Self{
//             update_interval,
//             redraw_request_interval,
//         }
//     }

    pub fn new()->EventLoop{
        Self{

        }
    }

    pub fn run<F:FnMut(Event,&mut LoopControl)>(&self,display:&Display,mut f:F){
        unsafe{
            let mut loop_control=LoopControl::Run;
            let mut event:XEvent=zeroed();

            f(Event::EventLoopStart,&mut loop_control);

            loop{
                match loop_control{
                    LoopControl::Run=>{
                        
                    }

                    LoopControl::Lazy=>{
                        XNextEvent(display.handle().as_raw(),&mut event);
                    }

                    LoopControl::Break=>{
                        break
                    }
                }

                let event_type:EventType=transmute(event.type_);

                match event_type{
                    _=>{
                        f(Event::WindowEvent(event_type),&mut loop_control);
                    }
                }
            }

            f(Event::EventLoopBreak,&mut loop_control);
        }
    }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
#[repr(i32)]
pub enum EventType{
    MotionNotify=MotionNotify,
    ButtonPress=ButtonPress,
    ButtonRelease=ButtonRelease,
    ColormapNotify=ColormapNotify,
    EnterNotify=EnterNotify,
    Expose=Expose,
    GraphicsExpose=GraphicsExpose,
    NoExpose=NoExpose,
    FocusIn=FocusIn,
    FocusOut=FocusOut,
    KeymapNotify=KeymapNotify,
    KeyPress=KeyPress,
    KeyRelease=KeyRelease,
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Event{
    EventLoopStart,
    Update,
    WindowEvent(EventType),
    EventLoopBreak,
}