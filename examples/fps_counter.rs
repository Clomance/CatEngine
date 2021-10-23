use std::time::{
    Duration,
    Instant,
};

use cat_engine::{
    app::{
        App,
        AppAttributes,
        Window,
        WindowEvent,
        AppWindowProcedure,
        Event,
        ProcessEvent,
        VirtualKeyCode,
        Monitor,
        OpenGLRenderContext,
        Fullscreen,
        WindowResizeType,
        quit,
    },
    graphics::{
        Graphics,
        BlendingFunction,
        PrimitiveType,
        TexturedVertex2D,
    },
    texture::{
        ImageBase,
        ImageObject,
        Texture
    },
};

struct DrawData{
    last_redraw:Instant,
    current_fps:u32,
}

struct RenderData{
    texture:Texture,
    redraw:DrawData,
}

struct WindowHandle;

impl AppWindowProcedure<RenderData,()> for WindowHandle{
    fn create(_window:&Window,_data:(&mut OpenGLRenderContext,&mut Graphics,()))->RenderData{
        RenderData{
            texture:Texture::from_path("logo_400x400.png").unwrap(),
            redraw:DrawData{
                last_redraw:Instant::now(),
                current_fps:0u32,
            }
        }
    }

    fn close_request(_window:&Window,_data:(&mut OpenGLRenderContext,&mut Graphics,&mut RenderData)){
        quit(0)
    }

    fn destroy(_window:&Window,_data:(&mut OpenGLRenderContext,&mut Graphics,&mut RenderData)){}

    fn paint(
        _window:&Window,
        (_render_context,graphics,data):(&mut OpenGLRenderContext,&mut Graphics,&mut RenderData)
    ){
        let draw_start=Instant::now();
        let redraw_event_period=draw_start.duration_since(data.redraw.last_redraw);
        data.redraw.last_redraw=draw_start;

        data.redraw.current_fps=
            (Duration::from_secs(1).as_nanos()/redraw_event_period.as_nanos()) as u32;

        graphics.clear_colour([1f32;4]);

        graphics.draw_stack_textured_object(0,data.texture.texture_2d());
        graphics.draw_stack_textured_object(1,data.texture.texture_2d());
        graphics.draw_stack_textured_object(2,data.texture.texture_2d());
    }

    #[cfg(feature="set_cursor_event")]
    fn set_cursor(_window:&Window,_data:(&mut OpenGLRenderContext,&mut Graphics,&mut RenderData)){}

    fn resized(
        _client_size:[u16;2],
        _:WindowResizeType,
        _:&Window,
        _:(&mut OpenGLRenderContext,&mut Graphics,&mut RenderData)
    ){}

    fn moved(
        _client_position:[i16;2],
        _:&Window,
        _:(&mut OpenGLRenderContext,&mut Graphics,&mut RenderData)
    ){}

    fn handle(
        event:WindowEvent,
        window:&Window,
        _data:(&mut OpenGLRenderContext,&mut Graphics,&mut RenderData)
    ){
        match event{
            WindowEvent::KeyPress(VirtualKeyCode::A)=>{
                window.set_fullscreen(Fullscreen::Monitor(Monitor::get_primary_monitor()))
            }

            WindowEvent::KeyPress(VirtualKeyCode::S)=>{
                window.set_fullscreen(Fullscreen::None)
            }
            _=>{}
        }
    }

    #[cfg(feature="wnd_proc_catch_panic")]
    fn catch_panic(
        _window:&Window,
        _data:(*mut OpenGLRenderContext,*mut Graphics,*mut RenderData),
        _error:Box<dyn std::any::Any+Send>
    ){}
}

fn main(){
    let app_attributes=AppAttributes::new();
    let app=App::new::<WindowHandle,()>(app_attributes,()).unwrap();

    let graphics=app.graphics();

    graphics.parameters.blend.enable();
    graphics.parameters.blend.set_function(
        BlendingFunction::SourceAlpha,
        BlendingFunction::OneMinusSourceAlpha
    );

    let vertices=[
        TexturedVertex2D::new(
            [400f32,0f32],
            [1f32,1f32],
            [1.0,0.5,0.5,0.0]
        ),
        TexturedVertex2D::new([400f32,400f32],[1f32,0f32],[0.5,0.5,0.5,0.6]),
        TexturedVertex2D::new([0f32,400f32],[0f32,0f32],[0.5,0.5,0.5,1.0]),
        TexturedVertex2D::new([0f32,0f32],[0f32,1f32],[0.5,0.5,0.5,1.0]),
    ];

    let _image1=graphics.push_textured_object_raw(
        &vertices,
        &[0,1,3,1,2,3],
        PrimitiveType::Triangles
    ).unwrap();

    let image_base=ImageBase::new(
        [400f32,0f32,400f32,400f32],
        [0.5,0.5,0.5,1.0]
    );
    let _image2=graphics.push_textured_object(&image_base).unwrap();

    // CREATING WITH IMAGEOBJECT
    let image_base=ImageObject::new(
        [800f32,0f32,200f32,400f32], // position and size
        [0f32,0f32,0.5f32,1f32], // texture position and size
        [1.0;4] // colour filter
    );
    let _image3=graphics.push_textured_object(&image_base).unwrap();

    let mut updates=0;
    app.event_loop.run(|event,_app_control|{
        match event{
            Event::Process(ProcessEvent::Update(_))=>{
                updates+=1;
                if updates==50{
                    println!("current fps - {}",app.storage().redraw.current_fps);
                    updates=0;
                }
            }

            Event::Window(WindowEvent::KeyPress(VirtualKeyCode::W))=>{

            }

            Event::Window(WindowEvent::KeyPress(VirtualKeyCode::S))=>{

            }

            _=>{}
        }
    });
}