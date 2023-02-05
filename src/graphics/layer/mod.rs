pub trait Layer{
    fn draw(&mut self);
}

pub enum LayerType{
    Simple,
    Textured,
    Text,
}

pub struct LayerInfo{
    pub layer_type:LayerType,
    pub index:usize,
}

pub struct Layers{
    pub layer_info:Vec<LayerInfo>,
    pub layers:Vec<&'static mut dyn Layer>
}

impl Layers{
    pub fn new()->Layers{
        Self{
            layer_info:Vec::new(),
            layers:Vec::new(),
        }
    }

    pub fn len(&self)->usize{
        self.layers.len()
    }

    pub fn push(&mut self,layer:&'static mut dyn Layer,info:LayerInfo){
        self.layer_info.push(info);
        self.layers.push(layer);
    }

    pub fn insert(&mut self,location:usize,layer:&mut dyn Layer,info:LayerInfo){
        self.layer_info.insert(location,info);
        self.layers.insert(location,unsafe{std::mem::transmute(layer)});
    }

    pub fn remove(&mut self,layer:usize)->LayerInfo{
        self.layers.remove(layer);
        self.layer_info.remove(layer)
    }

    pub fn pop(&mut self)->LayerInfo{
        self.layers.pop();
        self.layer_info.pop().unwrap()
    }

    pub fn layers(&mut self)->&mut Vec<&'static mut dyn Layer>{
        &mut self.layers
    }
}