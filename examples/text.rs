use cat_engine::{
    PagedWindow,
    Window,
    WindowEvent,
    text::{TextBase,load_font},
    KeyboardButton,
};

fn main(){
    let mut window=PagedWindow::new(|_,s|{
        s.vsync=true;
    }).unwrap();

    let font=load_font("font").unwrap();

    let mut angle=0f32;

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
                    let base=TextBase::new([100f32;2],50f32,[1f32;4]);
                    g.clear_colour([0f32,0f32,0f32,1f32]);
                    base.draw_str("Hello?",&font,p,g).unwrap();

                    let base=TextBase::new([150f32;2],50f32,[1f32;4]);
                    base.draw_rotate_str("Hello?",[150f32,120f32],angle,&font,p,g).unwrap();
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