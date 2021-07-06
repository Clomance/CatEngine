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
        OpenGraphicsLibrary,
        OpenGLRenderContext,
        OpenGLRenderContextAttributes,
        quit,
    },
    event::{WindowEvent,Event},
};

use cat_engine::{
    texture::{
        Texture,
        ImageBase,
    },
    graphics::{
        Graphics,
        Graphics2DAttributes,
    },
};

/// An empty struct for an empty window procedure.
struct Handler0;

impl WindowProcedure<WindowGraphics> for Handler0{
    fn handle(_:&Window,_:&mut WindowGraphics,_:WindowEvent){}
}

struct Handler1;

impl WindowProcedure<WindowGraphics> for Handler1{
    fn handle(window:&Window,args:&mut WindowGraphics,event:WindowEvent){
        match event{
            WindowEvent::Redraw=>{
                args.context.make_current(true).unwrap_or_else(|_|{quit()});
                let [width,height]=window.client_size();
                args.graphics.core().viewport().set([0,0,width as i32,height as i32]);
                args.graphics.draw_parameters().change_viewport([0f32,0f32,width as f32,height as f32]);
                args.graphics.clear_colour();
                args.graphics.draw_stack_textured_object(0,args.texture.texture_2d());

                unsafe{cat_engine_basement::graphics::gl::Finish()};
                args.context.swap_buffers().unwrap_or_else(|_|{quit()});
            }

            WindowEvent::CloseRequest=>window.destroy(),

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
    let mut event_loop=EventLoop::new(ea);

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
    let context=OpenGLRenderContext::new(window.get_context(),ca).unwrap();

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

    unsafe{ // not to drop the allocated zero-context and zero-texture (line 75)
        (&mut wg as *mut WindowGraphics).write(WindowGraphics{context,graphics,texture})
    }

    unsafe{ // Setting out handler.
        window.set_window_handle::<Handler1,WindowGraphics>()
    }

    let mut updates=0;

    event_loop.run(|event,control|{
        match event{
            Event::EventLoopStart=>*control=LoopControl::Run,
            
            Event::Update(_)=>{
                updates+=1;
                if updates==400{
                    *control=LoopControl::Break
                }
            },

            _=>{}
        }
    });
}