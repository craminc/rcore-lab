#![no_main]
#![no_std]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod logging;

use core::arch::global_asm;

use log::*;

use crate::logging::init;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn skernel();
        fn ekernel();
    }
    let stext_addr = stext as usize;
    let etext_addr = etext as usize;
    let srodata_addr = srodata as usize;
    let erodata_addr = erodata as usize;
    let sdata_addr = sdata as usize;
    let edata_addr = edata as usize;
    let sbss_addr = sbss as usize;
    let ebss_addr = ebss as usize;
    let skernel_addr = skernel as usize;
    let ekernel_addr = ekernel as usize;

    clear_bss();
    init();

    trace!("  text: 0x{:X}-0x{:X}", stext_addr, etext_addr);
    debug!("rodata: 0x{:X}-0x{:X}", srodata_addr, erodata_addr);
    info!("  data: 0x{:X}-0x{:X}", sdata_addr, edata_addr);
    warn!("   bss: 0x{:X}-0x{:X}", sbss_addr, ebss_addr);
    error!("kernel: 0x{:X}-0x{:X}", skernel_addr, ekernel_addr);
    
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
