use defmt::Format;
use zerocopy_derive::{Immutable, KnownLayout, TryFromBytes};

use super::endian::{U16LE, U32LE};

/// Lidar data output. Contains distance, amp and temperatur information
#[derive(Clone, Debug, Format, TryFromBytes, KnownLayout, Immutable)]
pub struct NineBytePerCm {
    dist: U16LE,
    amp: U16LE,
    temp: U16LE,
    checksum: u8,
}

/// Lidar data output. Contains distance encode as float string ("X.YZ/r/n")
#[derive(Clone, Debug, Format, TryFromBytes, KnownLayout, Immutable)]
pub struct Pix {
    dist: f32,
}

/// Lidar data output. Contains distance (mm), amp and temperatur information
#[derive(Clone, Debug, Format, TryFromBytes, KnownLayout, Immutable)]
pub struct NineBytePerMm {
    dist: U16LE,
    amp: U16LE,
    temp: U16LE,
}

/// Lidar data output. Contains distance (cm), amp and timestamp information
#[derive(Clone, Debug, Format, TryFromBytes, KnownLayout, Immutable)]
pub struct FourByteWithTimestamp {
    dist: U16LE,
    amp: U16LE,
    timestamp: U32LE,
}

/// Lidar data output. Contains distance (cm), amp and timestamp information
#[derive(Clone, Debug, Format, TryFromBytes, KnownLayout, Immutable)]
pub struct EightBytePerCm {
    dist: U16LE,
    amp: U16LE,
    timestamp: U32LE,
}

/// Lidar data output. Contains distance (cm), amp, identifier and timestamp information
#[derive(Clone, Debug, Format, TryFromBytes, KnownLayout, Immutable)]
pub struct OutputWithDeviceId {
    dist: U16LE,
    amp: U16LE,
    timestamp: U32LE,
    device_id: u8,
}

/// Encapsulation of the different lidar data output
#[derive(Clone, Debug, Format)]
#[allow(missing_docs)]
pub enum LidarData {
    NineBytePerCm(NineBytePerCm),
    Pix(Pix),
    NineBytePerMm(NineBytePerMm),
    FourByteWithTimestamp(FourByteWithTimestamp),
    EightBytePerCm(EightBytePerCm),
    OutputWithDeviceId(OutputWithDeviceId),
}
