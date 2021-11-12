macro_rules! implement_handle_wrapper{
    ($wrapper:ty,$raw_handle:ty) => {
        impl $wrapper{
            #[inline(always)]
            pub fn from_raw(raw_handle:$raw_handle)->Option<$wrapper>{
                unsafe{
                    transmute(raw_handle)
                }
            }

            #[inline(always)]
            pub unsafe fn from_raw_unchecked(raw_handle:$raw_handle)->$wrapper{
                transmute(raw_handle)
            }

            #[inline(always)]
            pub fn to_raw(handle:Option<$wrapper>)->$raw_handle{
                unsafe{
                    transmute(handle)
                }
            }

            #[inline(always)]
            pub fn as_raw(&self)->$raw_handle{
                unsafe{
                    transmute_copy(self)
                }
            }
        }

        unsafe impl Sync for $wrapper{}
        unsafe impl Send for $wrapper{}
    };
}

pub mod core;
use self::core::X11Core;

mod display;
pub use display::Display;

mod event_loop;
pub use event_loop::{
    EventLoop,
    EventType,
    Event,
    LoopControl,
};

mod render_context;
pub use render_context::RenderContext;

mod window;
pub use window::{
    Window,
};

pub use x11;

pub static mut XCore:X11Core=X11Core::new();