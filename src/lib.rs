#![no_std]

//! Provides a global allocator using the [umm_malloc][0] library.
//! You must call [`umm_malloc::init()`](fn@init) exactly once
//! before allocating anything using the global memory allocator.
//!
//! All allocations from this allocator are aligned by 8 bytes.
//! Requesting a larger alignment is not implemented and will panic.
//!
//! [0]: https://github.com/rhempel/umm_malloc

use umm_malloc_sys as ffi;

/// Initializes the heap
///
/// This function must be called BEFORE you run any code that makes use of the
/// allocator.
///
/// `start_addr` is the address where the heap will be located.
///
/// `size` is the size of the heap in bytes.
///
/// Note that:
///
/// - The heap grows "upwards", towards larger addresses. Thus `end_addr` must
///   be larger than `start_addr`
///
/// - The size of the heap is `(end_addr as usize) - (start_addr as usize)`. The
///   allocator won't use the byte at `end_addr`.
///
/// # Safety
///
/// Obey these or Bad Stuff will happen.
///
/// - This function must be called exactly ONCE.
/// - `size > 0`
#[inline]
pub unsafe fn init(start_addr: usize, size: usize) {
    ffi::umm_init(start_addr as *mut _, size)
}

struct UmmHeap {}

#[global_allocator]
static ALLOCATOR: UmmHeap = UmmHeap {};

/// All allocations from this allocator are aligned to `MIN_ALIGN`.
/// Alignments larger than `MIN_ALIGN` are currently not supported.
/// Calling `alloc()` or `realloc()` with a `layout` requesting a larger
/// alignment will panic.
pub const MIN_ALIGN: usize = 8;

unsafe impl core::alloc::GlobalAlloc for UmmHeap {
    #[inline]
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        if layout.align() <= MIN_ALIGN {
            ffi::umm_malloc(layout.size()).cast()
        } else {
            unimplemented!("Aligned alloc not implemented");
        }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        ffi::umm_free(ptr.cast());
    }

    #[inline]
    // The contract of realloc requires `new_size` to be greater than zero. This method will
    // `free()` and return `null`.
    unsafe fn realloc(&self, ptr: *mut u8, layout: core::alloc::Layout, new_size: usize) -> *mut u8 {
        if layout.align() <= MIN_ALIGN {
            ffi::umm_realloc(ptr.cast(), new_size).cast()
        } else {
            unimplemented!("Aligned alloc not implemented");
        }
    }

    // umm_calloc doesn't do anything special
}

#[cfg(all(feature = "sync", feature = "cortex-m"))]
mod cortex_m_sync {
    use core::sync::atomic::{AtomicBool, Ordering};

    // umm-malloc doesn't use nested critical sections, so we can use a
    // single global to remember if interrupts were enabled at the beginning
    // of the critical section.
    static NEEDS_ENABLE: AtomicBool = AtomicBool::new(false);

    #[no_mangle]
    #[inline]
    unsafe extern "C" fn _umm_critical_entry() {
        let primask = cortex_m::register::primask::read();
        cortex_m::interrupt::disable();
        NEEDS_ENABLE.store(primask.is_active(), Ordering::SeqCst);
    }
    
    #[no_mangle]
    #[inline]
    unsafe extern "C" fn _umm_critical_exit() {
        let needs_enable = NEEDS_ENABLE.swap(false, Ordering::SeqCst);
        if needs_enable {
            cortex_m::interrupt::enable();
        }        
    }
}
