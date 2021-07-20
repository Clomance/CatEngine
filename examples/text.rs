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
    fn handle(event:WindowEvent,window:&Window,window_inner:&mut WindowInner<Option<CachedFont>>){
        match event{
            WindowEvent::Redraw=>{
                window_inner.draw(window,|_,graphics,font|{
                    graphics.clear_colour([0f32,0f32,0f32,1f32]);

                    // read here (line 83)
                    if let Some(font)=font.as_ref(){
                        graphics.draw_char('a',[1f32;4],[100f32;2],&mut 0f32,Scale::new(1f32,1f32),font);
                        graphics.draw_char('Р',[1f32;4],[150f32,250f32],&mut 0f32,Scale::new(0.1f32,0.1f32),font);
                        graphics.draw_char('B',[1f32;4],[350f32,250f32],&mut 0f32,Scale::new(1f32,1f32),font);

                    }
                }).unwrap_or_else(|_|{quit()});
                window.redraw();
            }

            WindowEvent::CloseRequest=>quit(),
            _=>{}
        }
    }
}

fn main(){
    let font:Option<CachedFont>=None;

    let app_attributes=AppAttributes::new();
    let mut app=App::new::<WindowHandle>(app_attributes,font);

    let graphics=app.window_graphics_mut();

    // Setting blending
    graphics.core().blending.enable();
    graphics.core().blending.set_function(
        BlendingFunction::SourceAlpha,
        BlendingFunction::OneMinusSourceAlpha
    );

    let font_owner=FontOwner::load("resources/font1").unwrap();
    *app.app_storage_mut()=Some(
        CachedFont::new_alphabet(font_owner,"aAbBcCwW",Scale::new(0.1f32,0.1f32),graphics.graphics_2d())
    );

    app.run(|event,_app_control|{
        match event{

            _=>{}
        }
    });
}