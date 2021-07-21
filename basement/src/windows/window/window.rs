use crate::windows::{
    WinCore,
    WinError
};

use super::{
    // structures
    WindowClass,
    Monitor,
    // enums
    WindowEvent,
    // trait
    WindowProcedure,
    // functions
    window_procedure,
    // consts
    window_settings_auto_redraw,
};

use winapi::{
    shared::{
        windef::{
            HWND,
            HDC,
            POINT,
        }
    },

    um::{
        winuser::{
            // ShowWindow,
            // SetFocus,
            // SetForegroundWindow,
            // SetCapture,
            SendMessageW,
            GetDC,
            RedrawWindow,
            SetWindowLongPtrW,
            GetWindowLongPtrW,
            GetCursorPos,
            SetCursorPos,
            ShowCursor,
            ClientToScreen,
            ScreenToClient,
            // window styles
            WS_BORDER,
            WS_CAPTION,
            WS_CHILD,
            WS_CHILDWINDOW,
            WS_CLIPCHILDREN,
            WS_CLIPSIBLINGS,
            WS_DISABLED,
            WS_DLGFRAME,
            WS_GROUP,
            WS_HSCROLL,
            WS_ICONIC,
            WS_MAXIMIZE,
            WS_MAXIMIZEBOX,
            WS_MINIMIZE,
            WS_MINIMIZEBOX,
            WS_OVERLAPPED,
            WS_POPUP,
            WS_SIZEBOX,
            WS_SYSMENU,
            WS_TABSTOP,
            WS_THICKFRAME,
            WS_VISIBLE,
            WS_VSCROLL,
            // extended window styles
            WS_EX_ACCEPTFILES,
            WS_EX_APPWINDOW,
            WS_EX_CLIENTEDGE,
            WS_EX_COMPOSITED,
            WS_EX_CONTEXTHELP,
            WS_EX_CONTROLPARENT,
            WS_EX_DLGMODALFRAME,
            WS_EX_LAYERED,
            WS_EX_LAYOUTRTL,
            WS_EX_LEFT,
            WS_EX_LEFTSCROLLBAR,
            WS_EX_LTRREADING,
            WS_EX_MDICHILD,
            WS_EX_NOACTIVATE,
            WS_EX_NOINHERITLAYOUT,
            WS_EX_NOPARENTNOTIFY,
            WS_EX_NOREDIRECTIONBITMAP,
            WS_EX_RIGHT,
            WS_EX_RIGHTSCROLLBAR,
            WS_EX_RTLREADING,
            WS_EX_STATICEDGE,
            WS_EX_TOOLWINDOW,
            WS_EX_TOPMOST,
            WS_EX_TRANSPARENT,
            WS_EX_WINDOWEDGE,
            // other
            GWL_EXSTYLE,
            GWL_STYLE,
            GWLP_USERDATA,
            GWLP_WNDPROC,
            CW_USEDEFAULT,
            RDW_INTERNALPAINT,
            RDW_INVALIDATE,
            SWP_SHOWWINDOW,
            SWP_NOREPOSITION,
            SWP_NOSIZE,
            SWP_NOMOVE,
            SWP_FRAMECHANGED,
        },
    }
};

use std::{
    ptr::null_mut,
    ffi::{
        OsString,
    },
    os::windows::ffi::OsStrExt,
    mem::transmute,
};


pub enum Fullscreen{
    None,
    Monitor(Monitor)
}

pub struct CreateParameters<A>{
    pub window_procedure:unsafe extern "system" fn(
        handle:HWND,
        message:u32,
        w_param:usize,
        l_param:isize,
    )->isize,
    pub window_procedure_args:*mut A,
    /// Defines whether redraw is requested immediately after redraw event processes.
    pub auto_redraw:bool,
}

/// A window handle.
/// 
/// The window mustn't outlive its class otherwise the class won't be unregistered properly.
/// 
/// Note that all windows/classes that an application creates/registers are destroyed/unregistered
/// when it terminates.
/// So there no need to do it when the application closes,
/// but it makes sense when you create-destroy windows and register-unregister classes at the run time.
#[derive(Clone)]
pub struct Window{
    pub (crate) handle:HWND,
}

impl Window{
    pub fn new<W:WindowProcedure<A>,A>(
        class:&WindowClass,
        attributes:WindowAttributes,
        window_procedure_args:&mut A,
    )->Result<Window,WinError>{
        let window_name:Vec<u16>=attributes.name
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect();

        let mut style=WS_SYSMENU|WS_BORDER;
        let mut extended_style=0;

        // Enabling file dropping
        #[cfg(feature="file_drop")]{
            extended_style|=WS_EX_ACCEPTFILES;
        }

        if attributes.visible{
            style|=WS_VISIBLE;
        }

        if attributes.topmost{
            extended_style|=WS_EX_TOPMOST;
        }

        // Размер, установленный пользователем
        let [mut width,mut height]=if let Some(size)=attributes.size{
            size
        }
        else{
            [CW_USEDEFAULT,CW_USEDEFAULT]
        };
        // Положение, установленное пользователем
        let [mut x,mut y]=if let Some(position)=attributes.position{
            position
        }
        else{
            [CW_USEDEFAULT,CW_USEDEFAULT]
        };

        match attributes.fullscreen{
            Fullscreen::None=>{
                style|=WS_SIZEBOX|WS_CAPTION|WS_MAXIMIZEBOX|WS_MINIMIZEBOX;
            }
            Fullscreen::Monitor(monitor)=>{
                if let Some(info)=monitor.get_monitor_info(){
                    style|=WS_POPUP;
                    style&=!(WS_SIZEBOX|WS_CAPTION|WS_MAXIMIZEBOX|WS_MINIMIZEBOX);

                    extended_style|=WS_EX_APPWINDOW;

                    x=info.rcMonitor.left;
                    y=info.rcMonitor.top;
                    width=info.rcMonitor.right-info.rcMonitor.left;
                    height=info.rcMonitor.bottom-info.rcMonitor.top;
                }
            }
        };

        unsafe{
            let mut create_parameters=CreateParameters{
                window_procedure:window_procedure::<W,A>,
                window_procedure_args,
                auto_redraw:attributes.auto_redraw,
            };

            let window_handle=WinCore.window.create(
                class.as_ptr(),
                window_name.as_ptr(),
                style,
                extended_style,
                [x,y,width,height],
                null_mut(),
                null_mut(),
                null_mut(),
                &mut create_parameters,
            );

            if window_handle.is_null(){
                Err(WinError::get_last_error())
            }
            else{
                Ok(Self{
                    handle:window_handle,
                })
            }
        }
    }

    pub fn handle(&self)->HWND{
        self.handle
    }

    pub fn get_context(&self)->HDC{
        unsafe{
            GetDC(self.handle)
        }
    }
}

/// Requests and sending events.
impl Window{
    pub fn redraw(&self){
        unsafe{
            RedrawWindow(self.handle,null_mut(),null_mut(),RDW_INVALIDATE);
        }
    }

    pub fn destroy(&self)->Result<(),WinError>{
        unsafe{
            if WinCore.window.destroy(self.handle){
                Ok(())
            }
            else{
                Err(WinError::get_last_error())
            }
        }
    }
}

/// Window sizes and positions.
impl Window{
    /// Returns the window size.
    /// 
    /// Возвращает размеры окна.
    /// 
    /// [width, height]
    pub fn size(&self)->[u32;2]{
        let mut window_rectangle=[0i32;4];
        unsafe{
            WinCore.window.get_window_rectangle(self.handle,&mut window_rectangle);
        }
        let [x1,y1,x2,y2]=window_rectangle;
        [
            (x2-x1) as u32,
            (y2-y1) as u32,
        ]
    }

    /// Returns coordinates of window's upper-left corner.
    /// 
    /// Возвращает координаты верхнего левого угла окна.
    /// 
    /// [x, y]
    pub fn position(&self)->[i32;2]{
        let mut window_rectangle=[0i32;4];
        unsafe{
            WinCore.window.get_window_rectangle(self.handle,&mut window_rectangle);
        }
        let [x1,y1,_,_]=window_rectangle;
        [
            x1,
            y1,
        ]
    }

    /// Returns the window rectangle with the coordinates of it's upper-left and lower-right corners.
    /// 
    /// Возвращает оконный прямоугольник с координатами верхнего левого и нижнего правого углов.
    /// 
    /// [x1, y1, x2, y2]
    pub fn rectangle(&self)->[i32;4]{
        let mut window_rectangle=[0i32;4];
        unsafe{
            WinCore.window.get_window_rectangle(self.handle,&mut window_rectangle);
        }
        window_rectangle
    }

    /// Returns window's client area size.
    /// 
    /// Возвращает размеры клиентской области окна.
    /// 
    /// [width, height]
    pub fn client_size(&self)->[u32;2]{
        let mut client_rectangle=[0i32;4];
        unsafe{
            WinCore.window.get_client_rectangle(self.handle,&mut client_rectangle);
        }
        let [_,_,width,height]=client_rectangle;

        [
            width as u32,
            height as u32,
        ]
    }
}

/// Styles and positioning.
impl Window{
    pub fn set_fullscreen(&self,fullscreen:Fullscreen){
        unsafe{
            let mut style=self.get_style();
            let mut extended_style=self.get_extended_style();

            match fullscreen{
                Fullscreen::None=>{
                    style&=!WS_POPUP;
                    style|=WS_SIZEBOX|WS_CAPTION|WS_MAXIMIZEBOX|WS_MINIMIZEBOX;
                    extended_style&=!WS_EX_APPWINDOW;

                    self.set_style(style);
                    self.set_extended_style(extended_style);
                    WinCore.window.set_window_position(
                        self.handle,null_mut(),
                        [0,0,0,0],
                        SWP_SHOWWINDOW|SWP_NOMOVE|SWP_NOSIZE|SWP_FRAMECHANGED
                    );
                }
                Fullscreen::Monitor(monitor)=>{
                    if let Some(info)=monitor.get_monitor_info(){
                        style&=!(WS_SIZEBOX|WS_CAPTION|WS_MAXIMIZEBOX|WS_MINIMIZEBOX);

                        extended_style|=WS_EX_APPWINDOW;

                        let x=info.rcMonitor.left;
                        let y=info.rcMonitor.top;
                        let width=info.rcMonitor.right-info.rcMonitor.left;
                        let height=info.rcMonitor.bottom-info.rcMonitor.top;

                        self.set_style(style);
                        self.set_extended_style(extended_style);
                        self.set_window_position([x,y,width,height])
                    }
                }
            }
        }
    }

    /// Will not make effect until you call the `Window::set_window_position()`.
    pub unsafe fn set_extended_style(&self,style:u32){
        WinCore.window.set_window_long_ptr(self.handle,GWL_EXSTYLE,style as isize);
    }

    /// Will not make effect until you call the `Window::set_window_position()`.
    pub unsafe fn get_extended_style(&self)->u32{
        GetWindowLongPtrW(self.handle,GWL_EXSTYLE) as u32
    }

    /// Will not make effect until you call the `Window::set_window_position()`.
    pub unsafe fn set_style(&self,style:u32){
        WinCore.window.set_window_long_ptr(self.handle,GWL_STYLE,style as isize);
    }

    /// Will not make effect until you call the `Window::set_window_position()`.
    pub unsafe fn get_style(&self)->u32{
        GetWindowLongPtrW(self.handle,GWL_STYLE) as u32
    }

    pub unsafe fn set_window_position(&self,[x,y,width,height]:[i32;4]){
        WinCore.window.set_window_position(self.handle,null_mut(),[x,y,width,height],SWP_SHOWWINDOW);
    }
}

impl Window{
    pub unsafe fn set_auto_redraw(&self,enabled:bool){
        WinCore.window.set_window_long_ptr(self.handle,window_settings_auto_redraw,enabled as isize);
    }


    pub (crate) unsafe fn set_user_data<D:Sized>(&self,data:&mut D){
        WinCore.window.set_window_long_ptr(self.handle,GWLP_USERDATA,data as *const D as isize);
    }

    pub (crate) unsafe fn get_user_data(&self)->isize{
        GetWindowLongPtrW(self.handle,GWLP_USERDATA)
    }

    pub unsafe fn set_window_procedure(&self,procedure:unsafe extern "system" fn(HWND,u32,usize,isize)->isize){
        WinCore.window.set_window_long_ptr(self.handle,GWLP_WNDPROC,procedure as isize);
    }

    pub unsafe fn set_window_handle<W:WindowProcedure<A>,A>(&self){
        WinCore.window.set_window_long_ptr(self.handle,GWLP_WNDPROC,window_procedure::<W,A> as isize);
    }
}

/// Cursor functions.
impl Window{
    /// Returns the window's cursor position.
    /// 
    /// Возвращает положение курсора окна.
    pub fn cursor_position(&self)->[i32;2]{
        unsafe{
            let mut point:POINT=std::mem::zeroed();
            GetCursorPos(&mut point);
            ScreenToClient(self.handle,&mut point);
            transmute(point)
        }
    }

    /// Sets the window's cursor position.
    /// 
    /// Устанавливает положение курсора окна.
    pub fn set_cursor_position(&self,[x,y]:[i32;2]){
        unsafe{
            let mut point=POINT{x,y};
            ClientToScreen(self.handle,&mut point);
            SetCursorPos(point.x,point.y);
        }
    }

    pub fn show_cursor(&self,show:bool){
        // This function sets an internal display counter that determines
        // whether the cursor should be displayed.
        // The cursor is displayed only if the display count is greater than or equal to 0.
        // If a mouse is installed, the initial display count is 0.
        // If no mouse is installed, the display count is –1.
        unsafe{
            let counter=ShowCursor(show as i32);

            if show{
                if counter>0{
                    ShowCursor(false as i32);
                }
            }
            else{
                if counter<(-1){
                    ShowCursor(true as i32);
                }
            }
        }
    }
}

impl Drop for Window{
    fn drop(&mut self){
        self.destroy();
    }
}


pub struct WindowAttributes{
    /// The window name and title.
    pub name:OsString,

    /// The window size.
    /// 
    /// The default is `None`.
    pub size:Option<[i32;2]>,

    /// The window position.
    pub position:Option<[i32;2]>,

    /// The default is `true`.
    pub visible:bool,

    /// The window should be placed above all non-topmost windows
    /// and should stay above them,
    /// even when the window is deactivated.
    /// 
    /// The default is `false`.
    pub topmost:bool,

    /// Defines whether a new redraw event is requested
    /// directly after processing the last one.
    /// 
    /// The default is `true`.
    pub auto_redraw:bool,

    /// If `Fullscreen::Monitor` is set the size and position are ignored,
    /// but if some error accures they are used to build a window.
    /// 
    /// The default is `false`.
    pub fullscreen:Fullscreen,
}

impl WindowAttributes{
    pub fn new(name:&str)->WindowAttributes{
        Self{
            name:OsString::from(name),
            size:None,
            position:None,
            visible:true,
            topmost:false,
            auto_redraw:true,
            fullscreen:Fullscreen::None,
        }
    }
}