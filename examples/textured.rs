use cat_engine::{
    texture::{ImageBase,Texture},
    Window,
    WindowPage,
    MouseButton,
    KeyboardButton,
    MouseScrollDelta,
    ModifiersState,
    graphics::Graphics2D,
};

use std::path::PathBuf;

pub struct Page{
    angle:f32,
    texture:usize,
    graphics:Graphics2D,
}


impl WindowPage<'static> for Page{
    type Output=();

    fn on_window_close_requested(&mut self,_window:&mut Window){
        println!("Closing");
    }

    #[cfg(not(feature="lazy"))]
    fn on_update_requested(&mut self,_window:&mut Window){
        self.angle+=0.001;
    }

    fn on_redraw_requested(&mut self,window:&Window){
        #[cfg(feature="lazy")]{
            self.angle+=0.001;
        }

        window.draw(&self.graphics,|g|{
            g.clear_colour([1.0;4]);
            // Drawing static image
            g.draw_textured_object(self.texture).unwrap();
            // Drawing rotating image
            g.draw_rotate_textured_object(self.texture,[200f32,200f32],self.angle).unwrap();
        }).unwrap();
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

    fn on_modifiers_changed(&mut self,_window:&mut Window,_modifiers:ModifiersState){}

    #[cfg(feature="file_drop")]
    fn on_file_dropped(&mut self,_:&mut Window,_:PathBuf){}
    #[cfg(feature="file_drop")]
    fn on_file_hovered(&mut self,_:&mut Window,_:PathBuf){}
    #[cfg(feature="file_drop")]
    fn on_file_hovered_canceled(&mut self,_:&mut Window){}

    fn on_event_loop_closed(&mut self,_:&mut Window){}
}


fn main(){
    let image_base=ImageBase::new(
        [
            100f32,
            100f32,
            400f32,
            400f32
        ],
        [1.0;4],
    );

    let (mut window,mut graphics)=Window::new(|_,settings|{
        settings.vsync=true;

        // we don't use 'active' drawing so we don't need space for it
        settings.graphics_base_settings.texture.vertex_buffer_offset=0;
        settings.graphics_base_settings.texture.index_buffer_offset=0;
    }).unwrap();

    let texture=Texture::from_path("logo_400x400.png",window.display()).unwrap();

    graphics.add_texture(texture);

    let texture=graphics.add_textured_object(&image_base,0).unwrap();

    let mut page=Page{
        angle:0f32,
        texture,
        graphics
    };

    window.run_page(&mut page);
}