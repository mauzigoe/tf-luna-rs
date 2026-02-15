use defmt::Format;

#[derive(Clone, Debug, Format)]
pub enum LidarError {
    RequestFailed,
    WriteInterfaceNotInitialized,
}
