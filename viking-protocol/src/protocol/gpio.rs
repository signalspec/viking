pub mod pin {
    pub const PROTOCOL: u16 = 0x0110;

    pub mod cmd {
        pub const FLOAT: u8 = 0;
        pub const READ: u8 = 1;
        pub const LOW: u8 = 2;
        pub const HIGH: u8 = 3;
    }
}

pub mod level_interrupt {
    pub const PROTOCOL: u16 = 0x0120;

    pub mod cmd {
        pub const WAIT_LOW: u8 = 0;
        pub const WAIT_HIGH: u8 = 1;
        pub const EVT_LOW: u8 = 2;
        pub const EVT_HIGH: u8 = 3;
    }
}
