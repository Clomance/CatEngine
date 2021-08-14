use cat_engine_basement::{
    graphics::{
        GCore,
        core::ClearMask,
        Colour,
    },
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
        quit,
        WindowEvent,
        ProcessEvent,
        Event,
    },
};

/// An empty struct for an empty window procedure.
struct EmptyHandler;

impl WindowProcedure<RenderData> for EmptyHandler{
    fn handle(_:WindowEvent,_:&Window,_:&mut RenderData){}
}

struct Handler;

impl WindowProcedure<RenderData> for Handler{
    fn handle(event:WindowEvent,window:&Window,args:&mut RenderData){
        match event{
            WindowEvent::Redraw=>{
                // use it when you have more than one window
                args.context.make_current(true).unwrap_or_else(|_|{quit()});

                // set viewport if a window may change it's size
                // or if you have more than one window
                // otherwise set it after creating the window
                let [width,height]=window.client_size();

                unsafe{
                    GCore.viewport.set([0,0,width as i32,height as i32]);

                    args.colour[0]+=0.01;
                    if args.colour[0]>=1f32{
                        args.colour[0]=0f32;
                    }

                    GCore.set_clear_colour(args.colour);
                    GCore.clear(ClearMask::Colour);
                }

                args.context.swap_buffers().unwrap_or_else(|_|{quit()});
            }

            WindowEvent::CloseRequest=>window.destroy().unwrap(),

            WindowEvent::Destroy=>quit(),
            _=>{}
        }
    }
}

struct RenderData{
    context:OpenGLRenderContext,
    colour:Colour,
}

fn main(){
    let ea=EventLoopAttributes::new();
    let event_loop=EventLoop::new(ea);

    let wca=WindowClassAttributes::new("CatEngineBasementWindowClass");
    let wc=WindowClass::new(wca).unwrap();

    // We need a reference to an unmovable structure for the window procedure,
    // so do not move `render_data` any where.
    // Allocating a zeroed structure
    // because we can't create a texture without our window's context.
    let zero=std::mem::MaybeUninit::zeroed();
    let mut render_data=unsafe{zero.assume_init()};

    let wa=WindowAttributes::new("CatEngineBesementWindow");
    // Creating a window with empty handler to avoid using a zeroed argument in the window procedure.
    let window=Window::new::<EmptyHandler,RenderData>(&wc,wa,&mut render_data).unwrap();

    let ca=OpenGLRenderContextAttributes::new();
    let context=OpenGLRenderContext::new(&window,ca).unwrap();

    let library=OpenGraphicsLibrary::new();
    library.load_functions(); // only after render context creation


    unsafe{ // not to drop the allocated zero-context and zero-texture (line 75)
        (&mut render_data as *mut RenderData).write(RenderData{context,colour:[0f32;4]})
    }

    unsafe{ // Setting out handler.
        window.set_window_handle::<Handler,RenderData>()
    }

    let mut updates=0;

    event_loop.run(|event,control|{
        match event{
            Event::Process(ProcessEvent::EventLoopStart)=>*control=LoopControl::Run,

            Event::Process(ProcessEvent::Update(_))=>{
                updates+=1;
                if updates==800{
                    *control=LoopControl::Break
                }
            },

            _=>{}
        }
    });
}