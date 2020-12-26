use cat_engine::{
    Window,
    WindowPage,
    MouseButton,
    KeyboardButton,
    MouseScrollDelta,
    ModifiersState,
    glium::draw_parameters::PolygonMode,
    shapes::Rectangle,
    graphics::Graphics2D,
};

use std::path::PathBuf;

pub struct Page{
    rect:Rectangle,
    angle:f32,
    shift:[f32;2],
    graphics:Graphics2D,
}

impl WindowPage<'static> for Page{
    type Output=();

    fn on_window_close_requested(&mut self,_window:&mut Window){
        println!("Closing");
    }

    #[cfg(not(feature="lazy"))]
    fn on_update_requested(&mut self,_window:&mut Window){
        self.angle+=0.01;
        self.shift[0]+=1f32;
    }

    fn on_redraw_requested(&mut self,window:&Window){
        window.draw(&self.graphics,|g|{
            g.clear_colour([1.0;4]);
            self.rect.draw(g).unwrap();

            g.draw_parameters.polygon_mode=PolygonMode::Line;
            // rotating and drawing
            self.rect.draw_rotate([150f32;2],self.angle,g).unwrap();

            g.draw_parameters.polygon_mode=PolygonMode::Fill;
            // shifting and drawing
            self.rect.draw_shift(self.shift,g).unwrap();
        }).unwrap()
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
    let (mut window,graphics)=Window::new(|_,s|{
        s.vsync=true;
    }).unwrap();

    let rect=Rectangle::new([100.0;4],[1.0,0.0,0.0,1.0]);
    let mut page=Page{
        rect,
        shift:[0f32;2],
        angle:0f32,
        graphics,
    };

    window.run_page(&mut page);
}