use cat_engine_basement::graphics::{
    level0::BufferUsage,
    level1::buffer::UniformBuffer,
};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
#[repr(u32)]
pub enum DrawMode{
    /// Applied firstly.
    Shift               =0b1<<0,
    /// Applied after shift.
    Rotation            =0b1<<1,
    // / Applied after rotation.
    // Scale               =0b1<<2,
    // /// Ignores `Shift`, `Rotation` and `Scale` modes.
    // Transformation      =0b1<<3,
    // /// Applied firstly.
    // ColourInversion     =0b1<<4,
    // /// Applied after colour inversion.
    // GreyScale           =0b1<<5,
}

struct DrawParametersUniform{
    /// Applied in the end.
    viewport:[f32;4],

    mode:[u32;2],
    shift:[f32;2],
    rotation:[f32;4],
    scale:[f32;2],
}

impl DrawParametersUniform{
    pub fn new()->DrawParametersUniform{
        Self{
            viewport:[0f32;4],
            mode:[0u32;2],
            shift:[0f32;2],
            rotation:[0f32;4],
            scale:[1f32;2],
        }
    }
}

pub struct DrawParameters{
    uniform_buffer:UniformBuffer<DrawParametersUniform>,
    uniform:DrawParametersUniform,
}

impl DrawParameters{
    pub fn new()->DrawParameters{
        let uniform=DrawParametersUniform::new();
        let uniform_buffer=UniformBuffer::new(&uniform,BufferUsage::StaticDraw);
        Self{
            uniform_buffer,
            uniform,
        }
    }

    pub fn bind_uniform(&self){
        self.uniform_buffer.bind_base(0);
    }

    pub fn switch_raw(&mut self,mode:u32){
        self.uniform.mode[0]^=mode
    }

    pub fn enable_raw(&mut self,mode:u32){
        self.uniform.mode[0]|=mode
    }

    pub fn disable_raw(&mut self,mode:u32){
        self.uniform.mode[0]&=!mode
    }

    pub fn switch(&mut self,mode:DrawMode){
        self.uniform.mode[0]^=mode as u32
    }

    pub fn enable(&mut self,mode:DrawMode){
        self.uniform.mode[0]|=mode as u32
    }

    pub fn disable(&mut self,mode:DrawMode){
        self.uniform.mode[0]&=!(mode as u32)
    }
}

impl DrawParameters{
    pub fn update(&self){
        self.uniform_buffer.bind().write(&self.uniform)
    }

    pub fn update_viewport(&self){
        let data=&self.uniform.viewport[0] as *const f32 as *const DrawParametersUniform;
        unsafe{
            self.uniform_buffer.bind().raw().write_raw(data,0,16)
        }
    }

    pub fn update_mode(&self){
        let data=&self.uniform.mode[0] as *const u32 as *const DrawParametersUniform;
        unsafe{
            self.uniform_buffer.bind().raw().write_raw(data,16,8)
        }
    }

    pub fn update_shift(&self){
        let data=&self.uniform.shift[0] as *const f32 as *const DrawParametersUniform;
        unsafe{
            self.uniform_buffer.bind().raw().write_raw(data,24,8)
        }
    }

    pub fn update_rotation(&self){
        let data=&self.uniform.rotation[0] as *const f32 as *const DrawParametersUniform;
        unsafe{
            self.uniform_buffer.bind().raw().write_raw(data,32,16)
        }
    }

    pub fn update_rotation_cos_sin(&self){
        let data=&self.uniform.rotation[0] as *const f32 as *const DrawParametersUniform;
        unsafe{
            self.uniform_buffer.bind().raw().write_raw(data,32,8)
        }
    }

    pub fn update_rotation_center(&self){
        let data=&self.uniform.rotation[2] as *const f32 as *const DrawParametersUniform;
        unsafe{
            self.uniform_buffer.bind().raw().write_raw(data,40,8)
        }
    }
}

impl DrawParameters{
    pub fn change_enable(&mut self,mode:DrawMode){
        self.uniform.mode[0]|=mode as u32;
        self.update_mode()
    }

    pub fn change_disable(&mut self,mode:DrawMode){
        self.uniform.mode[0]&=!(mode as u32);
        self.update_mode()
    }

    pub fn change_switch(&mut self,mode:DrawMode){
        self.uniform.mode[0]^=mode as u32;
        self.update_mode()
    }

    pub fn change_viewport(&mut self,viewport:[f32;4]){
        self.uniform.viewport=viewport;
        self.update_viewport()
    }

    pub fn change_shift(&mut self,shift:[f32;2]){
        self.uniform.shift=shift;
        self.update_shift()
    }

    pub fn change_add_shift(&mut self,shift:[f32;2]){
        self.uniform.shift[0]+=shift[0];
        self.uniform.shift[1]+=shift[1];
        self.update_shift()
    }

    pub fn change_rotation(&mut self,[cos,sin,rotation_center_x,rotation_center_y]:[f32;4]){
        self.uniform.rotation=[cos,sin,rotation_center_x,rotation_center_y];
        self.update_rotation()
    }

    pub fn change_rotation_cos_sin(&mut self,[cos,sin]:[f32;2]){
        self.uniform.rotation[0]=cos;
        self.uniform.rotation[1]=sin;
        self.update_rotation()
    }

    pub fn change_rotation_center(&mut self,[x,y]:[f32;2]){
        self.uniform.rotation[2]=x;
        self.uniform.rotation[3]=y;
        self.update_rotation()
    }
}

impl DrawParameters{
    pub fn set_viewport(&mut self,viewport:[f32;4]){
        self.uniform.viewport=viewport;
    }

    pub fn set_shift(&mut self,shift:[f32;2]){
        self.uniform.shift=shift
    }

    pub fn add_shift(&mut self,shift:[f32;2]){
        self.uniform.shift[0]+=shift[0];
        self.uniform.shift[1]+=shift[1];
    }

    pub fn set_rotation(&mut self,[cos,sin,rotation_center_x,rotation_center_y]:[f32;4]){
        self.uniform.rotation=[cos,sin,rotation_center_x,rotation_center_y];
    }

    pub fn set_rotation_cos_sin(&mut self,[cos,sin]:[f32;2]){
        self.uniform.rotation[0]=cos;
        self.uniform.rotation[1]=sin;
    }

    pub fn set_rotation_center(&mut self,[x,y]:[f32;2]){
        self.uniform.rotation[2]=x;
        self.uniform.rotation[3]=y;
    }
}

impl DrawParameters{
    pub fn is_shift(&self)->bool{
        self.uniform.mode[0]&DrawMode::Shift as u32==DrawMode::Shift as u32
    }

    pub fn is_rotation(&self)->bool{
        self.uniform.mode[0]&DrawMode::Rotation as u32==DrawMode::Rotation as u32
    }
}

impl DrawParameters{
    pub fn viewport(&self)->[f32;4]{
        self.uniform.viewport
    }

    pub fn flag(&self)->u32{
        self.uniform.mode[0]
    }

    pub fn shift(&self)->Option<[f32;2]>{
        if self.is_shift(){
            Some(self.uniform.shift)
        }
        else{
            None
        }
    }

    pub fn rotation(&self)->Option<[f32;4]>{
        if self.is_rotation(){
            Some(self.uniform.rotation)
        }
        else{
            None
        }
    }
}