#![no_std]
#![no_main]
#![deny(warnings)]

#[macro_use]
mod lang_items;
mod sbi;
use crate::sbi::console_putchar;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));


#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();

    for c in "hello, world!".chars() {
        console_putchar(c as usize);
    }

    loop {}
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize .. ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

#[allow(warnings)]
fn clear_bss_test() {
    extern "C" {
        static sbss: usize;
        static ebss: usize;
    }
    unsafe {
        let mut ptr = sbss as *mut u8;
        let end = ebss as *mut u8;
        while ptr < end {
            ptr.write_volatile(0);
            ptr = ptr.offset(1);
        }
    }
}

