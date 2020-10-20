use cat_engine::{
    PagedWindow,
    Window,
    WindowEvent,
    text::{
        FontOwner,
        TextBase,
        GlyphCache,
        CachedFont,
        Scale,
    },
    KeyboardButton,
};

fn main(){
    // Creating a window
    let mut window=PagedWindow::new(|_,s|{
        s.vsync=true;
        // Max size for glyph images
        s.graphics_base_settings.text.glyph_texture_size=[500,500];
    }).unwrap();

    // Loading a font
    let font1=FontOwner::load("resources/font1").unwrap();


    let font=FontOwner::load("resources/font2").unwrap();
    let font2=font.face_wrapper();

    // Creating a glyph cache
    let glyphs=GlyphCache::new_alphabet(font1.face(),"HelloWorld?$",Scale::new(0.1,0.1),window.display());

    let mut angle=0f32;

    // Creating a cached font
    let cached_font=CachedFont::raw(font1,glyphs);

    window.run(|window,event|{
        match event{
            WindowEvent::CloseRequested=>{
                println!("Exit");
            }

            #[cfg(not(feature="lazy"))]
            WindowEvent::Update=>{
                angle+=0.01;
            }

            WindowEvent::RedrawRequested=>{
                window.draw(|p,g|{
                    g.clear_colour([0f32,0f32,0f32,1f32]);

                    let base=TextBase::new([300f32,400f32],Scale::new(0.1,0.1),[1f32;4]);

                    base.draw_str("HelloWorld$ ?",&font2,p,g).unwrap();

                    let base=TextBase::new([350f32,250f32],Scale::new(0.3,0.3),[1f32;4]);
                    base.draw_rotate_str_glyph_cache("Hello Mario",[420f32,380f32],angle,&cached_font,p,g).unwrap();
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