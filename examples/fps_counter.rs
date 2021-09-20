use std::time::{
    Duration,
    Instant,
};

use cat_engine::{
    app::{
        App,
        AppAttributes,
        Window,
        WindowInner,
        WindowEvent,
        WindowProcedure,
        Event,
        ProcessEvent,
        VirtualKeyCode,
        quit,
    },
    graphics::{
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
    texture:Option<Texture>,
    redraw:DrawData,
}

struct WindowHandle;

impl WindowProcedure<WindowInner<RenderData>> for WindowHandle{
    fn render(window:&Window,window_inner:&mut WindowInner<RenderData>){
        let draw_start=Instant::now();
                let redraw_event_period=draw_start.duration_since(window_inner.storage().redraw.last_redraw);
                window_inner.storage().redraw.last_redraw=draw_start;

                window_inner.storage().redraw.current_fps=
                    (Duration::from_secs(1).as_nanos()/redraw_event_period.as_nanos()) as u32;

                window_inner.draw(window,|_window,graphics,render_data|{
                    graphics.clear_colour([1f32;4]);

                    if let Some(texture)=render_data.texture.as_ref(){
                        graphics.draw_stack_textured_object(0,texture.texture_2d());
                        graphics.draw_stack_textured_object(1,texture.texture_2d());
                        graphics.draw_stack_textured_object(2,texture.texture_2d());
                    }
                }).unwrap_or_else(|_|{quit()});
    }

    fn handle(event:WindowEvent,_window:&Window,_window_inner:&mut WindowInner<RenderData>){
        match event{
            WindowEvent::CloseRequest=>quit(),
            _=>{}
        }
    }
}

fn main(){
    let app_attributes=AppAttributes::new();

    let render_data=RenderData{
        texture:None,
        redraw:DrawData{
            last_redraw:Instant::now(),
            current_fps:0u32,
        }
    };

    let app=App::new::<WindowHandle>(app_attributes,render_data);

    let graphics=app.graphics();

    graphics.core().blending.enable();
    graphics.core().blending.set_function(
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

    app.storage().texture=Some(Texture::from_path("logo_400x400.png").unwrap());

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