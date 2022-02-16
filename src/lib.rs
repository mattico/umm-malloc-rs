#![no_std]

//! Provides a global allocator using the [umm_malloc][0] library.
//! You must call [`umm_malloc::init_heap()`](fn@init) exactly once
//! before allocating anything using the global memory allocator.
//!
//! All allocations from this allocator are aligned by 8 bytes.
//! Requesting a larger alignment is not implemented and will panic.
//!
//! # Global Allocator Critical Sections
//!
//! Concurrent access to the global allocator is Undefined Behavior. Enable only one of the following cargo features to
//! configure how access to the global allocator is controlled.
//!
//! - `cortex-m-interrupt-critical-section`: interrupt-disabled critical section for ARM Cortex-M processors.
//! - `extern-critical-section`: Uses the extern functions `void _umm_critical_entry(uint32_t*)` and
//!   `void _umm_critical_exit(uint32_t*)` to implement the global allocator critical sections. You MUST supply those
//!   functions via some other means.
//!   Note that critical sections may nest.
//! - `unsafe-no-critical-section`: no critical sections around the global allocator. You MUST prevent concurrent use
//!   of the global allcator to avoid Undefined Behavior.
//!
//! [0]: https://github.com/rhempel/umm_malloc

use umm_malloc_sys as ffi;

/// Initializes the heap from extern addresses
///
/// This function initializes the heap using the address defined by the extern pointer `UMM_MALLOC_CFG_HEAP_ADDR` with
/// size defined by the extern u32 `UMM_MALLOC_CFG_HEAP_ADDR`. You must define symbols with those (unmangled) names
/// elsewhere, e.g. in your linker script.
///
/// See [`umm_malloc::init_heap()`](fn@init) for runtime initialization.
///
/// # Note
///
/// - The heap grows "upwards", towards larger addresses. Thus `end_addr` must
///   be larger than `start_addr`.
/// - The size of the heap is `(end_addr as usize) - (start_addr as usize)`. The
///   allocator won't use the byte at `end_addr`.
///
/// # Safety
///
/// Obey these or Bad Stuff will happen.
///
/// - This function must be called exactly ONCE.
/// - This function must be called BEFORE you run any code that makes use of the allocator unless the cargo feature
///   `init-if-uninitialized` or `hang-if-uninitialized` is enabled.
#[inline]
pub unsafe fn init() {
    ffi::umm_init()
}

/// Initializes the heap with the given block of memory
///
/// `start_addr` is the address where the heap will be located.
///
/// `size` is the size of the heap in bytes.
///
/// # Note
///
/// - The heap grows "upwards", towards larger addresses. Thus `end_addr` must
///   be larger than `start_addr`.
/// - The size of the heap is `(end_addr as usize) - (start_addr as usize)`. The
///   allocator won't use the byte at `end_addr`.
/// - This memory will be zeroed by the allocator.
///
/// # Safety
///
/// Obey these or Bad Stuff will happen.
///
/// - This function must be called exactly ONCE.
/// - `size > 0`
/// - This function must be called BEFORE you run any code that makes use of the allocator unless the cargo feature
///   `init-if-uninitialized` or `hang-if-uninitialized` is enabled.
#[inline]
pub unsafe fn init_heap(start_addr: usize, size: usize) {
    ffi::umm_init_heap(start_addr as *mut _, size)
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
    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        layout: core::alloc::Layout,
        new_size: usize,
    ) -> *mut u8 {
        if layout.align() <= MIN_ALIGN {
            ffi::umm_realloc(ptr.cast(), new_size).cast()
        } else {
            unimplemented!("Aligned alloc not implemented");
        }
    }

    // umm_calloc doesn't do anything special
}

#[cfg(any(
    all(
        feature = "cortex-m-interrupt-critical-section",
        any(
            feature = "unsafe-no-critical-section",
            feature = "extern-critical-section"
        )
    ),
    all(
        feature = "unsafe-no-critical-section",
        any(
            feature = "cortex-m-interrupt-critical-section",
            feature = "extern-critical-section"
        )
    ),
    all(
        feature = "extern-critical-section",
        any(
            feature = "cortex-m-interrupt-critical-section",
            feature = "unsafe-no-critical-section"
        )
    ),
))]
compile_error!("Only enable one critical section cargo feature");

#[cfg(not(any(
    feature = "cortex-m-interrupt-critical-section",
    feature = "extern-critical-section",
    feature = "unsafe-no-critical-section"
)))]
compile_error!("A critical section cargo feature must be enabled");
