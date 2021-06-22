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
        [400f32,400f32,400f32,400f32], // position and size
        [1.0;4] // colour filter
    );
    let image=graphics.push_textured_object(&image_base).unwrap();

    let mut angle=0f32;
    app.run(|event,app_control|{
        match event{
            Event::Update(_)=>angle+=0.01,

            Event::Redraw=>{
                let window=app_control.get_window_unchecked(0);

                let window_size=window.client_size();
                let window_center=[
                    window_size[0] as f32/2f32,
                    window_size[1] as f32/2f32
                ];

                let graphics=app_control.get_graphics_unchecked_mut(0);
                graphics.clear_colour();

                graphics.draw_parameters().switch(DrawMode::Rotation);

                graphics.draw_parameters().set_viewport([0,0,window_size[0] as i32,window_size[1] as i32]);

                graphics.draw_parameters().set_rotation([angle.cos(),angle.sin(),window_center[0],window_center[1]]);
                graphics.draw_stack_textured_object(image,texture.texture_2d());
                graphics.draw_parameters().switch(DrawMode::Rotation);

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