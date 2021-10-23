use crate::windows::{
    WinCore,
    WinError,
    core::window::{
        WindowData,
        WindowHandle
    },
    core::device_context::DeviceContextHandle,
    // level0::cursor::Cursor,
};

pub use crate::windows::core::window::{
    WindowStyle,
    WindowStyles,
    ExtendedWindowStyle,
    ExtendedWindowStyles,
};

use super::{
    // structures
    WindowClass,
    Monitor,
    // trait
    WindowProcedure,
    // functions
    window_procedure,
    // consts
    window_settings_auto_redraw,
};

use winapi::{
    um::{
        winuser::{
            RedrawWindow,
            // other
            CW_USEDEFAULT,
            RDW_INVALIDATE,
            SWP_SHOWWINDOW,
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
};


pub enum Fullscreen{
    None,
    Monitor(Monitor)
}

pub struct CreateParameters<A>{
    pub window_procedure:unsafe extern "system" fn(
        handle:WindowHandle,
        message:u32,
        w_param:usize,
        l_param:isize,
    )->isize,

    pub create_parameters:*mut A,
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
#[repr(transparent)]
pub struct Window{
    pub (crate) handle:WindowHandle,
}

impl Window{
    pub fn new<W:WindowProcedure>(
        class:&WindowClass,
        attributes:WindowAttributes,
        create_parameters:&mut W::CreateParameters,
    )->Result<Window,WinError>{
        let window_name:Vec<u16>=attributes.name
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect();

        let mut style=WindowStyles::new()
            .set(WindowStyle::SystemMenu)
            .set(WindowStyle::Caption);

        let mut extended_style=ExtendedWindowStyles::new();

        // Enabling file dropping
        #[cfg(feature="file_drop")]{
            extended_style|=WS_EX_ACCEPTFILES;
        }

        if attributes.visible{
            style=style.set(WindowStyle::Visible);
        }

        if attributes.topmost{
            extended_style=extended_style.set(ExtendedWindowStyle::TopMost);
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
                style=style.set(WindowStyle::SizeBox)
                    .set(WindowStyle::MaximizeBox)
                    .set(WindowStyle::MaximizeBox);
            }
            Fullscreen::Monitor(monitor)=>{
                if let Some(info)=monitor.get_monitor_info(){
                    style=style.set(WindowStyle::PopUp);
                    style=style.remove(WindowStyle::SizeBox)
                        .remove(WindowStyle::Caption)
                        .remove(WindowStyle::MaximizeBox)
                        .remove(WindowStyle::MaximizeBox);

                    extended_style=extended_style.set(ExtendedWindowStyle::AppWwindow);

                    x=info.rcMonitor.left;
                    y=info.rcMonitor.top;
                    width=info.rcMonitor.right-info.rcMonitor.left;
                    height=info.rcMonitor.bottom-info.rcMonitor.top;
                }
            }
        };

        unsafe{
            let mut create_parameters=CreateParameters{
                window_procedure:window_procedure::<W>,
                create_parameters,
                auto_redraw:attributes.auto_redraw,
            };

            if let Some(window_handle)=WinCore.window.create(
                class.identifier(),
                window_name.as_ptr(),
                style,
                extended_style,
                [x,y,width,height],
                None,
                None,
                None,
                Some(&mut create_parameters),
            ){
                Ok(Self{
                    handle:window_handle,
                })
            }
            else{
                Err(WinError::get_last_error())
            }
        }
    }

    pub fn handle(&self)->WindowHandle{
        self.handle
    }

    pub fn get_context(&self)->Option<DeviceContextHandle>{
        unsafe{
            WinCore.window.get_device_context(Some(self.handle))
        }
    }

    pub unsafe fn get_context_unchecked(&self)->DeviceContextHandle{
        WinCore.window.get_device_context_unchecked(Some(self.handle))
    }
}

/// Requests and sending events.
impl Window{
    pub fn redraw(&self){
        unsafe{
            RedrawWindow(self.handle.as_raw(),null_mut(),null_mut(),RDW_INVALIDATE);
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
                    style=style.set(WindowStyle::SizeBox)
                    .set(WindowStyle::Caption)
                    .set(WindowStyle::MaximizeBox)
                    .set(WindowStyle::MaximizeBox)
                    .remove(WindowStyle::PopUp);

                    extended_style=extended_style.remove(ExtendedWindowStyle::AppWwindow);

                    self.set_style(style);
                    self.set_extended_style(extended_style);
                    WinCore.window.set_window_position(
                        self.handle,
                        None,
                        [0,0,0,0],
                        SWP_SHOWWINDOW|SWP_NOMOVE|SWP_NOSIZE|SWP_FRAMECHANGED
                    );
                }
                Fullscreen::Monitor(monitor)=>{
                    if let Some(info)=monitor.get_monitor_info(){
                        style=style.remove(WindowStyle::SizeBox)
                        .remove(WindowStyle::Caption)
                        .remove(WindowStyle::MaximizeBox)
                        .remove(WindowStyle::MaximizeBox);

                        extended_style=extended_style.set(ExtendedWindowStyle::AppWwindow);

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
    pub unsafe fn set_extended_style(&self,style:ExtendedWindowStyles){
        WinCore.window.set_window_long_ptr(self.handle,WindowData::ExtendedStyle,style.flag as isize);
    }

    pub unsafe fn get_extended_style(&self)->ExtendedWindowStyles{
        ExtendedWindowStyles::raw(WinCore.window.get_window_long_ptr(self.handle,WindowData::ExtendedStyle) as u32)
    }

    /// Will not make effect until you call the `Window::set_window_position()`.
    pub unsafe fn set_style(&self,style:WindowStyles){
        WinCore.window.set_window_long_ptr(self.handle,WindowData::Style,style.flag as isize);
    }

    pub unsafe fn get_style(&self)->WindowStyles{
        WindowStyles::raw(WinCore.window.get_window_long_ptr(self.handle,WindowData::Style) as u32)
    }

    pub unsafe fn set_window_position(&self,[x,y,width,height]:[i32;4]){
        WinCore.window.set_window_position(self.handle,None,[x,y,width,height],SWP_SHOWWINDOW);
    }
}

/// Special functions.
impl Window{
    pub fn set_auto_redraw(&self,enabled:bool){
        unsafe{
            WinCore.window.set_window_long_ptr(self.handle,window_settings_auto_redraw,enabled as isize);
        }
    }

    pub (crate) unsafe fn set_user_data<D:Sized>(&self,data:&mut D){
        WinCore.window.set_window_long_ptr(self.handle,WindowData::UserData,data as *const D as isize);
    }

    pub (crate) unsafe fn get_user_data(&self)->isize{
        WinCore.window.get_window_long_ptr(self.handle,WindowData::UserData)
    }

    pub unsafe fn set_window_procedure(&self,procedure:unsafe extern "system" fn(WindowHandle,u32,usize,isize)->isize){
        WinCore.window.set_window_long_ptr(self.handle,WindowData::WindowProcedure,procedure as isize);
    }

    pub unsafe fn set_window_handle<W:WindowProcedure>(&self){
        WinCore.window.set_window_long_ptr(self.handle,WindowData::WindowProcedure,window_procedure::<W> as isize);
    }
}

/// Cursor functions.
impl Window{
    /// Returns window's cursor position.
    /// 
    /// Возвращает положение курсора окна.
    pub fn cursor_position(&self)->[i32;2]{
        unsafe{
            let mut point=[0i32;2];
            WinCore.cursor.get_position(&mut point);

            WinCore.window.screen_to_client(self.handle,&mut point);

            point
        }
    }

    /// Sets window's cursor position.
    /// 
    /// Устанавливает положение курсора окна.
    pub fn set_cursor_position(&self,mut point:[i32;2]){
        unsafe{
            WinCore.window.client_to_screen(self.handle,&mut point);
            WinCore.cursor.set_position(point);
        }
    }

    pub fn show_cursor(&self,show:bool){
        // This function sets an internal display counter that determines
        // whether the cursor should be displayed.
        // The cursor is displayed only if the display count is greater than or equal to 0.
        // If a mouse is installed, the initial display count is 0.
        // If no mouse is installed, the display count is –1.
        unsafe{
            let counter=WinCore.cursor.show(show);

            if show{
                if counter>0{
                    WinCore.cursor.show(false);
                }
            }
            else{
                if counter<(-1){
                    WinCore.cursor.show(true);
                }
            }
        }
    }
}

impl Drop for Window{
    fn drop(&mut self){
        let _=self.destroy();
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