use x11::{
    xlib::{
        XOpenDisplay,
        XLockDisplay,
        XUnlockDisplay,
        XCloseDisplay,

        Display as XDisplay,
    },
};

use std::{
    ptr::{
        null_mut,
        NonNull,
    },
    mem::{
        transmute,
        transmute_copy,
    },
};

#[derive(Debug,Clone,Copy)]
#[repr(transparent)]
pub struct DisplayHandle{
    inner:NonNull<()>,
}
implement_handle_wrapper!(DisplayHandle,*mut XDisplay);


/// When the X server's connection to a client is closed either by an explicit call to `Display::close`
/// or by a process that exits, the X server performs the following automatic operations:
/// - It disowns all selections owned by the client (see `XSetSelectionOwner`).
/// - It performs an `XUngrabPointer` and `XUngrabKeyboard`
/// if the client has actively grabbed the pointeror the keyboard.
/// - It performs an `XUngrabServer` if the client has grabbed the server.
/// - It releases all passive grabs made by the client.
/// - It marks all resources (including colormap entries) allocatedby the client either as permanent or temporary,
/// depending on whether the close-down mode is `RetainPermanent` or `RetainTemporary`.
/// However, this does not prevent other client applications from explicitly destroying the resources (see `XSetCloseDownMode`).
pub struct Display{}

impl Display{
    pub const fn new()->Display{
        Self{

        }
    }
}

impl Display{
    /// Opens a connection to the X server that controls a display.
    /// `name` - Specifies the hardware display name, which determines the display and communications domain to be used.
    /// On a POSIX-conformant system, if the display_name is NULL,
    /// it defaults to the value of the DISPLAY environment variable.
    /// 
    /// The encoding and interpretation of the display name are implementation-dependent.
    /// Strings in the Host Portable Character Encoding are supported;
    /// support for other characters is implementation-dependent.
    /// On POSIX-conformant systems, the display name or DISPLAY environment variable can be a string in the format:
    /// `protocol/hostname:number.screen_number`.
    /// 
    /// `protocol` - Specifies a protocol family or an alias for a protocol family.
    /// Supported protocol families are implementation dependent.
    /// The protocol entry is optional.
    /// If protocol is not specified, the / separating protocol and hostname must also not be specified.
    /// 
    /// `hostname` - Specifies the name of the host machine on which the display is physically attached.
    /// You follow the hostname with either a single colon (:) or a double colon (::).
    /// 
    /// `number` - Specifies the number of the display server on that host machine.
    /// You may optionally follow this display number with a period (.).
    /// A single CPU can have more than one display.
    /// Multiple displays are usually numbered starting with zero.
    /// 
    /// `screen_number` - Specifies the screen to be used on that server.
    /// Multiple screens can be controlled by a single X server.
    /// The `screen_number` sets an internal variable
    /// that can be accessed by using the the `XDefaultScreen` function.
    /// 
    /// For example, the following would specify screen 1 of display 0 on the machine named “dual-headed”:
    /// `dual-headed:0.1`.
    /// 
    /// Returns a `Display` structure that serves as the connection to the X server
    /// and that contains all the information about that X server.
    /// `Display::open` connects your application to the X server through TCP or DECnet communications protocols,
    /// or through some local inter-process communication protocol.
    /// If the protocol is specified as "tcp", "inet", or "inet6",
    /// or if no protocol is specified and the hostname is a host machine name
    /// and a single colon (:) separates the hostname and display number, `Display::open` connects using TCP streams.
    /// (If the protocol is specified as "inet", TCP over IPv4 is used.
    /// If the protocol is specified as "inet6", TCP over IPv6 is used.
    /// Otherwise, the implementation determines which IP version is used.)
    /// If the hostname and protocol are both not specified,
    /// Xlib uses whatever it believes is the fastest transport.
    /// If the hostname is a host machine name and a double colon (::) separates the hostname
    /// and display number,`Display::open` connects using DECnet.
    /// A single X server can support any or all of these transport mechanisms simultaneously.
    /// A particular Xlib implementation can support many more of these transport mechanisms.
    /// 
    /// If successful, `Display::open` returns a pointer to a `Display` structure (`DisplayHandle`).
    /// If `Display::open` does not succeed, it returns `None`.
    /// After a successful call to `Display::open`, all of the screens in the display can be used by the client.
    /// The screen number specified in the `display_name` argument is returned by the `XDefaultScreen` function.
    /// You can access elements of the `Display` and `Screen` structures only by using the information macros or functions.
    /// For information about using macros and functions to obtain information from the `Display` structure.
    /// 
    /// X servers may implement various types of access control mechanisms.
    pub fn open(&self,name:*const i8)->Option<DisplayHandle>{
        unsafe{
            DisplayHandle::from_raw(XOpenDisplay(name))
        }
    }

    /// Locks a display across several Xlib calls.
    /// 
    /// Locks out all other threads from using the specified display.
    /// Other threads attempting to use the display will block until the display is unlocked by this thread.
    /// Nested calls to `Display::lock` work correctly; the display will not actually be unlocked
    /// until `Display::unlock` has been called the same number of times as Display::lock`.
    /// This function has no effect unless Xlib was successfully initialized for threads using `XInitThreads`.
    pub fn lock(&self,display:DisplayHandle){
        unsafe{
            XLockDisplay(display.as_raw())
        }
    }

    /// Unlock a display.
    /// 
    /// Allows other threads to use the specified display again.
    /// Any threads that have blocked on the display are allowed to continue.
    /// Nested locking works correctly;
    /// if `Display::unlock` has been called multiple times by a thread,
    /// then `Display::unlock` must be called an equal number of times before the display is actually unlocked.
    /// This function has no effect unless Xlib was successfully initialized for threads using `XInitThreads`.
    pub fn unlock(&self,display:DisplayHandle){
        unsafe{
            XUnlockDisplay(display.as_raw())
        }
    }

    /// Closes a display or disconnects from the X server.
    /// 
    /// Closes the connection to the X server for the display
    /// specified in the `Display` structure and destroys all windows,
    /// resource IDs (Window, Font, Pixmap, Colormap, Cursor, and GContext),
    /// or other resources that the client has created on this display,
    /// unless the close-down mode of the client has been changed (see `XSetCloseDownMode`).
    /// Therefore, these windows, resource IDs,
    /// and other resources should never be referenced again
    /// or an error will be generated. Before exiting,
    /// you should call `Display::close` explicitly so that any pending errors are reported
    /// as `Display::close` performs a final XSync operation.
    /// 
    /// Can generate a `BadGC` (`13i32`) error otherwise it returns `0i32`.
    pub fn close(&self,display:DisplayHandle)->i32{
        unsafe{
            XCloseDisplay(display.as_raw())
        }
    }
}