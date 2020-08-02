use cat_engine::{
    texture::{ImageBase,Texture},
    Window,
    DynamicWindow,
    WindowPage,
    MouseButton,
    KeyboardButton,
    MouseScrollDelta,
    ModifiersState,
};

use std::path::PathBuf;

pub struct Page<'a>{
    angle:f32,
    image_base:ImageBase,
    texture:Texture,
    page2:Option<&'a mut dyn WindowPage<'a,Window=DynamicWindow<'a>,Output=()>>,
}


impl<'a> WindowPage<'a> for Page<'a>{
    type Window=DynamicWindow<'a>;
    type Output=();

    fn on_window_close_requested(&mut self,_window:&mut DynamicWindow<'a>){
        println!("Closing");
    }

    #[cfg(not(feature="lazy"))]
    fn on_update_requested(&mut self,_window:&mut DynamicWindow<'a>){
        self.angle+=0.001;
    }

    fn on_redraw_requested(&mut self,window:&mut DynamicWindow<'a>){
        #[cfg(not(feature="lazy"))]{
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

    fn on_mouse_pressed(&mut self,_window:&mut DynamicWindow<'a>,_button:MouseButton){}
    fn on_mouse_released(&mut self,_window:&mut DynamicWindow<'a>,_button:MouseButton){}
    fn on_mouse_moved(&mut self,_window:&mut DynamicWindow<'a>,_:[f32;2]){}
    fn on_mouse_scrolled(&mut self,_window:&mut DynamicWindow<'a>,_:MouseScrollDelta){}

    fn on_keyboard_pressed(&mut self,window:&mut DynamicWindow<'a>,button:KeyboardButton){
        match button{
            // Press A to switch the pages
            KeyboardButton::A=>{
                if let Some(page)=self.page2.take(){
                    window.set_page(page);
                }
                else{
                    let page1=window.take_old_page().unwrap();
                    window.set_page(page1);
                }
            }
            KeyboardButton::Escape=>{
                // break out of the page
                let _=window.stop_events();
            }
            _=>{}
        }
    }

    fn on_keyboard_released(&mut self,_window:&mut DynamicWindow<'a>,_button:KeyboardButton){}

    fn on_character_recieved(&mut self,_window:&mut DynamicWindow<'a>,_character:char){}

    fn on_window_resized(&mut self,_window:&mut DynamicWindow<'a>,_new_size:[u32;2]){}

    fn on_suspended(&mut self,_window:&mut DynamicWindow<'a>){}
    fn on_resumed(&mut self,_window:&mut DynamicWindow<'a>){}

    fn on_window_moved(&mut self,_window:&mut DynamicWindow<'a>,_:[i32;2]){}

    fn on_window_focused(&mut self,_window:&mut DynamicWindow<'a>,_:bool){}

    fn on_modifiers_changed(&mut self,_window:&mut DynamicWindow<'a>,_modifiers:ModifiersState){}

    #[cfg(feature="file_drop")]
    fn on_file_dropped(&mut self,_:&mut DynamicWindow<'a>,_:PathBuf){}
    #[cfg(feature="file_drop")]
    fn on_file_hovered(&mut self,_:&mut DynamicWindow<'a>,_:PathBuf){}
    #[cfg(feature="file_drop")]
    fn on_file_hovered_canceled(&mut self,_:&mut DynamicWindow<'a>){}

    fn on_event_loop_closed(&mut self,_:&mut DynamicWindow<'a>){}
}


pub struct Page2;


impl<'a> WindowPage<'a> for Page2{
    type Window=DynamicWindow<'a>;
    type Output=();

    fn on_window_close_requested(&mut self,_window:&mut DynamicWindow<'a>){
        println!("Closing");
    }

    #[cfg(not(feature="lazy"))]
    fn on_update_requested(&mut self,_window:&mut DynamicWindow<'a>){}

    fn on_redraw_requested(&mut self,window:&mut DynamicWindow<'a>){
        window.draw(|_,g|{
            g.clear_colour([1.0;4]);
        }).unwrap();
    }

    fn on_mouse_pressed(&mut self,_window:&mut DynamicWindow<'a>,_button:MouseButton){}
    fn on_mouse_released(&mut self,_window:&mut DynamicWindow<'a>,_button:MouseButton){}
    fn on_mouse_moved(&mut self,_window:&mut DynamicWindow<'a>,_:[f32;2]){}
    fn on_mouse_scrolled(&mut self,_window:&mut DynamicWindow<'a>,_:MouseScrollDelta){}

    fn on_keyboard_pressed(&mut self,window:&mut DynamicWindow<'a>,button:KeyboardButton){
        match button{
            // Press A to switch the pages
            KeyboardButton::A=>{
                let page1=window.take_old_page().unwrap();
                window.set_page(page1);
            }
            KeyboardButton::Escape=>{
                // break out of the page
                let _=window.stop_events();
            }
            _=>{}
        }
    }

    fn on_keyboard_released(&mut self,_window:&mut DynamicWindow<'a>,_button:KeyboardButton){}

    fn on_character_recieved(&mut self,_window:&mut DynamicWindow<'a>,_character:char){}

    fn on_window_resized(&mut self,_window:&mut DynamicWindow<'a>,_new_size:[u32;2]){}

    fn on_suspended(&mut self,_window:&mut DynamicWindow<'a>){}
    fn on_resumed(&mut self,_window:&mut DynamicWindow<'a>){}

    fn on_window_moved(&mut self,_window:&mut DynamicWindow<'a>,_:[i32;2]){}

    fn on_window_focused(&mut self,_window:&mut DynamicWindow<'a>,_:bool){}

    fn on_modifiers_changed(&mut self,_window:&mut DynamicWindow<'a>,_modifiers:ModifiersState){}

    #[cfg(feature="file_drop")]
    fn on_file_dropped(&mut self,_:&mut DynamicWindow<'a>,_:PathBuf){}
    #[cfg(feature="file_drop")]
    fn on_file_hovered(&mut self,_:&mut DynamicWindow<'a>,_:PathBuf){}
    #[cfg(feature="file_drop")]
    fn on_file_hovered_canceled(&mut self,_:&mut DynamicWindow<'a>){}

    fn on_event_loop_closed(&mut self,_:&mut DynamicWindow<'a>){}
}

fn main(){
    let image_base=ImageBase::new([1.0;4],[
        100f32,
        100f32,
        400f32,
        400f32
    ]);

    let mut window=DynamicWindow::new(|_,_|{}).unwrap();

    let texture=Texture::from_path("logo_400x400.png",window.display()).unwrap();

    let mut page2=Page2;

    let mut page=Page{
        angle:0f32,
        image_base,
        texture,
        page2:Some(&mut page2),
    };

    window.change_page(&mut page);

    window.run();
}