use core::cell::RefCell;

use critical_section::Mutex;
use embedded_io::Write;

use crate::error::LidarError;

pub fn interrupt_write_to_handler_buf<W: Write>(
    handler_read_buffer: &Mutex<RefCell<W>>,
    buf: &[u8],
) -> Result<(), LidarError> {
    critical_section::with(|cs| {
        let mut read_buf = handler_read_buffer.borrow_ref_mut(cs);
        read_buf.write_all(&buf)
    }).or(Err(LidarError::WriteToHandlerBufferFailed))
}
