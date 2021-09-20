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
            quit,
        },
    }
};

/// An empty struct for an empty window procedure.
struct Handler0;

impl WindowProcedure<WindowGraphics> for Handler0{
    fn render(_:&Window,_:&mut WindowGraphics){}
    fn handle(_:WindowEvent,_:&Window,_:&mut WindowGraphics){}
}

struct Handler1;

impl WindowProcedure<WindowGraphics> for Handler1{
    fn render(window:&Window,args:&mut WindowGraphics){
        // use it when you have more than one window
        args.context.make_current(true).unwrap_or_else(|_|{quit()});

        // set viewport if a window may change it's size
        // or if you have more than one window
        // otherwise set it after creating the window
        let [width,height]=window.client_size();
        unsafe{
            args.graphics.core().viewport.set([0,0,width as i32,height as i32]);
        }
        args.graphics.draw_parameters().set_viewport([0f32,0f32,width as f32,height as f32]);

        args.graphics.clear_colour([1f32;4]);
        args.graphics.draw_stack_textured_object(0,args.texture.texture_2d());

        args.graphics.core().finish();
        args.context.swap_buffers().unwrap_or_else(|_|{quit()});
    }

    fn handle(event:WindowEvent,window:&Window,_args:&mut WindowGraphics){
        match event{
            WindowEvent::CloseRequest=>window.destroy().unwrap(),

            WindowEvent::Destroy=>quit(),
            _=>{}
        }
    }
}

struct WindowGraphics{
    context:OpenGLRenderContext,
    graphics:Graphics,
    texture:Texture,
}

fn main(){
    let ea=EventLoopAttributes::new();
    let event_loop=EventLoop::new(ea);

    let wca=WindowClassAttributes::new("CatEngineWindowClass");
    let wc=WindowClass::new(wca).unwrap();

    // We need a reference to an unmovable structure for the window procedure,
    // so do not move `wg` any where.
    // Allocating a zeroed structure
    // because we can't create a texture without our window's context.
    let zero=std::mem::MaybeUninit::zeroed();
    let mut wg=unsafe{zero.assume_init()};

    let wa=WindowAttributes::new("CatEngineWindow");
    // Creating a window with empty handler to avoid using a zeroed argument in the window procedure.
    let window=Window::new::<Handler0,WindowGraphics>(&wc,wa,&mut wg).unwrap();

    let ca=OpenGLRenderContextAttributes::new();
    let context=OpenGLRenderContext::new(&window,ca).unwrap();

    let library=OpenGraphicsLibrary::new();
    library.load_functions(); // only after render context creation

    let ga=Graphics2DAttributes::new();
    let mut graphics=Graphics::new(ga);

    // Now we can create a texture.
    let texture=Texture::from_path("logo_400x400.png").unwrap();

    let image_base=ImageBase::new(
        [0f32,0f32,400f32,400f32], // position and size
        [1.0;4] // colour filter
    );

    graphics.push_textured_object(&image_base);

    unsafe{ // not to drop the zero-context and zero-texture (line 89)
        (&mut wg as *mut WindowGraphics).write(WindowGraphics{context,graphics,texture})
    }

    unsafe{ // Setting out handler.
        window.set_window_handle::<Handler1,WindowGraphics>()
    }

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