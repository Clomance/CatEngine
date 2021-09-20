use cat_engine::{
    app::{
        App,
        AppAttributes,
        WindowEvent,
        WindowProcedure,
        WindowInner,
        Window,
        quit,
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

struct WindowHandle;

impl WindowProcedure<WindowInner<Option<CachedFont>>> for WindowHandle{
    fn render(window:&Window,window_inner:&mut WindowInner<Option<CachedFont>>){
        window_inner.draw(window,|_,graphics,font|{
            graphics.clear_colour([0f32,0f32,0f32,1f32]);

            if let Some(font)=font.as_ref(){
                let mut position=[120f32,240f32];
                let mut horizontal_advance=0f32;
                for character in "Hello, world!!!".chars(){
                    graphics.draw_char(
                        character,
                        [1f32;4],
                        position,
                        Some(&mut horizontal_advance),
                        Scale::new(0.1f32,0.1f32),
                        font,
                    );

                    position[0]+=horizontal_advance;
                }
            }
        }).unwrap_or_else(|_|{quit()});
    }

    fn handle(event:WindowEvent,_window:&Window,_window_inner:&mut WindowInner<Option<CachedFont>>){
        match event{
            WindowEvent::CloseRequest=>quit(),
            _=>{}
        }
    }
}

fn main(){
    let font:Option<CachedFont>=None;

    let app_attributes=AppAttributes::new();
    let app=App::new::<WindowHandle>(app_attributes,font);

    let graphics=app.graphics();

    // Setting blending
    graphics.core().blending.enable();
    graphics.core().blending.set_function(
        BlendingFunction::SourceAlpha,
        BlendingFunction::OneMinusSourceAlpha
    );

    let font_owner=FontOwner::load("resources/font1").unwrap();
    *app.storage()=Some(
        // Cached only two characters, the others will be built dynamically.
        CachedFont::new_alphabet(font_owner,"He",Scale::new(0.1f32,0.1f32),graphics.graphics_2d())
    );

    app.event_loop.run(|event,_app_control|{
        match event{

            _=>{}
        }
    });
}