use crate::windows::{
    Ticks,
};

use super::{
    // structs
    Icon,
    WindowSubclassArguments,
    // enums
    Event,
    WindowEvent,
    KeyboardButton,
    VirtualKeyCode,
    MouseButton,
    LoopControl,
};

use winapi::{
    shared::{
        minwindef::{
            LPARAM,
            UINT,
            WPARAM,
            LRESULT,
        },
        windef::{
            HWND,
        }
    },

    um::{
        winuser::{
            // structs
            CREATESTRUCTW,
            MONITORINFO,
            // functions
            PostQuitMessage,
            PostMessageW,
            SendMessageW,
            PostThreadMessageW,
            DefWindowProcW,
            DestroyWindow,
            GetDC,
            ReleaseDC,
            UpdateWindow,
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

            // The window sizing and positioning flags
            SWP_NOSIZE,
            SWP_NOREDRAW,
        },

        wingdi::{
            CreateSolidBrush,
            RGB,
            CreateBitmap,
        },
        commctrl::{
            // functions
            SetWindowSubclass,
            DefSubclassProc,
        },
        //errhandlingapi::{GetLastError},
    }
};

use image::RgbaImage;

use gl::{
    Viewport,
};

use std::{
    ptr::null_mut,
    mem::transmute,
};

const DESTROY_EVENT:u32=WM_APP+1;

pub unsafe extern "system" fn window_subclass_procedure(
    window:HWND,
    message:UINT,
    w_param:WPARAM,
    l_param:LPARAM,
    _uIdSubclass:usize,
    dwRefData:usize,
)->LRESULT{
    let window_subclass_arguments_ptr=dwRefData as *const WindowSubclassArguments;
    let window_subclass_arguments=&*window_subclass_arguments_ptr;

    let main_thread_id=window_subclass_arguments.main_thread_id;
    let window_id=window_subclass_arguments.window_id;

    let (window_event,return_value):(WindowEvent,isize)=match message{
        // Sent prior to the WM_CREATE message when a window is first created.
        // wParam - This parameter is not used.
        // lParam - A pointer to the CREATESTRUCT structure.
        // If an application processes this message, it should return TRUE to continue creation of the window.
        // If the application returns FALSE, the CreateWindow or CreateWindowEx function will return a NULL handle.
        WM_NCCREATE=>{
            return 1;
        }

        // The window procedure of the new window receives this message after the window is created,
        // but before the window becomes visible.
        // wParam - This parameter is not used.
        // lParam - A pointer to the CREATESTRUCT structure.
        // If an application processes this message, it should return zero to continue creation of the window.
        // If the application returns –1, the window is destroyed and the CreateWindowEx or CreateWindow function returns a NULL handle.
        WM_CREATE=>{
            return 0;
        }

        // Запрос на закрытие окна
        WM_CLOSE=>{
            (
                WindowEvent::CloseRequest,
                0
            )
        }

        // Sent when a window is being destroyed.
        // It is sent to the window procedure of the window being destroyed
        // after the window is removed from the screen.
        // This message is sent first to the window being destroyed
        // and then to the child windows (if any) as they are destroyed.
        // During the processing of the message, it can be assumed that all child windows still exist.
        // A window receives this message through its WindowProc function.
        // If the window being destroyed is part of the clipboard viewer chain (set by calling the SetClipboardViewer function),
        // the window must remove itself from the chain by processing the ChangeClipboardChain function before returning from the WM_DESTROY message.
        WM_DESTROY=>{
            (
                WindowEvent::Destroy,
                0
            )
        }

        // Запрос на перерисовку содержимого окна
        // WM_PAINT=>{
        //     (
        //         WindowEvent::Redraw,
        //         0
        //     )
        // }

        // Изменение размера окна
        WM_SIZE=>{
            let [width,height,_,_]:[u16;4]=transmute(l_param);
            (
                WindowEvent::Resize([width,height]),
                0
            )
        }

        // Сдвиг окна
        WM_MOVE=>{
            let [x,y,_,_]:[i16;4]=transmute(l_param);
            (
                WindowEvent::Move([x,y]),
                0
            )
        }

        // Mouse events
        // События мыши
        // Движение мыши
        WM_MOUSEMOVE=>{
            let [x,y,_,_]:[u16;4]=transmute(l_param);
            (
                WindowEvent::MouseMove([x,y]),
                0
            )
        }
        // Нажата левая кнопка мыши
        WM_LBUTTONDOWN=>{
            let [x,y,_,_]:[u16;4]=transmute(l_param);
            (
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
            (
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
            (
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

            (
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
            (
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
            (
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
            (
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

            (
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
            (
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
            (
                WindowEvent::KeyPress(transmute(virtual_key as u8)),
                0
            )
        }
        // Клавиша отпущена
        WM_KEYUP=>{
            let [_count1,_count2,scan_code,_flags]:[u8;4]=transmute(l_param as u32);
            let virtual_key=MapVirtualKeyW(scan_code as u32,MAPVK_VSC_TO_VK);
            (
                WindowEvent::KeyRelease(transmute(virtual_key as u8)),
                0
            )
        }
        // Ввод символов (обычные|системные|составные - é)
        WM_CHAR|WM_SYSCHAR|WM_DEADCHAR=>{
            let utf16_character=w_param as u16;
            let character=std::char::decode_utf16(vec![utf16_character]).next().unwrap().unwrap();
            (
                WindowEvent::CharacterInput(character),
                0
            )
        }

        // Additional events
        // Дополнительные события
        DESTROY_EVENT=>{
            DestroyWindow(window);
            return 0;
        }

        _=>{
            return DefSubclassProc(window,message,w_param,l_param)
        }
    };

    // Упаковывание... подарочков :)
    let boxed_event=Box::new(window_event);
    let arguments_ptr=Box::leak(boxed_event) as *mut WindowEvent as isize;
    // Отправка подарочков в главный поток
    PostThreadMessageW(main_thread_id as u32,WM_APP,window_id,arguments_ptr);

    return_value
}