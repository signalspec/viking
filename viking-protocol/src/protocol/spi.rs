use crate::flags::flags;
use zerocopy::little_endian::U32;
use zerocopy::{FromBytes, IntoBytes, Unaligned};

pub mod controller {
    use super::*;

    pub const PROTOCOL: u16 = 0x0200;

    flags! {
        pub struct ModeFlags: u16 {
            const PINS = 1 << 0;
            const SPEED = 1 << 1;
            const MODE0 = 1 << 2;
            const MODE1 = 1 << 3;
            const MODE2 = 1 << 4;
            const MODE3 = 1 << 5;
            const MSB_FIRST = 1 << 6;
            const LSB_FIRST = 1 << 7;
        }
    }

    #[derive(IntoBytes, FromBytes, Unaligned)]
    #[repr(C)]
    pub struct DescribeMode {
        pub flags: ModeFlags,
        pub base_clock: U32,
        pub min_div: U32,
        pub max_div: U32,
        pub max_div_pow: u8,
    }

    flags! {
        pub struct ConfigFlags: u16 {
            const CPOL = 1 << 0;
            const CPHA = 1 << 1;
            const LSB_FIRST = 1 << 2;
        }
    }

    #[derive(IntoBytes, FromBytes, Unaligned)]
    #[repr(C)]
    pub struct Config {
        flags: ConfigFlags,
        clock_div: U32,
        div_pow: u8,
    }

    pub mod cmd {
        pub const READ: u8 = 1;
        pub const WRITE: u8 = 2;
        pub const TRANSFER: u8 = 3;
    }
}

pub mod sck_pin {
    pub const PROTOCOL: u16 = 0x0210;
}

pub mod sdo_pin {
    pub const PROTOCOL: u16 = 0x0211;
}

pub mod sdi_pin {
    pub const PROTOCOL: u16 = 0x0212;
}
