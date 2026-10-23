pub const VGA_WIDTH: usize = 80;
pub const VGA_HEIGHT: usize = 25;
pub const VGA_BUFFER: *mut u16 = 0xb8000 as *mut u16;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

pub struct VgaWriter {
    row: usize,
    col: usize,
    color: u8,
}

impl VgaWriter {
    pub const fn new() -> Self {
        VgaWriter {
            row: 0,
            col: 0,
            color: ((Color::Black as u8) << 4) | (Color::LightCyan as u8),
        }
    }

    pub fn set_color(&mut self, fg: Color, bg: Color) {
        self.color = ((bg as u8) << 4) | (fg as u8);
    }

    pub unsafe fn clear(&mut self) {
        let blank = ((self.color as u16) << 8) | (b' ' as u16);
        for i in 0..(VGA_WIDTH * VGA_HEIGHT) {
            core::ptr::write_volatile(VGA_BUFFER.add(i), blank);
        }
        self.row = 0;
        self.col = 0;
    }

    pub unsafe fn write_char(&mut self, c: u8) {
        if c == b'\n' {
            self.row += 1;
            self.col = 0;
            return;
        }

        if self.col >= VGA_WIDTH {
            self.col = 0;
            self.row += 1;
        }
        if self.row >= VGA_HEIGHT {
            self.row = 0;
        }

        let offset = self.row * VGA_WIDTH + self.col;
        let entry = ((self.color as u16) << 8) | (c as u16);
        core::ptr::write_volatile(VGA_BUFFER.add(offset), entry);
        self.col += 1;
    }

    pub unsafe fn write_str(&mut self, s: &str) {
        for b in s.bytes() {
            self.write_char(b);
        }
    }
}
