#![no_std]
pub mod protocol;
pub mod request;

mod flags;

pub use zerocopy::little_endian::{U16, U32};
pub use zerocopy::IntoBytes;

pub mod descriptor;
