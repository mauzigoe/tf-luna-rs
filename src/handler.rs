use core::cell::RefCell;

use circular_buffer::CircularBuffer;
use critical_section::Mutex;
use defmt::println;
use embedded_io::Write;
use zerocopy::{FromBytes, IntoBytes, TryFromBytes};

use crate::{
    error::LidarError,
    message::{
        DATA_BYTE, LidarOutput, LidarOutputHeader, RESPONSE_BYTE,
        message_data::{
            EightBytePerCm, FourByteWithTimestamp, LidarData, NineBytePerCm, NineBytePerMm,
            OutputWithDeviceId,
        },
        message_request::LidarRequest,
        message_response::{
            LidarResponse, ResponseBaudRate, ResponseFrameChecksumEn, ResponseGetFullVersion,
            ResponseGetVersion, ResponseI2cSlaveAddress, ResponseIdType, ResponseOutputEn,
            ResponseOutputFormat, ResponseRestoreDefault, ResponseSampleFreq, ResponseSampleTrig,
            ResponseSaveSettings, ResponseSoftReset,
        },
        req_matches_res, waits_for_response,
    },
};

/// Configuration of the Tf Luna and how distance data is sent
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LidarSettingOutput {
    /// Tf Luna sends the distance (cm) and meta data (temp, amp) prepended with 3 byte header
    NineBytePerCm = 0x01,
    /// Tf Luna sends the distance (cm) data encoded as float string (Foramt: "X.YZ\r\n")
    Pix = 0x02,
    /// Tf Luna sends the distance (mm) and meta data (temp, amp) prepended with 3 byte header
    NineBytePerMm = 0x06,
    /// Tf Luna sends the distance (mm) and meta data (temp, timestamp) prepended with 3 byte header
    FourByteWithTimestamp = 0x07,
    //IdZeroOutput,
    /// Tf Luna sends the distance (cm) and meta data (amp, timestamp) prepended with 3 byte header
    EightBytePerCm = 0x09,
    /// Tf Luna sends the distance (mm) and meta data (temp, timestamp, device id) prepended with 3 byte header
    OutputWithDeviceId = 0x0A,
}

/// Current used lidar setting
struct LidarSettings {
    output: LidarSettingOutput,
}

/// Handler to interact with the TF Luna Lidar Sensor
pub struct TfLunaDriver<'a, const READ_BUF_SIZE: usize, WriteBuf: Write + 'a> {
    /// Store of TfLuna Lidar Driver Settings
    settings: LidarSettings,
    /// Interface to write peripheral buffer driver (uart/i2c)
    write_interface: &'a Mutex<RefCell<Option<WriteBuf>>>,
    /// Interface to read from received uart/i2c inputs (uart/i2c)
    // TODO: Replace with Read Interface
    read_buf: &'a Mutex<RefCell<CircularBuffer<READ_BUF_SIZE, u8>>>,
}

impl<'a, const READ_BUF_SIZE: usize, WriteBuf: Write + 'a> TfLunaDriver<'a, READ_BUF_SIZE, WriteBuf> {
    /// Return a lidar handler
    pub fn new(
        read_buf: &'a Mutex<RefCell<CircularBuffer<READ_BUF_SIZE, u8>>>,
        write_interface: &'a Mutex<RefCell<Option<WriteBuf>>>,
    ) -> Self {
        let settings = LidarSettings {
            output: LidarSettingOutput::NineBytePerCm,
        };
        Self {
            settings,
            read_buf,
            write_interface,
        }
    }
    fn length_of_read_buffer(&self) -> usize {
        critical_section::with(|cs| self.read_buf.borrow_ref(cs).len())
    }
    fn read_next_response_inner(id: ResponseIdType, read_buf: &[u8]) -> Option<LidarResponse> {
        match id {
            ResponseIdType::GetVersion => ResponseGetVersion::read_from_bytes(read_buf)
                .ok()
                .and_then(|x| Some(LidarResponse::GetVersion(x))),
            ResponseIdType::SoftReset => ResponseSoftReset::read_from_bytes(read_buf)
                .ok()
                .and_then(|x| Some(LidarResponse::SoftReset(x))),
            ResponseIdType::SampleFreq => ResponseSampleFreq::read_from_bytes(read_buf)
                .ok()
                .and_then(|x| Some(LidarResponse::SampleFreq(x))),
            ResponseIdType::SampleTrig => ResponseSampleTrig::read_from_bytes(read_buf)
                .ok()
                .and_then(|x| Some(LidarResponse::SampleTrig(x))),
            ResponseIdType::OutputFormat => ResponseOutputFormat::read_from_bytes(read_buf)
                .ok()
                .and_then(|x| Some(LidarResponse::OutputFormat(x))),
            ResponseIdType::BaudRate => ResponseBaudRate::read_from_bytes(read_buf)
                .ok()
                .and_then(|x| Some(LidarResponse::BaudRate(x))),
            ResponseIdType::OutputEn => ResponseOutputEn::read_from_bytes(read_buf)
                .ok()
                .and_then(|x| Some(LidarResponse::OutputEn(x))),
            ResponseIdType::FrameChecksumEn => ResponseFrameChecksumEn::read_from_bytes(read_buf)
                .ok()
                .and_then(|x| Some(LidarResponse::FrameChecksumEn(x))),
            ResponseIdType::I2cSlaveAddress => ResponseI2cSlaveAddress::read_from_bytes(read_buf)
                .ok()
                .and_then(|x| Some(LidarResponse::I2cSlaveAddress(x))),
            ResponseIdType::RestoreDefault => ResponseRestoreDefault::read_from_bytes(read_buf)
                .ok()
                .and_then(|x| Some(LidarResponse::RestoreDefault(x))),
            ResponseIdType::SaveSettings => ResponseSaveSettings::read_from_bytes(read_buf)
                .ok()
                .and_then(|x| Some(LidarResponse::SaveSettings(x))),
            ResponseIdType::GetFullVersion => ResponseGetFullVersion::read_from_bytes(read_buf)
                .ok()
                .and_then(|x| Some(LidarResponse::GetFullVersion(x))),
            _ => None,
        }
    }
    fn read_next_data_inner(
        setting: LidarSettingOutput,
        read_buf: &[u8],
    ) -> Option<(LidarData, usize)> {
        match setting {
            LidarSettingOutput::NineBytePerCm => NineBytePerCm::try_read_from_prefix(read_buf)
                .ok()
                .map(|(x, y)| (LidarData::NineBytePerCm(x), y.len())),
            LidarSettingOutput::NineBytePerMm => NineBytePerMm::try_read_from_prefix(read_buf)
                .ok()
                .map(|(x, y)| (LidarData::NineBytePerMm(x), y.len())),
            LidarSettingOutput::FourByteWithTimestamp => {
                FourByteWithTimestamp::try_read_from_prefix(read_buf)
                    .ok()
                    .map(|(x, y)| (LidarData::FourByteWithTimestamp(x), y.len()))
            }
            LidarSettingOutput::EightBytePerCm => EightBytePerCm::try_read_from_prefix(read_buf)
                .ok()
                .map(|(x, y)| (LidarData::EightBytePerCm(x), y.len())),
            LidarSettingOutput::OutputWithDeviceId => {
                OutputWithDeviceId::try_read_from_prefix(read_buf)
                    .ok()
                    .map(|(x, y)| (LidarData::OutputWithDeviceId(x), y.len()))
            }
            _ => None,
        }
    }
    /// Send an requets to the lidar and (blocking) wait a response.
    ///
    /// While waiting received lidar data is discarded
    pub fn make_request(
        &mut self,
        lidar_request: LidarRequest,
    ) -> Result<Option<LidarResponse>, LidarError> {
        let req_bytes = match &lidar_request {
            LidarRequest::GetVersion(req) => req.as_bytes(),
            LidarRequest::SoftReset(req) => req.as_bytes(),
            LidarRequest::SampleFreq(req) => req.as_bytes(),
            LidarRequest::SampleTrig(req) => req.as_bytes(),
            LidarRequest::OutputFormat(req) => req.as_bytes(),
            LidarRequest::BaudRate(req) => req.as_bytes(),
            LidarRequest::OutputEn(req) => req.as_bytes(),
            LidarRequest::FrameChecksumEn(req) => req.as_bytes(),
            LidarRequest::I2cSlaveAddr(req) => req.as_bytes(),
            LidarRequest::RestoreDefault(req) => req.as_bytes(),
            LidarRequest::SaveSettings(req) => req.as_bytes(),
            LidarRequest::ReadManuBin(req) => req.as_bytes(),
            LidarRequest::GetFullVersion(req) => req.as_bytes(),
            LidarRequest::AmpThreshold(req) => req.as_bytes(),
            LidarRequest::TimestampSync(req) => req.as_bytes(),
            LidarRequest::LowConsumption(req) => req.as_bytes(),
            LidarRequest::DistLimit(req) => req.as_bytes(),
            LidarRequest::OnOffMode(req) => req.as_bytes(),
            LidarRequest::LowSampleRate(req) => req.as_bytes(),
            LidarRequest::GetConfigPara(req) => req.as_bytes(),
        };

        critical_section::with(|cs| {
            if let Some(write_interface) = self.write_interface.borrow_ref_mut(cs).as_mut() {
                write_interface
                    .write(req_bytes)
                    .or(Err(LidarError::RequestFailed))
            } else {
                Err(LidarError::WriteInterfaceNotInitialized)
            }
        })?;

        if waits_for_response(&lidar_request) {
            loop {
                if let Some(LidarOutput::Response(lidar_response)) = self.read_next() {
                    if req_matches_res(&lidar_request, &lidar_response) {
                        println!("found");
                        return Ok(Some(lidar_response));
                    }
                }
            }
        };
        Ok(None)
    }
    /// Read next received message from a circular buffer
    pub fn read_next(&mut self) -> Option<LidarOutput> {
        if self.length_of_read_buffer() < 8 {
            return None;
        }
        critical_section::with(|cs| {
            let mut read_buf = self.read_buf.borrow_ref_mut(cs);
            read_buf.make_contiguous();
            let (read_buf_slice_first, _) = read_buf.as_slices();
            match LidarOutputHeader::try_read_from_prefix(read_buf_slice_first) {
                Ok((LidarOutputHeader(DATA_BYTE, DATA_BYTE), rest)) => {
                    let (data, size) =
                        Self::read_next_data_inner(self.settings.output.clone(), rest)?;
                    read_buf.drain(0..size);
                    Some(LidarOutput::Data(data))
                }
                Ok((LidarOutputHeader(RESPONSE_BYTE, len), rest)) => {
                    println!("Response");
                    let (id, rest) = ResponseIdType::try_read_from_prefix(rest).ok()?;
                    let data = Self::read_next_response_inner(id, rest)?;
                    let ret = Some(LidarOutput::Response(data));
                    read_buf.drain(0..len as usize);
                    ret
                }
                _ => {
                    println!("Drain");
                    read_buf.drain(0..1 as usize);
                    return None;
                }
            }
        })
    }
}
