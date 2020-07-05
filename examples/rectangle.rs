// The paged window format \\
// The default window format is below \\
#[cfg(feature="paged_format")]
use cat_engine::{
    Window,
    WindowPage,
    MouseButton,
    KeyboardButton,
    glium::glutin::event::MouseScrollDelta
};

#[cfg(feature="paged_format")]
pub struct Page{
    rect:cat_engine::graphics::Rectangle,
}

#[cfg(feature="paged_format")]
impl WindowPage for Page{
    fn on_close_requested(&mut self,_window:&mut Window){
        println!("Closing");
    }

    fn on_redraw_requested(&mut self,window:&mut Window){
        window.draw(|p,g|{
            g.clear_colour([1.0;4]);
            self.rect.draw(p,g).unwrap();
        })
    }

    fn on_mouse_pressed(&mut self,_window:&mut Window,_button:MouseButton){}
    fn on_mouse_released(&mut self,_window:&mut Window,_button:MouseButton){}
    fn on_mouse_moved(&mut self,_window:&mut Window,_:[f32;2]){}
    fn on_mouse_scrolled(&mut self,_window:&mut Window,_:MouseScrollDelta){}

    fn on_keyboard_pressed(&mut self,_window:&mut Window,_button:KeyboardButton){}
    fn on_keyboard_released(&mut self,_window:&mut Window,_button:KeyboardButton){}

    fn on_character_recieved(&mut self,_window:&mut Window,_character:char){}

    fn on_window_resized(&mut self,_window:&mut Window,_new_size:[u32;2]){}

    fn on_suspended(&mut self,_window:&mut Window){}
    fn on_resumed(&mut self,_window:&mut Window){}

    fn on_window_moved(&mut self,_window:&mut Window,_:[i32;2]){}

    fn on_window_focused(&mut self,_window:&mut Window,_:bool){}
}

#[cfg(feature="paged_format")]
fn main(){
    let rect=cat_engine::graphics::Rectangle::new([100.0;4],[1.0,0.0,0.0,1.0]);
    let mut page=Page{
        rect,
    };

    let mut window=Window::new(|_,_|{}).unwrap();

    window.run(&mut page);
}


// The default window format \\
#[cfg(not(feature="paged_format"))]
use cat_engine::{
    Window,
    WindowEvent,
};

#[cfg(not(feature="paged_format"))]
fn main(){
    let mut window=Window::new(|_,_|{}).unwrap();

    let rect=cat_engine::graphics::Rectangle::new([100.0;4],[1.0,0.0,0.0,1.0]);

    while let Some(event)=window.next_event(){
        match event{
            WindowEvent::Exit=>{
                break
            }
            WindowEvent::Draw=>{
                window.draw(|p,g|{
                    g.clear_colour([1.0;4]);
                    rect.draw(p,g).unwrap();
                })
            }
            _=>{}
        }
    }
}