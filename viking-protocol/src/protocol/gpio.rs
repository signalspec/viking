use crate::ConstU16;
use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

pub mod pin {
    use super::*;

    pub const PROTOCOL: u16 = 0x0110;

    #[derive(AsBytes, FromBytes, Unaligned, FromZeroes)]
    #[repr(C)]
    pub struct DescribeMode {
        pub protocol: ConstU16<PROTOCOL>,
    }

    pub mod cmd {
        pub const FLOAT: u8 = 0;
        pub const READ: u8 = 1;
        pub const LOW: u8 = 2;
        pub const HIGH: u8 = 3;
    }
}
