use defmt::Format;
use zerocopy_derive::{Immutable, KnownLayout, TryFromBytes};

use crate::handler::LidarSettings;

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

impl LidarData {
    fn get_amp(&self) -> Option<u16> {
	match &self {
	    LidarData::NineBytePerCm( NineBytePerCm { amp, .. }) => Some(amp.clone().into()),
	    LidarData::Pix(Pix { .. }) => None,
	    LidarData::NineBytePerMm( NineBytePerMm { amp, .. }) => Some(amp.clone().into()),
	    LidarData::FourByteWithTimestamp( FourByteWithTimestamp { dist, amp, timestamp }) => Some(amp.clone().into()),
	    LidarData::EightBytePerCm( EightBytePerCm { amp, .. }) => Some(amp.clone().into()),
	    LidarData::OutputWithDeviceId( OutputWithDeviceId { amp, .. }) => Some(amp.clone().into()),
	}
    }
    /// Return the distance in cm
    pub fn get_dist_cm(&self) -> u16 {
	match &self {
	    LidarData::NineBytePerCm( NineBytePerCm { dist, .. }) => { let dist_u16: u16 = dist.clone().into(); dist_u16 },
	    LidarData::Pix(Pix { dist }) => (dist.clone()*100.0) as u16,
	    LidarData::NineBytePerMm( NineBytePerMm { dist, .. }) => { let dist_u16: u16 = dist.clone().into(); dist_u16/10 },
	    LidarData::FourByteWithTimestamp( FourByteWithTimestamp { dist, .. }) => { let dist_u16: u16 = dist.clone().into(); dist_u16 },
	    LidarData::EightBytePerCm( EightBytePerCm { dist, .. }) => { let dist_u16: u16 = dist.clone().into(); dist_u16 },
	    LidarData::OutputWithDeviceId( OutputWithDeviceId { dist, .. }) => { let dist_u16: u16 = dist.clone().into(); dist_u16 },
	}
    }
    /// Return the distance in cm
    pub fn get_dist_mm(&self) -> u16 {
	match &self {
	    LidarData::NineBytePerCm( NineBytePerCm { dist, .. }) => { let dist_u16: u16 = dist.clone().into(); dist_u16*10 },
	    LidarData::Pix(Pix { dist }) => (dist.clone()*100.0) as u16,
	    LidarData::NineBytePerMm( NineBytePerMm { dist, .. }) => { let dist_u16: u16 = dist.clone().into(); dist_u16 },
	    LidarData::FourByteWithTimestamp( FourByteWithTimestamp { dist, .. }) => { let dist_u16: u16 = dist.clone().into(); dist_u16*10 },
	    LidarData::EightBytePerCm( EightBytePerCm { dist, .. }) => { let dist_u16: u16 = dist.clone().into(); dist_u16*10 },
	    LidarData::OutputWithDeviceId( OutputWithDeviceId { dist, .. }) => { let dist_u16: u16 = dist.clone().into(); dist_u16*10 },
	}
    }
    pub fn datapoint_is_valid(&self, settings: &LidarSettings) -> bool {
	let amp = self.get_amp();
	let dist = self.get_dist_cm();

	let mut ret: bool = true;

	let dist_in_range: bool = dist > settings.dist_limit.get_lower_limit_cm() && dist < settings.dist_limit.get_upper_limit_cm();
	ret = ret && dist_in_range;

	if let Some(amp) = amp {
	    // TODO: make numbers into const u16 or enums
	    let amp_in_range: bool = (amp > 100) && (amp != 65535);
	    ret = ret && amp_in_range;
	}

	ret
    }
}
