use crate::flags::flags;
use zerocopy::little_endian::U32;
use zerocopy::{FromBytes, IntoBytes, Unaligned};

pub mod controller {
    use super::*;

    pub const PROTOCOL: u16 = 0x0200;

    flags! {
        pub struct ModeFlags: u16 {
            const PINS = 1 << 0;
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
        pub max_div: U32,
    }

    flags! {
        pub struct ConfigFlags: u16 {
            const CPHA = 1 << 0;
            const CPOL = 1 << 1;
            const LSB_FIRST = 1 << 2;
        }
    }

    impl ConfigFlags {
        pub const fn for_mode(mode: u8) -> Self {
            match mode {
                0 => Self::EMPTY,
                1 => Self::CPHA,
                2 => Self::CPOL,
                3 => Self::CPHA.union(Self::CPOL),
                _ => Self::EMPTY,
            }
        }

        pub fn mode(&self) -> u8 {
            self.contains(Self::CPHA) as u8 | (self.contains(Self::CPOL) as u8) << 1
        }
    }

    #[derive(IntoBytes, FromBytes, Unaligned)]
    #[repr(C)]
    pub struct Config {
        pub flags: ConfigFlags,
        pub clock_div: U32,
    }

    impl Default for Config {
        fn default() -> Self {
            Self {
                flags: ConfigFlags::EMPTY,
                clock_div: U32::new(0),
            }
        }
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
