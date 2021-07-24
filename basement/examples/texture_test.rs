use cat_engine_basement::{
    graphics::{
        GCore,
        core::ClearMask,
        core::texture::{
            Texture2DInternalFormat,
            Texture2DRewriteTarget,
            TextureBindTarget,
            ImageDataFormat,
        },
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
    },
    event::{WindowEvent,ProcessEvent},
};

const texture_2d_internal_formats:&[Texture2DInternalFormat]=&[
    Texture2DInternalFormat::R8,
    Texture2DInternalFormat::R16,

    Texture2DInternalFormat::RG8,
    Texture2DInternalFormat::RG16,

    Texture2DInternalFormat::RGB8,
    Texture2DInternalFormat::RGB16,

    Texture2DInternalFormat::RGBA8,
    Texture2DInternalFormat::RGBA16,
];

const image_formats:&[ImageDataFormat]=&[
    ImageDataFormat::R_I8,
    ImageDataFormat::R_U8,
    ImageDataFormat::R_I16,
    ImageDataFormat::R_U16,
    ImageDataFormat::R_I32,
    ImageDataFormat::R_U32,
    ImageDataFormat::R_F32,

    ImageDataFormat::RG_I8,
    ImageDataFormat::RG_U8,
    ImageDataFormat::RG_I16,
    ImageDataFormat::RG_U16,
    ImageDataFormat::RG_I32,
    ImageDataFormat::RG_U32,
    ImageDataFormat::RG_F32,

    ImageDataFormat::RGB_I8,
    ImageDataFormat::RGB_U8,
    ImageDataFormat::RGB_I16,
    ImageDataFormat::RGB_U16,
    ImageDataFormat::RGB_I32,
    ImageDataFormat::RGB_U32,
    ImageDataFormat::RGB_F32,

    ImageDataFormat::BGR_I8,
    ImageDataFormat::BGR_U8,
    ImageDataFormat::BGR_I16,
    ImageDataFormat::BGR_U16,
    ImageDataFormat::BGR_I32,
    ImageDataFormat::BGR_U32,
    ImageDataFormat::BGR_F32,

    ImageDataFormat::RGBA_I8,
    ImageDataFormat::RGBA_U8,
    ImageDataFormat::RGBA_I16,
    ImageDataFormat::RGBA_U16,
    ImageDataFormat::RGBA_I32,
    ImageDataFormat::RGBA_U32,
    ImageDataFormat::RGBA_F32,

    ImageDataFormat::BGRA_I8,
    ImageDataFormat::BGRA_U8,
    ImageDataFormat::BGRA_I16,
    ImageDataFormat::BGRA_U16,
    ImageDataFormat::BGRA_I32,
    ImageDataFormat::BGRA_U32,
    ImageDataFormat::BGRA_F32,
];

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
    let mut event_loop=EventLoop::new(ea);

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

    // 2D FORMAT TEST
    unsafe{
        let mut id=0;
        GCore.texture.generate_one(&mut id);
        GCore.texture.bind(TextureBindTarget::Texture2D,id);
        let data=vec![0u8;1024*1024*32];
        for internal in texture_2d_internal_formats{
            for image in image_formats{
                GCore.texture.rewrite_image_2d(
                    Texture2DRewriteTarget::Texture2D,
                    0,
                    *internal,
                    [1024,1024],
                    *image,
                    data.as_ptr()
                );
                if GCore.get_error().is_error(){
                    println!("texture {:?} - image {:?}",internal,image);
                }
            }
        }
        GCore.texture.delete_one(&id);
    }

    let mut updates=0;

    event_loop.run(|event,control|{
        match event{
            ProcessEvent::EventLoopStart=>*control=LoopControl::Run,
            
            ProcessEvent::Update(_)=>{
                updates+=1;
                if updates==800{
                    *control=LoopControl::Break
                }
            },

            _=>{}
        }
    });
}