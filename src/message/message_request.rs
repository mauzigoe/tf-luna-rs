use crate::handler::LidarSettingOutput;

use super::endian::{U16LE, U32LE};
use defmt::Format;
use zerocopy::IntoBytes;
use zerocopy_derive::{Immutable, IntoBytes, Unaligned};

const LIDAR_REQUEST_HEAD: u8 = 0x5A;

/// Enumeration of the request messgage identifiers
#[derive(Clone, Copy, Debug, Format, Immutable, IntoBytes, Unaligned)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum LidarRequestId {
    GetVersion = 0x01,
    SoftReset = 0x02,
    SampleFreq = 0x03,
    SampleTrig = 0x04,
    OutputFormat = 0x05,
    BaudRate = 0x06,
    OutputEn = 0x07,
    FrameChecksumEn = 0x08,
    I2cSlaveAddr = 0x0B,
    RestoreDefault = 0x10,
    SaveSettings = 0x11,
    ReadManuBin = 0x12,
    GetFullVersion = 0x14,
    AmpThreshold = 0x22,
    TimestampSync = 0x31,
    LowConsumption = 0x35,
    DistLimit = 0x3A,
    OnOffMode = 0x3B,
    LowSampleRate = 0x3E,
    GetConfigPara = 0x3F,
}

const LIDAR_REQUEST_LEN_GET_VERSION: u8 = 4;
const LIDAR_REQUEST_LEN_SOFT_RESET: u8 = 4;
const LIDAR_REQUEST_LEN_SAMPLE_FREQ: u8 = 6;
const LIDAR_REQUEST_LEN_SAMPLE_TRIG: u8 = 4;
const LIDAR_REQUEST_LEN_OUTPUT_FORMAT: u8 = 5;
const LIDAR_REQUEST_LEN_BAUD_RATE: u8 = 8;
const LIDAR_REQUEST_LEN_OUTPUT_EN: u8 = 5;
const LIDAR_REQUEST_LEN_FRAME_CHECKSUM_EN: u8 = 5;
const LIDAR_REQUEST_LEN_I2C_SLAVE_ADDR: u8 = 5;
const LIDAR_REQUEST_LEN_RESTORE_DEFAULT: u8 = 4;
const LIDAR_REQUEST_LEN_SAVE_SETTINGS: u8 = 4;
const LIDAR_REQUEST_LEN_READ_MANU_BIN: u8 = 4;
const LIDAR_REQUEST_LEN_GET_FULL_VERSION: u8 = 4;
const LIDAR_REQUEST_LEN_AMP_THRESHOLD: u8 = 7;
const LIDAR_REQUEST_LEN_TIMESTAMP_SYNC: u8 = 8;
const LIDAR_REQUEST_LEN_LOW_CONSUMPTION: u8 = 6;
const LIDAR_REQUEST_LEN_DIST_LIMIT: u8 = 9;
const LIDAR_REQUEST_LEN_ON_OFF_MODE: u8 = 13;
const LIDAR_REQUEST_LEN_LOW_SAMPLE_RATE: u8 = 12;
const LIDAR_REQUEST_LEN_GET_CONFIG_PARA: u8 = 5;

/// Header for request messages sent to the lidar
#[derive(Clone, Debug, Format, Immutable, IntoBytes, Unaligned)]
#[repr(C)]
pub struct RequestHeader {
    head: u8,
    len: u8,
    id: LidarRequestId,
}

impl RequestHeader {
    /// Return request message header for the respective message identifier `id`
    pub fn new(id: LidarRequestId) -> Self {
        match id {
            LidarRequestId::GetVersion => Self {
                head: LIDAR_REQUEST_HEAD,
                len: LIDAR_REQUEST_LEN_GET_VERSION,
                id: LidarRequestId::GetVersion,
            },
            LidarRequestId::SoftReset => Self {
                head: LIDAR_REQUEST_HEAD,
                len: LIDAR_REQUEST_LEN_SOFT_RESET,
                id: LidarRequestId::SoftReset,
            },
            LidarRequestId::SampleFreq => Self {
                head: LIDAR_REQUEST_HEAD,
                len: LIDAR_REQUEST_LEN_SAMPLE_FREQ,
                id: LidarRequestId::SampleFreq,
            },
            LidarRequestId::SampleTrig => Self {
                head: LIDAR_REQUEST_HEAD,
                len: LIDAR_REQUEST_LEN_SAMPLE_TRIG,
                id: LidarRequestId::SampleTrig,
            },
            LidarRequestId::OutputFormat => Self {
                head: LIDAR_REQUEST_HEAD,
                len: LIDAR_REQUEST_LEN_OUTPUT_FORMAT,
                id: LidarRequestId::OutputFormat,
            },
            LidarRequestId::BaudRate => Self {
                head: LIDAR_REQUEST_HEAD,
                len: LIDAR_REQUEST_LEN_BAUD_RATE,
                id: LidarRequestId::BaudRate,
            },
            LidarRequestId::OutputEn => Self {
                head: LIDAR_REQUEST_HEAD,
                len: LIDAR_REQUEST_LEN_OUTPUT_EN,
                id: LidarRequestId::OutputEn,
            },
            LidarRequestId::FrameChecksumEn => Self {
                head: LIDAR_REQUEST_HEAD,
                len: LIDAR_REQUEST_LEN_FRAME_CHECKSUM_EN,
                id: LidarRequestId::FrameChecksumEn,
            },
            LidarRequestId::I2cSlaveAddr => Self {
                head: LIDAR_REQUEST_HEAD,
                len: LIDAR_REQUEST_LEN_I2C_SLAVE_ADDR,
                id: LidarRequestId::I2cSlaveAddr,
            },
            LidarRequestId::RestoreDefault => Self {
                head: LIDAR_REQUEST_HEAD,
                len: LIDAR_REQUEST_LEN_RESTORE_DEFAULT,
                id: LidarRequestId::RestoreDefault,
            },
            LidarRequestId::SaveSettings => Self {
                head: LIDAR_REQUEST_HEAD,
                len: LIDAR_REQUEST_LEN_SAVE_SETTINGS,
                id: LidarRequestId::SaveSettings,
            },
            LidarRequestId::ReadManuBin => Self {
                head: LIDAR_REQUEST_HEAD,
                len: LIDAR_REQUEST_LEN_READ_MANU_BIN,
                id: LidarRequestId::ReadManuBin,
            },
            LidarRequestId::GetFullVersion => Self {
                head: LIDAR_REQUEST_HEAD,
                len: LIDAR_REQUEST_LEN_GET_FULL_VERSION,
                id: LidarRequestId::GetFullVersion,
            },
            LidarRequestId::AmpThreshold => Self {
                head: LIDAR_REQUEST_HEAD,
                len: LIDAR_REQUEST_LEN_AMP_THRESHOLD,
                id: LidarRequestId::AmpThreshold,
            },
            LidarRequestId::TimestampSync => Self {
                head: LIDAR_REQUEST_HEAD,
                len: LIDAR_REQUEST_LEN_TIMESTAMP_SYNC,
                id: LidarRequestId::TimestampSync,
            },
            LidarRequestId::LowConsumption => Self {
                head: LIDAR_REQUEST_HEAD,
                len: LIDAR_REQUEST_LEN_LOW_CONSUMPTION,
                id: LidarRequestId::LowConsumption,
            },
            LidarRequestId::DistLimit => Self {
                head: LIDAR_REQUEST_HEAD,
                len: LIDAR_REQUEST_LEN_DIST_LIMIT,
                id: LidarRequestId::DistLimit,
            },
            LidarRequestId::OnOffMode => Self {
                head: LIDAR_REQUEST_HEAD,
                len: LIDAR_REQUEST_LEN_ON_OFF_MODE,
                id: LidarRequestId::OnOffMode,
            },
            LidarRequestId::LowSampleRate => Self {
                head: LIDAR_REQUEST_HEAD,
                len: LIDAR_REQUEST_LEN_LOW_SAMPLE_RATE,
                id: LidarRequestId::LowSampleRate,
            },
            LidarRequestId::GetConfigPara => Self {
                head: LIDAR_REQUEST_HEAD,
                len: LIDAR_REQUEST_LEN_GET_CONFIG_PARA,
                id: LidarRequestId::GetConfigPara,
            },
        }
    }
    /// calculate the sum of the header fields as an overflowing addition of u8 integers
    pub fn sum(&self) -> u8 {
        (self.head.clone() as u8)
            .overflowing_add(self.len)
            .0
            .overflowing_add(self.id.clone() as u8)
            .0
    }
}

/// Request message for the lidar version
#[derive(Clone, Debug, Format, Immutable, IntoBytes, Unaligned)]
#[repr(C)]
pub struct RequestGetVersion {
    header: RequestHeader,
    check_sum: u8,
}

impl RequestGetVersion {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        let header = RequestHeader::new(LidarRequestId::GetVersion);
        let check_sum: u8 = header.sum();
        Self { header, check_sum }
    }
}

/// Request message to perform a soft reset
#[derive(Clone, Debug, Format, Immutable, IntoBytes, Unaligned)]
#[repr(C)]
pub struct RequestSoftReset {
    header: RequestHeader,
    check_sum: u8,
}

impl RequestSoftReset {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        let header = RequestHeader::new(LidarRequestId::SoftReset);
        let check_sum = header.sum();
        Self { header, check_sum }
    }
}

/// Request to change sample frequency. If `[RequestSampleFreq::freq] == 0` trigger mode is used.
#[derive(Clone, Debug, Format, Immutable, IntoBytes, Unaligned)]
#[repr(C)]
pub struct RequestSampleFreq {
    header: RequestHeader,
    freq: U16LE,
    check_sum: u8,
}

impl RequestSampleFreq {
    #[allow(missing_docs)]
    pub fn new(freq: u16) -> Self {
        let header = RequestHeader::new(LidarRequestId::SampleFreq);
        let check_sum: u8 = header
            .sum()
            .overflowing_add(overflowing_sum(freq.as_bytes()))
            .0;
        Self {
            header,
            freq: freq.into(),
            check_sum,
        }
    }
}

/// Request message to trigger a measurement.
#[derive(Clone, Debug, Format, Immutable, IntoBytes, Unaligned)]
#[repr(C)]
pub struct RequestSampleTrig {
    header: RequestHeader,
    check_sum: u8,
}

impl RequestSampleTrig {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        let header = RequestHeader::new(LidarRequestId::SampleTrig);
        let check_sum = header.sum();
        Self { header, check_sum }
    }
}

/// Request message for the current output format.
#[derive(Clone, Debug, Format, Immutable, IntoBytes, Unaligned)]
#[repr(C)]
pub struct RequestOutputFormat {
    header: RequestHeader,
    format: u8,
    check_sum: u8,
}

impl RequestOutputFormat {
    #[allow(missing_docs)]
    pub fn new(format: LidarSettingOutput) -> Self {
        let header = RequestHeader::new(LidarRequestId::OutputFormat);
        let check_sum = header.sum().overflowing_add(format as u8).0;
        Self {
            header,
            format: format as u8,
            check_sum,
        }
    }
}

/// Reqeuest message for the current baudrate
#[derive(Clone, Debug, Format, Immutable, IntoBytes, Unaligned)]
#[repr(C)]
pub struct RequestBaudRate {
    header: RequestHeader,
    baudrate: U32LE,
    check_sum: u8,
}

impl RequestBaudRate {
    #[allow(missing_docs)]
    pub fn new(baudrate: u32) -> Self {
        let header = RequestHeader::new(LidarRequestId::BaudRate);
        let check_sum = header
            .sum()
            .overflowing_add(overflowing_sum(baudrate.as_bytes()))
            .0;
        Self {
            header,
            baudrate: baudrate.into(),
            check_sum,
        }
    }
}

/// Reqeuest to dis-/enable output of measurements.
#[derive(Clone, Debug, Format, Immutable, IntoBytes, Unaligned)]
#[repr(C)]
pub struct RequestOutputEn {
    header: RequestHeader,
    enable: bool,
    check_sum: u8,
}

impl RequestOutputEn {
    #[allow(missing_docs)]
    pub fn new(enable: bool) -> Self {
        let header = RequestHeader::new(LidarRequestId::OutputEn);
        let check_sum = header.sum().overflowing_add(enable as u8).0;
        Self {
            header,
            enable,
            check_sum,
        }
    }
}

/// Request to enable checking of checksums for messages
#[derive(Clone, Debug, Format, Immutable, IntoBytes, Unaligned)]
#[repr(C)]
pub struct RequestFrameChecksumEn {
    header: RequestHeader,
    enable: bool,
    check_sum: u8,
}

impl RequestFrameChecksumEn {
    #[allow(missing_docs)]
    pub fn new(enable: bool) -> Self {
        let header = RequestHeader::new(LidarRequestId::FrameChecksumEn);
        let check_sum = header.sum().overflowing_add(enable as u8).0;
        Self {
            header,
            enable,
            check_sum,
        }
    }
}

/// Request message for the I2C slave address
#[derive(Clone, Debug, Format, Immutable, IntoBytes, Unaligned)]
#[repr(C)]
pub struct RequestI2cSlaveAddr {
    header: RequestHeader,
    i2c_slave_addr: u8,
    check_sum: u8,
}

impl RequestI2cSlaveAddr {
    #[allow(missing_docs)]
    pub fn new(i2c_slave_addr: u8) -> Self {
        let header = RequestHeader::new(LidarRequestId::I2cSlaveAddr);
        let check_sum = header.sum().overflowing_add(i2c_slave_addr).0;
        Self {
            header,
            i2c_slave_addr,
            check_sum,
        }
    }
}

/// Request message to restore default settings
#[derive(Clone, Debug, Format, Immutable, IntoBytes, Unaligned)]
#[repr(C)]
pub struct RequestRestoreDefault {
    header: RequestHeader,
    check_sum: u8,
}

impl RequestRestoreDefault {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        let header = RequestHeader::new(LidarRequestId::RestoreDefault);
        let check_sum = header.sum();
        Self { header, check_sum }
    }
}

/// Request to save settings
#[derive(Clone, Debug, Format, Immutable, IntoBytes, Unaligned)]
#[repr(C)]
pub struct RequestSaveSettings {
    header: RequestHeader,
    check_sum: u8,
}

impl RequestSaveSettings {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        let header = RequestHeader::new(LidarRequestId::SaveSettings);
        let check_sum = header.sum();
        Self { header, check_sum }
    }
}

// Request product bar code
#[derive(Clone, Debug, Format, Immutable, IntoBytes, Unaligned)]
#[repr(C)]
#[allow(missing_docs)]
pub struct RequestReadManuBin {
    header: RequestHeader,
    check_sum: u8,
}

impl RequestReadManuBin {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        let header = RequestHeader::new(LidarRequestId::ReadManuBin);
        let check_sum = header.sum();
        Self { header, check_sum }
    }
}

/// Request full version number
#[derive(Clone, Debug, Format, Immutable, IntoBytes, Unaligned)]
#[repr(C)]
pub struct RequestGetFullVersion {
    header: RequestHeader,
    check_sum: u8,
}

impl RequestGetFullVersion {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        let header = RequestHeader::new(LidarRequestId::GetFullVersion);
        let check_sum = header.sum();
        Self { header, check_sum }
    }
}

// Request amp threshold
#[derive(Clone, Debug, Format, Immutable, IntoBytes, Unaligned)]
#[repr(C)]
#[allow(missing_docs)]
pub struct RequestAmpThreshold {
    header: RequestHeader,
    amp_threshold: u8,
    dummy_dist: U16LE,
    check_sum: u8,
}

impl RequestAmpThreshold {
    #[allow(missing_docs)]
    pub fn new(amp_threshold: u8, dummy_dist: u16) -> Self {
        let header = RequestHeader::new(LidarRequestId::AmpThreshold);
        let check_sum = header.sum();
        Self {
            header,
            amp_threshold,
            dummy_dist: dummy_dist.into(),
            check_sum,
        }
    }
}

/// Request message to set timestamp to specific value `std`
#[derive(Clone, Debug, Format, Immutable, IntoBytes, Unaligned)]
#[repr(C)]
pub struct RequestTimestampSync {
    header: RequestHeader,
    std: U32LE,
    check_sum: u8,
}

impl RequestTimestampSync {
    #[allow(missing_docs)]
    pub fn new(std: u32) -> Self {
        let header = RequestHeader::new(LidarRequestId::TimestampSync);
        let check_sum = header.sum();
        Self {
            header,
            std: std.into(),
            check_sum,
        }
    }
}

/// Request to set low-power mode
#[derive(Clone, Debug, Format, Immutable, IntoBytes, Unaligned)]
#[repr(C)]
pub struct RequestLowConsumption {
    header: RequestHeader,
    sample_rate: U16LE,
    check_sum: u8,
}

impl RequestLowConsumption {
    #[allow(missing_docs)]
    pub fn new(sample_rate: u16) -> Self {
        let header = RequestHeader::new(LidarRequestId::LowConsumption);
        let check_sum = header
            .sum()
            .overflowing_add(
                sample_rate
                    .as_bytes()
                    .iter()
                    .fold(0, |old: u8, x: &u8| old.overflowing_add(*x).0),
            )
            .0;
        Self {
            header,
            sample_rate: sample_rate.into(),
            check_sum,
        }
    }
}

/// Request to set minimum and maximum distance output.
/// Optionally via `silence`, disable output if data crosses the boundaries
#[derive(Clone, Debug, Format, Immutable, IntoBytes, Unaligned)]
#[repr(C)]
pub struct RequestDistLimit {
    header: RequestHeader,
    dist_min: U16LE,
    dist_max: U16LE,
    silence: u8,
    check_sum: u8,
}

impl RequestDistLimit {
    #[allow(missing_docs)]
    pub fn new(dist_min: u16, dist_max: u16, silence: u8) -> Self {
        let header = RequestHeader::new(LidarRequestId::DistLimit);
        let check_sum = header
            .sum()
            .overflowing_add(
                dist_min
                    .as_bytes()
                    .iter()
                    .fold(0, |old: u8, x: &u8| old.overflowing_add(*x).0),
            )
            .0
            .overflowing_add(
                dist_max
                    .as_bytes()
                    .iter()
                    .fold(0, |old: u8, x: &u8| old.overflowing_add(*x).0),
            )
            .0
            .overflowing_add(silence)
            .0;
        Self {
            header,
            dist_min: dist_min.into(),
            dist_max: dist_max.into(),
            silence,
            check_sum,
        }
    }
}

/// Request to use On-Off mode via additional pin (e.g. GPIO).
/// On-Off mode signals if object is within a zone.
/// See manual for further details.
#[derive(Clone, Debug, Format, Immutable, IntoBytes, Unaligned)]
#[repr(C)]
pub struct RequestOnOffMode {
    header: RequestHeader,
    mode: u8,
    dist: U16LE,
    zone: U16LE,
    delay_1: U16LE,
    delay_2: U16LE,
    check_sum: u8,
}

impl RequestOnOffMode {
    #[allow(missing_docs)]
    pub fn new(mode: u8, dist: u16, zone: u16, delay_1: u16, delay_2: u16) -> Self {
        let header = RequestHeader::new(LidarRequestId::OnOffMode);
        let check_sum = header
            .sum()
            .overflowing_add(mode)
            .0
            .overflowing_add(
                dist.as_bytes()
                    .iter()
                    .fold(0, |old: u8, x: &u8| old.overflowing_add(*x).0),
            )
            .0
            .overflowing_add(
                zone.as_bytes()
                    .iter()
                    .fold(0, |old: u8, x: &u8| old.overflowing_add(*x).0),
            )
            .0
            .overflowing_add(
                delay_1
                    .as_bytes()
                    .iter()
                    .fold(0, |old: u8, x: &u8| old.overflowing_add(*x).0),
            )
            .0
            .overflowing_add(
                delay_2
                    .as_bytes()
                    .iter()
                    .fold(0, |old: u8, x: &u8| old.overflowing_add(*x).0),
            )
            .0;
        Self {
            header,
            mode,
            dist: dist.into(),
            zone: zone.into(),
            delay_1: delay_1.into(),
            delay_2: delay_2.into(),
            check_sum,
        }
    }
}

/// Request to use low sample rate. `output_period_s`>0 sets the periodic sample rate
/// with rates greater than 1Hz. `one_shot_frame`>0 sets period (s) during which the average
/// distance is measured, after each `output_period_s`.
#[derive(Clone, Debug, Format, Immutable, IntoBytes, Unaligned)]
#[repr(C)]
pub struct RequestLowSampleRate {
    header: RequestHeader,
    output_period_s: U32LE,
    one_shot_frames: U32LE,
    check_sum: u8,
}

impl RequestLowSampleRate {
    #[allow(missing_docs)]
    pub fn new(output_period_s: u32, one_shot_frames: u32) -> Self {
        let header = RequestHeader::new(LidarRequestId::LowSampleRate);
        let check_sum = header
            .sum()
            .overflowing_add(
                output_period_s
                    .as_bytes()
                    .iter()
                    .fold(0, |old: u8, x: &u8| old.overflowing_add(*x).0),
            )
            .0
            .overflowing_add(
                one_shot_frames
                    .as_bytes()
                    .iter()
                    .fold(0, |old: u8, x: &u8| old.overflowing_add(*x).0),
            )
            .0;
        Self {
            header,
            output_period_s: output_period_s.into(),
            one_shot_frames: one_shot_frames.into(),
            check_sum,
        }
    }
}

#[derive(Clone, Debug, Format, Immutable, IntoBytes, Unaligned)]
#[repr(C)]
/// Request the Config Parameter for a given Request ID `id_input`
pub struct RequestGetConfigPara {
    header: RequestHeader,
    id_input: LidarRequestId,
    check_sum: u8,
}

/// Request Config parameter via message id `id_input`
impl RequestGetConfigPara {
    #[allow(missing_docs)]
    pub fn new(id_input: LidarRequestId) -> Self {
        let header = RequestHeader::new(LidarRequestId::GetConfigPara);
        let check_sum = header.sum().overflowing_add(id_input as u8).0;
        Self {
            header,
            id_input,
            check_sum,
        }
    }
}

/// Request Message sent to the Lidar. See Tf Luna manual
#[derive(Clone, Debug, Format)]
#[allow(missing_docs)]
pub enum LidarRequest {
    GetVersion(RequestGetVersion),
    SoftReset(RequestSoftReset),
    SampleFreq(RequestSampleFreq),
    SampleTrig(RequestSampleTrig),
    OutputFormat(RequestOutputFormat),
    BaudRate(RequestBaudRate),
    OutputEn(RequestOutputEn),
    FrameChecksumEn(RequestFrameChecksumEn),
    I2cSlaveAddr(RequestI2cSlaveAddr),
    RestoreDefault(RequestRestoreDefault),
    SaveSettings(RequestSaveSettings),
    ReadManuBin(RequestReadManuBin),
    GetFullVersion(RequestGetFullVersion),
    AmpThreshold(RequestAmpThreshold),
    TimestampSync(RequestTimestampSync),
    LowConsumption(RequestLowConsumption),
    DistLimit(RequestDistLimit),
    OnOffMode(RequestOnOffMode),
    LowSampleRate(RequestLowSampleRate),
    GetConfigPara(RequestGetConfigPara),
}

/// Overflowing sum of a byte array
///
/// Used to calculate the checksum 
pub(crate) fn overflowing_sum(bytes: &[u8]) -> u8 {
    bytes
        .iter()
        .fold(0, |old: u8, x: &u8| old.overflowing_add(*x).0)
}

#[cfg(test)]
pub mod tests {
    use zerocopy::IntoBytes;

    use crate::message::message_request::{
        RequestBaudRate, RequestFrameChecksumEn, RequestGetVersion, RequestI2cSlaveAddr,
        RequestOutputEn, RequestOutputFormat, RequestRestoreDefault, RequestSampleFreq,
        RequestSampleTrig, RequestSoftReset, overflowing_sum,
    };

    #[test]
    pub fn test_overflowing_sum() {
        let sum = 1;
        let bytes = [0xFF, 2];

        assert_eq!(overflowing_sum(&bytes), sum);
    }
    #[test]
    pub fn test_request_message_get_version() {
        let mut byte_slice = [0u8; 4];
        RequestGetVersion::new().write_to(&mut byte_slice);

        let byte_slice_cmp = [0x5A, 0x4, 0x1, 0x5F];

        assert_eq!(byte_slice, byte_slice_cmp);
    }
    #[test]
    pub fn test_request_message_soft_reset() {
        let mut byte_slice = [0u8; 4];
        RequestSoftReset::new().write_to(&mut byte_slice);

        let byte_slice_cmp = [0x5A, 0x4, 0x2, 0x60];

        assert_eq!(byte_slice, byte_slice_cmp);
    }
    #[test]
    pub fn test_request_message_sample_freq() {
        let mut byte_slice = [0u8; 6];
        RequestSampleFreq::new(10).write_to(&mut byte_slice);

        let byte_slice_cmp = [0x5A, 0x6, 0x3, 0xA, 0x0, 0x6D];

        assert_eq!(byte_slice, byte_slice_cmp);
    }
    #[test]
    pub fn test_request_message_sample_trig() {
        let mut byte_slice = [0u8; 4];
        RequestSampleTrig::new().write_to(&mut byte_slice);

        let byte_slice_cmp = [0x5A, 0x4, 0x4, 0x62];

        assert_eq!(byte_slice, byte_slice_cmp);
    }
    #[test]
    pub fn test_request_message_output_format() {
        let mut byte_slice = [0u8; 5];
        RequestOutputFormat::new(crate::handler::LidarSettingOutput::NineBytePerCm)
            .write_to(&mut byte_slice);

        let byte_slice_cmp = [0x5A, 0x5, 0x5, 0x01, 0x65];

        assert_eq!(byte_slice, byte_slice_cmp);
    }
    #[test]
    pub fn test_request_message_id_baud_rate() {
        let mut byte_slice = [0u8; 8];
        RequestBaudRate::new(115200).write_to(&mut byte_slice);

        let byte_slice_cmp = [0x5A, 0x08, 0x06, 0x00, 0xC2, 0x01, 0x0, 0x2B];

        assert_eq!(byte_slice, byte_slice_cmp);
    }
    #[test]
    pub fn test_request_message_output_en() {
        let mut byte_slice = [0u8; 5];
        RequestOutputEn::new(true).write_to(&mut byte_slice);

        let byte_slice_cmp = [0x5A, 0x05, 0x07, 0x01, 0x67];

        assert_eq!(byte_slice, byte_slice_cmp);
    }
    #[test]
    pub fn test_request_frame_checksum_en() {
        let mut byte_slice = [0u8; 5];
        RequestFrameChecksumEn::new(true).write_to(&mut byte_slice);

        let byte_slice_cmp = [0x5A, 0x05, 0x08, 0x01, 0x68];

        assert_eq!(byte_slice, byte_slice_cmp);
    }
    #[test]
    pub fn test_request_i2c_slave_address() {
        let mut byte_slice = [0u8; 5];
        RequestI2cSlaveAddr::new(32).write_to(&mut byte_slice);

        let byte_slice_cmp = [0x5A, 0x05, 0x0B, 0x20, 0x8A];

        assert_eq!(byte_slice, byte_slice_cmp);
    }
    #[test]
    pub fn test_request_restore_default() {
        let mut byte_slice = [0u8; 4];
        RequestRestoreDefault::new().write_to(&mut byte_slice);

        let byte_slice_cmp = [0x5A, 0x04, 0x10, 0x6E];

        assert_eq!(byte_slice, byte_slice_cmp);
    }
}
