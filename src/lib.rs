#![feature(lang_items)]
#![feature(panic_implementation)]
#![feature(core_intrinsics)]
#![feature(ptr_internals)]

/* Disable stdlib */
#![no_std]
#![feature(unique)]
#![feature(const_fn)]
extern crate rlibc;
extern crate volatile;
extern crate spin;
use core::intrinsics;
use core::panic::PanicInfo;
#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern "C" fn rust_main() {
    vga_buffer::clear_screen();
    println!("****************************\nWelcome to HeliOS\n**************************** \
              \neven though you cannot really do much for now..\n\n");
    println!("But good stuff is coming!", );

    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}
#[no_mangle]
pub extern "C" fn rust_begin_unwind() { }
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_implementation]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}