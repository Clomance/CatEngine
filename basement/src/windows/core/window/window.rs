use super::{
    ClassData,
    WindowData,
    ShowCommand,
    WindowStyles,
    WindowHandle,
    InstanceHandle,
    ExtendedWindowStyles,
    window_class::ClassIdentifier,
    device_context::DeviceContextHandle,
    menu::MenuHandle,
};

use core::{
    mem::{
        transmute,
    },
};

use winapi::{
    um::{
        winuser::{
            ShowWindow,
            // SetFocus,
            // SetForegroundWindow,
            // SetCapture,
            CreateWindowExA,
            CreateWindowExW,
            DestroyWindow,
            // SendMessageW,
            GetDC,
            GetWindowRect,
            GetClientRect,
            // RedrawWindow,
            SetWindowPos,
            SetWindowLongPtrW,
            GetWindowLongPtrW,
            ClientToScreen,
            ScreenToClient,

            SetClassLongPtrW,
            GetClassLongPtrW,
        },
    }
};


pub struct Window;

impl Window{
    pub const fn new()->Window{
        Self
    }
}

impl Window{
    /// Creates a window.
    /// 
    /// The class can be any name registered with `WindowClass::register`,
    /// provided that the module that registers the class is also the module that creates the window.
    /// The class can also be any of the predefined system class names.
    /// 
    /// `window_name` - The window name.
    /// A null-terminated string.
    /// If the window style specifies a title bar,
    /// the window title pointed to by `window_name` is displayed in the title bar.
    /// When using `Window::create` to create controls,
    /// such as buttons, check boxes, and static controls, use lpWindowName to specify the text of the control. When creating a static control with the SS_ICON style, use lpWindowName to specify the icon name or identifier. To specify an identifier, use the syntax "#num".
    #[inline(always)]
    pub fn create<P>(
        &self,
        class:ClassIdentifier,
        window_name:*const u16,
        style:WindowStyles,
        extended_style:ExtendedWindowStyles,
        [x,y,width,height]:[i32;4],
        parent_window:Option<WindowHandle>,
        menu:Option<MenuHandle>,
        instance:Option<InstanceHandle>,
        create_parameters:Option<&mut P>,
    )->Option<WindowHandle>{
        unsafe{
            WindowHandle::from_raw(
                CreateWindowExW(
                    extended_style.flag,
                    class.as_ptr(),
                    window_name,
                    style.flag,
                    x,y,width,height,
                    WindowHandle::to_raw(parent_window),
                    MenuHandle::to_raw(menu),
                    InstanceHandle::to_raw(instance),
                    transmute(create_parameters)
                )
            )
        }
    }

    // /// Same as `Window::create` but for ANSI encoding.
    // #[inline(always)]
    // pub fn create_ansi<P>(
    //     &self,
    //     class:ClassIdentifier,
    //     window_name:&str,
    //     style:WindowStyles,
    //     extended_style:ExtendedWindowStyles,
    //     [x,y,width,height]:[i32;4],
    //     parent_window:Option<WindowHandle>,
    //     menu:Option<MenuHandle>,
    //     instance:Option<InstanceHandle>,
    //     create_parameters:Option<&mut P>,
    // )->Option<WindowHandle>{
    //     unsafe{
    //         WindowHandle::from_raw(
    //             CreateWindowExA(
    //                 extended_style.flag,
    //                 class.as_ptr_ansi(),
    //                 window_name.as_ptr() as *const i8,
    //                 style.flag,
    //                 x,y,width,height,
    //                 WindowHandle::to_raw(parent_window),
    //                 MenuHandle::to_raw(menu),
    //                 InstanceHandle::to_raw(instance),
    //                 transmute(create_parameters)
    //             )
    //         )
    //     }
    // }

    /// Destroys the specified window.
    /// 
    /// The function sends `WM_DESTROY` and `WM_NCDESTROY` messages
    /// to the window to deactivate it and remove the keyboard focus from it.
    /// The function also destroys the window's menu,
    /// flushes the thread message queue,
    /// destroys timers, removes clipboard ownership,
    /// and breaks the clipboard viewer chain (if the window is at the top of the viewer chain).
    /// 
    /// If the specified window is a parent or owner window,
    /// `Window::destroy` automatically destroys the associated child or owned windows
    /// when it destroys the parent or owner window.
    /// The function first destroys child or owned windows,
    /// and then it destroys the parent or owner window.
    /// 
    /// `Window::destroy` also destroys modeless dialog boxes created by the `CreateDialog` function.
    /// 
    /// If the function succeeds, returns `true`.
    /// If the function fails, returns `false`.
    /// To get extended error information, call `WinCore::get_last_error`.
    /// 
    /// A thread cannot use `Window::destroy` to destroy a window created by a different thread.
    /// 
    /// If the window being destroyed is a child window that does not have the `WS_EX_NOPARENTNOTIFY` style, a `WM_PARENTNOTIFY` message is sent to the parent.
    #[inline(always)]
    pub fn destroy(&self,window:WindowHandle)->bool{
        unsafe{
            DestroyWindow(window.as_raw())!=0
        }
    }

    /// Retrieves a handle to a device context (DC) for the client area of a specified window or for the entire screen.
    /// You can use the returned handle in subsequent GDI functions to draw in the DC.
    /// The device context is an opaque data structure, whose values are used internally by GDI.
    /// 
    /// `window` is a handle to the window whose DC is to be retrieved;
    /// if this value is NULL, `Window::get_device_context` retrieves the DC for the entire screen.
    /// 
    /// Note that the handle to the DC can only be used by a single thread at any one time.
    /// After painting with a common DC,
    /// the `Window::release_device_context` function must be called to release the DC.
    /// Class and private DCs do not have to be released.
    /// `Window::release_device_context` must be called from the same thread that called `Window::get_device_context`.
    /// The number of DCs is limited only by available memory.
    /// 
    /// If the function succeeds,
    /// the return value is a handle to the DC for the specified window's client area.
    /// If the function fails, the return value is `NULL`.
    #[inline(always)]
    pub fn get_device_context(&self,window:Option<WindowHandle>)->Option<DeviceContextHandle>{
        unsafe{
            DeviceContextHandle::from_raw(GetDC(WindowHandle::to_raw(window)))
        }
    }

    #[inline(always)]
    pub fn get_device_context_unchecked(&self,window:Option<WindowHandle>)->DeviceContextHandle{
        unsafe{
            DeviceContextHandle::from_raw_unchecked(GetDC(WindowHandle::to_raw(window)))
        }
    }

    /// Retrieves the dimensions of the bounding rectangle of the specified window.
    /// The dimensions are given in screen coordinates that are relative to the upper-left corner of the screen.
    /// 
    /// If the function succeeds, the return value is nonzero.
    /// If the function fails, the return value is zero.
    /// To get extended error information, call `WinCore::get_last_error`.
    /// 
    /// In conformance with conventions for the `RECT` structure,
    /// the bottom-right coordinates of the returned rectangle are exclusive.
    /// In other words, the pixel at (right, bottom) lies immediately outside the rectangle.
    /// 
    /// `Window::get_window_rectangle` is virtualized for DPI.
    /// 
    /// In Windows Vista and later, the Window Rect now includes the area occupied by the drop shadow.
    /// 
    /// Calling `Window::get_window_rectangle` will have different behavior depending on
    /// whether the window has ever been shown or not.
    /// If the window has not been shown before,
    /// `Window::get_window_rectangle` will not include the area of the drop shadow.
    /// 
    /// To get the window bounds excluding the drop shadow,
    /// use `DwmGetWindowAttribute`, specifying `DWMWA_EXTENDED_FRAME_BOUNDS`.
    /// Note that unlike the Window Rect,
    /// the DWM Extended Frame Bounds are not adjusted for DPI.
    /// Getting the extended frame bounds can only be done after the window has been shown at least once.
    #[inline(always)]
    pub fn get_window_rectangle(&self,window:WindowHandle,rectangle:&mut [i32;4])->bool{
        unsafe{
            GetWindowRect(window.as_raw(),transmute(rectangle))!=0
        }
    }

    /// Retrieves the coordinates of a window's client area.
    /// The client coordinates specify the upper-left and lower-right corners of the client area.
    /// Because client coordinates are relative to the upper-left corner of a window's client area,
    /// the coordinates of the upper-left corner are (0,0).
    /// 
    /// If the function succeeds, the return value is nonzero.
    /// 
    /// If the function fails, the return value is zero.
    /// To get extended error information, call `WinCore::get_last_error`.
    /// 
    /// In conformance with conventions for the `RECT` structure,
    /// the bottom-right coordinates of the returned rectangle are exclusive.
    /// In other words, the pixel at (right, bottom) lies immediately outside the rectangle.
    #[inline(always)]
    pub fn get_client_rectangle(&self,window:WindowHandle,rectangle:&mut [i32;4])->bool{
        unsafe{
            GetClientRect(window.as_raw(),transmute(rectangle))!=0
        }
    }

    /// Changes the size, position, and Z order of a child, pop-up, or top-level window.
    /// These windows are ordered according to their appearance on the screen.
    /// The topmost window receives the highest rank and is the first window in the Z order.
    /// 
    /// If the function succeeds, the return value is nonzero.
    /// If the function fails, the return value is zero.
    /// To get extended error information, call `WinCore::get_last_error`.
    /// 
    /// If you have changed certain window data using SetWindowLong,
    /// you must call SetWindowPos for the changes to take effect.
    /// Use the following combination for `flag`: `SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED`.
    /// 
    /// A window can be made a topmost window either by setting
    /// the `insert_after` parameter to `WindowHandle_TOPMOST` and ensuring that the `SWP_NOZORDER` flag is not set,
    /// or by setting a window's position in the Z order
    /// so that it is above any existing topmost windows.
    /// When a non-topmost window is made topmost,
    /// its owned windows are also made topmost.
    /// Its owners, however, are not changed.
    /// 
    /// If neither the `SWP_NOACTIVATE` nor `SWP_NOZORDER` flag is specified
    /// (that is, when the application requests that a window be simultaneously activated and its position in the Z order changed),
    /// the value specified in `insert_after` is used only in the following circumstances.
    /// 
    /// Neither the `WindowHandle_TOPMOST` nor `WindowHandle_NOTOPMOST` flag is specified in `insert_after`.
    /// The window identified by `window` is not the active window.
    /// An application cannot activate an inactive window without also bringing it to the top of the Z order.
    /// Applications can change an activated window's position in the Z order without restrictions,
    /// or it can activate a window and then move it to the top of the topmost or non-topmost windows.
    /// If a topmost window is repositioned to the bottom (`WindowHandle_BOTTOM`) of the Z order or after any non-topmost window,
    /// it is no longer topmost.
    /// When a topmost window is made non-topmost,
    /// its owners and its owned windows are also made non-topmost windows.
    /// 
    /// A non-topmost window can own a topmost window, but the reverse cannot occur.
    /// Any window (for example, a dialog box) owned by a topmost window is itself made a topmost window,
    /// to ensure that all owned windows stay above their owner.
    /// 
    /// If an application is not in the foreground, and should be in the foreground,
    /// it must call the `SetForegroundWindow` function.
    /// 
    /// To use `Window::set_window_position` to bring a window to the top,
    /// the process that owns the window must have `SetForegroundWindow` permission.
    #[inline(always)]
    pub fn set_window_position(&self,window:WindowHandle,insert_after:Option<WindowHandle>,[x,y,width,height]:[i32;4],flag:u32)->bool{
        unsafe{
            SetWindowPos(window.as_raw(),WindowHandle::to_raw(insert_after),x,y,width,height,flag)!=0
        }
    }

    /// Changes an attribute of the specified window.
    /// The function also sets a value at the specified offset in the extra window memory.
    /// 
    /// If the function succeeds, the return value is the previous value of the specified offset.
    /// If the function fails, the return value is zero.
    /// To get extended error information, call `WinCore::get_last_error`.
    /// If the previous value is zero and the function succeeds,
    /// the return value is zero,
    /// but the function does not clear the last error information.
    /// To determine success or failure,
    /// clear the last error information by calling `SetLastError` with 0,
    /// then call `Window::set_window_long_ptr`.
    /// Function failure will be indicated
    /// by a return value of zero and a `WinCore::get_last_error` result that is nonzero.
    #[inline(always)]
    pub unsafe fn set_window_long_ptr(&self,window:WindowHandle,index:WindowData,value:isize)->isize{
        SetWindowLongPtrW(window.as_raw(),index as i32,value)
    }

    /// Retrieves information about the specified window.
    /// The function also retrieves the value at a specified offset into the extra window memory.
    /// 
    /// If the function succeeds, the return value is the requested value.
    /// If the function fails, the return value is zero.
    /// To get extended error information, call `WinCore::get_last_error`.
    /// If `Window::set_window_long_ptr` has not been called previously,
    /// `Window::get_window_long_ptr` returns zero for values in the extra window or class memory.
    #[inline(always)]
    pub unsafe fn get_window_long_ptr(&self,window:WindowHandle,index:WindowData)->isize{
        GetWindowLongPtrW(window.as_raw(),index as i32)
    }

    /// Sets the specified window's show state.
    /// 
    /// The `command` parameter is ignored the first time an application calls `ShowCommand::Show`,
    /// if the program that launched the application provides a `STARTUPINFO` structure.
    /// Otherwise, the first time `Window::show_window` is called,
    /// the value should be the value obtained by the `WinMain` function in its `command` parameter.
    /// 
    /// To perform certain special effects when showing or hiding a window, use `AnimateWindow`.
    /// The first time an application calls `Window::show_window`,
    /// it should use the `WinMain` function's `command` parameter as its `command` parameter.
    /// Subsequent calls to `Window::show_window` must use one of the values in the given list,
    /// instead of the one specified by the `WinMain` function's `command` parameter.
    /// As noted in the discussion of the `command` parameter,
    /// the `command` value is ignored in the first call to `Window::show_window`
    /// if the program that launched the application specifies startup information in the structure.
    /// In this case, `Window::show_window` uses the information specified in the `STARTUPINFO` structure to show the window.
    /// On subsequent calls, the application must call `Window::show_window` with `command` set to `ShowCommand::ShowDefault`
    /// to use the startup information provided by the program that launched the application.
    /// This behavior is designed for the following situations:
    /// - Applications create their main window by calling `CreateWindow` with the `WS_VISIBLE` flag set.
    /// - Applications create their main window by calling `CreateWindow` with the `WS_VISIBLE` flag cleared,
    /// and later call `Window::show_window` with the `ShowCommand::Show` flag set to make it visible.
    /// 
    /// If the window was previously visible, returns `true`.
    /// If the window was previously hidden, returns `false`.
    #[inline(always)]
    pub unsafe fn show_window(&self,window:WindowHandle,command:ShowCommand)->bool{
        ShowWindow(window.as_raw(),command as i32)!=0
    }

    /// Converts the client-area coordinates of a specified point to screen coordinates.
    /// 
    /// `point` contains the client coordinates to be converted.
    /// The new screen coordinates are copied into this structure if the function succeeds.
    /// 
    /// The function replaces the client-area coordinates in `point` with the screen coordinates.
    /// The screen coordinates are relative to the upper-left corner of the screen.
    /// Note, a screen-coordinate point that is above the window's client area has a negative y-coordinate.
    /// Similarly, a screen coordinate to the left of a client area has a negative x-coordinate.
    /// 
    /// All coordinates are device coordinates.
    /// 
    /// If the function succeeds, returns `true`.
    /// If the function fails, return `false`.
    #[inline(always)]
    pub fn client_to_screen(&self,window:WindowHandle,point:&mut [i32;2])->bool{
        unsafe{
            ClientToScreen(window.as_raw(),transmute(point))!=0
        }
    }

    /// Converts the screen coordinates of a specified point on the screen to client-area coordinates.
    /// 
    /// `point` contains the client coordinates to be converted.
    /// 
    /// The function uses the window identified by the `window` parameter
    /// and the screen coordinates given in `point` to compute client coordinates.
    /// It then replaces the screen coordinates with the client coordinates.
    /// The new coordinates are relative to the upper-left corner of the specified window's client area.
    /// 
    /// The `Window::screen_to_client` function assumes the specified point is in screen coordinates.
    /// 
    /// All coordinates are in device units.
    /// 
    /// Do not use `Window::screen_to_client` when in a mirroring situation, that is,
    /// when changing from left-to-right layout to right-to-left layout.
    /// Instead, use `MapWindowPoints`.
    /// For more information, see "Window Layout and Mirroring" in Window Features.
    /// 
    /// If the function succeeds, returns `true`.
    /// If the function fails, return `false`.
    #[inline(always)]
    pub fn screen_to_client(&self,window:WindowHandle,point:&mut [i32;2])->bool{
        unsafe{
            ScreenToClient(window.as_raw(),transmute(point))!=0
        }
    }
}

impl Window{
    /// Replaces the specified value at the specified offset in the extra class memory
    /// or the `WNDCLASSEX` structure for the class to which the specified window belongs.
    /// 
    /// If you use the `Window::set_class_long_ptr` function and the `ClassData::WindowProcedure` index to replace the window procedure,
    /// the window procedure must conform to the guidelines specified in the description of the `WindowProc` callback function.
    /// 
    /// Calling `Window::set_class_long_ptr` with the `ClassData::WindowProcedure` index creates a subclass of the window class
    /// that affects all windows subsequently created with the class.
    /// An application can subclass a system class,
    /// but should not subclass a window class created by another process.
    /// 
    /// Reserve extra class memory by specifying a nonzero value in the `class_extra_data` member of the `WNDCLASSEX` structure used with the `WindowClass::register` function.
    /// 
    /// Use the `Window::set_class_long_ptr` function with care.
    /// For example,
    /// it is possible to change the background colour for a class by using `Window::set_class_long_ptr`,
    /// but this change does not immediately repaint all windows belonging to the class.
    /// 
    /// If the function succeeds, the return value is the previous value of the specified offset.
    /// If this was not previously set, the return value is zero.
    /// 
    /// If the function fails, the return value is zero.
    /// To get extended error information, call `WinCore::get_last_error`.
    #[inline(always)]
    pub unsafe fn set_class_long_ptr(&self,window:WindowHandle,index:ClassData,value:isize)->usize{
        SetClassLongPtrW(window.as_raw(),index as i32,value)
    }

    /// Retrieves the specified value from the `WNDCLASSEX` structure associated with the specified window.
    /// 
    /// Reserve extra class memory by specifying a nonzero value in the `class_extra_data` member
    /// of the `WNDCLASSEX` structure used with the `WindowClass::register` function.
    /// 
    /// If the function succeeds, the return value is the requested value.
    /// 
    /// If the function fails, the return value is zero.
    /// To get extended error information, call `WinCore::get_last_error`.
    #[inline(always)]
    pub unsafe fn get_class_long_ptr(&self,window:WindowHandle,index:ClassData)->usize{
        GetClassLongPtrW(window.as_raw(),index as i32)
    }
}