use cpal::Sample;

/// Represents a value of a single sample.
///
/// This trait is implemented by default on three types: `i16`, `u16` and `f32`.
///
/// - For `i16`, silence corresponds to the value `0`. The minimum and maximum amplitudes are
///   represented by `i16::min_value()` and `i16::max_value()` respectively.
/// - For `u16`, silence corresponds to the value `u16::max_value() / 2`. The minimum and maximum
///   amplitudes are represented by `0` and `u16::max_value()` respectively.
/// - For `f32`, silence corresponds to the value `0.0`. The minimum and maximum amplitudes are
///  represented by `-1.0` and `1.0` respectively.
///
/// You can implement this trait on your own type as well if you wish so.

pub trait SampleTransform:Sample+Copy{
// Обычные преобразования
    /// Переводит `Self` в `i16`.
    /// 
    /// Converts `Self` to `i16`.
    fn into_i16(self)->i16;
    /// Переводит `Self` в `u16`.
    /// 
    /// Converts `Self` to `u16`.
    fn into_u16(self)->u16;
    /// Переводит `Self` в `f32`.
    /// 
    /// Converts `Self` to `f32`.
    fn into_f32(self)->f32;

// Расширенные преобразования
    /// Переводит `Self` в `u16` с усилением.
    /// 
    /// Converts `Self` to `u16` with amplification.
    fn to_u16(self,volume:f32)->u16;
    /// Переводит `Self` в `i16` с усилением.
    /// 
    /// Converts `Self` to `i16` with amplification.
    fn to_i16(self,volume:f32)->i16;
    /// Переводит `Self` в `f32` с усилением.
    /// 
    /// Converts `Self` to `f32` with amplification.
    fn to_f32(self,volume:f32)->f32;

    /// Переводит `Self` в `f32` с усилением.
    /// 
    /// Converts `Self` to `f32` with amplification.
    fn from<S:SampleTransform>(sample:S,volume:f32)->Self;

// Сложные преобразования
    /// Linear interpolation between two samples.
    /// 
    /// The result should be equal to
    /// `first * numerator / denominator + second * (1 - numerator / denominator)`.
    fn lerp(first:Self,second:Self,numerator:u32,denominator:u32)->Self;
    /// Multiplies the value of this sample by the given amount.
    fn amplify(self,value:f32)->Self;
    /// Calls `saturating_add` on the sample.
    fn saturating_add(self,other:Self)->Self;
    /// Returns the value corresponding to the absence of sound.
    fn zero_value()->Self;
}

impl SampleTransform for i16{
    // 
    #[inline(always)]
    fn into_i16(self)->i16{
        self
    }
    // 
    fn into_u16(self)->u16{
        if self<0i16{
            (self-std::i16::MIN) as u16
        }
        else{
            (self as u16)+32768u16
        }
    }
    // 
    fn into_f32(self)->f32{
        if self<0i16{
            self as f32/-(std::i16::MIN as f32)
        }
        else{
            self as f32/std::i16::MAX as f32
        }
    }
    // 
    fn to_u16(self,volume:f32)->u16{
        let s=(self as f32*volume) as i16;
        s.into_u16()
    }
    // 
    fn to_i16(self,volume:f32)->i16{
        (self as f32*volume) as i16
    }
    // 
    fn to_f32(self,volume:f32)->f32{
        let s=self.into_f32();
        s*volume
    }
    // 
    #[inline(always)]
    fn from<S:SampleTransform>(sample:S,volume:f32)->Self{
        sample.to_i16(volume)
    }
    // 
    fn lerp(first:i16,second:i16,numerator:u32,denominator:u32)->i16{
        (first as i32+(second as i32-first as i32)*numerator as i32/denominator as i32) as i16
    }
    // 
    #[inline(always)]
    fn amplify(self,value:f32)->i16{
        self.to_i16(value)
    }
    // 
    #[inline(always)]
    fn saturating_add(self,other:i16)->i16{
        self.saturating_add(other)
    }
    // 
    #[inline(always)]
    fn zero_value()->i16{
        0i16
    }
}

impl SampleTransform for u16{
    // 
    fn into_i16(self)->i16{
        if self>=32768u16{
            (self-32768u16) as i16
        }
        else{
            (self as i16)-32767i16-1i16
        }
    }
    // 
    #[inline(always)]
    fn into_u16(self)->u16{
        self
    }
    // 
    fn into_f32(self)->f32{
        if self>=32768u16{
            (self-32768u16) as f32/std::i16::MAX as f32
        }
        else{
            self as f32-32768f32/-(std::i16::MIN as f32)
        }
    }
    // 
    fn to_u16(self,volume:f32)->u16{
        if self>=32768u16{
            let s=(self-32768u16) as f32*volume;
            s as u16+32768u16
        }
        else{
            let s=(32768u16-self) as f32*volume;
            32768u16-s as u16
        }
    }
    // 
    fn to_i16(self,volume:f32)->i16{
        (self.into_i16() as f32*volume) as i16
    }
    // 
    fn to_f32(self,volume:f32)->f32{
        self.into_f32()*volume
    }
    // 
    #[inline(always)]
    fn from<S:SampleTransform>(sample:S,volume:f32)->Self{
        sample.to_u16(volume)
    }
    // 
    fn lerp(first:u16,second:u16,numerator:u32,denominator:u32)->u16{
        (first as u32+(second as u32-first as u32)*numerator/denominator) as u16
    }
    // 
    #[inline(always)]
    fn amplify(self,value:f32)->u16{
        self.to_u16(value)
    }
    // 
    #[inline(always)]
    fn saturating_add(self,other:u16)->u16{
        self.saturating_add(other)
    }
    // 
    #[inline(always)]
    fn zero_value()->u16{
        32768u16
    }
}

impl SampleTransform for f32{
    // 
    fn into_i16(self)->i16{
        if self.is_sign_negative(){
            (self*-(std::i16::MIN as f32)) as i16
        }
        else{
            (self*std::i16::MAX as f32) as i16
        }
    }
    // 
    fn into_u16(self)->u16{
        ((self+1f32)*0.5f32*std::u16::MAX as f32).round() as u16
    }
    // 
    #[inline(always)]
    fn into_f32(self)->f32{
        self
    }
    // 
    fn to_u16(self,volume:f32)->u16{
        if self.is_sign_negative(){
            let s=(-self*volume*32768f32) as u16;
            32768u16-s
        }
        else{
            ((self*volume+1f32)*32768f32) as u16
        }
    }
    // 
    fn to_i16(self,volume:f32)->i16{
        if self.is_sign_negative(){
            (self*volume*-(std::i16::MIN as f32)) as i16
        }
        else{
            (self*volume*std::i16::MAX as f32) as i16
        }
    }
    // 
    #[inline(always)]
    fn to_f32(self,volume:f32)->f32{
        self*volume
    }
    // 
    #[inline(always)]
    fn from<S:SampleTransform>(sample:S,volume:f32)->Self{
        sample.to_f32(volume)
    }
    // 
    #[inline(always)]
    fn lerp(first:f32,second:f32,numerator:u32,denominator:u32)->f32{
        first+(second-first)*numerator as f32/denominator as f32
    }
    // 
    #[inline(always)]
    fn amplify(self,value:f32)->f32{
        self*value
    }
    // 
    #[inline(always)]
    fn saturating_add(self,other:f32)->f32{
        self+other
    }
    // 
    #[inline(always)]
    fn zero_value()->f32{
        0f32
    }
}