#![no_std]

mod bindings;

pub use bindings::{umm_init, umm_malloc, umm_free, umm_calloc, umm_realloc};