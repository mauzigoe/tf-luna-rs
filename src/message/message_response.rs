use defmt::Format;
use zerocopy_derive::{FromBytes, TryFromBytes};

use super::endian::{U16LE, U32LE};

/// Calculate checksum over an array of length `len`
pub fn calc_checksum(buf: &[u8], len: usize) -> u8 {
    let mut sum: u8 = 0;
    for x in buf[0..len].iter() {
        sum += *x;
    }
    sum
}

/// Response for a request [GetVersion][crate::message::message_request::RequestGetVersion]
#[derive(Clone, Debug, Format, FromBytes)]
pub struct ResponseGetVersion {
    /// Minor Version Number
    version_minor: u8,
    /// Mid Version Number
    version_mid: u8,
    /// Minor Version Number
    version_major: u8,
}

/// Response for a request [SoftReset][crate::message::message_request::RequestSoftReset]
#[derive(Clone, Debug, Format, FromBytes)]
pub struct ResponseSoftReset {
    /// Success status
    status: u8,
}

/// Response for a request [SampleFreq][crate::message::message_request::RequestSampleFreq]
#[derive(Clone, Debug, Format, FromBytes)]
pub struct ResponseSampleFreq {
    /// frequency set
    freq: U16LE,
}

/// Response for a request [SampleTrig][crate::message::message_request::RequestSampleTrig]
#[derive(Clone, Debug, Format, FromBytes)]
pub struct ResponseSampleTrig {}

/// Response for a request [OutputFormat][crate::message::message_request::RequestOutputFormat]
#[derive(Clone, Debug, Format, FromBytes)]
pub struct ResponseOutputFormat {
    /// Data output set
    format: u8,
}

/// Response for a request [BaudRate][crate::message::message_request::RequestBaudRate]
#[derive(Clone, Debug, Format, FromBytes)]
pub struct ResponseBaudRate {
    /// Baudrate set
    bautrate: U32LE,
}

/// Response for a request [OutputEn][crate::message::message_request::RequestOutputEn]
#[derive(Clone, Debug, Format, FromBytes)]
pub struct ResponseOutputEn {
    /// Output set status
    enable: u8,
}

/// Response for a request [FrameChecksumEn][crate::message::message_request::RequestFrameChecksumEn]
#[derive(Clone, Debug, Format, FromBytes)]
pub struct ResponseFrameChecksumEn {
    /// Usage of checksum for request/respose messages (frames)
    enable: u8,
}

/// Response for a request [I2cSlaveAddr][crate::message::message_request::RequestI2cSlaveAddr]
#[derive(Clone, Debug, Format, FromBytes)]
pub struct ResponseI2cSlaveAddress {
    /// I2c slave address
    i2c_slave_addr: u8,
}

/// Response for a request [RestoreDefault][crate::message::message_request::RequestRestoreDefault]
#[derive(Clone, Debug, Format, FromBytes)]
pub struct ResponseRestoreDefault {
    // Success status
    status: u8,
}

/// Response for a request [SaveSettings][crate::message::message_request::RequestSaveSettings]
#[derive(Clone, Debug, Format, FromBytes)]
pub struct ResponseSaveSettings {
    /// Success status
    status: u8,
}

/// Response for a request [GetFullVersion][crate::message::message_request::RequestGetFullVersion]
#[derive(Clone, Debug, Format, FromBytes)]
pub struct ResponseGetFullVersion {
    /// NOT IMPLEMENTED YET
    full_version: [u8; 29],
}

/// Response for a request [AmpThreshold][crate::message::message_request::RequestAmpThreshold]
#[derive(Clone, Debug, Format, FromBytes)]
pub struct ResponseAmpThreshold {
    /// Amp threshold configured
    amp_threshold: u8,
    /// Dummy distance used when `amp < amp_theshold * 10`
    dummy_dist: u8,
}

/// Response for a request [LowConsumption][crate::message::message_request::RequestLowConsumption]
#[derive(Clone, Debug, Format, FromBytes)]
pub struct ResponseLowConsumption {
    /// Sample rate used
    sample_rate: u8,
}

/// Response for a request [DistLimit][crate::message::message_request::RequestDistLimit]
#[derive(Clone, Debug, Format, FromBytes)]
pub struct ResponseDistLimit {
    ///
    dist_min: U16LE,
    dist_max: U16LE,
    silence: u8,
}

/// Response for a request [LowSampleRate][crate::message::message_request::RequestLowSampleRate]
#[derive(Clone, Debug, Format, FromBytes)]
pub struct ResponseLowSampleRate {
    /// Output Period (s) used
    output_period_s: U32LE,
    /// Frame (s) over which the distance is averaged
    one_shot_frames: U32LE,
}

/// Enumration of response identifiers
#[derive(Clone, Debug, Format, TryFromBytes)]
#[repr(u8)]
pub enum ResponseIdType {
    IdZeroOutput = 0u8.to_le(),
    GetVersion = 1u8.to_le(),
    SoftReset = 2u8.to_le(),
    SampleFreq = 3u8.to_le(),
    SampleTrig = 4u8.to_le(),
    OutputFormat = 5u8.to_le(),
    BaudRate = 6u8.to_le(),
    OutputEn = 7u8.to_le(),
    FrameChecksumEn = 8u8.to_le(),
    I2cSlaveAddress = 0xBu8.to_le(),
    RestoreDefault = 0x10u8.to_le(),
    SaveSettings = 0x11u8.to_le(),
    GetFullVersion = 0x14u8.to_le(),
}

/// Encapsulation of lidar response messages
#[derive(Clone, Debug, Format)]
pub enum LidarResponse {
    /// Response messages for Request [GetVersion][crate::message::message_request::RequestGetVersion]
    GetVersion(ResponseGetVersion),
    /// Response messages for Request [SoftReset][crate::message::message_request::RequestSoftReset]
    SoftReset(ResponseSoftReset),
    /// Response messages for Request [SampleFreq][crate::message::message_request::RequestSampleFreq]
    SampleFreq(ResponseSampleFreq),
    /// Response messages for Request [SampleTrig][crate::message::message_request::RequestSampleTrig]
    SampleTrig(ResponseSampleTrig),
    /// Response messages for Request [OutputFormat][crate::message::message_request::RequestOutputFormat]
    OutputFormat(ResponseOutputFormat),
    /// Response messages for Request [BaudRate][crate::message::message_request::RequestBaudRate]
    BaudRate(ResponseBaudRate),
    /// Response messages for Request [OutputEn][crate::message::message_request::RequestOutputEn]
    OutputEn(ResponseOutputEn),
    /// Response messages for Request [FrameChecksumEn][crate::message::message_request::RequestFrameChecksumEn]
    FrameChecksumEn(ResponseFrameChecksumEn),
    /// Response messages for Request [RequestAmpThreshold][crate::message::message_request::RequestAmpThreshold]
    AmpThreshold(ResponseAmpThreshold),
    /// Response messages for Request [LowConsumption][crate::message::message_request::RequestLowConsumption]
    LowConsumption(ResponseLowConsumption),
    /// Response messages for Request [DistLimit][crate::message::message_request::RequestDistLimit]
    DistLimit(ResponseDistLimit),
    /// Response messages for Request [LowSampleRate][crate::message::message_request::RequestLowSampleRate]
    LowSampleRate(ResponseLowSampleRate),
    /// Response messages for Request [I2cSlaveAddr][crate::message::message_request::RequestI2cSlaveAddr]
    I2cSlaveAddress(ResponseI2cSlaveAddress),
    /// Response messages for Request [RestoreDefault][crate::message::message_request::RequestRestoreDefault]
    RestoreDefault(ResponseRestoreDefault),
    /// Response messages for Request [SaveSettings][crate::message::message_request::RequestSaveSettings]
    SaveSettings(ResponseSaveSettings),
    /// Response messages for Request [GetFullVersion][crate::message::message_request::RequestGetFullVersion]
    GetFullVersion(ResponseGetFullVersion),
}
