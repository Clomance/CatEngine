use cat_engine::{
    app::{
        App,
        AppAttributes,
        WindowEvent,
        AppWindowProcedure,
        OpenGLRenderContext,
        Window,
        WindowResizeType,
        quit,
    },
    graphics::{
        BlendingFunction,
        Graphics,
    },
    text::{
        FontOwner,
        CachedFont,
        Scale,
    },
};

struct WindowHandle;

impl AppWindowProcedure<CachedFont,()> for WindowHandle{
    fn create(
        _window:&Window,
        data:(&mut OpenGLRenderContext,&mut Graphics,())
    )->CachedFont{
        let font_owner=FontOwner::load("resources/font1").unwrap();
        CachedFont::new_alphabet(font_owner,"He",Scale::new(0.1f32,0.1f32),&mut data.1.graphics_2d)
    }

    fn close_request(_window:&Window,_data:(&mut OpenGLRenderContext,&mut Graphics,&mut CachedFont)){
        quit(0)
    }

    fn destroy(_window:&Window,_data:(&mut OpenGLRenderContext,&mut Graphics,&mut CachedFont)){}

    fn paint(
        _window:&Window,
        (_render_context,graphics,font):(&mut OpenGLRenderContext,&mut Graphics,&mut CachedFont)
    ){
        graphics.clear_colour([0f32,0f32,0f32,1f32]);

        let mut position=[120f32,240f32];
        let mut horizontal_advance=0f32;
        for character in "Hello, world!!!".chars(){
            graphics.draw_char(
                character,
                [1f32;4],
                position,
                Some(&mut horizontal_advance),
                Scale::new(0.1f32,0.1f32),
                font,
            );

            position[0]+=horizontal_advance;
        }
    }

    #[cfg(feature="set_cursor_event")]
    fn set_cursor(_window:&Window,_data:(&mut OpenGLRenderContext,&mut Graphics,&mut CachedFont)){}

    fn resized(
        _client_size:[u16;2],
        _:WindowResizeType,
        _:&Window,_:(&mut OpenGLRenderContext,&mut Graphics,&mut CachedFont)
    ){}

    fn moved(
        _client_position:[i16;2],
        _:&Window,_:(&mut OpenGLRenderContext,&mut Graphics,&mut CachedFont)
    ){}

    fn handle(
        _event:WindowEvent,
        _window:&Window,
        _data:(&mut OpenGLRenderContext,&mut Graphics,&mut CachedFont)
    ){}

    #[cfg(feature="wnd_proc_catch_panic")]
    fn catch_panic(
        _window:&Window,
        _data:(*mut OpenGLRenderContext,*mut Graphics,*mut CachedFont),
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

    app.event_loop.run(|event,_app_control|{
        match event{

            _=>{}
        }
    });
}