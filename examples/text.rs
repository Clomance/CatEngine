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
    let mut window=PagedWindow::new(|_,s|{
        s.vsync=true;
        s.graphics_base_settings.text.glyph_texture_size=[2000,2000];
    }).unwrap();

    let font1=FontOwner::load("resources/font1").unwrap();

    let font2=FontOwner::load("resources/font2").unwrap();
    let face=font2.face_wrapper();

    let scale=Scale::new(0.1,0.1);
    let mut glyphs=GlyphCache::new_alphabet(font1.face(),"Hello$",scale,window.display());

    glyphs.insert_str(font1.face(),"Word?",scale,window.display());

    let mut angle=0f32;

    let cached_font=CachedFont::raw(font1,glyphs);

    let rect=cat_engine::shapes::Rectangle::new([150f32,300f32,150f32,100f32],[1f32;4]);

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
                    rect.draw(p,g).unwrap();

                    let base=TextBase::new([100f32,100f32],scale,[1f32;4]);
                    base.draw_str("abAFvAW$Ф",&face,p,g).unwrap();

                    let base=TextBase::new([300f32,400f32],scale,[1f32;4]);

                    
                    base.draw_str_glyph_cache("HelloWord$?",&cached_font,p,g).unwrap();
                    //base.draw_str_glyph_cache("      ?",&glyphs,p,g).unwrap();

                    let base=TextBase::new([350f32,250f32],scale,[1f32;4]);
                    base.draw_rotate_str_glyph_cache("Hello?",[420f32,380f32],angle,&cached_font,p,g).unwrap();
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