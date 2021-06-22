#[derive(Clone,Copy,Debug)]
#[repr(u32)]
pub enum DrawMode{
    Shift           =0b1<<0,
    Rotation        =0b1<<1,
    // Scale           =0b1<<2,
    // Ignores `Shift`, `Rotation` and `Scale`.
    // Transform       =0b1<<3,
    // ColourInversion =0b1<<4,
    // GreyScale       =0b1<<5,
}

pub struct DrawParameters{
    viewport:[i32;4],
    mode:u32,
    shift:[f32;2],
    rotation:[f32;4], // [cos,sin,center.x,center.y]
}

impl DrawParameters{
    pub fn new()->DrawParameters{
        Self{
            viewport:[0i32;4],
            mode:0u32,
            shift:[0f32;2],
            rotation:[0f32;4],
        }
    }

    pub fn switch_raw(&mut self,mode:u32){
        self.mode^=mode
    }

    pub fn enable_raw(&mut self,mode:u32){
        self.mode|=mode
    }

    pub fn disable_raw(&mut self,mode:u32){
        self.mode&=!mode
    }

    pub fn switch(&mut self,mode:DrawMode){
        self.mode^=mode as u32
    }

    pub fn enable(&mut self,mode:DrawMode){
        self.mode|=mode as u32
    }

    pub fn disable(&mut self,mode:DrawMode){
        self.mode&=!(mode as u32)
    }
}

impl DrawParameters{
    pub fn set_viewport(&mut self,viewport:[i32;4]){
        self.viewport=viewport
    }

    pub fn set_shift(&mut self,shift:[f32;2]){
        self.shift=shift
    }

    pub fn add_shift(&mut self,shift:[f32;2]){
        self.shift[0]+=shift[0];
        self.shift[1]+=shift[1];
    }

    pub fn set_rotation(&mut self,[cos,sin,rotation_center_x,rotation_center_y]:[f32;4]){
        self.rotation=[cos,sin,rotation_center_x,rotation_center_y];
    }

    pub fn set_rotation_cos_sin(&mut self,[cos,sin]:[f32;2]){
        self.rotation[0]=cos;
        self.rotation[1]=sin;
    }

    pub fn set_rotation_center(&mut self,[x,y]:[f32;2]){
        self.rotation[2]=x;
        self.rotation[3]=y;
    }
}

impl DrawParameters{
    pub fn is_shift(&self)->bool{
        self.mode&DrawMode::Shift as u32==DrawMode::Shift as u32
    }

    pub fn is_rotation(&self)->bool{
        self.mode&DrawMode::Rotation as u32==DrawMode::Rotation as u32
    }
}

impl DrawParameters{
    pub fn viewport(&self)->[i32;4]{
        self.viewport
    }

    pub fn flag(&self)->u32{
        self.mode
    }

    pub fn shift(&self)->Option<[f32;2]>{
        if self.is_shift(){
            Some(self.shift)
        }
        else{
            None
        }
    }

    pub fn rotation(&self)->Option<[f32;4]>{
        if self.is_rotation(){
            Some(self.rotation)
        }
        else{
            None
        }
    }
}