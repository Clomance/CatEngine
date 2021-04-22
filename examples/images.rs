use cat_engine::{
    App,
    AppAttributes,
    Event,
    WindowEvent,
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

fn main(){
    let app_attributes=AppAttributes::new();
    let mut app=App::new(app_attributes);

    let graphics=app.get_graphics_unchecked_mut(0);

    { // Setting blending
        let blending=graphics.parameters().blending();
        blending.enable();
        blending.set_blending_function(
            BlendingFunction::SourceAlpha,
            BlendingFunction::OneMinusSourceAlpha
        );
    }

    let texture=Texture::from_path("logo_400x400.png").unwrap();

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
    let image1=graphics.push_textured_object_raw(
        &vertices, // vertices
        &[0,1,3,1,2,3], // indicies associated with the given vertices
        PrimitiveType::Triangles // drawing type
    ).unwrap();

    // CREATING WITH IMAGEBASE
    let image_base=ImageBase::new(
        [400f32,0f32,400f32,400f32], // position and size
        [0.5,0.5,0.5,1.0] // colour filter
    );
    let image2=graphics.push_textured_object(&image_base).unwrap();

    // CREATING WITH IMAGEOBJECT
    let image_base=ImageObject::new(
        [800f32,0f32,200f32,400f32], // position and size
        [0f32,0f32,0.5f32,1f32], // texture position and size
        [1.0;4] // colour filter
    );
    let image3=graphics.push_textured_object(&image_base).unwrap();

    let mut colour=[1f32,1f32,1f32,1f32];
    app.run(|event,app_control,_control|{
        match event{
            Event::WindowEvent{window_event,argument,..}=>match window_event{
                WindowEvent::Redraw=>{
                    let graphics=app_control.get_graphics_unchecked_mut(argument as usize);
                    if colour[0]<1f32{
                        colour[0]+=0.01;
                    }
                    else{
                        colour[0]=0f32;
                    };
                    graphics.parameters().set_clear_colour(colour);
                    graphics.clear_colour();
                    // Drawing the object that is located in the stack-type buffer.
                    graphics.draw_stack_textured_object(image1,texture.texture_2d());
                    graphics.draw_stack_textured_object(image2,texture.texture_2d());
                    graphics.draw_stack_textured_object(image3,texture.texture_2d());
                }
                _=>{}
            }

            _=>{}
        }
    });
}