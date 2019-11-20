//! Initializes and handles sending of data to the
//! serial port

use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    pub static ref SERIAL_A: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3f8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) -> () {
    use core::fmt::Write;
    SERIAL_A
        .lock()
        .write_fmt(args)
        .expect("Unable to print to serial");
}

/// Prints(host) through the serial interface
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

/// Prints with appending new line to serial
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($args:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($args)*
    ));

}
