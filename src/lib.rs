#![cfg_attr(not(test), no_std)]

//! Driver implementation for [TF Luna Lidar](https://en.benewake.com/uploadfiles/2024/04/20240426135946148.pdf)
//!
//! Features:
//! - Interface-agnostic: the driver is not implement for a specific UART/I2C driver, but expects the input via a shared read buffer
//! - Full implementation of the serial communication protocol (Appendix II). All messages are implemented as `struct`s and can be sent/received via the driver

/// Error Codes
pub mod error;
/// Handler implementation of the TF Luna Driver
pub mod handler;
/// Interrupt function declaration
pub mod interrupts;
/// Messages sent and received by the driver
pub mod message;
