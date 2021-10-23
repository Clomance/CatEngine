use cat_engine::{
    texture::{
        Texture,
        ImageBase,
    },
    graphics::{
        Graphics,
        Graphics2DAttributes,
    },
    basement::{
        windows::{
            EventLoop,
            LoopControl,
            EventLoopAttributes,
            Window,
            WindowAttributes,
            WindowClass,
            WindowClassAttributes,
            WindowProcedure,
            OpenGraphicsLibrary,
            OpenGLRenderContext,
            OpenGLRenderContextAttributes,
            Event,
            WindowEvent,
            ProcessEvent,
            WinError,
            WindowResizeType,
            quit,
        },
    }
};

struct WindowGraphics{
    context:OpenGLRenderContext,
    graphics:Graphics,
    texture:Texture,
}

struct Handler;

impl WindowProcedure for Handler{
    type CreateParameters=(OpenGLRenderContextAttributes,Graphics2DAttributes);
    type Data=*mut WindowGraphics;

    fn create(window:&Window,create_parameters:&mut Self::CreateParameters)->Result<Self::Data,WinError>{
        let render_context=OpenGLRenderContext::new(window,create_parameters.0.clone()).unwrap();

        let library=OpenGraphicsLibrary::new();
        library.load_functions(); // only after render context creation

        let mut graphics=Graphics::new(create_parameters.1.clone());

        // Now we can create a texture.
        let texture=Texture::from_path("logo_400x400.png").unwrap();

        let image_base=ImageBase::new(
            [0f32,0f32,400f32,400f32], // position and size
            [1.0;4] // colour filter
        );

        graphics.push_textured_object(&image_base);

        let window_data=Box::leak(Box::new(WindowGraphics{
            context:render_context,
            graphics,
            texture
        }));

        Ok(window_data)
    }

    fn close_request(window:&Window,_data:Self::Data){
        window.destroy().unwrap();
    }

    fn destroy(_window:&Window,data:Self::Data){
        unsafe{Box::from_raw(data)};
        quit(0)
    }

    fn paint(window:&Window,data:Self::Data){
        let data=unsafe{&mut*data};
        // use it when you have more than one window
        data.context.make_current(true).unwrap_or_else(|_|{quit(0)});

        // set viewport if a window may change it's size
        // or if you have more than one window
        // otherwise set it after creating the window
        let [width,height]=window.client_size();
        unsafe{
            data.graphics.core().parameters.viewport.set([0,0,width as i32,height as i32]);
        }
        data.graphics.graphics_2d.draw_parameters().set_viewport([0f32,0f32,width as f32,height as f32]);

        data.graphics.clear_colour([1f32;4]);
        data.graphics.draw_stack_textured_object(0,data.texture.texture_2d());

        unsafe{
            data.graphics.core().finish()
        }
        data.context.swap_buffers().unwrap_or_else(|_|{quit(0)});
    }

    #[cfg(feature="set_cursor_event")]
    fn set_cursor(_window:&Window,_parameters:Self::Data){}

    fn resized(
        _client_size:[u16;2],
        _:WindowResizeType,
        _:&Window,
        _:Self::Data
    ){}

    fn moved(
        _client_position:[i16;2],
        _:&Window,
        _:Self::Data
    ){}

    fn handle(event:WindowEvent,_window:&Window,_args:Self::Data){
        match event{

            _=>{}
        }
    }

    #[cfg(feature="wnd_proc_catch_panic")]
    fn catch_panic(_window:&Window,_data:Self::Data,error:Box<dyn std::any::Any+Send>){
        println!("{:?}",error);
        quit(0)
    }
}

fn main(){
    let ea=EventLoopAttributes::new();
    let event_loop=EventLoop::new(ea);

    let wca=WindowClassAttributes::new("CatEngineWindowClass");
    let wc=WindowClass::new(wca).unwrap();

    let mut wga=(OpenGLRenderContextAttributes::new(),Graphics2DAttributes::new());

    let wa=WindowAttributes::new("CatEngineWindow");
    // Creating a window with empty handler to avoid using a zeroed argument in the window procedure.
    let _window=Window::new::<Handler>(&wc,wa,&mut wga).unwrap();

    let mut updates=0;

    event_loop.run(|event,control|{
        match event{
            Event::Process(ProcessEvent::EventLoopStart)=>*control=LoopControl::Run,

            Event::Process(ProcessEvent::Update(_))=>{
                updates+=1;
                if updates==400{
                    *control=LoopControl::Break
                }
            },

            _=>{}
        }
    });
}