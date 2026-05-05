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

pub fn protocol_name(protocol: u16) -> Option<&'static str> {
    Some(match protocol {
        gpio::pin::PROTOCOL => "gpio_pin",
        gpio::level_interrupt::PROTOCOL => "gpio_level_interrupt",
        led::binary::PROTOCOL => "led",
        i2c::controller::PROTOCOL => "i2c_controller",
        i2c::scl::PROTOCOL => "i2c_sda_pin",
        i2c::sda::PROTOCOL => "i2c_scl_pin",
        spi::controller::PROTOCOL => "spi_controller",
        spi::sck_pin::PROTOCOL => "spi_sck_pin",
        spi::sdi_pin::PROTOCOL => "spi_sdi_pin",
        spi::sdo_pin::PROTOCOL => "spi_sdo_pin",
        _ => return None
    })
}
