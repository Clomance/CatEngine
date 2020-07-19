use cat_engine::{
    DefaultWindow,
    WindowEvent,
    Window,
};

fn main(){
    let mut window=DefaultWindow::new(|_,sets|{
        sets.general.updates_per_second=20;
    }).unwrap();

    let mut rect=cat_engine::graphics::Rectangle::new([100.0;4],[0.0,0.0,0.0,1.0]);

    while let Some(event)=window.next_event(){
        match event{
            WindowEvent::CloseRequested=>{
                break
            }

            #[cfg(not(feature="lazy"))]
            WindowEvent::Update=>{
                if rect.colour[0]>=1f32{
                    if rect.colour[1]>=1f32{
                        if rect.colour[2]>=1f32{
                            rect.colour=[0.0,0.0,0.0,1.0];
                        }
                        else{
                            rect.colour[2]+=0.0625f32;
                        }
                    }
                    else{
                        rect.colour[1]+=0.0625f32;
                    }
                }
                else{
                    rect.colour[0]+=0.0625f32;
                }
            }

            WindowEvent::RedrawRequested=>{
                #[cfg(feature="lazy")]
                if rect.colour[0]>=1f32{
                    if rect.colour[1]>=1f32{
                        if rect.colour[2]>=1f32{
                            rect.colour=[0.0,0.0,0.0,1.0];
                        }
                        else{
                            rect.colour[2]+=0.0625f32;
                        }
                    }
                    else{
                        rect.colour[1]+=0.0625f32;
                    }
                }
                else{
                    rect.colour[0]+=0.0625f32;
                }

                window.draw(|p,g|{
                    g.clear_colour([1.0;4]);
                    rect.draw(p,g).unwrap();
                }).unwrap();
            }
            _=>{}
        }
    }
}