use crate::windows::{
    Window,
    Event,
    ProcessEvent,
    WindowEvent,
    MouseButton,
};

use winapi::um::{
    winuser::{
        MSG,
        GetMessageW,
        PeekMessageW,
        TranslateMessage,
        DispatchMessageW,
        MapVirtualKeyW,

        WHEEL_DELTA,
        MAPVK_VSC_TO_VK,
        PM_REMOVE,
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
    profileapi::{
        QueryPerformanceCounter,
        QueryPerformanceFrequency,
    },
};

use std::{
    mem::{
        transmute,
        zeroed
    },
    ptr::null_mut,
};

#[derive(Clone,Debug)]
pub enum LoopControl{
    /// The loop is running with the defeault settings.
    Run,

    /// Updates are disabled.
    Lazy,

    /// The loop will be closed.
    Break,
}

unsafe impl Sync for LoopControl{}
unsafe impl Send for LoopControl{}

/// Represents interval between events or an event rate.
/// 
/// The event is disabled if an interval or rate is `0`.
#[derive(Clone,Copy,Debug)]
pub enum EventInterval{
    Ticks(u32),
    EventsPerSecond(u32),
    NanoSeconds(u32),
    Seconds(u32),
}

impl EventInterval{
    pub fn into_ticks(self)->i64{
        let mut frequency=0i64;
        unsafe{
            QueryPerformanceFrequency(transmute(&mut frequency));
        }

        match self{
            EventInterval::Ticks(ticks)=>ticks as i64,
            EventInterval::EventsPerSecond(updates)=>{
                if updates==0{
                    0i64
                }
                else{
                    frequency/updates as i64
                }
            }
            EventInterval::NanoSeconds(nanoseconds)=>{
                (nanoseconds as i64*frequency)/1_000_000_000i64
            }
            EventInterval::Seconds(nanoseconds)=>{
                nanoseconds as i64*frequency
            }
        }
    }
}

#[derive(Debug,Clone,Copy)]
pub struct Ticks(pub u64);

impl Ticks{
    pub fn as_seconds(self)->u64{
        let mut frequency=0u64;
        unsafe{
            QueryPerformanceFrequency(transmute(&mut frequency));
        }
        self.0/frequency
    }

    pub fn as_nanoseconds(self)->u64{
        let mut frequency=0u64;
        unsafe{
            QueryPerformanceFrequency(transmute(&mut frequency));
        }
        (self.0*1_000_000_000u64)/frequency
    }
}

// Mister Programmer, may I have some loops?
//      /\_____/\
//     /  o   o  \
//    ( ==  ^  == )
//     )         (
//    (           )
//   ( (  )   (  ) )
//  (__(__)___(__)__)

pub struct EventLoop{
    // в тактах
    update_interval:i64,
    redraw_request_interval:i64,
}

impl EventLoop{
    pub fn new(attributes:EventLoopAttributes)->EventLoop{

        let update_interval=attributes.update_interval.into_ticks();

        let redraw_request_interval=attributes.redraw_request_interval.into_ticks();

        Self{
            update_interval,
            redraw_request_interval,
        }
    }

    /// Runs an event loop.
    /// 
    /// Запускает цикл событий.
    pub fn run<F:FnMut(Event,&mut LoopControl)>(&mut self,mut f:F){
        unsafe{
            let mut message:MSG=zeroed();

            // Флаг управления циклом
            let mut loop_control=LoopControl::Run;

            // Время последнего события обновления в тактах
            let mut last_update=0i64;
            let mut current_ticks=0i64;
            QueryPerformanceCounter(transmute(&mut last_update));
            let mut last_redraw_request=last_update;

            // Начальное событие
            f(Event::Process(ProcessEvent::EventLoopStart),&mut loop_control);

            loop{
                match loop_control{
                    LoopControl::Run=>{
                        if PeekMessageW(&mut message,null_mut(),0,0,PM_REMOVE)==1{
                            match message.message{
                                WM_QUIT=>{
                                    f(Event::Process(ProcessEvent::Quit),&mut loop_control);
                                    break
                                }
                                _=>event_handler(&message,&mut f,&mut loop_control),
                            }
                        }

                        // если интервал не нулевой (событие включёно)
                        if self.update_interval!=0{
                            // Текущее время в тактах
                            QueryPerformanceCounter(transmute(&mut current_ticks));
                            // проверка интервала события обновления
                            let ticks_passed=current_ticks-last_update;
                            if ticks_passed>=self.update_interval{
                                if ticks_passed<self.update_interval<<1{
                                    last_update+=self.update_interval;
                                }
                                else{
                                    last_update=current_ticks;
                                }

                                f(
                                    Event::Process(ProcessEvent::Update(Ticks(ticks_passed as u64))),
                                    &mut loop_control
                                );
                            }
                        }
                    },

                    LoopControl::Lazy=>
                        match GetMessageW(&mut message,null_mut(),0,0){
                            -1=>break,
                            0=>{
                                f(Event::Process(ProcessEvent::Quit),&mut loop_control);
                                break
                            }

                            _=>event_handler(&message,&mut f,&mut loop_control),
                        }

                    LoopControl::Break=>break,
                }

                if !message.hwnd.is_null(){
                    // если интервал не нулевой (событие включёно)
                    if self.redraw_request_interval!=0{
                        // текущее время в тактах
                        QueryPerformanceCounter(transmute(&mut current_ticks));
                        // проверка интервала события отрисовки
                        let ticks_passed=current_ticks-last_redraw_request;
                        if ticks_passed>=self.redraw_request_interval{
                            if ticks_passed<self.redraw_request_interval<<1{
                                last_redraw_request+=self.redraw_request_interval;
                            }
                            else{
                                last_redraw_request=current_ticks;
                            }

                            let window:&Window=transmute(&message.hwnd);
                            window.redraw();
                        }
                    }
                }
            }

            // Завершение цикла
            f(Event::Process(ProcessEvent::EventLoopBreak),&mut loop_control);
        }
    }
}

impl EventLoop{
    pub fn set_update_interval(&mut self,interval:EventInterval){
        self.update_interval=interval.into_ticks()
    }

    pub fn set_redraw_request_interval(&mut self,interval:EventInterval){
        self.redraw_request_interval=interval.into_ticks()
    }
}

#[derive(Clone,Debug)]
pub struct EventLoopAttributes{
    /// Defines an interval between `Process::Update` events.
    /// 
    /// The event is disabled if an interval or rate is `0`.
    /// 
    /// The default is `EventInteval::UpdatesPerSecond(50u32)`.
    pub update_interval:EventInterval,

    /// Defines an interval between `Window::Redraw` requests.
    /// 
    /// The event is disabled if an interval or rate is `0`.
    /// 
    /// The default is `EventInteval::UpdatesPerSecond(0u32)`
    /// (disabled).
    pub redraw_request_interval:EventInterval,
}

impl EventLoopAttributes{
    pub fn new()->EventLoopAttributes{
        Self{
            update_interval:EventInterval::EventsPerSecond(50u32),
            redraw_request_interval:EventInterval::EventsPerSecond(0u32),
        }
    }
}

fn event_handler<F:FnMut(Event,&mut LoopControl)>(message:&MSG,f:&mut F,control:&mut LoopControl){
    unsafe{
        DispatchMessageW(message);

        let event=match message.message{
            // Keyboard events
            // События клавиатуры
            // Клавиша нажата
            WM_KEYDOWN=>{
                TranslateMessage(message);
                let [_count1,_count2,scan_code,_flags]:[u8;4]=transmute(message.lParam as u32);
                let virtual_key=MapVirtualKeyW(scan_code as u32,MAPVK_VSC_TO_VK);
                WindowEvent::KeyPress(transmute(virtual_key as u8))
            }

            // Клавиша отпущена
            WM_KEYUP=>{
                TranslateMessage(message);
                let [_count1,_count2,scan_code,_flags]:[u8;4]=transmute(message.lParam as u32);
                let virtual_key=MapVirtualKeyW(scan_code as u32,MAPVK_VSC_TO_VK);
                WindowEvent::KeyRelease(transmute(virtual_key as u8))
            }

            WM_SYSKEYDOWN|WM_SYSKEYUP=>{
                TranslateMessage(message);
                return
            }

            // Ввод символов (обычные|системные|составные - é)
            WM_CHAR|WM_SYSCHAR|WM_DEADCHAR=>{
                let utf16_character=message.wParam as u16;
                let character=std::char::decode_utf16(vec![utf16_character]).next().unwrap().unwrap();
                WindowEvent::CharacterInput(character)
            }

            // Mouse events
            // События мыши
            // Движение мыши
            WM_MOUSEMOVE=>{
                let [x,y,_,_]:[u16;4]=transmute(message.lParam);
                WindowEvent::MouseMove([x,y])
            }
            // Нажата левая кнопка мыши
            WM_LBUTTONDOWN=>{
                let [x,y,_,_]:[u16;4]=transmute(message.lParam);
                WindowEvent::MousePress{
                    cursor_position:[x,y],
                    button:MouseButton::Left,
                }
            }
            // Нажата средняя кнопка мыши
            WM_MBUTTONDOWN=>{
                let [x,y,_,_]:[u16;4]=transmute(message.lParam);
                WindowEvent::MousePress{
                    cursor_position:[x,y],
                    button:MouseButton::Middle,
                }
            }
            // Нажата правая кнопка мыши
            WM_RBUTTONDOWN=>{
                let [x,y,_,_]:[u16;4]=transmute(message.lParam);
                WindowEvent::MousePress{
                    cursor_position:[x,y],
                    button:MouseButton::Right,
                }
            }
            // Нажата дополнительная кнопка мыши (4 или 5)
            WM_XBUTTONDOWN=>{
                let [x,y,_,_]:[u16;4]=transmute(message.lParam);
                let [_,button,_,_]:[u16;4]=transmute(message.wParam);
                let button=if button==0{
                    MouseButton::Button4
                }
                else{
                    MouseButton::Button5
                };

                WindowEvent::MousePress{
                    cursor_position:[x,y],
                    button,
                }
            }
            // Отпущена левая кнопка мыши
            WM_LBUTTONUP=>{
                let [x,y,_,_]:[u16;4]=transmute(message.lParam);
                WindowEvent::MouseRelease{
                    cursor_position:[x,y],
                    button:MouseButton::Left,
                }
            }
            // Отпущена средняя кнопка мыши
            WM_MBUTTONUP=>{
                let [x,y,_,_]:[u16;4]=transmute(message.lParam);
                WindowEvent::MouseRelease{
                    cursor_position:[x,y],
                    button:MouseButton::Middle,
                }
            }
            // Отпущена правая кнопка мыши
            WM_RBUTTONUP=>{
                let [x,y,_,_]:[u16;4]=transmute(message.lParam);
                WindowEvent::MouseRelease{
                    cursor_position:[x,y],
                    button:MouseButton::Right,
                }
            }
            // Отпущена дополнительная кнопка мыши (4 или 5)
            WM_XBUTTONUP=>{
                let [x,y,_,_]:[u16;4]=transmute(message.lParam);
                let [_,button,_,_]:[u16;4]=transmute(message.wParam);
                let button=if button==0{
                    MouseButton::Button4
                }
                else{
                    MouseButton::Button5
                };

                WindowEvent::MouseRelease{
                    cursor_position:[x,y],
                    button,
                }
            }
            // Прокр колёсика
            WM_MOUSEWHEEL=>{
                let [_,scroll_delta,_,_]:[i16;4]=transmute(message.wParam);
                WindowEvent::MouseScroll(scroll_delta/WHEEL_DELTA)
            }
            _=>return,
        };
        f(Event::Window(event),control);
    }
}