use cat_engine_basement::{
    windows::{
        EventLoop,
        LoopControl,
        EventLoopAttributes,
        Window,
        WindowAttributes,
        WindowClass,
        WindowClassAttributes,
        WindowProcedure,
        quit,
        WindowEvent,
        WindowResizeType,
        ProcessEvent,
        Event,
        WinError,
        StandardSystemColour,
        Background,
    },
};

use std::mem::transmute;

struct Handler;

impl WindowProcedure for Handler{
    type CreateParameters=();
    type Data=();

    fn create(_:&Window,_create_paramters:&mut ())->Result<Self::Data,WinError>{
        Ok(())
    }

    fn close_request(window:&Window,_parameters:Self::Data){
        window.destroy().unwrap();
    }

    fn destroy(_window:&Window,_parameters:Self::Data){
        quit(0)
    }

    fn paint(_:&Window,_:()){}

    #[cfg(feature="set_cursor_event")]
    fn set_cursor(window:&Window,_parameters:Self::Data){
        unsafe{
            // Getting and setting the class cursor
            let raw_cursor=WinCore.window.get_class_long_ptr(window.handle(),ClassData::CursorHandle);
            let cursor:CursorHandle=transmute(raw_cursor);
            WinCore.cursor.set(Some(cursor));
        }
    }

    fn resized(_client_size:[u16;2],_:WindowResizeType,_:&Window,_:Self::Data){}

    fn moved(_client_position:[i16;2],_:&Window,_:Self::Data){}

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

    let mut wca=WindowClassAttributes::new("CatEngineBasementWindowClass");
    wca.background=Background::SystemColour(WindowBackgroundSystemColour::Window);
    let wc=WindowClass::new(wca).unwrap();

    let wa=WindowAttributes::new("CatEngineBesementWindow");
    let mut nothing=();
    // Creating a window with empty handler to avoid using a zeroed argument in the window procedure.
    let window=Window::new::<Handler>(&wc,wa,&mut nothing).unwrap();

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