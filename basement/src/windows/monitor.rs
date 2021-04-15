use super::Window;

use winapi::{
    shared::windef::HMONITOR,
    um::winuser::{
        // structs
        MONITORINFO,
        // functions
        MonitorFromWindow,
        MonitorFromPoint,
        GetMonitorInfoW,
        // constants
        MONITOR_DEFAULTTOPRIMARY,
        MONITOR_DEFAULTTONEAREST,
    }
};

pub struct Monitor{
    handle:HMONITOR,
}

impl Monitor{
    pub fn get_primary_monitor()->Monitor{
        unsafe{
            Self{
                handle:MonitorFromPoint(core::mem::zeroed(),MONITOR_DEFAULTTOPRIMARY)
            }
        }
    }

    /// Returns the nearest to the window monitor.
    /// 
    /// Возращает ближайщий к окну монитор.
    pub fn get_monitor_from_window(window:&Window)->Monitor{
        unsafe{
            Self{
                handle:MonitorFromWindow(window.handle(),MONITOR_DEFAULTTONEAREST)
            }
        }
    }

    pub fn get_monitor_info(&self)->Option<MONITORINFO>{
        unsafe{
            let mut monitor_info:MONITORINFO=core::mem::zeroed();
            monitor_info.cbSize=core::mem::size_of::<MONITORINFO>() as u32;

            if GetMonitorInfoW(self.handle,&mut monitor_info)!=0{
                Some(monitor_info)
            }
            else{
                None
            }
        }
    }
}