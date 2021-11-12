use cat_engine_basement::{
    linux::{
        Display,
        Window,
        EventLoop,
        Event,
        EventType,
        LoopControl,
        RenderContext,
        x11::glx::{
            glXChooseVisual,
            GLX_RGBA,
            GLX_DEPTH_SIZE,
            GLX_DOUBLEBUFFER,
        },
    },
    graphics::GLCore,
};

extern "C"{
    pub fn glClear(mask:u32);
}

fn main(){
    let display=Display::open(None).unwrap();

    let window=Window::new(&display);

    let mut visual_attributes=[GLX_RGBA,GLX_DEPTH_SIZE,0,GLX_DOUBLEBUFFER,0];
    let mut visual=unsafe{glXChooseVisual(display.handle().as_raw(),0,&mut visual_attributes[0] as *mut _)};
    let context=RenderContext::create(&display,visual,true);
    context.make_current(&display,&window);

    let event_loop=EventLoop::new();

    event_loop.run(&display,|event,control|{
        println!("{:?}",event);
        match event{
            Event::EventLoopStart=>*control=LoopControl::Lazy,

            Event::WindowEvent(EventType::Expose)=>unsafe{
                GLCore.parameters.viewport.set([0,0,200,200]);
                GLCore.parameters.set_clear_colour([1f32,1f32,1f32,1f32]);
                glClear(0x00004000);
                window.swap_buffers(&display);
            }

            Event::WindowEvent(EventType::KeyPress)=>*control=LoopControl::Break,
            _=>{}
        }
    });
}