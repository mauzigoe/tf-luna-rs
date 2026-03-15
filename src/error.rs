use defmt::Format;

/// Error cases for the TF Luna Lidar Handler
#[derive(Clone, Debug, Format)]
pub enum LidarError {
    /// Request to the Tf Luna Driver failed
    RequestFailed,
    /// Tf Luna's `W: Write` interface is not initaialized
    WriteInterfaceNotInitialized,
    /// Write to buffer of the TF Luna handler failed
    WriteToHandlerBufferFailed,
}
