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

pub trait SampleTransform:Sample{
    fn into_f32(self)->f32;

    fn to_u16(self,volume:f32)->u16;

    fn to_i16(self,volume:f32)->i16;

    fn to_f32(self,volume:f32)->f32;

    fn from<S:SampleTransform>(sample:S,volume:f32)->Self;

    /// Linear interpolation between two samples.
    ///
    /// The result should be equal to
    /// `first * numerator / denominator + second * (1 - numerator / denominator)`.
    fn lerp(first:Self,second:Self,numerator:u32,denominator:u32)->Self;
    /// Multiplies the value of this sample by the given amount.
    fn amplify(self,value:f32)->Self;

    /// Calls `saturating_add` on the sample.
    fn saturating_add(self, other: Self)->Self;

    /// Returns the value corresponding to the absence of sound.
    fn zero_value()->Self;
}

impl SampleTransform for i16{
    fn into_f32(self)->f32{
        if self<0{
            self as f32 / -(std::i16::MIN as f32)
        }
        else{
            self as f32 / std::i16::MAX as f32
        }
    }

    fn to_u16(self,volume:f32)->u16{
        let s=(self as f32*volume) as i16;

        if s < 0 {
            (s - ::std::i16::MIN) as u16
        } else {
            (s as u16) + 32768
        }
    }

    fn to_i16(self,volume:f32)->i16{
        (self as f32 * volume) as i16
    }

    fn to_f32(self,volume:f32)->f32{
        if self < 0 {
            (self as f32)*volume / -(::std::i16::MIN as f32)
        } else {
            (self as f32)*volume / ::std::i16::MAX as f32
        }
    }

    fn from<S:SampleTransform>(sample:S,volume:f32)->Self{
        sample.to_i16(volume)
    }

    #[inline]
    fn lerp(first: i16, second: i16, numerator: u32, denominator: u32) -> i16 {
        (first as i32 + (second as i32 - first as i32) * numerator as i32 / denominator as i32)
            as i16
    }

    #[inline]
    fn amplify(self, value: f32) -> i16 {
        ((self as f32) * value) as i16
    }

    #[inline]
    fn saturating_add(self, other: i16) -> i16 {
        self.saturating_add(other)
    }

    #[inline]
    fn zero_value() -> i16 {
        0
    }
}

impl SampleTransform for u16{
    fn into_f32(self)->f32{
        if self >= 32768 {
            (self - 32768) as f32 / std::i16::MAX as f32
        }
        else{
            self as f32 - 32768f32 / -(std::i16::MIN as f32)
        }
    }

    fn to_u16(self,_volume:f32)->u16{
        self
    }

    fn to_i16(self,_volume:f32)->i16{
        if self >= 32768 {
            (self - 32768) as i16
        } else {
            (self as i16) - 32767 - 1
        }
    }

    fn to_f32(self,_volume:f32)->f32{
        if self >= 32768 {
            (self - 32768) as f32/ 32768f32
        } else {
            ((self as i16) - 32767 - 1) as f32/ (32768f32)
        }
    }

    fn from<S:SampleTransform>(sample:S,volume:f32)->Self{
        sample.to_u16(volume)
    }
    
    #[inline]
    fn lerp(first: u16, second: u16, numerator: u32, denominator: u32) -> u16 {
        (first as u32 + (second as u32 - first as u32) * numerator / denominator) as u16
    }

    #[inline]
    fn amplify(self, value: f32) -> u16 {
        let s=Sample::to_i16(&self).amplify(value);
        Sample::to_u16(&s)
    }

    #[inline]
    fn saturating_add(self, other: u16) -> u16 {
        self.saturating_add(other)
    }

    #[inline]
    fn zero_value() -> u16 {
        32768
    }
}

impl SampleTransform for f32{
    fn into_f32(self)->f32{
        self
    }

    fn to_u16(self,volume:f32)->u16{
        let s=(self as f32*volume) as i16;

        if s < 0 {
            (s - ::std::i16::MIN) as u16
        } else {
            (s as u16) + 32768
        }
    }

    fn to_i16(self,volume:f32)->i16{
        (self as f32 * volume) as i16
    }

    fn to_f32(self,volume:f32)->f32{
        self*volume
    }

    fn from<S:SampleTransform>(sample:S,volume:f32)->Self{
        sample.to_f32(volume)
    }

    #[inline]
    fn lerp(first: f32, second: f32, numerator: u32, denominator: u32) -> f32 {
        first + (second - first) * numerator as f32 / denominator as f32
    }

    #[inline]
    fn amplify(self, value: f32) -> f32 {
        self * value
    }

    #[inline]
    fn saturating_add(self, other: f32) -> f32 {
        self + other
    }

    #[inline]
    fn zero_value() -> f32 {
        0.0
    }
}