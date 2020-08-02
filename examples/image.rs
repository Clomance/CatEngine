use cat_engine::{
    texture::{ImageBase,Texture},
    Window,
    PagedWindow,
    WindowPage,
    MouseButton,
    KeyboardButton,
    MouseScrollDelta,
    ModifiersState,
};

use std::path::PathBuf;

pub struct Page{
    angle:f32,
    image_base:ImageBase,
    texture:Texture,
}


impl WindowPage<'static> for Page{
    type Window=PagedWindow;
    type Output=();

    fn on_window_close_requested(&mut self,_window:&mut PagedWindow){
        println!("Closing");
    }

    #[cfg(not(feature="lazy"))]
    fn on_update_requested(&mut self,_window:&mut PagedWindow){
        self.angle+=0.001;
    }

    fn on_redraw_requested(&mut self,window:&mut PagedWindow){
        #[cfg(feature="lazy")]{
            self.angle+=0.001;
        }

        window.draw(|p,g|{
            g.clear_colour([1.0;4]);
            // Drawing static image
            self.image_base.draw(&self.texture,p,g).unwrap();
            // Drawing rotating image
            self.image_base.draw_rotate(&self.texture,[200f32,200f32],self.angle,p,g).unwrap();
        }).unwrap();
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

    fn on_modifiers_changed(&mut self,_window:&mut PagedWindow,_modifiers:ModifiersState){}

    #[cfg(feature="file_drop")]
    fn on_file_dropped(&mut self,_:&mut PagedWindow,_:PathBuf){}
    #[cfg(feature="file_drop")]
    fn on_file_hovered(&mut self,_:&mut PagedWindow,_:PathBuf){}
    #[cfg(feature="file_drop")]
    fn on_file_hovered_canceled(&mut self,_:&mut PagedWindow){}

    fn on_event_loop_closed(&mut self,_:&mut PagedWindow){}
}


fn main(){
    let image_base=ImageBase::new([1.0;4],[
        100f32,
        100f32,
        400f32,
        400f32
    ]);

    let mut window=PagedWindow::new(|_,_|{}).unwrap();

    let texture=Texture::from_path("logo_400x400.png",window.display()).unwrap();

    let mut page=Page{
        angle:0f32,
        image_base,
        texture,
    };

    window.run_page(&mut page);
}