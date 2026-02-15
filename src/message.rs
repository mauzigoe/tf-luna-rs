use defmt::Format;

pub const INPUT_MESSAGE_MAX_LEN: usize = 0x13;
pub const OUTPUT_MESSAGE_MAX_LEN: usize = 0x30;

use message_data::LidarData;
use message_request::LidarRequest;
use message_response::LidarResponse;

use zerocopy_derive::{Immutable, KnownLayout, TryFromBytes};

pub mod endian;
pub mod message_data;
pub mod message_request;
pub mod message_response;

pub const RESPONSE_BYTE: u8 = 0x5A;
pub const DATA_BYTE: u8 = 0x59;

#[derive(Clone, Copy, TryFromBytes, KnownLayout, Immutable)]
pub struct LidarOutputHeader(pub u8, pub u8);

#[derive(Clone, Debug, Format)]
pub enum LidarOutput {
    Data(LidarData),
    Response(LidarResponse),
}

#[derive(Debug, Format)]
pub enum LidarInput {
    Request(LidarRequest),
}

pub fn waits_for_response(req: &LidarRequest) -> bool {
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
pub fn req_matches_res(req: &LidarRequest, res: &LidarResponse) -> bool {
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
