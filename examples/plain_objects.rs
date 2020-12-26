use cat_engine::{
    Window,
    WindowEvent,
    KeyboardButton,
    glium::draw_parameters::PolygonMode,
    shapes::Rectangle,
};

fn main(){
    let (mut window,mut graphics)=Window::new(|_,s|{
        s.vsync=true;
        // zeroing the offset because we won't use 'active' drawing functions
        // and we need space for it
        s.graphics_base_settings.simple.vertex_buffer_offset=0;
        // the default size of the vertex buffer is 128 - fits 32 rectangles,
        // so we're lessening the capability to 12 - for three rectangles
        s.graphics_base_settings.simple.vertex_buffer_size=12;
        // the default is 16, we're setting 3 - for three rectangles
        s.graphics_base_settings.simple.object_buffer_size=3;
    }).unwrap();

    { // No need for the rectangle after adding it to the array
        let mut rect=Rectangle::new([100f32;4],[0.0,0.0,0.0,1.0]);

        graphics.add_simple_object(&rect).unwrap();

        rect.colour=[1.0,0.0,0.0,1.0];

        graphics.add_simple_object(&rect).unwrap();

        rect.colour=[1.0,0.0,0.0,1.0];

        graphics.add_simple_object(&rect).unwrap();
    }

    let mut angle=0f32;
    let mut shift=[0f32;2];

    window.run(|window,event|{
        match event{
            WindowEvent::CloseRequested=>{
                println!("Exit");
            }

            #[cfg(not(feature="lazy"))]
            WindowEvent::Update=>{
                angle+=0.01;
                shift[0]+=1f32;
            }

            WindowEvent::RedrawRequested=>{
                window.draw(&graphics,|graphics|{
                    // filling the window with the white colour
                    graphics.clear_colour([1.0;4]);
                    
                    // drawing the first rectangle
                    graphics.draw_simple_object(0).unwrap();

                    graphics.draw_parameters.polygon_mode=PolygonMode::Line;
                    // rotating and drawing the second
                    graphics.draw_rotate_simple_object(1,[150f32;2],angle).unwrap();

                    graphics.draw_parameters.polygon_mode=PolygonMode::Fill;
                    // shifting and drawing the third
                    graphics.draw_shift_simple_object(2,shift).unwrap();
                }).unwrap();
            }

            WindowEvent::KeyboardPressed(button)=>match button{
                KeyboardButton::Escape=>{
                    let _=window.stop_events(); // break out of the loop
                }

                _=>{}
            }

            _=>{}
        }
    });
}