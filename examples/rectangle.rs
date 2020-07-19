use cat_engine::{
    PagedWindow,
    Window,
    WindowPage,
    WindowEvent,
    MouseButton,
    KeyboardButton,
    glium::glutin::event::MouseScrollDelta
};

use std::path::PathBuf;

pub struct Page{
    rect:cat_engine::graphics::Rectangle,
}

impl WindowPage<'static> for Page{
    type Window=PagedWindow;
    type Output=();

    fn on_close_requested(&mut self,_window:&mut PagedWindow){
        println!("Closing");
    }

    #[cfg(not(feature="lazy"))]
    fn on_update_requested(&mut self,_window:&mut PagedWindow){}

    fn on_redraw_requested(&mut self,window:&mut PagedWindow){
        window.draw(|p,g|{
            g.clear_colour([1.0;4]);
            self.rect.draw(p,g).unwrap();
        }).unwrap()
    }

    fn on_mouse_pressed(&mut self,_window:&mut PagedWindow,_button:MouseButton){}
    fn on_mouse_released(&mut self,_window:&mut PagedWindow,_button:MouseButton){}
    fn on_mouse_moved(&mut self,_window:&mut PagedWindow,_:[f32;2]){}
    fn on_mouse_scrolled(&mut self,_window:&mut PagedWindow,_:MouseScrollDelta){}

    fn on_keyboard_pressed(&mut self,window:&mut PagedWindow,button:KeyboardButton){
        match button{
            KeyboardButton::Escape=>{
                // break out of the page
                let _=window.stop_events();
            }
            _=>{}
        }
    }

    fn on_keyboard_released(&mut self,_window:&mut PagedWindow,_button:KeyboardButton){}

    fn on_character_recieved(&mut self,_window:&mut PagedWindow,_character:char){}

    fn on_window_resized(&mut self,_window:&mut PagedWindow,_new_size:[u32;2]){}

    fn on_suspended(&mut self,_window:&mut PagedWindow){}
    fn on_resumed(&mut self,_window:&mut PagedWindow){}

    fn on_window_moved(&mut self,_window:&mut PagedWindow,_:[i32;2]){}

    fn on_window_focused(&mut self,_window:&mut PagedWindow,_:bool){}

    #[cfg(feature="file_drop")]
    fn on_file_dropped(&mut self,_:&mut PagedWindow,_:PathBuf){}
    #[cfg(feature="file_drop")]
    fn on_file_hovered(&mut self,_:&mut PagedWindow,_:PathBuf){}
    #[cfg(feature="file_drop")]
    fn on_file_hovered_canceled(&mut self,_:&mut PagedWindow){}

    fn on_event_loop_closed(&mut self,_:&mut PagedWindow){}
}

fn main(){
    // One way
    let rect=cat_engine::graphics::Rectangle::new([100.0;4],[1.0,0.0,0.0,1.0]);
    let mut page=Page{
        rect,
    };

    let mut window=PagedWindow::new(|_,_|{}).unwrap();

    window.run_page(&mut page);

    // or another
    let rect=cat_engine::graphics::Rectangle::new([100.0;4],[0.0,0.0,0.0,1.0]);

    window.run(|window,event|{
        match event{
            WindowEvent::Exit=>{
                println!("Exit");
            }
            WindowEvent::Draw=>{
                // I'm gonna draw ya
                window.draw(|p,g|{
                    g.clear_colour([1.0;4]);
                    rect.draw(p,g).unwrap();
                }).unwrap();
            }
            WindowEvent::KeyboardPressed(button)=>match button{
                KeyboardButton::Escape=>{
                    // I'm gonna break ya, break ya, break ya, break ya
                    let _=window.stop_events(); // break out of the loop
                }
                _=>{}
            }
            _=>{}
        }
    });
}