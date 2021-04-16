use crate::windows::{
    Ticks,
};

use super::{
    // structs
    Icon,
    WindowReference,
    EventHandler,
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
            DefWindowProcW,
            DestroyWindow,
            GetDC,
            UpdateWindow,
            MapVirtualKeyW,
            // constants
            MAPVK_VSC_TO_VK,
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
    sync::{
        Mutex,
        TryLockResult,
    },
    collections::VecDeque,
};

/// Sended only to the main window.
const UPDATE_EVENT:u32=WM_USER;

pub unsafe extern "system" fn window_subclass_procedure(
    window:HWND,
    message:UINT,
    w_param:WPARAM,
    l_param:LPARAM,
    _uIdSubclass:usize,
    dwRefData:usize,
)->LRESULT{
    let (event,return_value):(Event,Option<isize>)=match message{
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
            DestroyWindow(window);
            return DefSubclassProc(window,message,w_param,l_param)
        }

        // Закрытие окна
        WM_DESTROY=>{
            PostQuitMessage(0);
            (
                Event::WindowEvent(WindowEvent::Close),
                None
            )
        }

        // Запрос на перерисовку содержимого окна
        WM_PAINT=>{
            (
                Event::WindowEvent(WindowEvent::Redraw),
                Some(0)
            )
        }

        // Изменение размера окна
        WM_SIZE=>{
            let [width,height,_,_]:[u16;4]=transmute(l_param);
            (
                Event::WindowEvent(WindowEvent::Resize([width,height])),
                Some(0)
            )
        }

        // Сдвиг окна
        WM_MOVE=>{
            let [x,y,_,_]:[i16;4]=transmute(l_param);
            (
                Event::WindowEvent(WindowEvent::Move([x,y])),
                Some(0)
            )
        }

        // Движение мыши
        WM_MOUSEMOVE=>{
            let [x,y,_,_]:[u16;4]=transmute(l_param);
            (
                Event::WindowEvent(WindowEvent::MouseMove([x,y])),
                None
            )
        }

        WM_KEYDOWN=>{
            let [_count1,_count2,scan_code,_flags]:[u8;4]=transmute(l_param as u32);
            let virtual_key=MapVirtualKeyW(scan_code as u32,MAPVK_VSC_TO_VK);
            (
                Event::WindowEvent(WindowEvent::KeyPress(transmute(virtual_key as u8))),
                Some(0)
            )
        }

        WM_KEYUP=>{
            let [_count1,_count2,scan_code,_flags]:[u8;4]=transmute(l_param as u32);
            let virtual_key=MapVirtualKeyW(scan_code as u32,MAPVK_VSC_TO_VK);
            (
                Event::WindowEvent(WindowEvent::KeyRelease(transmute(virtual_key as u8))),
                Some(0)
            )
        }

        UPDATE_EVENT=>{
            let ticks=Ticks(l_param as u64);
            (
                Event::Update(ticks),
                Some(0)
            )
        }

        _=>{
            return DefSubclassProc(window,message,w_param,l_param)
        }
    };

    let event_handler_ptr=dwRefData as *const Mutex<Box<dyn FnMut(Event,&mut LoopControl)>>;
    let event_handler_lock=&*event_handler_ptr;

    let result=std::panic::catch_unwind(||{
        if let TryLockResult::Ok(mut event_handler)=event_handler_lock.try_lock(){
            let mut loop_control=LoopControl::Run;
            event_handler(event,&mut loop_control);
            if let LoopControl::Break=loop_control{
                PostQuitMessage(0);
            }
        }
    });

    if let Err(err)=result{
        println!("{:?}",err);
        PostQuitMessage(0);
    }

    match return_value{
        Some(value)=>value,
        None=>DefSubclassProc(window,message,w_param,l_param)
    }
}