//! Handles printing to VGA

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
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
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
                let colour_code = self.colour_code;

                self.buffer.chars[row][col] = ScreenChar {
                    ascii_char: byte,
                    colour_code,
                };
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
    fn new_line(&mut self) {
        //
    }
}

/// Tests printing of characters to screen
pub fn chekout_print() -> () {
    let mut writer = ScreenWriter {
        column_position: 0,
        colour_code: ColourCode::new(Colour::Green, Colour::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'I');
    writer.write_string("chigyIchigy");
    writer.write_string(" gū");
}
