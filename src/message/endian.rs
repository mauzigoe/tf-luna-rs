use defmt::Format;
use zerocopy::{LittleEndian, U16, U32};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// [zerocopy::byteorder::U16] wrapper to implement [defmt::Format]
#[derive(Clone, Debug, FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C)]
pub struct U16LE(U16<LittleEndian>);

impl Format for U16LE {
    fn format(&self, fmt: defmt::Formatter) {
        u16::from_le_bytes(self.0.to_bytes()).format(fmt)
    }
}

impl From<u16> for U16LE {
    fn from(value: u16) -> Self {
        U16LE(U16::<LittleEndian>::new(value))
    }
}

/// [zerocopy::byteorder::U32] wrapper to implement [defmt::Format]
#[derive(Clone, Debug, FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C)]
pub struct U32LE(U32<LittleEndian>);

impl Format for U32LE {
    fn format(&self, fmt: defmt::Formatter) {
        u32::from_le_bytes(self.0.to_bytes()).format(fmt)
    }
}

impl From<u32> for U32LE {
    fn from(value: u32) -> Self {
        U32LE(U32::<LittleEndian>::new(value))
    }
}
