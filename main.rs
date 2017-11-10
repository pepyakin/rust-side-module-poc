#![feature(lang_items)]
#![no_std]
#![no_main]

#[lang = "panic_fmt"]
pub fn panic_fmt(fmt: core::fmt::Arguments, _file_line: &(&'static str, u32)) -> !
{
    loop {};
}

#[lang = "eh_personality"] extern fn eh_personality() {}

#[repr(C)]
struct Args {
    a: u8,
    b: u8
}

extern "C" {
    fn derp(descriptor: *const Args) -> u8;
}

#[no_mangle]
pub fn takes_slice(a: &[u8]) -> u8 {
  a[228]
}

#[no_mangle]
pub fn call(descriptor: *const u8) -> u8 {
    unsafe {
        let args = Args {
            a: *descriptor,
            b: *descriptor.offset(1),
        };

        derp(&args as *const Args)
    }
}
