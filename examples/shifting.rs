use cat_engine::{
    app::{
        App,
        AppAttributes,
        Event,
        WindowEvent,
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
    let app_attributes=AppAttributes::new();
    let mut app=App::new(app_attributes);

    let graphics=app.get_graphics_unchecked_mut(0);

    { // Setting blending and the clear colour
        graphics.core().set_clear_colour([1f32;4]);
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
        [1.0;4] // colour filter
    );
    let image=graphics.push_textured_object(&image_base).unwrap();

    let mut shift=0f32;
    app.run(|event,app_control|{
        match event{
            Event::Update(_)=>shift+=0.1,

            Event::Redraw=>{
                let window=app_control.get_window_unchecked(0);
                let [width,height]=window.client_size();

                let graphics=app_control.get_graphics_unchecked_mut(0);
                graphics.clear_colour();

                graphics.core().viewport().set([0,0,width as i32,height as i32]);

                graphics.draw_parameters().set_viewport([0f32,0f32,width as f32,height as f32]);
                graphics.draw_parameters().switch(DrawMode::Shift);
                graphics.draw_parameters().set_shift([shift,shift]);
                // using the `update` function to load all the parameters
                graphics.draw_parameters().update();

                graphics.draw_stack_textured_object(image,texture.texture_2d());

                // using the `change` function to load only one parameter
                graphics.draw_parameters().change_switch(DrawMode::Shift);

                app_control.get_render_context_unchecked(0).swap_buffers().unwrap();
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