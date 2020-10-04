use cat_engine::{
    PagedWindow,
    Window,
    WindowEvent,
    text::{
        Font,
        TextBase,
        GlyphCache,
        Scale,
    },
    KeyboardButton,
};

fn main(){
    let mut window=PagedWindow::new(|_,s|{
        s.vsync=true;
    }).unwrap();

    let font1=Font::load("resources/font1").unwrap();

    let font2=Font::load("resources/font2").unwrap();

    let scale=Scale::new(0.21,0.21);
    let mut glyphs=GlyphCache::new_alphabet(font1.face(),"Hello",scale,window.display());

    glyphs.insert_str(font2.face(),"Word?",scale,window.display());

    let mut angle=0f32;

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
                    let base=TextBase::new([300f32,400f32],100f32,[1f32;4]);

                    base.draw_str_glyph_cache("HelloWord?",&glyphs,p,g).unwrap();
                    //base.draw_str_glyph_cache("      ?",&glyphs,p,g).unwrap();

                    let base=TextBase::new([350f32,250f32],50f32,[1f32;4]);
                    base.draw_rotate_str_glyph_cache("Hello?",[420f32,380f32],angle,&glyphs,p,g).unwrap();
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