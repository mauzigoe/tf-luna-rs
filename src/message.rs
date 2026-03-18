use defmt::Format;

/// The largest possible message sent to the lidar
pub const INPUT_MESSAGE_MAX_LEN: usize = 0x13;
/// The largest possible message sent from the lidar
pub const OUTPUT_MESSAGE_MAX_LEN: usize = 0x30;

use message_data::LidarData;
use message_request::LidarRequest;
use message_response::LidarResponse;

use zerocopy_derive::{Immutable, KnownLayout, TryFromBytes};

/// Wrapper defintion for `defmt` + `zerocopy`
pub mod endian;
/// message defintions for lidar data 
pub mod message_data;
/// message defintions for lidar requests
pub mod message_request;
/// message defintions for lidar responses
pub mod message_response;

/// Byte header value for response messages
pub const RESPONSE_BYTE: u8 = 0x5A;
/// Byte header value for data messages
pub const DATA_BYTE: u8 = 0x59;

#[derive(Clone, Copy, TryFromBytes, KnownLayout, Immutable)]
/// Struct to identify the start of a data or response message
pub struct LidarOutputHeader(pub u8, pub u8);

#[derive(Clone, Debug, Format)]
/// Variant of lidar messages sent by the lidar
pub enum LidarOutput {
    /// Lidar Data message
    Data(LidarData),
    /// Lidar Response message
    Response(LidarResponse),
}

#[derive(Debug, Format)]
/// Variants of lidar messages sent to the lidar
pub enum LidarInput {
    /// Lidar Request message
    Request(LidarRequest),
}

/// Return if the request sent to the lidar expects an response
pub(crate) fn waits_for_response(req: &LidarRequest) -> bool {
    match req {
        LidarRequest::SampleTrig(_) => false,
        LidarRequest::TimestampSync(_) => false,
        LidarRequest::OnOffMode(_) => false,
        LidarRequest::GetConfigPara(_) => false,
        _ => true,
    }
}

/// Check if reqeuest matches the response as expected
/// TODO: Change in the future as method for [LidarRequest] with additional check for content
pub(crate) fn req_matches_res(req: &LidarRequest, res: &LidarResponse) -> bool {
    match (req, res) {
        (LidarRequest::GetVersion(_), LidarResponse::GetVersion(_)) => true,
        (LidarRequest::SoftReset(_), LidarResponse::SoftReset(_)) => true,
        (LidarRequest::SampleFreq(_), LidarResponse::SampleFreq(_)) => true,
        (LidarRequest::SampleTrig(_), LidarResponse::SampleTrig(_)) => true,
        (LidarRequest::OutputFormat(_), LidarResponse::OutputFormat(_)) => true,
        (LidarRequest::BaudRate(_), LidarResponse::BaudRate(_)) => true,
        (LidarRequest::OutputEn(_), LidarResponse::OutputEn(_)) => true,
        (LidarRequest::FrameChecksumEn(_), LidarResponse::FrameChecksumEn(_)) => true,
        (LidarRequest::RestoreDefault(_), LidarResponse::RestoreDefault(_)) => true,
        (LidarRequest::SaveSettings(_), LidarResponse::SaveSettings(_)) => true,
        (LidarRequest::GetFullVersion(_), LidarResponse::GetFullVersion(_)) => true,
        (LidarRequest::AmpThreshold(_), LidarResponse::AmpThreshold(_)) => true,
        (LidarRequest::LowConsumption(_), LidarResponse::LowConsumption(_)) => true,
        (LidarRequest::DistLimit(_), LidarResponse::DistLimit(_)) => true,
        (LidarRequest::LowSampleRate(_), LidarResponse::LowSampleRate(_)) => true,
        _ => false,
    }
}
