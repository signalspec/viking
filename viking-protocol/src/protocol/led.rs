pub mod binary {
    pub const PROTOCOL: u16 = 0x0130;

    pub mod cmd {
        pub const OFF: u8 = 0;
        pub const ON: u8 = 1;
    }

    pub mod color {
        pub const RED: u8 = 1;
        pub const GREEN: u8 = 2;
        pub const BLUE: u8 = 3;
        pub const WHITE: u8 = 4;
        pub const AMBER: u8 = 5;
        pub const YELLOW: u8 = 6;
        pub const ORANGE: u8 = 7;
        pub const PINK: u8 = 8;
        pub const PURPLE: u8 = 9;
        pub const INFRARED: u8 = 10;
        pub const ULTRAVIOLET: u8 = 11;

        pub const fn name(value: u8) -> Option<&'static str> {
            match value {
                RED => Some("red"),
                GREEN => Some("green"),
                BLUE => Some("blue"),
                WHITE => Some("white"),
                AMBER => Some("amber"),
                YELLOW => Some("yellow"),
                ORANGE => Some("orange"),
                PINK => Some("pink"),
                PURPLE => Some("purple"),
                INFRARED => Some("infrared"),
                ULTRAVIOLET => Some("ultraviolet"),
                _ => None,
            }
        }
    }
}
