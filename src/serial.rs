//! Initializes and handles sending of data to the
//! serial port

use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    pub static ref SERIAL_A: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x358) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}
