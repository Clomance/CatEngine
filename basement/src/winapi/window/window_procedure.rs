use crate::winapi::{
    WindowEvent,
    MouseButton,
    backend::core::window::{
        Window as WindowFunctions,
        WindowData,
        WindowHandle,

        default_window_procedure_wide,
    },
};

use super::{
    // structs
    Window,
    CreateParameters,
    // traits
    WindowProcedure,
    // enums
    WindowMessage,
    WindowResizeType,
};

use core::mem::transmute;

use winapi::{
    um::{
        winuser::{
            // structs
            CREATESTRUCTW,
            // functions
            MapVirtualKeyW,
            // constants
            MAPVK_VSC_TO_VK,
            WHEEL_DELTA,
            // window messages
            WM_NULL,
            WM_CREATE,
            WM_DESTROY,
            WM_MOVE,
            WM_SIZE,
            WM_ACTIVATE,
            WM_SETFOCUS,
            WM_KILLFOCUS,
            WM_ENABLE,
            WM_SETREDRAW,
            WM_SETTEXT,
            WM_GETTEXT,
            WM_GETTEXTLENGTH,
            WM_PAINT,
            WM_CLOSE,
            WM_QUERYENDSESSION,
            WM_QUERYOPEN,
            WM_ENDSESSION,
            WM_QUIT,
            WM_ERASEBKGND,
            WM_SYSCOLORCHANGE,
            WM_SHOWWINDOW,
            WM_WININICHANGE,
            WM_SETTINGCHANGE,
            WM_DEVMODECHANGE,
            WM_ACTIVATEAPP,
            WM_FONTCHANGE,
            WM_TIMECHANGE,
            WM_CANCELMODE,
            WM_SETCURSOR,
            WM_MOUSEACTIVATE,
            WM_CHILDACTIVATE,
            WM_QUEUESYNC,
            WM_GETMINMAXINFO,
            WM_PAINTICON,
            WM_ICONERASEBKGND,
            WM_NEXTDLGCTL,
            WM_SPOOLERSTATUS,
            WM_DRAWITEM,
            WM_MEASUREITEM,
            WM_DELETEITEM,
            WM_VKEYTOITEM,
            WM_CHARTOITEM,
            WM_SETFONT,
            WM_GETFONT,
            WM_SETHOTKEY,
            WM_GETHOTKEY,
            WM_QUERYDRAGICON,
            WM_COMPAREITEM,
            WM_GETOBJECT,
            WM_COMPACTING,
            WM_COMMNOTIFY,
            WM_WINDOWPOSCHANGING,
            WM_WINDOWPOSCHANGED,
            WM_POWER,
            WM_COPYDATA,
            WM_CANCELJOURNAL,
            WM_NOTIFY,
            WM_INPUTLANGCHANGEREQUEST,
            WM_INPUTLANGCHANGE,
            WM_TCARD,
            WM_HELP,
            WM_USERCHANGED,
            WM_NOTIFYFORMAT,
            WM_CONTEXTMENU,
            WM_STYLECHANGING,
            WM_STYLECHANGED,
            WM_DISPLAYCHANGE,
            WM_GETICON,
            WM_SETICON,
            WM_NCCREATE,
            WM_NCDESTROY,
            WM_NCCALCSIZE,
            WM_NCHITTEST,
            WM_NCPAINT,
            WM_NCACTIVATE,
            WM_GETDLGCODE,
            WM_SYNCPAINT,
            WM_NCMOUSEMOVE,
            WM_NCLBUTTONDOWN,
            WM_NCLBUTTONUP,
            WM_NCLBUTTONDBLCLK,
            WM_NCRBUTTONDOWN,
            WM_NCRBUTTONUP,
            WM_NCRBUTTONDBLCLK,
            WM_NCMBUTTONDOWN,
            WM_NCMBUTTONUP,
            WM_NCMBUTTONDBLCLK,
            WM_NCXBUTTONDOWN,
            WM_NCXBUTTONUP,
            WM_NCXBUTTONDBLCLK,
            WM_INPUT_DEVICE_CHANGE,
            WM_INPUT,
            WM_KEYFIRST,
            WM_KEYDOWN,
            WM_KEYUP,
            WM_CHAR,
            WM_DEADCHAR,
            WM_SYSKEYDOWN,
            WM_SYSKEYUP,
            WM_SYSCHAR,
            WM_SYSDEADCHAR,
            WM_UNICHAR,
            WM_KEYLAST,
            WM_IME_STARTCOMPOSITION,
            WM_IME_ENDCOMPOSITION,
            WM_IME_COMPOSITION,
            WM_IME_KEYLAST,
            WM_INITDIALOG,
            WM_COMMAND,
            WM_SYSCOMMAND,
            WM_TIMER,
            WM_HSCROLL,
            WM_VSCROLL,
            WM_INITMENU,
            WM_INITMENUPOPUP,
            WM_GESTURE,
            WM_GESTURENOTIFY,
            WM_MENUSELECT,
            WM_MENUCHAR,
            WM_ENTERIDLE,
            WM_MENURBUTTONUP,
            WM_MENUDRAG,
            WM_MENUGETOBJECT,
            WM_UNINITMENUPOPUP,
            WM_MENUCOMMAND,
            WM_CHANGEUISTATE,
            WM_UPDATEUISTATE,
            WM_QUERYUISTATE,
            WM_CTLCOLORMSGBOX,
            WM_CTLCOLOREDIT,
            WM_CTLCOLORLISTBOX,
            WM_CTLCOLORBTN,
            WM_CTLCOLORDLG,
            WM_CTLCOLORSCROLLBAR,
            WM_CTLCOLORSTATIC,
            WM_MOUSEFIRST,
            WM_MOUSEMOVE,
            WM_LBUTTONDOWN,
            WM_LBUTTONUP,
            WM_LBUTTONDBLCLK,
            WM_RBUTTONDOWN,
            WM_RBUTTONUP,
            WM_RBUTTONDBLCLK,
            WM_MBUTTONDOWN,
            WM_MBUTTONUP,
            WM_MBUTTONDBLCLK,
            WM_MOUSEWHEEL,
            WM_XBUTTONDOWN,
            WM_XBUTTONUP,
            WM_XBUTTONDBLCLK,
            WM_MOUSEHWHEEL,
            WM_MOUSELAST,
            WM_PARENTNOTIFY,
            WM_ENTERMENULOOP,
            WM_EXITMENULOOP,
            WM_NEXTMENU,
            WM_SIZING,
            WM_CAPTURECHANGED,
            WM_MOVING,
            WM_POWERBROADCAST,
            WM_DEVICECHANGE,
            WM_MDICREATE,
            WM_MDIDESTROY,
            WM_MDIACTIVATE,
            WM_MDIRESTORE,
            WM_MDINEXT,
            WM_MDIMAXIMIZE,
            WM_MDITILE,
            WM_MDICASCADE,
            WM_MDIICONARRANGE,
            WM_MDIGETACTIVE,
            WM_MDISETMENU,
            WM_ENTERSIZEMOVE,
            WM_EXITSIZEMOVE,
            WM_DROPFILES,
            WM_MDIREFRESHMENU,
            WM_POINTERDEVICECHANGE,
            WM_POINTERDEVICEINRANGE,
            WM_POINTERDEVICEOUTOFRANGE,
            WM_TOUCH,
            WM_NCPOINTERUPDATE,
            WM_NCPOINTERDOWN,
            WM_NCPOINTERUP,
            WM_POINTERUPDATE,
            WM_POINTERDOWN,
            WM_POINTERUP,
            WM_POINTERENTER,
            WM_POINTERLEAVE,
            WM_POINTERACTIVATE,
            WM_POINTERCAPTURECHANGED,
            WM_TOUCHHITTESTING,
            WM_POINTERWHEEL,
            WM_POINTERHWHEEL,
            WM_IME_SETCONTEXT,
            WM_IME_NOTIFY,
            WM_IME_CONTROL,
            WM_IME_COMPOSITIONFULL,
            WM_IME_SELECT,
            WM_IME_CHAR,
            WM_IME_REQUEST,
            WM_IME_KEYDOWN,
            WM_IME_KEYUP,
            WM_MOUSEHOVER,
            WM_MOUSELEAVE,
            WM_NCMOUSEHOVER,
            WM_NCMOUSELEAVE,
            WM_WTSSESSION_CHANGE,
            WM_TABLET_FIRST,
            WM_TABLET_LAST,
            WM_CUT,
            WM_COPY,
            WM_PASTE,
            WM_CLEAR,
            WM_UNDO,
            WM_RENDERFORMAT,
            WM_RENDERALLFORMATS,
            WM_DESTROYCLIPBOARD,
            WM_DRAWCLIPBOARD,
            WM_PAINTCLIPBOARD,
            WM_VSCROLLCLIPBOARD,
            WM_SIZECLIPBOARD,
            WM_ASKCBFORMATNAME,
            WM_CHANGECBCHAIN,
            WM_HSCROLLCLIPBOARD,
            WM_QUERYNEWPALETTE,
            WM_PALETTEISCHANGING,
            WM_PALETTECHANGED,
            WM_HOTKEY,
            WM_PRINT,
            WM_PRINTCLIENT,
            WM_APPCOMMAND,
            WM_THEMECHANGED,
            WM_CLIPBOARDUPDATE,
            WM_DWMCOMPOSITIONCHANGED,
            WM_DWMNCRENDERINGCHANGED,
            WM_DWMCOLORIZATIONCOLORCHANGED,
            WM_DWMWINDOWMAXIMIZEDCHANGE,
            WM_DWMSENDICONICTHUMBNAIL,
            WM_DWMSENDICONICLIVEPREVIEWBITMAP,
            WM_GETTITLEBARINFOEX,
            WM_HANDHELDFIRST,
            WM_HANDHELDLAST,
            WM_AFXFIRST,
            WM_AFXLAST,
            WM_PENWINFIRST,
            WM_PENWINLAST,
            WM_APP,
            WM_USER,
        },
    },
    shared::winerror::ERROR_UNWIND
};

/// Стартовая функция для всех окон.
/// 
/// Устанавливает начальные параметры.
pub unsafe extern "system" fn setup_window_procedure(
    window_handle:WindowHandle,
    message:u32,
    w_param:usize,
    l_param:isize,
)->isize{
    let message:WindowMessage=transmute(message);

    // 
    if let WindowMessage::NonClientCreate=message{
        let create_struct:&mut CREATESTRUCTW=transmute(l_param);

        let create_parameters:&mut CreateParameters<u8>=transmute(create_struct.lpCreateParams);

        // Установка новой функции окна
        WindowFunctions::set_window_long_ptr(window_handle,WindowData::WindowProcedure,create_parameters.window_procedure as isize);

        // Чтобы в `let data=*(window.get_user_data() as *mut W::Data);`
        // не использовался нулевой указатель (в `wrap_window_procedure`)
        WindowFunctions::set_window_long_ptr(window_handle,WindowData::UserData,l_param);
    }
    default_window_procedure_wide(window_handle,message as u32,w_param,l_param)
}

/// Процедура окна.
pub unsafe extern "system" fn window_procedure<W:WindowProcedure>(
    window_handle:WindowHandle,
    message:u32,
    w_param:usize,
    l_param:isize,
)->isize{
    let window:&Window=transmute(&window_handle);

    #[cfg(feature="wnd_proc_catch_panic")]{
        let result=std::panic::catch_unwind(||wrap_window_procedure::<W>(window,message,w_param,l_param));

        match result{
            Ok(result)=>result,
            Err(e)=>{
                // Установка функции окна по умолчанию
                WindowFunctions::set_window_long_ptr(window_handle,WindowData::WindowProcedure,default_window_procedure_wide as isize);

                let data=window.get_user_data() as *mut W::Data;
                W::catch_panic(window,Some(&mut *data),e);

                Box::from_raw(data);

                default_window_procedure_wide(window_handle,message as u32,w_param,l_param)
            }
        }
    }

    #[cfg(not(feature="wnd_proc_catch_panic"))]
    wrap_window_procedure::<W>(window,message,w_param,l_param)
}

unsafe extern "system" fn wrap_window_procedure<W:WindowProcedure>(window:&Window,message:u32,w_param:usize,l_param:isize)->isize{
    let message:WindowMessage=transmute(message);
    // Тут точно не нулевой указатель: в `default_window_procedure_wide` сюда устанавливается
    // ссылка на параметры окна. После, здесь этот указатель меняется.
    let data=&mut *(window.get_user_data() as *mut W::Data);

    match message{
        // 
        WindowMessage::Create=>{
            let create_struct:&mut CREATESTRUCTW=transmute(l_param);
            let create_parameters:&mut CreateParameters<W::CreateParameters>=transmute(create_struct.lpCreateParams);

            let parameters=create_parameters.create_parameters as isize;
            // Вызов функции пользователя
            #[cfg(feature="wnd_proc_catch_panic")]
            let result=std::panic::catch_unwind(||{W::create(window,transmute(parameters))});

            #[cfg(not(feature="wnd_proc_catch_panic"))]
            let result=Ok(W::create(window,transmute(parameters)));

            match result{
                Ok(result)=>{
                    match result{
                        Ok(data)=>{
                            // Упаковка данных
                            let boxed_data=Box::new(data);
                            let data_ptr=Box::leak(boxed_data) as *mut W::Data;
                            // Установка параметров
                            WindowFunctions::set_window_long_ptr(window.handle(),WindowData::UserData,data_ptr as isize);

                            W::data_packed(window,transmute(parameters),&mut *data_ptr);

                            0
                        }
                        Err(error)=>{
                            // Возвращение ошибки обратно
                            winapi::um::errhandlingapi::SetLastError(error.code());

                            // Остановка создания окна
                            -1
                        }
                    }
                }

                Err(error)=>{
                    // Чтобы при `WindowMessage::Destroy` не вызывать дескруктор для пустых данных
                    WindowFunctions::set_window_long_ptr(window.handle(),WindowData::UserData,0isize);

                    W::catch_panic(window,None,error);

                    // Возвращение ошибки обратно
                    winapi::um::errhandlingapi::SetLastError(ERROR_UNWIND);
                    -1
                }
            }
        }

        // 
        WindowMessage::Close=>{
            W::close(window,data);
            0
        }

        // 
        WindowMessage::Destroy=>{
            if !(data as *mut W::Data).is_null(){
                W::destroy(window,data);
                // Вызов ленивого деструктора
                Box::from_raw(data as *mut W::Data);
            }
            0
        }

        // Запрос на перерисовку содержимого окна
        WindowMessage::Paint=>{
            W::paint(window,data);
            0
        }

        #[cfg(feature="set_cursor_event")]
        WindowMessage::SetCursor=>{
            W::set_cursor(window,data);
            0
        }

        WindowMessage::Size=>{
            let [client_width,client_height,_,_]:[u16;4]=transmute(l_param);
            let resize_type:WindowResizeType=transmute(w_param);
            W::resized([client_width,client_height],resize_type,window,data);
            0
        }

        WindowMessage::Move=>{
            let [x,y,_,_]:[i16;4]=transmute(l_param);
            W::moved([x,y],window,data);
            0
        }

        WindowMessage::App=>{
            W::user_event(w_param,l_param,window,data);
            0
        }

        _=>{
            match wrap_event(window,message as u32,w_param,l_param){
                EventWrapResult::None(lresult)=>lresult,
                EventWrapResult::Event(window_event,lresult)=>{
                    W::handle(window_event,window,data);
                    lresult
                }
            }
        }
    }
}

enum EventWrapResult{
    None(isize),
    Event(WindowEvent,isize)
}

unsafe fn wrap_event(window:&Window,message:u32,w_param:usize,l_param:isize)->EventWrapResult{
    match message{
        // Mouse events
        // События мыши
        // Движение мыши
        WM_MOUSEMOVE=>{
            let [x,y,_,_]:[u16;4]=transmute(l_param);
            EventWrapResult::Event(
                WindowEvent::MouseMove([x,y]),
                0
            )
        }
        // Нажата левая кнопка мыши
        WM_LBUTTONDOWN=>{
            let [x,y,_,_]:[u16;4]=transmute(l_param);
            EventWrapResult::Event(
                WindowEvent::MousePress{
                    cursor_position:[x,y],
                    button:MouseButton::Left,
                },
                0
            )
        }
        // Нажата средняя кнопка мыши
        WM_MBUTTONDOWN=>{
            let [x,y,_,_]:[u16;4]=transmute(l_param);
            EventWrapResult::Event(
                WindowEvent::MousePress{
                    cursor_position:[x,y],
                    button:MouseButton::Middle,
                },
                0
            )
        }
        // Нажата правая кнопка мыши
        WM_RBUTTONDOWN=>{
            let [x,y,_,_]:[u16;4]=transmute(l_param);
            EventWrapResult::Event(
                WindowEvent::MousePress{
                    cursor_position:[x,y],
                    button:MouseButton::Right,
                },
                0
            )
        }
        // Нажата дополнительная кнопка мыши (4 или 5)
        WM_XBUTTONDOWN=>{
            let [x,y,_,_]:[u16;4]=transmute(l_param);
            let [_,button,_,_]:[u16;4]=transmute(w_param);
            let button=if button==0{
                MouseButton::Button4
            }
            else{
                MouseButton::Button5
            };

            EventWrapResult::Event(
                WindowEvent::MousePress{
                    cursor_position:[x,y],
                    button,
                },
                0
            )
        }
        // Отпущена левая кнопка мыши
        WM_LBUTTONUP=>{
            let [x,y,_,_]:[u16;4]=transmute(l_param);
            EventWrapResult::Event(
                WindowEvent::MouseRelease{
                    cursor_position:[x,y],
                    button:MouseButton::Left,
                },
                0
            )
        }
        // Отпущена средняя кнопка мыши
        WM_MBUTTONUP=>{
            let [x,y,_,_]:[u16;4]=transmute(l_param);
            EventWrapResult::Event(
                WindowEvent::MouseRelease{
                    cursor_position:[x,y],
                    button:MouseButton::Middle,
                },
                0
            )
        }
        // Отпущена правая кнопка мыши
        WM_RBUTTONUP=>{
            let [x,y,_,_]:[u16;4]=transmute(l_param);
            EventWrapResult::Event(
                WindowEvent::MouseRelease{
                    cursor_position:[x,y],
                    button:MouseButton::Right,
                },
                0
            )
        }
        // Отпущена дополнительная кнопка мыши (4 или 5)
        WM_XBUTTONUP=>{
            let [x,y,_,_]:[u16;4]=transmute(l_param);
            let [_,button,_,_]:[u16;4]=transmute(w_param);
            let button=if button==0{
                MouseButton::Button4
            }
            else{
                MouseButton::Button5
            };

            EventWrapResult::Event(
                WindowEvent::MouseRelease{
                    cursor_position:[x,y],
                    button,
                },
                0
            )
        }
        // Прокрутка колёсика
        WM_MOUSEWHEEL=>{
            let [_,scroll_delta,_,_]:[i16;4]=transmute(w_param);
            EventWrapResult::Event(
                WindowEvent::MouseScroll(scroll_delta/WHEEL_DELTA),
                0
            )
        }

        // Keyboard events
        // События клавиатуры
        // Клавиша нажата
        WM_KEYDOWN=>{
            let [_count1,_count2,scan_code,_flags]:[u8;4]=transmute(l_param as u32);
            let virtual_key=MapVirtualKeyW(scan_code as u32,MAPVK_VSC_TO_VK);
            EventWrapResult::Event(
                WindowEvent::KeyPress(transmute(virtual_key)),
                0
            )
        }
        // Клавиша отпущена
        WM_KEYUP=>{
            let [_count1,_count2,scan_code,_flags]:[u8;4]=transmute(l_param as u32);
            let virtual_key=MapVirtualKeyW(scan_code as u32,MAPVK_VSC_TO_VK);
            EventWrapResult::Event(
                WindowEvent::KeyRelease(transmute(virtual_key)),
                0
            )
        }
        // Ввод символов (обычные|системные|составные - é)
        WM_CHAR|WM_SYSCHAR|WM_DEADCHAR=>{
            let utf16_character=w_param as u16;
            let character=std::char::decode_utf16(vec![utf16_character]).next().unwrap().unwrap();
            EventWrapResult::Event(
                WindowEvent::CharacterInput(character),
                0
            )
        }

        _=>EventWrapResult::None(default_window_procedure_wide(window.handle,message,w_param,l_param))
    }
}