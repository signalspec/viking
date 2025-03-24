use zerocopy::{FromBytes, IntoBytes, Unaligned};

use crate::{U16, U32};

pub const DESCRIPTOR_TYPE_VIKING: u8 = 0x40;
pub const DESCRIPTOR_TYPE_IDENTIFIER: u8 = 0x41;
pub const DESCRIPTOR_TYPE_RESOURCE: u8 = 0x42;
pub const DESCRIPTOR_TYPE_MODE: u8 = 0x43;

#[derive(IntoBytes, FromBytes, Unaligned)]
#[repr(C)]
pub struct VikingDescriptor {
    pub total_len: U16,
    pub version: u8,
    pub rsvd: u8,
    pub max_cmd: U32,
    pub max_res: U32,
    pub max_evt: U32,
}
