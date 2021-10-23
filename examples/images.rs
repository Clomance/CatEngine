use cat_engine::{
    app::{
        App,
        AppAttributes,
        Window,
        WindowEvent,
        AppWindowProcedure,
        OpenGLRenderContext,
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

struct WindowHandle;

impl AppWindowProcedure<Texture,()> for WindowHandle{
    fn create(_window:&Window,_data:(&mut OpenGLRenderContext,&mut Graphics,()))->Texture{
        Texture::from_path("logo_400x400.png").unwrap()
    }

    fn close_request(_window:&Window,_data:(&mut OpenGLRenderContext,&mut Graphics,&mut Texture)){
        quit(0)
    }

    fn destroy(_window:&Window,_data:(&mut OpenGLRenderContext,&mut Graphics,&mut Texture)){}

    fn paint(
        _window:&Window,
        (_render_context,graphics,texture):(&mut OpenGLRenderContext,&mut Graphics,&mut Texture)
    ){
        graphics.clear_colour([1f32;4]);

        graphics.draw_stack_textured_object(0,texture.texture_2d());
        graphics.draw_stack_textured_object(1,texture.texture_2d());
        graphics.draw_heap_textured_object(0,texture.texture_2d());
    }

    #[cfg(feature="set_cursor_event")]
    fn set_cursor(_window:&Window,_data:(&mut OpenGLRenderContext,&mut Graphics,&mut Texture)){}

    fn resized(
        _client_size:[u16;2],
        _:WindowResizeType,
        _:&Window,
        _:(&mut OpenGLRenderContext,&mut Graphics,&mut Texture)
    ){}

    fn moved(
        _client_position:[i16;2],
        _:&Window,
        _:(&mut OpenGLRenderContext,&mut Graphics,&mut Texture)
    ){}

    fn handle(
        _event:WindowEvent,
        _window:&Window,
        _data:(&mut OpenGLRenderContext,&mut Graphics,&mut Texture)
    ){}

    #[cfg(feature="wnd_proc_catch_panic")]
    fn catch_panic(
        _window:&Window,
        _data:(*mut OpenGLRenderContext,*mut Graphics,*mut Texture),
        _error:Box<dyn std::any::Any+Send>
    ){}
}

fn main(){
    let app_attributes=AppAttributes::new();

    let app=App::new::<WindowHandle,()>(app_attributes,()).unwrap();

    let graphics=app.graphics();

    // Setting blending
    graphics.parameters.blend.enable();
    graphics.parameters.blend.set_function(
        BlendingFunction::SourceAlpha,
        BlendingFunction::OneMinusSourceAlpha
    );

    // CREATING FROM PARTS
    let vertices=[
        TexturedVertex2D::new(
            [400f32,0f32], // position
            [1f32,1f32], // texture coordinates
            [1.0,0.5,0.5,0.0] // colour filter
        ),
        TexturedVertex2D::new([400f32,400f32],[1f32,0f32],[0.5,0.5,0.5,0.6]),
        TexturedVertex2D::new([0f32,400f32],[0f32,0f32],[0.5,0.5,0.5,1.0]),
        TexturedVertex2D::new([0f32,0f32],[0f32,1f32],[0.5,0.5,0.5,1.0]),
    ];

    // Adding to heap-type buffer
    // Note that the heap-type buffer supports only 3-vertex and 3-index frames.
    // It means that `PrimitiveType` that uses previous vertices may work wrong.
    let _image1=graphics.add_textured_object_raw(
        &vertices, // vertices
        &[0,1,3,1,2,3], // indicies associated with the given vertices
        PrimitiveType::Triangles // drawing type
    ).unwrap();

    // CREATING WITH IMAGEBASE
    let image_base=ImageBase::new(
        [400f32,0f32,400f32,400f32], // position and size
        [0.5,0.5,0.5,1.0] // colour filter
    );
    // Pushing to the stack-type buffer
    let _image2=graphics.push_textured_object(&image_base).unwrap();

    // CREATING WITH IMAGEOBJECT
    let image_base=ImageObject::new(
        [800f32,0f32,400f32,400f32], // position and size
        [0f32,0f32,1f32,1f32], // texture position and size
        [1.0;4] // colour filter
    );
    let _image3=graphics.push_textured_object(&image_base).unwrap();

    app.event_loop.run(|event,_app_control|{
        match event{

            _=>{}
        }
    });
}