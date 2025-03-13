use crate::flags::flags;
use zerocopy::{FromBytes, IntoBytes, Unaligned};

pub mod controller {
    use super::*;

    pub const PROTOCOL: u16 = 0x0301;

    flags! {
        pub struct ModeFlags: u16 {
            const PINS = 1 << 0;
            const CLOCK_STRETCH = 1 << 1;
            const BYTE_AT_A_TIME = 1 << 2;
            const WRITE_THEN_READ = 1 << 3;
            const REPEATED_START_SAME_ADDRESS = 1 << 4;
            const REPEATED_START = 1 << 5;
            const ZERO_LEN_WRITE = 1 << 6;
            const ZERO_LEN_READ = 1 << 7;
        }
    }

    pub mod speed {
        pub const SLOW: u8 = 0;
        pub const STANDARD: u8 = 1;
        pub const FAST: u8 = 2;
        pub const FAST_PLUS: u8 = 3;
        pub const HIGH: u8 = 4;
    }

    flags! {
        pub struct SpeedFlags: u8 {
            /// 10K
            const SLOW = 1 << speed::SLOW;

            /// 100K
            const STANDARD = 1 << speed::STANDARD;

            /// 400K
            const FAST = 1 << speed::FAST;

            /// 1000k
            const FAST_PLUS = 1 << speed::FAST_PLUS;

            /// 3.4M
            const HIGH = 1 << speed::HIGH;

        }
    }

    #[derive(IntoBytes, FromBytes, Unaligned)]
    #[repr(C)]
    pub struct DescribeMode {
        pub flags: ModeFlags,
        pub speed: SpeedFlags,
    }

    #[derive(IntoBytes, FromBytes, Unaligned)]
    #[repr(C)]
    pub struct Config {
        speed: u8,
    }

    pub mod cmd {
        pub const START: u8 = 0;
        pub const STOP: u8 = 1;
        pub const READ: u8 = 2;
        pub const WRITE: u8 = 3;
    }
}

pub mod scl {
    pub const PROTOCOL: u16 = 0x0310;
}

pub mod sda {
    pub const PROTOCOL: u16 = 0x0311;
}
