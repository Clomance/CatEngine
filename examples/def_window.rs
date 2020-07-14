use cat_engine::{
    DefaultWindow,
    WindowEvent,
    Window,
};

fn main(){
    let mut window=DefaultWindow::new(|_,_|{}).unwrap();

    let rect=cat_engine::graphics::Rectangle::new([100.0;4],[1.0,0.0,0.0,1.0]);

    while let Some(event)=window.next_event(){
        match event{
            WindowEvent::Exit=>{
                break
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