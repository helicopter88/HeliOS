#![feature(lang_items)]
#![feature(panic_implementation)]
#![feature(ptr_internals)]
#![feature(alloc)]

/* Disable stdlib */
#![no_std]
#![feature(unique)]
#![feature(const_fn)]
#![feature(allocator_api)]

#[macro_use]
extern crate alloc;
#[macro_use]
extern crate bitflags;
extern crate multiboot2;
extern crate rlibc;
extern crate spin;
extern crate volatile;
extern crate x86_64;


use core::panic::PanicInfo;
use memory::FrameAllocator;
use memory::allocator::GlobalBumpAllocator;
use multiboot2::BootInformation;
use multiboot2::MemoryMapTag;

#[macro_use]
mod vga_buffer;
mod memory;

pub const HEAP_START: usize = 0o_000_001_000_000_0000;
pub const HEAP_SIZE: usize = 1024 * 100 * 1024; // 100 KiB

#[global_allocator]
static HEAP_ALLOCATOR: GlobalBumpAllocator = GlobalBumpAllocator::new(HEAP_START,
                                                          HEAP_START + HEAP_SIZE);
fn enable_nxe_bit() {
    use x86_64::registers::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86_64::registers::control_regs::{cr0, cr0_write, Cr0};

    unsafe { cr0_write(cr0() | Cr0::WRITE_PROTECT) };
}

#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    vga_buffer::clear_screen();
    println!("****************************\nWelcome to HeliOS\n**************************** \
              \neven though you cannot really do much for now..\n\n");
    println!("But good stuff is coming!");
    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };
    enable_nxe_bit();
    enable_write_protect_bit();

    memory::init(boot_info);
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

#[lang = "oom"]
#[no_mangle]
pub fn rust_oom() -> ! {
    panic!()
}