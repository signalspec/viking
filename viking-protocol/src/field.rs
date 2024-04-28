use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

/// U16 with a fixed value
///
/// Used for the `protocol` field.
#[derive(Clone, Copy, AsBytes, FromBytes, Unaligned, FromZeroes)]
#[repr(transparent)]
pub struct ConstU16<const V: u16>([u8; 2]);

impl<const V: u16> ConstU16<V> {
    pub const fn new() -> Self {
        ConstU16(V.to_le_bytes())
    }

    pub fn valid(&self) -> bool {
        u16::from_le_bytes(self.0) == V
    }
}
