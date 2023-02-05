use std::{num::NonZeroUsize, mem::transmute};

use winapi::um::winuser::{SetTimer, KillTimer, SetCoalescableTimer};

use super::window::WindowHandle;



pub type TimerProcedure=unsafe extern "system" fn(WindowHandle,u32,usize,u32);



#[repr(transparent)]
pub struct TimerIdentifier{
    inner:NonZeroUsize
}

impl TimerIdentifier{
    pub fn raw(identifier:usize)->Option<TimerIdentifier>{
        unsafe{transmute(identifier)}
    }

    pub fn inner(&self)->usize{
        self.inner.get()
    }
}



pub struct Timer;

impl Timer{
    /// Creates a timer with the specified time-out value.
    pub fn set(
        window:Option<WindowHandle>,
        timer:Option<TimerIdentifier>,
        elapse:u32,
        procedure:Option<TimerProcedure>
    )->Option<TimerIdentifier>{
        unsafe{
            transmute(SetTimer(WindowHandle::to_raw(window),transmute(timer),elapse,transmute(procedure)))
        }
    }

    pub fn set_coalescable(
        window:Option<WindowHandle>,
        timer:Option<TimerIdentifier>,
        elapse:u32,
        procedure:Option<TimerProcedure>,
        delay:u32
    )->Option<TimerIdentifier>{
        unsafe{
            transmute(SetCoalescableTimer(WindowHandle::to_raw(window),transmute(timer),elapse,transmute(procedure),delay))
        }
    }

    /// Destroys the specified timer.
    pub fn kill(window:Option<WindowHandle>,timer:TimerIdentifier)->bool{
        unsafe{
            KillTimer(WindowHandle::to_raw(window),timer.inner.get())!=0
        }
    }
}