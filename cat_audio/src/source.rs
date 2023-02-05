use crate::sample::SampleTransform;

use cat_engine_basement::utility::storage::StaticStorage;

pub enum SourceData{
    F32(Vec<f32>),
    I16(Vec<i16>),
    U16(Vec<u16>)
}

impl SourceData{
    pub fn len(&self)->usize{
        match self{
            SourceData::F32(data)=>{
                data.len()
            }

            SourceData::I16(data)=>{
                data.len()
            }

            SourceData::U16(data)=>{
                data.len()
            }
        }
    }

    pub unsafe fn get_unchecked(&self,index:usize)->SourceSample{
        match self{
            SourceData::F32(data)=>{
                SourceSample::F32(data.get_unchecked(index).clone())
            }

            SourceData::I16(data)=>{
                SourceSample::I16(data.get_unchecked(index).clone())
            }

            SourceData::U16(data)=>{
                SourceSample::U16(data.get_unchecked(index).clone())
            }
        }
    }
}

#[derive(Clone,Copy)]
pub enum SourceSample{
    F32(f32),
    I16(i16),
    U16(u16)
}

impl SampleTransform for SourceSample{
    fn into_i16(self)->i16{
        match self{
            SourceSample::F32(sample)=>{
                sample.into_i16()
            }

            SourceSample::I16(sample)=>{
                sample.into_i16()
            }

            SourceSample::U16(sample)=>{
                sample.into_i16()
            }
        }
    }

    fn into_u16(self)->u16{
        match self{
            SourceSample::F32(sample)=>{
                sample.into_u16()
            }

            SourceSample::I16(sample)=>{
                sample.into_u16()
            }

            SourceSample::U16(sample)=>{
                sample.into_u16()
            }
        }
    }

    fn into_f32(self)->f32{
        match self{
            SourceSample::F32(sample)=>{
                sample.into_f32()
            }

            SourceSample::I16(sample)=>{
                sample.into_f32()
            }

            SourceSample::U16(sample)=>{
                sample.into_f32()
            }
        }
    }

    fn to_u16(self,volume:f32)->u16{
        match self{
            SourceSample::F32(sample)=>{
                sample.to_u16(volume)
            }

            SourceSample::I16(sample)=>{
                sample.to_u16(volume)
            }

            SourceSample::U16(sample)=>{
                sample.to_u16(volume)
            }
        }
    }

    fn to_i16(self,volume:f32)->i16{
        match self{
            SourceSample::F32(sample)=>{
                sample.to_i16(volume)
            }

            SourceSample::I16(sample)=>{
                sample.to_i16(volume)
            }

            SourceSample::U16(sample)=>{
                sample.to_i16(volume)
            }
        }
    }

    fn to_f32(self,volume:f32)->f32{
        match self{
            SourceSample::F32(sample)=>{
                sample.to_f32(volume)
            }

            SourceSample::I16(sample)=>{
                sample.to_f32(volume)
            }

            SourceSample::U16(sample)=>{
                sample.to_f32(volume)
            }
        }
    }

    fn from<S:SampleTransform>(sample:S,volume:f32)->Self{
        SourceSample::F32(sample.to_f32(volume))
    }

    fn lerp(first:Self,second:Self,numerator:u32,denominator:u32)->Self{
        match first{
            SourceSample::F32(sample)=>{
                SourceSample::F32(f32::lerp(sample,second.into_f32(),numerator,denominator))
            }

            SourceSample::I16(sample)=>{
                SourceSample::I16(i16::lerp(sample,second.into_i16(),numerator,denominator))
            }

            SourceSample::U16(sample)=>{
                SourceSample::U16(u16::lerp(sample,second.into_u16(),numerator,denominator))
            }
        }
    }

    fn amplify(self,value:f32)->Self{
        match self{
            SourceSample::F32(sample)=>{
                SourceSample::F32(sample.amplify(value))
            }

            SourceSample::I16(sample)=>{
                SourceSample::I16(sample.amplify(value))
            }

            SourceSample::U16(sample)=>{
                SourceSample::U16(sample.amplify(value))
            }
        }
    }

    fn saturating_add(self,other:Self)->Self{
        match self{
            SourceSample::F32(sample)=>{
                SourceSample::F32(sample.saturating_add(other.into_f32()))
            }

            SourceSample::I16(sample)=>{
                SourceSample::I16(sample.saturating_add(other.into_i16()))
            }

            SourceSample::U16(sample)=>{
                SourceSample::U16(sample.saturating_add(other.into_u16()))
            }
        }
    }

    fn zero_value()->Self{
        SourceSample::F32(0f32)
    }
}

pub struct Source{
    channels:u8,
    sample_rate:u32,
    len:usize,
    data:SourceData,
}

impl Source{
    pub fn new(data:SourceData,channels:u8,rate:u32)->Source{
        Self{
            channels,
            sample_rate:rate,
            len:data.len(),
            data
        }
    }

    pub fn channels(&self)->u8{
        self.channels
    }

    pub fn sample_rate(&self)->u32{
        self.sample_rate
    }

    pub fn len(&self)->usize{
        self.len
    }

    pub fn data(&self)->&SourceData{
        &self.data
    }
}



pub struct SourceUnit{
    pub source:Source,
    pub reference_count:usize
}

impl SourceUnit{
    pub fn new(source:Source)->SourceUnit{
        Self{
            source,
            reference_count:0
        }
    }

    pub fn inc(&mut self){
        self.reference_count+=1
    }

    pub fn dec(&mut self){
        self.reference_count-=1
    }
}



pub struct SourceUnitReference{
    reference:&'static mut SourceUnit
}

impl SourceUnitReference{
    pub (crate) fn new(source:&mut SourceUnit)->SourceUnitReference{
        Self{
            reference:unsafe{std::mem::transmute(source)}
        }
    }

    pub fn source(&self)->&Source{
        &self.reference.source
    }
}

impl Drop for SourceUnitReference{
    fn drop(&mut self){
        self.reference.dec()
    }
}



pub struct Sources{
    sources:StaticStorage<SourceUnit>
}

impl Sources{
    pub fn new(capacity:usize)->Sources{
        Self{
            sources:StaticStorage::new(capacity),
        }
    }

    pub fn manager(&mut self)->ResourceManager{
        ResourceManager{
            sources:self
        }
    }

    pub fn add(&mut self,source:Source)->Option<usize>{
        self.sources.add(SourceUnit::new(source))
    }

    pub fn get(&mut self,id:usize)->Option<SourceUnitReference>{
        if let Some(unit)=self.sources.get_mut(id){
            unit.inc();
            Some(SourceUnitReference::new(unit))
        }
        else{
            None
        }
    }

    pub fn remove(&mut self,id:usize)->Option<Source>{
        if let Some(SourceUnit{reference_count,..})=self.sources.get(id){
            if *reference_count==0{
                Some(self.sources.remove(id).unwrap().source)
            }
            else{
                None
            }
        }
        else{
            None
        }
    }
}



pub struct ResourceManager<'m>{
    sources:&'m mut Sources
}

impl<'m> ResourceManager<'m>{
    pub fn add(&mut self,source:Source)->Option<usize>{
        self.sources.add(source)
    }

    pub fn get(&mut self,id:usize)->Option<SourceUnitReference>{
        self.sources.get(id)
    }

    pub fn remove_source(&mut self,id:usize)->Option<Source>{
        self.sources.remove(id)
    }
}