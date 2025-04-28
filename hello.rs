#![feature(c_size_t)]
#![no_std]

use panic_abort as _;
use core::ffi::{c_size_t, c_char, c_int, c_uint, c_ulong};

const BOTTOM: u8 = 63;
const RIGHT_END: u8 = 127;

const CHAR_WIDTH: u8 = 6;
const CHAR_HEIGHT: u8 = 8;

const MSG: *const c_char = b"Hello, Rust!\0" as *const u8 as *const c_char;

struct Environment {
    x: u8,
    y: u8,
    msg_len: u8
}

impl Environment {
    fn setup(&mut self) {
        begin_no_logo();
        set_frame_rate(30);
        let msg_len = strlen(MSG);
        debug_assert!(msg_len <= (core::u8::MAX as c_size_t));
        self.msg_len = msg_len as u8;
    }

    fn loop_(&mut self) {
        if !next_frame() {
            return;
        }

        if UP.pressed() && self.y > 0 {
            self.y -= 1;
        }
        if RIGHT.pressed() && self.x < RIGHT_END - CHAR_WIDTH * self.msg_len {
            self.x += 1;
        }
        if LEFT.pressed() && self.x > 0 {
            self.x -= 1;
        }
        if DOWN.pressed() && self.y < BOTTOM - CHAR_HEIGHT {
            self.y += 1;
        }

        if (A | B).pressed() {
            tone(0xff, 0x3f);
        }

        clear();
        set_cursor(self.x, self.y);
        print(MSG);
        display();
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct ButtonSet {
    flag_set: u8
}

impl ButtonSet {
    #[inline(always)]
    fn pressed(&self) -> bool {
        pressed(self.flag_set)
    }
}

impl core::ops::BitOr for ButtonSet {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, other: Self) -> Self {
        Self { flag_set: self.flag_set | other.flag_set }
    }
}

const UP:    ButtonSet = ButtonSet { flag_set: 0b10000000 };
const RIGHT: ButtonSet = ButtonSet { flag_set: 0b01000000 };
const LEFT:  ButtonSet = ButtonSet { flag_set: 0b00100000 };
const DOWN:  ButtonSet = ButtonSet { flag_set: 0b00010000 };
const A:     ButtonSet = ButtonSet { flag_set: 0b00001000 };
const B:     ButtonSet = ButtonSet { flag_set: 0b00000100 };

static mut E: Environment = Environment {
    x: 0,
    y: 0,
    msg_len: 0
};

#[unsafe(no_mangle)]
pub extern "C" fn setup() {
    unsafe { (&mut *&raw mut E).setup(); }
}

#[unsafe(export_name = "loop")]
pub extern "C" fn loop_() {
    unsafe { (&mut *&raw mut E).loop_(); }
}

unsafe extern "C" {
    #[link_name = "strlen"]
    fn c_strlen(cstr: *const c_char) -> c_size_t;

    fn arduboy_begin_no_logo();
    fn arduboy_set_frame_rate(rate: u8);
    fn arduboy_next_frame() -> c_int;
    fn arduboy_clear();
    fn arduboy_set_cursor(x: i16, y: i16);
    fn arduboy_print(cstr: *const c_char);
    fn arduboy_display();
    fn arduboy_pressed(buttons: u8) -> c_int;

    fn tunes_tone(frequency: c_uint, duration: c_ulong);
}

fn strlen(cstr: *const c_char) -> c_size_t {
    unsafe { c_strlen(cstr) }
}

fn begin_no_logo() {
    unsafe { arduboy_begin_no_logo(); }
}

fn set_frame_rate(rate: u8) {
    unsafe { arduboy_set_frame_rate(rate); }
}

fn next_frame() -> bool {
    unsafe { arduboy_next_frame() != 0 }
}

fn clear() {
    unsafe { arduboy_clear(); }
}

fn set_cursor(x: u8, y: u8) {
    unsafe { arduboy_set_cursor(x as i16, y as i16); }
}

fn print(cstr: *const c_char) {
    unsafe { arduboy_print(cstr); }
}

fn display() {
    unsafe { arduboy_display(); }
}

fn pressed(buttons: u8) -> bool {
    unsafe { arduboy_pressed(buttons) != 0 }
}

fn tone(frequency: u16, duration: u16) {
    unsafe { tunes_tone(frequency as c_uint, duration as c_ulong); }
}
