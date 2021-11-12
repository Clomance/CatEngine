use super::{
    InstanceHandle,
    icon::IconHandle,
};

use core::{
    mem::{
        transmute,
        transmute_copy,
        size_of,
    },
    ptr::NonNull,
};

use winapi::{
    shared::windef::HCURSOR,
    um::winuser::{
        ClipCursor,
        CopyIcon,
        CreateCursor,
        DestroyCursor,
        GetClipCursor,
        GetCursor,
        GetCursorInfo,
        GetCursorPos,
        GetPhysicalCursorPos,
        LoadCursorA,
        LoadCursorW,
        LoadCursorFromFileA,
        LoadCursorFromFileW,
        SetCursor,
        SetCursorPos,
        SetPhysicalCursorPos,
        SetSystemCursor,
        ShowCursor,
    }
};

#[derive(Clone,Copy,Debug)]
#[repr(transparent)]
pub struct CursorHandle{
    inner:NonNull<()>,
}
implement_handle_wrapper!(CursorHandle,HCURSOR);

impl CursorHandle{
    pub fn as_icon(&self)->IconHandle{
        unsafe{
            transmute(self.inner)
        }
    }
}

#[derive(Clone,Copy,Debug)]
#[repr(isize)]
pub enum SystemCursor{
    /// Standard arrow and small hourglass.
    AppStarting=32650,

    /// Standard arrow.
    Arrow=32512,

    /// Crosshair.
    Cross=32515,

    /// Hand.
    Hand=32649,

    /// Arrow and question mark.
    Help=32651,

    /// I-beam.
    IBeam=32513,

    /// Obsolete for applications marked version 4.0 or later.
    Icon=32641,

    /// Slashed circle
    No=32648,

    /// Obsolete for applications marked version 4.0 or later.
    /// Use IDC_SIZEALL.
    Size=32640,

    /// Four-pointed arrow pointing north, south, east, and west.
    SizeAll=32646,

    /// Double-pointed arrow pointing northeast and southwest.
    SizeNESW=32643,

    /// Double-pointed arrow pointing north and south.
    SizeNS=32645,

    /// Double-pointed arrow pointing northwest and southeast.
    SizeNWSE=32642,

    /// Double-pointed arrow pointing west and east.
    SizeWE=32644,

    /// Vertical arrow.
    UpArrow=32516,

    /// Hourglass.
    Wait=32514,
}

#[derive(Clone,Copy,Debug)]
#[repr(u32)]
pub enum SystemCursorId{
    /// Standard arrow and small hourglass.
    AppStarting=32650,

    /// Standard arrow.
    Arrow=32512,

    /// Crosshair.
    Cross=32515,

    /// Hand.
    Hand=32649,

    /// Arrow and question mark.
    Help=32651,

    /// I-beam.
    IBeam=32513,

    /// Slashed circle.
    No=32648,

    /// Four-pointed arrow pointing north, south, east, and west.
    SizeAll=32646,

    /// Double-pointed arrow pointing northeast and southwest.
    SizeNESW=32643,

    /// Double-pointed arrow pointing north and south.
    SizeNS=32645,

    /// Double-pointed arrow pointing northwest and southeast.
    SizeNWSE=32642,

    /// Double-pointed arrow pointing west and east.
    SizeWE=32644,

    /// Vertical arrow.
    UpArrow=32516,

    /// Hourglass.
    Wait=32514,
}

#[derive(Clone,Copy,Debug)]
#[repr(C)]
pub struct CursorInfo{
    size:u32,
    pub flags:u32,
    pub handle:CursorHandle,
    pub position:[i32;2],
}

/// Before closing, an application must call the `Cursor::destroy` function
/// to free any system resources associated with the cursor.
pub struct Cursor;

impl Cursor{
    pub const fn new()->Cursor{
        Self
    }
}

impl Cursor{
    /// Creates a cursor having the specified size, bit patterns, and hot spot.
    /// 
    /// `and_plane` - An array of bytes that contains the bit values for the AND mask of the cursor,
    /// as in a device-dependent monochrome bitmap.
    /// 
    /// `xor_plane` - An array of bytes that contains the bit values for the XOR mask of the cursor,
    /// as in a device-dependent monochrome bitmap.
    /// 
    /// The `width` and `height` parameters must specify a width and height
    /// that are supported by the current display driver,
    /// because the system cannot create cursors of other sizes.
    /// To determine the width and height supported by the display driver,
    /// use the `GetSystemMetrics` function, specifying the `SM_CXCURSOR` or `SM_CYCURSOR` value.
    /// 
    /// This API does not participate in DPI virtualization.
    /// The output returned is in terms of physical coordinates,
    /// and is not affected by the DPI of the calling thread.
    /// Note that the cursor created may still be scaled
    /// to match the DPI of any given window it is drawn into.
    /// 
    /// If the function succeeds, the return value is a handle to the cursor.
    /// 
    /// If the function fails, the return value is `None`.
    /// To get extended error information, call `WinCore::get_last_error`.
    #[inline(always)]
    pub fn create(
        &self,
        instance:Option<InstanceHandle>,
        [x,y,width,height]:[i32;4],
        and_plane:*const u8,
        xor_plane:*const u8,
    )->Option<CursorHandle>{
        unsafe{
            transmute(
                CreateCursor(
                    InstanceHandle::to_raw(instance),
                    x,y,width,height,
                    transmute(and_plane),
                    transmute(xor_plane),
                )
            )
        }
    }

    /// Copies the specified cursor.
    /// 
    /// `cursor` - A handle to the cursor to be copied.
    /// 
    /// Enables an application or DLL to obtain the handle to a cursor shape owned by another module.
    /// Then if the other module is freed, the application is still able to use the cursor shape.
    /// 
    /// Do not use the `Cursor::copy` function for animated cursors.
    /// Instead, use the `CopyImage` function.
    /// 
    /// `Cursor::copy` is implemented as a call to the `Icon::copy` function.
    /// 
    /// If the function succeeds, the return value is a handle to the duplicate icon.
    /// 
    /// If the function fails, the return value is `None`.
    /// To get extended error information, call `WinCore::get_last_error`.
    pub fn copy(&self,cursor:CursorHandle)->Option<CursorHandle>{
        unsafe{
            transmute(CopyIcon(cursor.as_icon().as_raw()))
        }
    }

    /// Destroys a cursor and frees any memory the cursor occupied.
    /// Do not use this function to destroy a shared cursor.
    /// 
    /// The cursor must not be in use.
    /// 
    /// The `Cursor::destroy` function destroys a nonshared cursor.
    /// Do not use this function to destroy a shared cursor.
    /// A shared cursor is valid as long as the module from which it was loaded remains in memory.
    /// The following functions obtain a shared cursor:
    /// - `Cursor::load`
    /// - `Cursor::load_from_file`
    /// - `LoadImage` (if you use the `LR_SHARED` flag)
    /// - `CopyImage` (if you use the `LR_COPYRETURNORG` flag and the `hImage` parameter is a shared cursor)
    /// 
    /// If the function succeeds, the return value is `true`.
    /// 
    /// If the function fails, the return value is `false`.
    /// To get extended error information, call `WinCore::get_last_error`.
    #[inline(always)]
    pub fn destroy(&self,cursor:CursorHandle)->bool{
        unsafe{
            DestroyCursor(cursor.as_raw())!=0
        }
    }
}

impl Cursor{
    /// Loads the specified cursor resource from the executable (.EXE) file associated with an application instance.
    /// 
    /// `instance` - A handle to an instance of the module
    /// whose executable file contains the cursor to be loaded.
    /// 
    /// `name` - The name of the cursor resource to be loaded.
    /// 
    /// The `Cursor::load` function loads the cursor resource
    /// only if it has not been loaded; otherwise,
    /// it retrieves the handle to the existing resource.
    /// This function returns a valid cursor handle
    /// only if the `name` parameter is a pointer to a cursor resource.
    /// If `name` is a pointer to any type of resource other than a cursor (such as an icon),
    /// the return value is not `None`, even though it is not a valid cursor handle.
    /// 
    /// The `Cursor::load` function searches the cursor resource most appropriate for the cursor for the current display device.
    /// The cursor resource can be a color or monochrome bitmap.
    /// 
    /// This API does not participate in DPI virtualization.
    /// The output returned is not affected by the DPI of the calling thread.
    /// 
    /// If the function succeeds, the return value is the handle to the newly loaded cursor.
    /// 
    /// If the function fails, the return value is `None`.
    /// To get extended error information, call `WinCore::get_last_error`.
    /// 
    /// This function has been superseded by the `LoadImage` function.
    #[inline(always)]
    pub fn load(&self,instance:Option<InstanceHandle>,name:*const u16)->Option<CursorHandle>{
        unsafe{
            CursorHandle::from_raw(
                LoadCursorW(
                    InstanceHandle::to_raw(instance),
                    name
                )
            )
        }
    }

    /// Same as `Cursor::load` but for the ANSI encoding.
    #[inline(always)]
    pub fn load_ansi(&self,instance:Option<InstanceHandle>,name:&str)->Option<CursorHandle>{
        unsafe{
            CursorHandle::from_raw(
                LoadCursorA(
                    InstanceHandle::to_raw(instance),
                    transmute(name.as_ptr())
                )
            )
        }
    }

    /// Same as `Cursor::load` but for the system cursors.
    #[inline(always)]
    pub fn load_system_cursor(&self,name:SystemCursor)->Option<CursorHandle>{
        unsafe{
            CursorHandle::from_raw(
                LoadCursorA(
                    0 as *mut _,
                    transmute(name)
                )
            )
        }
    }

    /// Creates a cursor based on data contained in a file.
    /// 
    /// `name` - The source of the file data to be used to create the cursor.
    /// The data in the file must be in either .CUR or .ANI format.
    /// If the high-order word of `name` is nonzero,
    /// it is a pointer to a string that is a fully qualified name of a file containing cursor data.
    /// 
    /// This API does not participate in DPI virtualization.
    /// The output returned is not affected by the DPI of the calling thread.
    /// 
    /// If the function is successful, the return value is a handle to the new cursor.
    /// 
    /// If the function fails, the return value is `None`.
    /// To get extended error information, call `WinCore::get_last_error`.
    /// `WinCore::get_last_error` may return the following value:
    /// `ERROR_FILE_NOT_FOUND` - The specified file cannot be found.
    #[inline(always)]
    pub fn load_from_file(&self,name:*const u16)->Option<CursorHandle>{
        unsafe{
            CursorHandle::from_raw(
                LoadCursorFromFileW(name)
            )
        }
    }

    /// Same as `Cursor::load_from_file` but for the ANSI encoding.
    #[inline(always)]
    pub fn load_from_file_ansi(&self,name:*const u8)->Option<CursorHandle>{
        unsafe{
            CursorHandle::from_raw(
                LoadCursorFromFileW(transmute(name))
            )
        }
    }
}

impl Cursor{
    /// Sets the cursor shape.
    /// 
    /// `cursor` - A handle to the cursor.
    /// The cursor must have been created by the `Cursor::create` function
    /// or loaded by the `Cursor::load` or `LoadImage` function.
    /// If this parameter is `None`, the cursor is removed from the screen.
    /// 
    /// The return value is the handle to the previous cursor, if there was one.
    /// If there was no previous cursor, the return value is `None`.
    /// 
    /// The cursor is set only if the new cursor is different from the previous cursor;
    /// otherwise, the function returns immediately.
    /// 
    /// The cursor is a shared resource.
    /// A window should set the cursor shape only
    /// when the cursor is in its client area
    /// or when the window is capturing mouse input.
    /// In systems without a mouse, the window should restore the previous cursor
    /// before the cursor leaves the client area
    /// or before it relinquishes control to another window.
    /// 
    /// If your application must set the cursor while it is in a window,
    /// make sure the class cursor for the specified window's class is set to `None`.
    /// If the class cursor is not `None`,
    /// the system restores the class cursor each time the mouse is moved.
    /// 
    /// The cursor is not shown on the screen
    /// if the internal cursor display count is less than zero.
    /// This occurs if the application uses the `Cursor::show` function to hide the cursor more times than to show the cursor.
    #[inline(always)]
    pub fn set(&self,cursor:Option<CursorHandle>)->Option<CursorHandle>{
        unsafe{
            CursorHandle::from_raw(SetCursor(CursorHandle::to_raw(cursor)))
        }
    }

    /// Enables an application to customize the system cursors.
    /// It replaces the contents of the system cursor specified by the id parameter
    /// with the contents of the cursor specified by the `cursor` parameter and then destroys `cursor`.
    /// 
    /// `cursor` - A handle to the cursor.
    /// The function replaces the contents of the system cursor specified by id with the contents of the cursor handled by `cursor`.
    /// The system destroys `cursor` by calling the `Cursor::destroy` function.
    /// Therefore, `cursor` cannot be a cursor loaded using the `Cursor::load` function.
    /// To specify a cursor loaded from a resource, copy the cursor using the `Cursor::copy` function,
    /// then pass the copy to `Cursor::set_system`.
    /// 
    /// `id` - The system cursor to replace with the contents of `cursor`.
    /// This parameter can be one of the following values.
    /// 
    /// If the function succeeds, the return value is `true`.
    /// 
    /// If the function fails, the return value is `false`.
    /// To get extended error information, call `WinCore::get_last_error`.
    #[inline(always)]
    pub fn set_system(&self,cursor:Option<CursorHandle>,id:SystemCursorId)->bool{
        unsafe{
            SetSystemCursor(CursorHandle::to_raw(cursor),id as u32)!=0
        }
    }

    /// Retrieves a handle to the current cursor.
    /// 
    /// To get information on the global cursor,
    /// even if it is not owned by the current thread, use `Cursor::get_info`.
    /// 
    /// The return value is the handle to the current cursor.
    /// If there is no cursor, the return value is `None`.
    #[inline(always)]
    pub fn get(&self)->Option<CursorHandle>{
        unsafe{
            CursorHandle::from_raw(GetCursor())
        }
    }

    /// Retrieves information about the global cursor.
    /// 
    /// If the function succeeds, the return value is `true`.
    /// 
    /// If the function fails, the return value is `false`.
    /// To get extended error information, call `WinCore::get_last_error`.
    #[inline(always)]
    pub fn get_info(&self,info:&mut CursorInfo)->bool{
        unsafe{
            GetCursorInfo(transmute(info))!=0
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
    #[inline(always)]
    pub fn show(&self,show:bool)->i32{
        unsafe{
            ShowCursor(show as i32)
        }
    }
}

impl Cursor{
    /// Confines the cursor to a rectangular area on the screen.
    /// If a subsequent cursor position (set by the `Cursor::set_position` function or the mouse) lies outside the rectangle,
    /// the system automatically adjusts the position to keep the cursor inside the rectangular area.
    /// 
    /// `rect` - A reference to the structure
    /// that contains the screen coordinates of the upper-left and lower-right corners of the confining rectangle.
    /// If this parameter is `None`, the cursor is free to move anywhere on the screen.
    /// 
    /// The cursor is a shared resource.
    /// If an application confines the cursor,
    /// it must release the cursor by using `Cursor::clip` before relinquishing control to another application.
    /// 
    /// The calling process must have `WINSTA_WRITEATTRIBUTES` access to the window station.
    /// 
    /// If the function succeeds, the return value is `true`.
    /// If the function fails, the return value is `false`.
    /// To get extended error information, call `WinCore::get_last_error`.
    #[inline(always)]
    pub fn clip(&self,rect:Option<&[i32;4]>)->bool{
        unsafe{
            ClipCursor(transmute(rect))!=0
        }
    }

    /// Retrieves the screen coordinates of the rectangular area to which the cursor is confined.
    /// 
    /// `rect` receives the screen coordinates of the confining rectangle,
    /// receives the dimensions of the screen if the cursor is not confined to a rectangle.
    /// 
    /// If the function succeeds, the return value is `true`.
    /// 
    /// The cursor is a shared resource.
    /// If an application confines the cursor with the `Cursor::clip` function,
    /// it must later release the cursor by using `Cursor::clip`
    /// before relinquishing control to another application.
    /// 
    /// The calling process must have `WINSTA_READATTRIBUTES` access to the window station.
    /// 
    /// If the function fails, the return value is `false`.
    /// To get extended error information, call `WinCore::get_last_error`.
    #[inline(always)]
    pub fn get_clip(&self,rect:&mut [i32;4])->bool{
        unsafe{
            GetClipCursor(transmute(rect))!=0
        }
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
    #[inline(always)]
    pub fn get_position(&self,point:&mut [i32;2])->bool{
        unsafe{
            GetCursorPos(transmute(point))!=0
        }
    }

    /// Moves the cursor to the specified screen coordinates.
    /// If the new coordinates are not within the screen rectangle set
    /// by the most recent `Cursor::clip` function call,
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
    #[inline(always)]
    pub fn set_position(&self,[x,y]:[i32;2])->bool{
        unsafe{
            SetCursorPos(x,y)!=0
        }
    }

    /// Sets the position of the cursor in physical coordinates.
    /// 
    /// Returns `true` if successful or `false` otherwise.
    /// To get extended error information, call `WinCore::get_last_error`.
    #[inline(always)]
    pub fn set_physical_position(&self,[x,y]:[i32;2])->bool{
        unsafe{
            SetPhysicalCursorPos(x,y)!=0
        }
    }

    /// Retrieves the position of the cursor in physical coordinates.
    /// 
    /// For a description of the difference
    /// between logical coordinates and physical coordinates, see `PhysicalToLogicalPoint`.
    /// 
    /// Returns `true` if successful or `false` otherwise.
    /// To get extended error information, call `WinCore::get_last_error`.
    #[inline(always)]
    pub fn get_physical_position(&self,point:&mut [i32;2])->bool{
        unsafe{
            GetPhysicalCursorPos(transmute(point))!=0
        }
    }
}