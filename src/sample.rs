use std::ops;

/// SampleFormat is the data encoding for an audio sample.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SampleFormat {
    /// Single-prevision floating-point.
    F32,
    /// Double-precision floating-point.
    F64,
    /// Signed 16-bit integer.
    S16,
}

impl SampleFormat {
    pub const fn format_tag(&self) -> u16 {
        match self {
            Self::F32 => 3,
            Self::F64 => 3,
            Self::S16 => 1,
        }
    }

    pub const fn bits_per_sample(&self) -> u16 {
        match self {
            Self::F32 => 32,
            Self::F64 => 64,
            Self::S16 => 16,
        }
    }

    pub fn from_format_tag_and_bits_per_sample(format_tag: u16, bits_per_sample: u16) -> Self {
        match format_tag {
            1 => match bits_per_sample {
                16 => Self::S16,
                _ => unimplemented!(),
            },
            3 => match bits_per_sample {
                32 => Self::F32,
                64 => Self::F64,
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }
}

/// Sample
pub trait Sample:
    Clone + Copy + ops::Add<Output = Self> + ops::Mul<Output = Self> + PartialOrd + PartialEq + Sized
{
    /// Sample Format.
    const FORMAT: SampleFormat;
    /// The number of significant bits in the valid sample range.
    const EFF_BITS: usize;
}

impl Sample for f32 {
    const FORMAT: SampleFormat = SampleFormat::F32;
    const EFF_BITS: usize = 24;
}

impl Sample for f64 {
    const FORMAT: SampleFormat = SampleFormat::F64;
    const EFF_BITS: usize = 53;
}

impl Sample for i16 {
    const FORMAT: SampleFormat = SampleFormat::S16;
    const EFF_BITS: usize = 16;
}
