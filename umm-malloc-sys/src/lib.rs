#![no_std]

#![doc = include_str!("../README.md")]

#[cfg(all(feature = "extern-critical-section", feature = "cortex-m-interrupt-critical-section"))]
compile_error!("Choose only one critical section implementation");

use core::ffi::c_void;

extern "C" {
    pub fn umm_init_heap(ptr: *mut c_void, size: usize);
    pub fn umm_init();
    pub fn umm_malloc(size: usize) -> *mut c_void;
    pub fn umm_calloc(num: usize, size: usize) -> *mut c_void;
    pub fn umm_realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    pub fn umm_free(ptr: *mut c_void);
}
