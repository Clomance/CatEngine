use cat_engine::{
    PagedWindow,
    Window,
    WindowEvent,
    KeyboardButton,
    glium::draw_parameters::PolygonMode,
    shapes::Rectangle,
};

fn main(){
    let mut window=PagedWindow::new(|_,s|{
        // changing the offset because we won't use common drawing functions
        // and we need space for three objects
        s.graphics_base_settings.simple.vertex_buffer_offset=0;
        // the default size of the vertex buffer is 8 - fits two rectangles,
        // so we're increasing the capability to 12 - for three rectangles
        s.graphics_base_settings.simple.vertex_buffer_size=12;
        // the default is 2, so we're setting 3 - for three rectangles
        s.graphics_base_settings.simple.object_buffer_size=3;
    }).unwrap();

    { // No need for the rectangle after adding it to the array
        let mut rect=Rectangle::new([100f32;4],[0.0,0.0,0.0,1.0]);

        let graphics=window.graphics2d();
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
                window.draw(|p,g|{
                    // filling the window with the white colour
                    g.clear_colour([1.0;4]);
                    
                    // drawing the first rectangle
                    g.draw_simple_object(0,p).unwrap();

                    p.polygon_mode=PolygonMode::Line;
                    // rotating and drawing the second
                    g.draw_rotate_simple_object(1,[150f32;2],angle,p).unwrap();

                    p.polygon_mode=PolygonMode::Fill;
                    // shifting and drawing the third
                    g.draw_shift_simple_object(2,shift,p).unwrap();
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

    // There is no need to do this at the end of the program
    // This is just an example
    window.graphics2d().clear_simple_object_array();
}