//! Handles printing to VGA
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

lazy_static! {
    pub static ref WRITER: Mutex<ScreenWriter> = Mutex::new(ScreenWriter {
        column_position: 0,
        colour_code: ColourCode::new(Colour::Yellow, Colour::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}
/// Colour bit variants
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Colour {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
/// Represents a full foreground and background
/// colour byte
struct ColourCode(u8);

impl ColourCode {
    /// Creates a colourcode of given foreground
    /// and background colour
    fn new(foreground: Colour, background: Colour) -> ColourCode {
        ColourCode((background as u8) << 4 | (foreground as u8))
    }
}

/// Represents a Screen character
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // Field ordering undefined in Rust
struct ScreenChar {
    ascii_char: u8,
    colour_code: ColourCode,
}
const BUFFER_WIDTH: usize = 25;
const BUFFER_HEIGHT: usize = 80;
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// Writes to Screen
pub struct ScreenWriter {
    column_position: usize,
    colour_code: ColourCode,
    buffer: &'static mut Buffer,
}

impl ScreenWriter {
    /// Writes a single ASCII byte
    pub fn write_byte(&mut self, byte: u8) -> () {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_char: byte,
                    colour_code: self.colour_code,
                });
                self.column_position += 1;
            }
        }
    }
    pub fn write_string(&mut self, s: &str) -> () {
        for byte in s.bytes() {
            match byte {
                //printable byte or ASCII newline between space and ~
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // Not in ASCII printable range
                _ => self.write_byte(0xfe), // ■
            }
        }
    }
    /// Shifts row lines up by wrapping the current line
    /// Iterates over all characters, shifting them a row up
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 1..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
            self.clear_row(BUFFER_HEIGHT - 1);
            self.column_position = 0;
        }
    }
    fn clear_row(&mut self, row: usize) -> () {
        let blank = ScreenChar {
            ascii_char: b' ',
            colour_code: self.colour_code,
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

/// Implement formatting macros
/// to allow printing of types
impl fmt::Write for ScreenWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}
