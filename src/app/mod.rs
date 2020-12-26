use super::{
    InnerWindowEvent,
    WindowSettings,
    WindowBase,
};

use glium::glutin::{
    ContextBuilder,
    NotCurrent,
    monitor::MonitorHandle,
    event_loop::{ControlFlow,EventLoop},
    event::{
        
        Event,
        WindowEvent,
        MouseScrollDelta,
        ModifiersState
    },
    window::WindowBuilder,
};

pub trait App:Sized+'static{
    type StartArgs;

    fn on_create(&mut self)->Self::StartArgs;
    fn on_start(&mut self,args:Self::StartArgs);

    fn on_update(&mut self);

    fn on_suspend(&mut self);
    fn on_resume(&mut self);

    /// Возвращаемое значение: true - закрыть, false - игнорировать.
    /// 
    /// Returned value: true - close, false - ignore.
    fn on_close_requested(&mut self)->bool;
    fn on_destroy(&mut self);

    fn run(mut self){
        let args:Self::StartArgs=self.on_create();
        self.on_start(args);

        let window_settings=WindowSettings::new();

        let mut window_builder=WindowBuilder::default();
        window_builder.window=window_settings.window_attributes;

        let mut context_builder=ContextBuilder::new();
        context_builder.gl_attr.vsync=true;
        context_builder.gl_attr.debug=window_settings.debug;

        context_builder.pf_reqs=window_settings.pixel_fmt_req;

        let mut event_loop=EventLoop::<InnerWindowEvent>::with_user_event();

        let (mut window,mut graphics)=WindowBase::raw(
            window_builder,
            context_builder,
            window_settings.graphics_base_settings,
            event_loop,
            window_settings.general,
        ).expect("WindowCreationError");

        window.event_loop.run(move|event,_,control_flow|{
            #[cfg(not(feature="lazy"))]{
                // Endless cycling checking events
                *control_flow=ControlFlow::Poll;
            }
            
            #[cfg(feature="lazy")]{
                // Waiting for any event except redraw event
                *control_flow=ControlFlow::Wait;
            }
            match event{
                Event::WindowEvent{event,..}=>
                    match event{
                        WindowEvent::CloseRequested=>{
                            if self.on_close_requested(){
                                *control_flow=ControlFlow::Exit
                            }
                        }
                        _=>{}
                    }

                Event::Suspended=>self.on_suspend(),
                Event::Resumed=>self.on_resume(),

                _=>{}
            }
        });

        //self.on_destroy();
    }
}