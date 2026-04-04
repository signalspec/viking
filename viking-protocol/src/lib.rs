#![no_std]
pub mod protocol;
pub mod request;

mod flags;

pub use zerocopy::little_endian::{U16, U32};
pub use zerocopy::IntoBytes;

pub mod descriptor;

pub mod errors {
    // 0x00 - 0x7F are successful status bytes
    pub const ERR_OK: u8 = 0x00;
    pub const MIN_ERR: u8 = 0x80;

    // 0x80 - 0x8F are invalid request errors that indicate a protocol violation by the host
    pub const ERR_INVALID_RESOURCE: u8 = 0x81;
    pub const ERR_INVALID_MODE: u8 = 0x82;
    pub const ERR_INVALID_COMMAND: u8 = 0x83;
    pub const ERR_INVALID_ARG: u8 = 0x84;
    pub const ERR_INVALID_STATE: u8 = 0x85;
    pub const ERR_MISSING_ARG: u8 = 0x86;
    pub const ERR_RESPONSE_FULL: u8 = 0x87;

    // 0x90 - 0x9F indicate a configuration that is not supported, for reasons that may
    // or may not be fully specified in the descriptor
    pub const ERR_UNSUPPORTED_CONFIG: u8 = 0x90;
    pub const ERR_UNSUPPORTED_CLOCK: u8 = 0x91;
    pub const ERR_CONFLICT: u8 = 0x9E;
    pub const ERR_BUSY: u8 = 0x9F;

    // 0xA0 - 0xAF are transient errors that depend on the state of the device or environment
    pub const ERR_TIMEOUT: u8 = 0xA0;

    // 0xC0 - 0xCF are protocol-specific errors.
    pub const ERR_ADDR_NACK: u8 = 0xC0;
    pub const ERR_DATA_NACK: u8 = 0xC1;
    pub const ERR_ARBITRATION_LOST: u8 = 0xC2;

    pub const ERR_UNKNOWN: u8 = 0xFF;
}
