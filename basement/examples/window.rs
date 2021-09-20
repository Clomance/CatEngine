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

impl WindowProcedure<()> for EmptyHandler{
    fn render(_:&Window,_:&mut ()){

    }
    fn handle(_:WindowEvent,_:&Window,_:&mut ()){}
}

struct Handler;

impl WindowProcedure<()> for Handler{
    fn render(_:&Window,_:&mut ()){}

    fn handle(event:WindowEvent,window:&Window,_args:&mut ()){
        match event{
            WindowEvent::CloseRequest=>window.destroy().unwrap(),

            WindowEvent::Destroy=>quit(),
            _=>{}
        }
    }
}

fn main(){
    let ea=EventLoopAttributes::new();
    let event_loop=EventLoop::new(ea);

    let wca=WindowClassAttributes::new("CatEngineBasementWindowClass");
    let wc=WindowClass::new(wca).unwrap();

    let wa=WindowAttributes::new("CatEngineBesementWindow");
    let mut nothing=();
    // Creating a window with empty handler to avoid using a zeroed argument in the window procedure.
    let window=Window::new::<Handler,()>(&wc,wa,&mut nothing).unwrap();

    unsafe{ // Setting out handler.
        window.set_window_handle::<Handler,()>()
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