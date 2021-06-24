use cat_engine::{
    app::{
        App,
        AppAttributes,
        Event,
        WindowEvent,
    },
    graphics::{
        BlendingFunction,
    },
    text::{
        FontOwner,
        CachedFont,
        Scale,
    },
};

fn main(){
    let app_attributes=AppAttributes::new();
    let mut app=App::new(app_attributes);

    let graphics=app.get_graphics_unchecked_mut(0);

    { // Setting blending
        let blending=graphics.core().blending();
        blending.enable();
        blending.set_function(
            BlendingFunction::SourceAlpha,
            BlendingFunction::OneMinusSourceAlpha
        );
    }

    let font_owner=FontOwner::load("resources/font1").unwrap();
    let font=CachedFont::new_alphabet(font_owner,"aAbBcCwW",Scale::new(0.1f32,0.1f32),graphics.graphics_2d());

    graphics.core().set_clear_colour([0f32,0f32,0f32,1f32]);
    app.run(|event,app_control|{
        match event{
            Event::Redraw=>{
                let window=app_control.get_window_unchecked(0);

                let window_size=window.client_size();

                let graphics=app_control.get_graphics_unchecked_mut(0);

                graphics.core().viewport().set([0,0,window_size[0] as i32,window_size[1] as i32]);
                graphics.draw_parameters().set_viewport([0f32,0f32,window_size[0] as f32,window_size[1] as f32]);
                graphics.draw_parameters().update();
                graphics.clear_colour();

                graphics.draw_char('a',[1f32;4],[100f32;2],&mut 0f32,Scale::new(1f32,1f32),&font);
                graphics.draw_char('Р',[1f32;4],[150f32,250f32],&mut 0f32,Scale::new(0.1f32,0.1f32),&font);
                graphics.draw_char('B',[1f32;4],[350f32,250f32],&mut 0f32,Scale::new(1f32,1f32),&font);

                app_control.get_render_context_unchecked(0).swap_buffers().unwrap();
            }

            Event::WindowEvent{window_event,window_id:_}=>match window_event{
                WindowEvent::CloseRequest=>{
                    app_control.exit();
                }
                _=>{}
            }

            _=>{}
        }
    });
}