use defmt::Format;

/// Error cases for the TF Luna Lidar Handler
#[derive(Clone, Debug, Format)]
pub enum LidarError {
    RequestFailed,
    WriteInterfaceNotInitialized,
}
