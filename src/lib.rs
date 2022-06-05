pub use self::metadata::{Chunk, WavMetadata};
pub use self::sample::{Sample, SampleFormat};

pub mod error;
pub mod io;
pub mod metadata;
pub mod sample;
mod utils;
