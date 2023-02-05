use cat_engine_basement::{
    utility::math::matrix::Matrix,
    opengl::buffer::{UniformBuffer,BufferUsage}
};

pub struct Camera{
    pub (crate) viewport:[f32;2],
    pub (crate) viewspace_size:[f32;3],
    pub (crate) viewspace_offset:[f32;3],
    pub (crate) matrix:Matrix,
    pub (crate) uniform_buffer:UniformBuffer<Matrix>,
}

impl Camera{
    pub (crate) fn new(viewport:[f32;2],viewspace:[f32;3])->Camera{
        let mut camera=Self{
            viewport,
            viewspace_size:viewspace,
            viewspace_offset:[0f32;3],
            matrix:Matrix::new(),
            uniform_buffer:UniformBuffer::empty(BufferUsage::DynamicDraw),
        };

        camera.reset();

        camera
    }

    pub (crate) fn reset(&mut self){
        self.matrix.reset();

        let view_scale_x=self.viewport[0]/self.viewspace_size[0];
        let view_scale_y=self.viewport[1]/self.viewspace_size[1];

        let view_scales=[
            2f32/self.viewspace_size[0]/view_scale_x,
            -2f32/self.viewspace_size[1]/view_scale_y,
            2f32/self.viewspace_size[2]
        ];
        self.matrix.scale(view_scales);

        self.matrix.translate([-self.viewspace_size[0]/2f32*view_scale_x,-self.viewspace_size[1]/2f32*view_scale_y,-self.viewspace_size[2]/2f32]);

        self.matrix.translate(self.viewspace_offset)
    }

    pub fn shift(&mut self,[dx,dy,dz]:[f32;3]){
        self.viewspace_offset[0]+=dx;
        self.viewspace_offset[1]+=dy;
        self.viewspace_offset[2]+=dz;

        self.matrix.translate([dx,dy,dz])
    }

    pub fn move_to(&mut self,[x,y,z]:[f32;3]){
        let dx=x-self.viewspace_offset[0];
        let dy=y-self.viewspace_offset[1];
        let dz=z-self.viewspace_offset[2];

        self.viewspace_offset[0]=x;
        self.viewspace_offset[1]=y;
        self.viewspace_offset[2]=z;

        self.matrix.translate([dx,dy,dz])
    }

    pub (crate) fn set_viewport(&mut self,[width,height]:[f32;2]){
        self.viewport=[width,height];
        self.reset();
    }

    pub fn set_view_space(&mut self,[width,height,depth]:[f32;3]){
        self.viewspace_size=[width,height,depth];
        self.reset();
    }
}