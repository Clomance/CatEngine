use cat_engine::{
    app::{
        App,
        AppAttributes,
        Window,
        Event,
        WindowEvent,
        WindowProcedure,
        WindowInner,
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

impl WindowProcedure<WindowInner<Option<(Texture,f32)>>> for WindowHandle{
    fn handle(window:&Window,window_inner:&mut WindowInner<Option<(Texture,f32)>>,event:WindowEvent){
        match event{
            WindowEvent::Redraw=>{
                window_inner.draw(window,|window,graphics,texture|{
                    graphics.clear_colour();

                    // read here (line 83)
                    if let Some((texture,angle))=texture.as_ref(){
                        let [width,height]=window.client_size();

                        graphics.draw_parameters().switch(DrawMode::Rotation);
                        graphics.draw_parameters().set_rotation(
                            [angle.cos(),angle.sin(),width as f32/2f32,height as f32/2f32]
                        );
                        // using the `update` function to load all the parameters
                        graphics.draw_parameters().update();

                        graphics.draw_stack_textured_object(0,texture.texture_2d());

                        graphics.draw_parameters().change_switch(DrawMode::Rotation);
                    }
                }).unwrap_or_else(|_|{quit()});
                window.redraw();
            }

            WindowEvent::CloseRequest=>quit(),
            _=>{}
        }
    }
}

fn main(){
    let texture:Option<(Texture,f32)>=None;

    let app_attributes=AppAttributes::new();
    let mut app=App::new::<WindowHandle>(app_attributes,texture);

    let graphics=app.window_graphics_mut();

    { // Setting blending and the clear colour
        graphics.core().set_clear_colour([1f32;4]);
        let blending=graphics.core().blending();
        blending.enable();
        blending.set_function(
            BlendingFunction::SourceAlpha,
            BlendingFunction::OneMinusSourceAlpha
        );
    }

    let image_base=ImageBase::new(
        [400f32,400f32,400f32,400f32], // position and size
        [1.0;4] // colour filter
    );
    let _image=graphics.push_textured_object(&image_base).unwrap();

    *app.app_storage_mut()=Some((Texture::from_path("logo_400x400.png").unwrap(),0f32));

    app.run(|event,app_control|{
        match event{
            // Written here (line 31)
            Event::Update(_)=>if let Some((_,angle))=app_control.app_storage_mut(){
                *angle+=0.01
            }
            _=>{}
        }
    });
}