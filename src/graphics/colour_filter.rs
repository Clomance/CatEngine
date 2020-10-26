use crate::Colour;

/// Функция для фильтрации.
/// A filtering function.
#[derive(Clone,Copy,Debug)]
pub enum FilteringFunction{
    /// Takes the maximum of two colours.
    Max,
    /// Takes the minimum of two colours.
    Min,
    /// Multiplicates two colours.
    Mul,
}

/// Цветовой фильтр.
/// A colour filter.
#[derive(Clone,Copy,Debug)]
pub struct ColourFilter{
    /// A filtering function.
    pub function:FilteringFunction,
    /// A colour that is used for filtering.
    pub colour:Colour,
}

impl ColourFilter{
    pub const fn new(function:FilteringFunction,colour:Colour)->ColourFilter{
        Self{
            function,
            colour,
        }
    }

    pub fn new_mul(colour:Colour)->ColourFilter{
        Self{
            function:FilteringFunction::Mul,
            colour
        }
    }

    /// Фильрует цвет.
    /// 
    /// Filters a colour.
    pub fn filter_colour(&self,colour:&mut Colour){
        match self.function{
            FilteringFunction::Max=>{
                for c in 0..4{
                    if colour[c]<self.colour[c]{
                        colour[c]=self.colour[c]
                    }
                }
            }

            FilteringFunction::Min=>{
                for c in 0..4{
                    if colour[c]>self.colour[c]{
                        colour[c]=self.colour[c]
                    }
                }
            }

            FilteringFunction::Mul=>{
                for c in 0..4{
                    colour[c]*=self.colour[c];
                }
            }
        }
    }
}