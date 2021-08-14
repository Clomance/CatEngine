use cat_engine::{
    app::{
        App,
        AppAttributes,
        Window,
        WindowInner,
        WindowEvent,
        Fullscreen,
        Monitor,
        WindowProcedure,
        VirtualKeyCode,
        quit,
    },
    graphics::{
        BlendingFunction,
        DrawMode,
    },
    texture::{
        ImageBase,
        Texture
    },
};

struct WindowHandle;

impl WindowProcedure<WindowInner<Option<Texture>>> for WindowHandle{
    fn handle(event:WindowEvent,window:&Window,window_inner:&mut WindowInner<Option<Texture>>){
        match event{
            WindowEvent::Redraw=>{
                window_inner.draw(window,|window,graphics,texture|{
                    graphics.clear_colour([1f32;4]);

                    if let Some(texture)=texture.as_ref(){
                        let [width,height]=window.client_size();

                        graphics.draw_parameters().set_shift([(width/2) as f32-200f32,(height/2) as f32-200f32]);
                        graphics.draw_stack_textured_object(0,texture.texture_2d());
                    }
                }).unwrap_or_else(|_|{quit()});
            }

            WindowEvent::KeyPress(VirtualKeyCode::A)=>{
                window.set_fullscreen(Fullscreen::Monitor(Monitor::get_primary_monitor()))
            }

            WindowEvent::KeyPress(VirtualKeyCode::S)=>{
                window.set_fullscreen(Fullscreen::None)
            }

            WindowEvent::CloseRequest=>quit(),
            _=>{}
        }
    }
}

fn main(){
    let mut app_attributes=AppAttributes::new();
    app_attributes.window.size=Some([640i32,720i32]);
    app_attributes.window.fullscreen=Fullscreen::Monitor(Monitor::get_primary_monitor());

    let texture:Option<Texture>=None;
    let app=App::new::<WindowHandle>(app_attributes,texture);

    let graphics=app.graphics();

    graphics.draw_parameters().enable(DrawMode::Shift);

    // Setting blending
    graphics.core().blending.enable();
    graphics.core().blending.set_function(
        BlendingFunction::SourceAlpha,
        BlendingFunction::OneMinusSourceAlpha
    );

    let image_base=ImageBase::new(
        [0f32,0f32,400f32,400f32], // position and size
        [1f32;4] // colour filter
    );
    let _image=graphics.push_textured_object(&image_base).unwrap();

    *app.storage()=Some(Texture::from_path("logo_400x400.png").unwrap());

    app.event_loop.run(|event,_control|{
        match event{
            

            _=>{}
        }
    });
}