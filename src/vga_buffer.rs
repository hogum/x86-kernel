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
        ColourCode(background as u8) << 4 | (foreground as u8)
    }
}
