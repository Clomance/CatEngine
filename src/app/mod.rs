use super::{
    WindowBase,
    Window,
    InnerWindowEvent,
    WindowSettings,
};

use glium::glutin::{
    ContextBuilder,
    NotCurrent,
    monitor::MonitorHandle,
    event_loop::EventLoop,
    event::{WindowEvent,MouseScrollDelta,ModifiersState},
    window::WindowBuilder,
    platform::desktop::EventLoopExtDesktop,
};

pub trait App:Sized+Window{
    type StartArgs;

    fn on_create(&mut self)->Self::StartArgs;
    fn on_start(&mut self,args:Self::StartArgs);

    fn on_suspend(&mut self);
    fn on_resume(&mut self);

    fn on_destroy(&mut self);

    fn run(mut self){
        let args:Self::StartArgs=self.on_create();
        self.on_start(args);

        let window_settings=WindowSettings::new();

        let mut window_builder=WindowBuilder::default();
        window_builder.window=window_settings.window_attributes;

        let mut context_builder=ContextBuilder::new();
        context_builder.gl_attr.vsync=window_settings.vsync;
        context_builder.gl_attr.debug=window_settings.debug;

        context_builder.pf_reqs=window_settings.pixel_fmt_req;

        let mut event_loop=EventLoop::<InnerWindowEvent>::with_user_event();

        let mut window=WindowBase::raw(
            window_builder,
            context_builder,
            window_settings.graphics_base_settings,
            event_loop,
            window_settings.general,
        ).expect("WindowCreationError");

        window.event_loop.run_return(|event,_,control_flow|{

        });

        self.on_destroy();
    }
}