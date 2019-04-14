use core::ptr;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
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

impl Color {
    pub fn GenColorCode(f: Color, b: Color) -> u8 {
        return ((b as u8) << 4) | (f as u8);
    }
}

struct Char {
    ch: u8,
    color_code: u8,
}

const M: usize = 25;
const N: usize = 80;


#[repr(transparent)]
pub struct Buffer {
    chars: [[Char; N]; M],
}

pub struct Writer {
    col: usize,
    raw: usize,
    color_code: u8,
    buffer: &'static mut Buffer,
}

impl Writer {

    pub fn new(front: Color, back: Color) -> Self {
        Self {
            col: 0,
            raw: 0,
            color_code: Color::GenColorCode(Color::Green, Color::Black),
            buffer: unsafe {&mut *(0xb8000 as *mut Buffer)},
        }
    }

    fn new_line(&mut self) {
        self.raw = (self.raw + 1) % M;
        self.col = 0;
    }

    fn next(&mut self) {
        self.col = (self.col + 1) % N;
        if self.col == 0 {
            self.raw = (self.raw + 1) % M;
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            _ => {
                let mut p = (&mut self.buffer.chars[self.raw][self.col]) as *mut Char;
                unsafe {
                    ptr::write_volatile(p, Char{color_code: self.color_code, ch: byte});
                }
                self.next();
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20...0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
}