#![no_std]

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::{umm_init, umm_init_heap, umm_malloc, umm_free, umm_calloc, umm_realloc};
