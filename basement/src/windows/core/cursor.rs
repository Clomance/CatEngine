use core::mem::transmute;

use winapi::{
    um::{
        winuser::{
            GetCursorPos,
            SetCursorPos,
            ShowCursor,
        }
    }
};

pub struct Cursor;

impl Cursor{
    pub const fn new()->Cursor{
        Self
    }
}

impl Cursor{
    /// Retrieves the position of the mouse cursor, in screen coordinates
    /// and writes it to `point`.
    /// 
    /// The cursor position is always specified in screen coordinates
    /// and is not affected by the mapping mode of the window that contains the cursor.
    /// 
    /// The calling process must have `WINSTA_READATTRIBUTES` access to the window station.
    /// 
    /// The input desktop must be the current desktop when you call `Cursor::get_position`.
    /// Call `OpenInputDesktop` to determine whether the current desktop is the input desktop.
    /// If it is not, call `SetThreadDesktop` with the `HDESK` returned by `OpenInputDesktop` to switch to that desktop.
    /// 
    /// Returns `true` if successful or `false` otherwise.
    /// To get extended error information, call `WinCore::get_last_error`.
    pub fn get_position(&self,point:&mut [i32;2])->bool{
        unsafe{
            GetCursorPos(transmute(point))!=0
        }
    }

    /// Moves the cursor to the specified screen coordinates.
    /// If the new coordinates are not within the screen rectangle set by the most recent `ClipCursor` function call,
    /// the system automatically adjusts the coordinates so that the cursor stays within the rectangle.
    /// 
    /// The cursor is a shared resource.
    /// A window should move the cursor only when the cursor is in the window's client area.
    /// 
    /// The calling process must have `WINSTA_WRITEATTRIBUTES` access to the window station.
    /// 
    /// The input desktop must be the current desktop when you call `Cursor::set_position`.
    /// Call `OpenInputDesktop` to determine whether the current desktop is the input desktop.
    /// If it is not, call `SetThreadDesktop` with the `HDESK` returned by `OpenInputDesktop` to switch to that desktop.
    /// 
    /// Returns `true` if successful or `false` otherwise.
    /// To get extended error information, call `WinCore::get_last_error`.
    pub fn set_position(&self,[x,y]:[i32;2])->bool{
        unsafe{
            SetCursorPos(x,y)!=0
        }
    }

    /// Displays or hides the cursor.
    /// 
    /// This function sets an internal display counter that determines
    /// whether the cursor should be displayed.
    /// The cursor is displayed only if the display count is greater than or equal to 0.
    /// 
    /// If a mouse is installed, the initial display count is 0.
    /// If no mouse is installed, the display count is –1.
    pub fn show(&self,show:bool)->i32{
        unsafe{
            ShowCursor(show as i32)
        }
    }
}