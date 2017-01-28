#![feature(lang_items)]
/* Disable stdlib */
#![no_std]
#![feature(unique)]
#![feature(const_fn)]
extern crate rlibc;
extern crate volatile;
extern crate spin;
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

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt() -> ! {
    loop {}
}
