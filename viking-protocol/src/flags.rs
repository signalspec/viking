macro_rules! flags {
    (
        $vis:vis struct $name:ident: $int:ty {
            $($(#[$meta:meta])* const $flag:ident = 1 << $bit:expr;)*
        }
    ) => {
        #[repr(transparent)]
        #[derive(Copy, Clone, PartialEq, Eq, IntoBytes, FromBytes, Unaligned)]
        $vis struct $name([u8; ::core::mem::size_of::<$int>()]);

        impl $name {
            $(
                $(#[$meta])* $vis const $flag: Self = Self(((1 as $int) << ($bit as u32)).to_le_bytes());
            )*

            $vis const EMPTY: Self = Self([0; ::core::mem::size_of::<$int>()]);

            $vis const fn union(self, other: Self) -> Self {
                Self((<$int>::from_le_bytes(self.0) | <$int>::from_le_bytes(other.0)).to_le_bytes())
            }

            $vis const fn contains(self, other: Self) -> bool {
                <$int>::from_le_bytes(self.0) & <$int>::from_le_bytes(other.0) != 0
            }
        }
    }
}

pub(crate) use flags;
