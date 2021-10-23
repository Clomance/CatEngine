use cat_engine::{
    app::{
        App,
        AppAttributes,
        Window,
        WindowEvent,
        AppWindowProcedure,
        OpenGLRenderContext,
        WindowResizeType,
        quit,
    },
    graphics::{
        Graphics,
        BlendingFunction,
    },
    texture::{
        ImageBase,
        Texture
    },
};

struct WindowHandle;

impl AppWindowProcedure<Texture,()> for WindowHandle{
    fn create(_window:&Window,_data:(&mut OpenGLRenderContext,&mut Graphics,()))->Texture{
        Texture::from_path("logo_400x400.png").unwrap()
    }

    fn close_request(_window:&Window,_data:(&mut OpenGLRenderContext,&mut Graphics,&mut Texture)){
        quit(0)
    }

    fn destroy(_window:&Window,_data:(&mut OpenGLRenderContext,&mut Graphics,&mut Texture)){}

    fn paint(
        window:&Window,
        (_render_context,graphics,texture):(&mut OpenGLRenderContext,&mut Graphics,&mut Texture)
    ){
        graphics.clear_colour([1f32;4]);

        let [width,height]=window.client_size();

        graphics.graphics_2d.draw_parameters().set_shift([(width/2) as f32-200f32,(height/2) as f32-200f32]);
        graphics.draw_stack_textured_object(0,texture.texture_2d());
    }

    #[cfg(feature="set_cursor_event")]
    fn set_cursor(_window:&Window,_data:(&mut OpenGLRenderContext,&mut Graphics,&mut Texture)){}

    fn resized(
        _client_size:[u16;2],
        _:WindowResizeType,
        _:&Window,
        _:(&mut OpenGLRenderContext,&mut Graphics,&mut Texture)
    ){}

    fn moved(
        _client_position:[i16;2],
        _:&Window,
        _:(&mut OpenGLRenderContext,&mut Graphics,&mut Texture)
    ){}

    fn handle(
        event:WindowEvent,
        _window:&Window,
        _data:(&mut OpenGLRenderContext,&mut Graphics,&mut Texture)
    ){
        match event{

            _=>{}
        }
    }

    #[cfg(feature="wnd_proc_catch_panic")]
    fn catch_panic(
        _window:&Window,
        _data:(*mut OpenGLRenderContext,*mut Graphics,*mut Texture),
        _error:Box<dyn std::any::Any+Send>
    ){}
}


fn main(){
    let app_attributes=AppAttributes::new();

    let app=App::new::<WindowHandle,()>(app_attributes,()).unwrap();

    let graphics=app.graphics();

    // Setting blending
    graphics.parameters.blend.enable();
    graphics.parameters.blend.set_function(
        BlendingFunction::SourceAlpha,
        BlendingFunction::OneMinusSourceAlpha
    );

    // CREATING WITH IMAGEBASE
    let image_base=ImageBase::new(
        [400f32,0f32,400f32,400f32], // position and size
        [0.5,0.5,0.5,1.0] // colour filter
    );
    let _image=graphics.push_textured_object(&image_base).unwrap();

    app.event_loop.run(|event,_app_control|{
        match event{

            _=>{}
        }
    });
}