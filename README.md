# TF Luna Driver

Rust-based driver implementation for the [TF Luna Lidar](https://en.benewake.com/TFLuna/index.html) from Benewake. It offers the capability to decode the data and responses sent from the lidar.

## Interface

After initialization via `TfLunaDriver<_,_>::new()` the interface can be used via methods `make_request` to send configuration requests to the lidar or receive data via `read_next`

### read_next

Reads the data from a receive buffer (`&[u8]`) in a 'first-in-first-out' principle

### make_request

Sends a request to a transmit buffer (e.g. UART) and waits for a response message from the lidar. While waiting (blocking) it reads all the message in the receive buffer until it finds the suitable response message. While waiting data messages are discarded.

### Interrupts

To handle data received via UART/I2C, `crate::interrupts` contains an example how to pass received bytes from a UART/I2C buffer to the buffer referenced by the handler.
