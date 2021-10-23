use cat_engine::{
    app::{
        App,
        AppAttributes,
        Window,
        Event,
        ProcessEvent,
        WindowEvent,
        AppWindowProcedure,
        OpenGLRenderContext,
        WindowResizeType,
        quit,
    },
    graphics::{
        Graphics,
        BlendingFunction,
        DrawMode,
    },
    texture::{
        ImageBase,
        Texture
    },
};

struct WindowHandle;

impl AppWindowProcedure<(Texture,f32),()> for WindowHandle{
    fn create(_window:&Window,data:(&mut OpenGLRenderContext,&mut Graphics,()))->(Texture,f32){
        data.1.parameters.blend.enable();
        data.1.parameters.blend.set_function(
            BlendingFunction::SourceAlpha,
            BlendingFunction::OneMinusSourceAlpha
        );

        let image_base=ImageBase::new(
            [400f32,400f32,400f32,400f32], // position and size
            [1.0;4] // colour filter
        );
        let _image=data.1.push_textured_object(&image_base).unwrap();

        (Texture::from_path("logo_400x400.png").unwrap(),0f32)
    }

    fn close_request(_window:&Window,_data:(&mut OpenGLRenderContext,&mut Graphics,&mut (Texture,f32))){
        quit(0)
    }

    fn destroy(_window:&Window,_data:(&mut OpenGLRenderContext,&mut Graphics,&mut (Texture,f32))){}

    fn paint(
        window:&Window,
        (_render_context,graphics,(texture,angle)):(&mut OpenGLRenderContext,&mut Graphics,&mut (Texture,f32))
    ){
        graphics.clear_colour([1f32;4]);

        let [width,height]=window.client_size();

        graphics.graphics_2d.draw_parameters().switch(DrawMode::Rotation);
        graphics.graphics_2d.draw_parameters().set_rotation(
            [angle.cos(),angle.sin(),width as f32/2f32,height as f32/2f32]
        );

        graphics.draw_stack_textured_object(0,texture.texture_2d());

        graphics.graphics_2d.draw_parameters().switch(DrawMode::Rotation);
    }

    #[cfg(feature="set_cursor_event")]
    fn set_cursor(_window:&Window,_data:(&mut OpenGLRenderContext,&mut Graphics,&mut (Texture,f32))){}

    fn resized(
        _client_size:[u16;2],
        _:WindowResizeType,
        _:&Window,
        _:(&mut OpenGLRenderContext,&mut Graphics,&mut (Texture,f32))
    ){}

    fn moved(
        _client_position:[i16;2],
        _:&Window,
        _:(&mut OpenGLRenderContext,&mut Graphics,&mut (Texture,f32))
    ){}

    fn handle(
        _event:WindowEvent,
        _window:&Window,
        _data:(&mut OpenGLRenderContext,&mut Graphics,&mut (Texture,f32))
    ){}

    #[cfg(feature="wnd_proc_catch_panic")]
    fn catch_panic(
        _window:&Window,
        _data:(*mut OpenGLRenderContext,*mut Graphics,*mut (Texture,f32)),
        _error:Box<dyn std::any::Any+Send>
    ){}
}

fn main(){
    let app_attributes=AppAttributes::new();
    let app=App::new::<WindowHandle,()>(app_attributes,()).unwrap();

    app.event_loop.run(|event,_control|{
        match event{
            // Written here (line 31)
            Event::Process(ProcessEvent::Update(_))=>{
                let (_,angle)=app.storage();
                *angle+=0.01
            }
            _=>{}
        }
    });
}