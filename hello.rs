#![feature(lang_items, core_intrinsics)]
#![no_std]
use core::intrinsics;
use core::panic::PanicInfo;

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
    flag_set: u8,
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

// see https://gcc.gnu.org/wiki/avr-gcc#Type_Layout
#[allow(non_camel_case_types)] type c_size_t = u16;
#[allow(non_camel_case_types)] type c_char = i8;
#[allow(non_camel_case_types)] type c_int = i16;
#[allow(non_camel_case_types)] type c_uint = u16;
#[allow(non_camel_case_types)] type c_ulong = u32;
#[allow(non_camel_case_types)] type uint8_t = u8;
#[allow(non_camel_case_types)] type int16_t = i16;

static mut E: Environment = Environment {
    x: 0,
    y: 0,
    msg_len: 0
};

#[no_mangle]
pub extern "C" fn setup() {
    unsafe { E.setup(); }
}

#[no_mangle]
#[export_name = "loop"]
pub extern "C" fn loop_() {
    unsafe { E.loop_(); }
}

extern "C" {
    #[link_name = "strlen"]
    fn c_strlen(cstr: *const c_char) -> c_size_t;

    fn arduboy_begin_no_logo();
    fn arduboy_set_frame_rate(rate: uint8_t);
    fn arduboy_next_frame() -> c_int;
    fn arduboy_clear();
    fn arduboy_set_cursor(x: int16_t, y: int16_t);
    fn arduboy_print(cstr: *const c_char);
    fn arduboy_display();
    fn arduboy_pressed(buttons: uint8_t) -> c_int;

    fn tunes_tone(frequency: c_uint, duration: c_ulong);
}

fn strlen(cstr: *const c_char) -> c_size_t {
    unsafe { c_strlen(cstr) }
}

fn begin_no_logo() {
    unsafe { arduboy_begin_no_logo(); }
}

fn set_frame_rate(rate: u8) {
    unsafe { arduboy_set_frame_rate(rate as uint8_t); }
}

fn next_frame() -> bool {
    unsafe { arduboy_next_frame() != 0 }
}

fn clear() {
    unsafe { arduboy_clear(); }
}

fn set_cursor(x: u8, y: u8) {
    unsafe { arduboy_set_cursor(x as int16_t, y as int16_t); }
}

fn print(cstr: *const c_char) {
    unsafe { arduboy_print(cstr); }
}

fn display() {
    unsafe { arduboy_display(); }
}

fn pressed(buttons: u8) -> bool {
    unsafe { arduboy_pressed(buttons as uint8_t) != 0 }
}

fn tone(frequency: u16, duration: u16) {
    unsafe { tunes_tone(frequency as c_uint, duration as c_ulong); }
}

#[lang = "eh_personality"]
fn rust_eh_personality() {
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    intrinsics::abort()
}
