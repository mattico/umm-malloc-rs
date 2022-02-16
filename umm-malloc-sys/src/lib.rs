#![no_std]

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::{umm_init, umm_init_heap, umm_malloc, umm_free, umm_calloc, umm_realloc};

#[cfg(all(feature = "extern-critical-section", feature = "cortex-m-interrupt-critical-section"))]
compile_error!("Choose only one critical section implementation");
