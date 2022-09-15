use crate::winapi::{
    Error,
    backend::core::{
        window::{
            Window as WindowFunctions,
            PaintStruct,
        },
        device_context::DeviceContextHandle,
        cursor::Cursor as CursorFunctions,
    },
    monitor::Monitor,
};

pub use crate::winapi::backend::core::window::{
    WindowStyle,
    WindowStyles,
    ExtendedWindowStyle,
    ExtendedWindowStyles,
    WindowData,
    WindowHandle,
};

use super::{
    // structures
    WindowClass,
    // trait
    WindowProcedure,
    // functions
    window_procedure, messages::WindowMessage,
};

use winapi::{
    um::{
        winuser::{
            UpdateWindow,
            RedrawWindow,
            SendMessageW,

            WM_USER,

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
    )->Result<Window,Error>{
        let window_name:Vec<u16>=attributes.name
            .encode_utf16()
            .chain([0].into_iter())
            .collect();

        let mut style=WindowStyles::new()
            .set(WindowStyle::SystemMenu)
            .set(WindowStyle::Caption);

        let mut extended_style=ExtendedWindowStyles::new();

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
                    .set(WindowStyle::MinimizeBox)
                    .set(WindowStyle::MaximizeBox);
            }
            Fullscreen::Monitor(monitor)=>{
                if let Some(info)=monitor.get_monitor_info(){
                    style=style.set(WindowStyle::PopUp);
                    style=style.remove(WindowStyle::SizeBox)
                        .remove(WindowStyle::Caption)
                        .remove(WindowStyle::MinimizeBox)
                        .remove(WindowStyle::MaximizeBox);

                    extended_style=extended_style.set(ExtendedWindowStyle::AppWwindow);

                    x=info.rcMonitor.left;
                    y=info.rcMonitor.top;
                    width=info.rcMonitor.right-info.rcMonitor.left;
                    height=info.rcMonitor.bottom-info.rcMonitor.top;
                }
            }
        };

        let mut create_parameters=CreateParameters{
            window_procedure:window_procedure::<W>,
            create_parameters,
        };

        if let Some(window_handle)=WindowFunctions::create_extended_wide(
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
            Err(Error::get_last_error())
        }
    }

    pub fn handle(&self)->WindowHandle{
        self.handle
    }

    pub fn get_context(&self)->Option<DeviceContextHandle>{
        WindowFunctions::get_device_context(Some(self.handle))
    }

    pub unsafe fn get_context_unchecked(&self)->DeviceContextHandle{
        WindowFunctions::get_device_context_unchecked(Some(self.handle))
    }
}

/// Requests and sending events.
impl Window{
    pub fn redraw(&self){
        unsafe{
            RedrawWindow(self.handle.as_raw(),null_mut(),null_mut(),RDW_INVALIDATE);
        }
    }

    pub fn destroy(&self)->Result<(),Error>{
        if WindowFunctions::destroy(self.handle){
            Ok(())
        }
        else{
            Err(Error::get_last_error())
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
        WindowFunctions::get_window_rectangle(self.handle,&mut window_rectangle);
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
        WindowFunctions::get_window_rectangle(self.handle,&mut window_rectangle);
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
        WindowFunctions::get_window_rectangle(self.handle,&mut window_rectangle);
        window_rectangle
    }

    /// Returns window's client area size.
    /// 
    /// Возвращает размеры клиентской области окна.
    /// 
    /// [width, height]
    pub fn client_size(&self)->[u32;2]{
        let mut client_rectangle=[0i32;4];
        WindowFunctions::get_client_rectangle(self.handle,&mut client_rectangle);
        let [_,_,width,height]=client_rectangle;

        [
            width as u32,
            height as u32,
        ]
    }
}

/// Redrawing.
impl Window{
    pub fn draw<F:FnMut()>(&self,mut f:F)->bool{
        unsafe{
            #[allow(invalid_value)]
            let mut paint:PaintStruct=std::mem::MaybeUninit::uninit().assume_init();
            if let Some(_)=WindowFunctions::begin_paint(self.handle,&mut paint){
                f();
                WindowFunctions::end_paint(self.handle,&paint);
                true
            }
            else{
                false
            }
        }
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
                    .set(WindowStyle::MinimizeBox)
                    .set(WindowStyle::MaximizeBox)
                    .remove(WindowStyle::PopUp);

                    extended_style=extended_style.remove(ExtendedWindowStyle::AppWwindow);

                    self.set_style(style);
                    self.set_extended_style(extended_style);
                    WindowFunctions::set_window_position(
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
                        .remove(WindowStyle::MinimizeBox)
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
        WindowFunctions::set_window_long_ptr(self.handle,WindowData::ExtendedStyle,style.flag as isize);
    }

    pub unsafe fn get_extended_style(&self)->ExtendedWindowStyles{
        ExtendedWindowStyles::raw(WindowFunctions::get_window_long_ptr(self.handle,WindowData::ExtendedStyle) as u32)
    }

    /// Will not make effect until you call the `Window::set_window_position()`.
    pub unsafe fn set_style(&self,style:WindowStyles){
        WindowFunctions::set_window_long_ptr(self.handle,WindowData::Style,style.flag as isize);
    }

    pub unsafe fn get_style(&self)->WindowStyles{
        WindowStyles::raw(WindowFunctions::get_window_long_ptr(self.handle,WindowData::Style) as u32)
    }

    pub unsafe fn set_window_position(&self,[x,y,width,height]:[i32;4]){
        WindowFunctions::set_window_position(self.handle,None,[x,y,width,height],SWP_SHOWWINDOW);
    }
}

/// Special functions.
impl Window{
    pub fn send_app_message(&self,w_param:usize,l_param:isize){
        unsafe{
            SendMessageW(self.handle.as_raw(),WindowMessage::App as u32,w_param,l_param);
        }
    }

    pub unsafe fn get_user_data<Data>(&self)->*mut Data{
        WindowFunctions::get_window_long_ptr(self.handle,WindowData::UserData) as *mut Data
    }

    pub unsafe fn set_window_procedure(&self,procedure:unsafe extern "system" fn(WindowHandle,u32,usize,isize)->isize){
        WindowFunctions::set_window_long_ptr(self.handle,WindowData::WindowProcedure,procedure as isize);
    }

    pub unsafe fn set_window_handler<W:WindowProcedure>(&self){
        WindowFunctions::set_window_long_ptr(self.handle,WindowData::WindowProcedure,window_procedure::<W> as isize);
    }
}

/// Cursor functions.
impl Window{
    /// Returns window's cursor position.
    /// 
    /// Возвращает положение курсора окна.
    pub fn cursor_position(&self)->[i32;2]{
        let mut point=[0i32;2];
        CursorFunctions::get_position(&mut point);

        WindowFunctions::screen_to_client(self.handle,&mut point);

        point
    }

    /// Sets window's cursor position.
    /// 
    /// Устанавливает положение курсора окна.
    pub fn set_cursor_position(&self,mut point:[i32;2]){
        WindowFunctions::client_to_screen(self.handle,&mut point);
        CursorFunctions::set_position(point);
    }

    pub fn show_cursor(&self,show:bool){
        // This function sets an internal display counter that determines
        // whether the cursor should be displayed.
        // The cursor is displayed only if the display count is greater than or equal to 0.
        // If a mouse is installed, the initial display count is 0.
        // If no mouse is installed, the display count is –1.
        let counter=CursorFunctions::show(show);

        if show{
            if counter>0{
                CursorFunctions::show(false);
            }
        }
        else{
            if counter<(-1){
                CursorFunctions::show(true);
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
    pub name:String,

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

    /// If `Fullscreen::Monitor` is set the size and position are ignored,
    /// but if some error accures they are used to build a window.
    /// 
    /// The default is `false`.
    pub fullscreen:Fullscreen,
}

impl WindowAttributes{
    pub fn new(name:&str)->WindowAttributes{
        Self{
            name:String::from(name),
            size:None,
            position:None,
            visible:true,
            topmost:false,
            fullscreen:Fullscreen::None,
        }
    }
}