use cat_engine::{
    app::{
        App,
        AppAttributes,
        Event,
        WindowEvent,
        Fullscreen,
        Monitor,
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

fn main(){
    let mut app_attributes=AppAttributes::new();
    app_attributes.window.size=Some([640i32,720i32]);
    app_attributes.window.fullscreen=Fullscreen::Monitor(Monitor::get_primary_monitor());
    let mut app=App::new(app_attributes);

    let graphics=app.get_graphics_unchecked_mut(0);
    graphics.core().set_clear_colour([1f32;4]);

    graphics.draw_parameters().change_enable(DrawMode::Shift);

    { // Setting blending
        let blending=graphics.core().blending();
        blending.enable();
        blending.set_function(
            BlendingFunction::SourceAlpha,
            BlendingFunction::OneMinusSourceAlpha
        );
    }

    let texture=Texture::from_path("logo_400x400.png").unwrap();

    let image_base=ImageBase::new(
        [0f32,0f32,400f32,400f32], // position and size
        [1f32;4] // colour filter
    );
    let image=graphics.push_textured_object(&image_base).unwrap();

    app.run(|event,app_control|{
        match event{
            Event::Redraw=>{
                app_control.draw(0,|window,graphics|{
                    graphics.clear_colour();

                    let [width,height]=window.client_size();

                    graphics.draw_parameters().change_shift([(width/2-200) as f32,(height/2-200) as f32]);
                    graphics.draw_stack_textured_object(image,texture.texture_2d());

                }).unwrap();
            }

            Event::WindowEvent{window_event,window_id:_}=>match window_event{
                WindowEvent::CloseRequest=>{
                    app_control.exit();
                }
                _=>{}
            }

            _=>{}
        }
    });
}