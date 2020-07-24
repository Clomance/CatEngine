mod simple;
use simple::*;

/// Not implemented yet
pub struct Graphics3D{
    simple:SimpleGraphics3D,
}

impl Graphics3D{
    pub fn new()->Graphics3D{
        Self{
            simple:SimpleGraphics3D{},
        }
    }
}