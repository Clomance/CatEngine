pub mod display;
use display::Display;

pub mod window;
use window::Window;

pub struct X11Core{
    pub display:Display,
}

impl X11Core{
    pub const fn new()->X11Core{
        Self{
            display:Display::new(),
        }
    }
}