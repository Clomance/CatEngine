use std::mem::transmute;

use winapi::um::winuser::{
    // GetMessageA,
    GetMessageW,
    PostQuitMessage
};

use super::window::WindowHandle;


/// The message identifier.
/// Applications can only use the low word; the high word is reserved by the system.
#[repr(C)]
pub struct MessageIdentifier{
    inner:u32
}



/// Contains message information from a thread's message queue.
#[repr(C)]
pub struct MessageInfo{
    /// A handle to the window whose window procedure receives the message.
    /// This member is `None` when the message is a thread message.
    pub window_handle:Option<WindowHandle>,

    /// The message identifier.
    /// Applications can only use the low word;
    /// the high word is reserved by the system.
    pub message:MessageIdentifier,

    /// Additional information about the message.
    /// The exact meaning depends on the value of the message member.
    pub w_param:usize,

    /// Additional information about the message.
    /// The exact meaning depends on the value of the message member.
    pub l_param:isize,

    /// The time at which the message was posted.
    pub time:u32,

    /// The cursor position, in screen coordinates, when the message was posted.
    pub cursor:[i32;2],

    private:u32
}



pub struct Message;

impl Message{
    // BroadcastSystemMessage
    // BroadcastSystemMessageEx
    // DispatchMessage
    // GetInputState
    // 
    // GetMessageExtraInfo
    // GetMessagePos
    // GetMessageTime
    // GetQueueStatus
    // InSendMessage
    // InSendMessageEx
    // PeekMessage
    // PostMessage
    // 
    // PostThreadMessage
    // RegisterWindowMessage
    // ReplyMessage
    // SendAsyncProc
    // SendMessage
    // SendMessageCallback
    // SendMessageTimeout
    // SendNotifyMessage
    // SetMessageExtraInfo
    // TranslateMessage
    // WaitMessage

    /// Retrieves a message from the calling thread's message queue.
    /// The function dispatches incoming sent messages until a posted message is available for retrieval.
    /// 
    /// Unlike `Message::get`, the `Message::peek` function does not wait for a message to be posted before returning.
    /// 
    /// `message` - a pointer to an MSG structure that receives message information from the thread's message queue.
    /// 
    /// `window` - a handle to the window whose messages are to be retrieved.
    /// The window must belong to the current thread.
    /// 
    /// If `window` is `None`, `Message::get` retrieves messages for any window that belongs to the current thread,
    /// and any messages on the current thread's message queue whose `window` value is `None` (see the `MessageInfo` structure).
    /// Therefore if `window` is `None`, both window messages and thread messages are processed.
    /// 
    /// If `window` is `-1`, `Message::get` retrieves only messages on the current thread's message queue whose `window` value is `None`,
    /// that is, thread messages as posted by `Message::post` (when the `window` parameter is `None`) or `Message::post_thread`.
    /// 
    /// `min_filter` - the integer value of the lowest message value to be retrieved.
    /// Use WM_KEYFIRST (0x0100) to specify the first keyboard message or WM_MOUSEFIRST (0x0200) to specify the first mouse message.
    /// 
    /// `max_filter` - the integer value of the highest message value to be retrieved.
    /// Use WM_KEYLAST to specify the last keyboard message or WM_MOUSELAST to specify the last mouse message.
    /// 
    /// Use WM_INPUT `min_filter` and in `max_filter to specify only the WM_INPUT messages.
    /// 
    /// If `min_filter` and `max_filter` are both zero, `Message::get` returns all available messages (that is, no range filtering is performed).
    /// 
    /// 
    pub fn get(
        message:&mut MessageInfo,
        window:Option<WindowHandle>,
        min_filter:MessageIdentifier,
        max_filter:MessageIdentifier
    )->i32{
        unsafe{
            GetMessageW(
                message as *mut MessageInfo as *mut _,
                transmute(window),
                min_filter.inner,
                max_filter.inner
            )
        }
    }

    pub fn post_quit(exit_code:i32){
        unsafe{
            PostQuitMessage(exit_code)
        }
    }
}