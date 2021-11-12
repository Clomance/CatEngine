use cat_engine_basement::{
    graphics::{
        GLCore,
        Colour,
        core::ClearMask,
        core::drawing::PrimitiveType,
    },
    windows::{
        EventLoop,
        LoopControl,
        EventLoopAttributes,
        Window,
        WindowAttributes,
        WindowClass,
        CursorIcon,
        SystemCursor,
        WindowClassAttributes,
        WindowProcedure,
        OpenGraphicsLibrary,
        OpenGLRenderContext,
        OpenGLRenderContextAttributes,
        quit,
        WindowEvent,
        ProcessEvent,
        Event,
        WinError,
        WindowResizeType,
    },
};

struct Handler;

impl WindowProcedure for Handler{
    type CreateParameters=();
    type Data=*mut OpenGLRenderContext;

    fn create(window:&Window,_create_parameters:&mut ())->Result<Self::Data,WinError>{
        // create a render context
        let rca=OpenGLRenderContextAttributes::new();
        match OpenGLRenderContext::new(window,rca){
            Ok(render_context)=>{
                // load opengl functions
                let opengl_library=OpenGraphicsLibrary::new();
                opengl_library.load_functions();

                // save the pointer to the opengl context
                Ok(Box::leak(Box::new(render_context)) as *mut OpenGLRenderContext)
            }
            Err(error)=>Err(error),
        }
    }

    fn close_request(window:&Window,_data:Self::Data){
        window.destroy().unwrap();
    }

    fn destroy(_window:&Window,data:Self::Data){
        // thats the way we drop OpenGLRenderContext
        // or you may use anything else such as `drop_in_place` and `drop`
        unsafe{Box::from_raw(data)};
        quit(0)
    }

    fn paint(_:&Window,data:Self::Data){
        unsafe{
            GLCore.clear(ClearMask::Colour);
            GLCore.drawing.draw_arrays(0,1,PrimitiveType::Points);
            GLCore.drawing.draw_arrays_instanced(0,1,1,PrimitiveType::Points);
        }

        let render_context=unsafe{&*data};
        render_context.swap_buffers().unwrap();
    }

    #[cfg(feature="set_cursor_event")]
    fn set_cursor(_window:&Window,data:Self::Data){}

    fn resized(_client_size:[u16;2],_:WindowResizeType,_:&Window,_:Self::Data){}

    fn moved(_client_position:[i16;2],_:&Window,_:Self::Data){}

    fn handle(event:WindowEvent,_window:&Window,_data:Self::Data){
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

    let mut wca=WindowClassAttributes::new("CatEngineBasementWindowClass");
    wca.cursor_icon=CursorIcon::System(SystemCursor::Arrow);
    let wc=WindowClass::new(wca).unwrap();

    let wa=WindowAttributes::new("CatEngineBesementWindow");
    // Creating a window with empty handler to avoid using a zeroed argument in the window procedure.
    let window=Window::new::<Handler>(&wc,wa,&mut ()).unwrap();

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