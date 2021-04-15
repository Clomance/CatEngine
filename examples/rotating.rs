use cat_engine::{
    App,
    AppAttributes,
    Event,
    WindowEvent,
    window_center,
    graphics::{
        Graphics,
        Graphics2DAttributes,
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

    let attributes=Graphics2DAttributes::new();
    let mut graphics=Graphics::new(&app,attributes);

    { // Setting blending and the clear colour
        graphics.parameters().set_clear_colour([1f32;4]);
        let blending=graphics.parameters().blending();
        blending.enable();
        blending.set_blending_function(
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
    app.run(|event,_app_control,_window,_control|{
        match event{
            Event::Update(_)=>angle+=0.01,

            Event::WindowEvent(WindowEvent::Redraw)=>{
                graphics.clear_colour();

                graphics.draw_parameters().switch(DrawMode::Rotation);
                graphics.draw_parameters().set_rotation(unsafe{[angle.cos(),angle.sin(),window_center[0],window_center[1]]});
                graphics.draw_stack_textured_object(image,texture.texture_2d());
                graphics.draw_parameters().switch(DrawMode::Rotation);
            }

            _=>{}
        }
    });
}