#![cfg(all(feature="paged_format",feature="alpha_smoothing"))]

use cat_engine::{
    image::{ImageBase,Texture},
    Window,
    WindowPage,
    MouseButton,
    KeyboardButton,
    glium::glutin::event::MouseScrollDelta
};

use std::path::PathBuf;

pub struct Page{
    smoothing:bool,
    image_base:ImageBase,
    texture:Texture,
}


impl WindowPage for Page{
    fn on_close_requested(&mut self,_window:&mut Window){
        println!("Closing");
    }

    fn on_redraw_requested(&mut self,window:&mut Window){
        if self.smoothing{
            let next_alpha=window.draw_smooth(|alpha,p,g|{
                self.image_base.colour_filter[3]=alpha;

                g.clear_colour([1.0;4]);
                self.image_base.draw(&self.texture,p,g).unwrap();
            });

            if next_alpha>1.0{
                self.image_base.colour_filter[3]=1.0;
                self.smoothing=false;
            }
        }
        else{
            window.draw(|p,g|{
                g.clear_colour([1.0;4]);
                self.image_base.draw(&self.texture,p,g).unwrap();
            })
        }
    }

    fn on_mouse_pressed(&mut self,_window:&mut Window,_button:MouseButton){}
    fn on_mouse_released(&mut self,_window:&mut Window,_button:MouseButton){}
    fn on_mouse_moved(&mut self,_window:&mut Window,_:[f32;2]){}
    fn on_mouse_scrolled(&mut self,_window:&mut Window,_:MouseScrollDelta){}

    fn on_keyboard_pressed(&mut self,window:&mut Window,button:KeyboardButton){
        match button{
            KeyboardButton::Escape=>{
                // break out of the page
                let _=window.stop_events();
            }
            _=>{}
        }
    }

    fn on_keyboard_released(&mut self,_window:&mut Window,_button:KeyboardButton){}

    fn on_character_recieved(&mut self,_window:&mut Window,_character:char){}

    fn on_window_resized(&mut self,_window:&mut Window,_new_size:[u32;2]){}

    fn on_suspended(&mut self,_window:&mut Window){}
    fn on_resumed(&mut self,_window:&mut Window){}

    fn on_window_moved(&mut self,_window:&mut Window,_:[i32;2]){}

    fn on_window_focused(&mut self,_window:&mut Window,_:bool){}

    fn on_file_dropped(&mut self,_:&mut Window,_:PathBuf){}
    fn on_file_hovered(&mut self,_:&mut Window,_:PathBuf){}
    fn on_file_hovered_canceled(&mut self,_:&mut Window){}
}


fn main(){
    let image_base=ImageBase::new([1.0;4],[
        100f32,
        100f32,
        400f32,
        400f32
    ]);

    let mut window=Window::new(|_,sets|{
        sets.vsync=true;
    }).unwrap();

    let texture=Texture::from_path("logo_400x400.png",window.display()).unwrap();

    let mut page=Page{
        smoothing:true,
        image_base,
        texture,
    };


    window.set_new_smooth(0.0625);
    window.run_page(&mut page);
}