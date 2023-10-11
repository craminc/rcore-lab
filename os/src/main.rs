#![no_main]
#![no_std]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod lang_items;
mod sbi;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello World {}", "Rcore");
    panic!("Shutdown machine!");
}

pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a: usize| {
        unsafe {
            (a as *mut u8).write_volatile(0)
        }   
    });
}
