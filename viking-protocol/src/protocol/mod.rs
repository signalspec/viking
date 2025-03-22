pub mod gpio;
pub mod i2c;
pub mod led;
pub mod spi;

/// Base commands
///
/// These occupy the encoding space for reserved resource 0
pub mod cmd {
    pub const DELAY: u8 = 0;
}
