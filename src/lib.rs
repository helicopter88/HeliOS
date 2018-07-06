#![feature(lang_items)]
#![feature(panic_implementation)]
#![feature(ptr_internals)]

/* Disable stdlib */
#![no_std]
#![feature(unique)]
#![feature(const_fn)]
extern crate multiboot2;
extern crate rlibc;
extern crate spin;
extern crate volatile;

use core::panic::PanicInfo;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    vga_buffer::clear_screen();
    println!("****************************\nWelcome to HeliOS\n**************************** \
              \neven though you cannot really do much for now..\n\n");
    println!("But good stuff is coming!", );
    println!("{}", multiboot_information_address);

    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");

    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("    start: 0x{:x}, length: 0x{:x}",
                 area.base_addr, area.length);
    }

    println!("kernel sections:");
    let elf_sections = boot_info.elf_sections_tag().expect("Did not find elf section");
    for section in elf_sections.sections() {
        println!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
                 section.addr, section.size, section.flags);
    }
    let kernel_start = elf_sections.sections().map(|s| s.addr)
        .min().unwrap();
    let kernel_end = elf_sections.sections().map(|s| s.addr + s.size)
        .max().unwrap();
    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);
    println!("Kernel start: 0x{:x}, kernel end: 0x{:x}", kernel_start, kernel_end);
    println!("Multiboot start: 0x{:x}, multiboot  end: 0x{:x}", multiboot_start, multiboot_end);
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_begin_unwind() {}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_implementation]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}