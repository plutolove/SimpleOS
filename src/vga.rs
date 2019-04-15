use core::ptr;
use core::sync::atomic::AtomicPtr;
use core::cell::Cell;

use lazy_static::lazy_static;

lazy_static!{
    pub static ref WRITER: AtomicPtr<Writer> = AtomicPtr::new(Cell::new(Writer::new(Color::Cyan, Color::Black)).as_ptr());
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
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
    color_code: u8,
    buffer: &'static mut Buffer,
}

impl Writer {

    pub fn new(front: Color, back: Color) -> Self {
        Self {
            col: 0,
            color_code: Color::GenColorCode(front, back),
            buffer: unsafe {&mut *(0xb8000 as *mut Buffer)},
        }
    }

    fn clear_raw(&mut self, raw: usize) {
        let blank = Char {
            ch: b' ',
            color_code: self.color_code,
        };
        for col in 0..N {
            self.write_volatile(raw, col, blank);
        }
    }

    fn new_line(&mut self) {
        for i in 1..M {
            for j in 0..N {
                self.write_volatile(i-1, j, self.read_volatile(i, j));
            }
        }
        self.clear_raw(M-1);
        self.col = 0;
    }


    fn read_volatile(&self, x: usize, y: usize) -> Char {
        let p = (&self.buffer.chars[x][y]) as *const Char;
        let ret: Char;
        unsafe {
            ret = ptr::read_volatile(p);
        }
        return ret;
    }

    fn write_volatile(&mut self, x: usize, y: usize, ch: Char) {
        let mut p = (&mut self.buffer.chars[x][y]) as *mut Char;
        unsafe {
            ptr::write_volatile(p, ch);
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            _ => {
                if self.col >= N {
                    self.new_line();
                }
                let raw = M - 1;
                self.write_volatile(raw, self.col, Char{color_code: self.color_code, ch: byte});
                self.col += 1;
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
        //pub static WRITER: AtomicPtr<Writer> = AtomicPtr::new(Cell::new(Writer::new(Color::Cyan, Color::Black)).as_ptr());

    }
}
