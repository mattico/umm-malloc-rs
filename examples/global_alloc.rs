#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use alloc::vec::Vec;
use umm_malloc;
use core::alloc::Layout;
use core::panic::PanicInfo;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // Initialize the allocator BEFORE you use it
    let start = cortex_m_rt::heap_start() as usize;
    let size = 1024; // in bytes
    unsafe { umm_malloc::init_heap(start, size) };

    let mut xs = Vec::new();
    xs.push(1);

    loop { /* .. */ }
}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}