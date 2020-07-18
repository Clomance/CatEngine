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

    let mut colour=[0.0,0.0,0.0,1.0];

    while let Some(event)=window.next_event(){
        match event{
            WindowEvent::Exit=>{
                break
            }

            WindowEvent::Update=>{
                if colour[0]>=1f32{
                    if colour[1]>=1f32{
                        if colour[2]>=1f32{
                            colour=[0.0,0.0,0.0,1.0];
                        }
                        else{
                            colour[2]+=0.0625f32;
                        }
                    }
                    else{
                        colour[1]+=0.0625f32;
                    }
                }
                else{
                    colour[0]+=0.0625f32;
                }

                rect.colour=colour;
            }

            WindowEvent::Draw=>{
                window.draw(|p,g|{
                    g.clear_colour([1.0;4]);
                    rect.draw(p,g).unwrap();
                }).unwrap();
            }
            _=>{}
        }
    }
}