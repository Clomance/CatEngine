use cat_engine::{
    app::{
        App,
        AppAttributes,
        Window,
        Event,
        ProcessEvent,
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
    fn render(window:&Window,window_inner:&mut WindowInner<Option<(Texture,f32)>>){
        window_inner.draw(window,|_,graphics,texture|{
            graphics.clear_colour([1f32;4]);

            // read here (line 83)
            if let Some((texture,shift))=texture.as_ref(){
                graphics.draw_parameters().switch(DrawMode::Shift);
                graphics.draw_parameters().set_shift([*shift;2]);

                graphics.draw_stack_textured_object(0,texture.texture_2d());

                graphics.draw_parameters().switch(DrawMode::Shift);
            }
        }).unwrap_or_else(|_|{quit()});
    }

    fn handle(event:WindowEvent,_window:&Window,_window_inner:&mut WindowInner<Option<(Texture,f32)>>){
        match event{
            WindowEvent::CloseRequest=>quit(),
            _=>{}
        }
    }
}

fn main(){
    let texture:Option<(Texture,f32)>=None;

    let app_attributes=AppAttributes::new();
    let app=App::new::<WindowHandle>(app_attributes,texture);

    let graphics=app.graphics();

    // Setting blending
    graphics.core().blending.enable();
    graphics.core().blending.set_function(
        BlendingFunction::SourceAlpha,
        BlendingFunction::OneMinusSourceAlpha
    );

    let image_base=ImageBase::new(
        [0f32,0f32,400f32,400f32], // position and size
        [1.0;4] // colour filter
    );
    let _image=graphics.push_textured_object(&image_base).unwrap();

    *app.storage()=Some((Texture::from_path("logo_400x400.png").unwrap(),0f32));

    app.event_loop.run(|event,_control|{
        match event{
            Event::Process(ProcessEvent::Update(_))=>
                if let Some((_,shift))=app.storage(){
                    *shift+=1f32
                }

            _=>{}
        }
    });
}