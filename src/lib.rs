#![no_std]

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

unsafe impl core::alloc::GlobalAlloc for UmmHeap {
    #[inline]
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        ffi::umm_malloc(layout.size()).cast()
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        ffi::umm_free(ptr.cast());
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, _layout: core::alloc::Layout, new_size: usize) -> *mut u8 {
        // TODO: verify alignment
        ffi::umm_realloc(ptr.cast(), new_size).cast()
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



