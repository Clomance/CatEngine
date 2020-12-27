#![cfg(all(feature="fps_counter",feature="ups_counter"))]

use cat_engine::{
    fps,
    ups,
    Window,
    WindowEvent,
    text::{
        FontOwner,
        TextBase,
        Scale,
    },
    KeyboardButton,
};

fn main(){
    // Creating a window
    let (mut window,graphics)=Window::new(|_,s|{
        // max ups
        #[cfg(not(feature="lazy"))]{
            s.general.updates_per_second=30;
        }

        s.vsync=true;
        // Max size for glyph images
        s.graphics_base_settings.text.glyph_texture_size=[500,500];
    }).unwrap();

    let font=FontOwner::load("resources/font2").unwrap();
    let font1=font.face_wrapper();

    window.run(|window,event|{
        match event{
            WindowEvent::CloseRequested=>{
                println!("Exit");
            }

            WindowEvent::RedrawRequested=>{
                window.draw(&graphics,|g|{
                    g.clear_colour([0f32,0f32,0f32,1f32]);

                    let fps_str=unsafe{fps.to_string()};
                    let ups_str=unsafe{ups.to_string()};

                    let base=TextBase::new([300f32,400f32],Scale::new(0.1,0.1),[1f32;4]);

                    base.draw_str(&fps_str,&font1,g).unwrap();

                    let base=TextBase::new([650f32,400f32],Scale::new(0.1,0.1),[1f32;4]);

                    base.draw_str(&ups_str,&font1,g).unwrap();
        
                }).unwrap();
            }

            WindowEvent::KeyboardPressed(button)=>match button{
                KeyboardButton::Escape=>{
                    let _=window.stop_events(); // break out of the loop
                }

                _=>{}
            }

            _=>{}
        }
    });
}