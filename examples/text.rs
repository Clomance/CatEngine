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

    let font=load_font("resources/font").unwrap();
    let font2=load_font("resources/font2").unwrap();

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
                    let base=TextBase::new([300f32,400f32],150f32,[1f32;4]);
                    g.clear_colour([0f32,0f32,0f32,1f32]);
                    base.draw_str("Hello?",&font2,p,g).unwrap();

                    let base=TextBase::new([350f32,250f32],50f32,[1f32;4]);
                    base.draw_rotate_str("Hello?",[420f32,380f32],angle,&font,p,g).unwrap();
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