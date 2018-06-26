#![feature(lang_items, start)]
#![no_std]

extern crate libc;

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    const HELLO: &'static str = "Hello, world!\n";
    unsafe {
        libc::write(libc::STDOUT_FILENO, HELLO.as_ptr() as *const _, HELLO.len());
    }
    0
}
