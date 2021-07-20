use cat_engine::{
    app::{
        App,
        AppAttributes,
        Window,
        WindowInner,
        WindowEvent,
        WindowProcedure,
        VirtualKeyCode,
        quit,
    },
    graphics::{
        BlendingFunction,
        PrimitiveType,
        TexturedVertex2D,
        ShapeObject,
    },
    texture::{
        ImageBase,
        ImageObject,
        Texture
    },
};

struct WindowHandle;

impl WindowProcedure<WindowInner<Option<Texture>>> for WindowHandle{
    fn handle(event:WindowEvent,window:&Window,window_inner:&mut WindowInner<Option<Texture>>){
        match event{
            WindowEvent::Redraw=>{
                window_inner.context().make_current(true).unwrap_or_else(|_|{quit()});
                let [width,height]=window.client_size();
                unsafe{
                    window_inner.graphics().core().viewport.set([0,0,width as i32,height as i32]);
                }
                window_inner.graphics().draw_parameters().set_viewport([0f32,0f32,width as f32,height as f32]);

                if let Some(texture)=window_inner.storage().as_ref(){
                    window_inner.graphics_ref().clear_colour([1f32;4]);
                    window_inner.graphics_ref().draw_stack_textured_object(0,texture.texture_2d());
                    window_inner.graphics_ref().draw_stack_textured_object(1,texture.texture_2d());
                    window_inner.graphics_ref().draw_stack_textured_object(2,texture.texture_2d());

                    window_inner.graphics_ref().core().finish();
                    window_inner.context().swap_buffers().unwrap_or_else(|_|{quit()});
                }
                window.redraw();
            }

            WindowEvent::KeyPress(VirtualKeyCode::A)=>{
                let image_base=ImageBase::new(
                    [400f32,100f32,100f32,100f32], // position and size
                    [0.5,0.5,0.5,1.0] // colour filter
                );
                window_inner.graphics().write_stack_textured_object_vertices(1,&image_base.vertices());
            }

            WindowEvent::CloseRequest=>quit(),
            _=>{}
        }
    }
}

fn main(){
    let app_attributes=AppAttributes::new();

    let texture:Option<Texture>=None;
    let mut app=App::new::<WindowHandle>(app_attributes,texture);

    let graphics=app.window_graphics_mut();

    // Setting blending
    graphics.core().blending.enable();
    graphics.core().blending.set_function(
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

    // Pushing to the stack-type buffer.
    let _image1=graphics.push_textured_object_raw(
        &vertices, // vertices
        &[0,1,3,1,2,3], // indicies associated with the given vertices
        PrimitiveType::Triangles // drawing type
    ).unwrap();

    // CREATING WITH IMAGEBASE
    let image_base=ImageBase::new(
        [400f32,0f32,400f32,400f32], // position and size
        [0.5,0.5,0.5,1.0] // colour filter
    );
    let _image2=graphics.push_textured_object(&image_base).unwrap();

    // CREATING WITH IMAGEOBJECT
    let image_base=ImageObject::new(
        [800f32,0f32,200f32,400f32], // position and size
        [0f32,0f32,0.5f32,1f32], // texture position and size
        [1.0;4] // colour filter
    );
    let _image3=graphics.push_textured_object(&image_base).unwrap();

    *app.app_storage_mut()=Some(Texture::from_path("logo_400x400.png").unwrap());

    app.run(|event,_app_control|{
        match event{
            

            _=>{}
        }
    });
}