# umm-malloc-rs

[![downloads](https://img.shields.io/crates/d/umm-malloc.svg)](https://crates.io/crates/umm-malloc)
[![crates.io](https://img.shields.io/crates/v/umm-malloc.svg)](https://crates.io/crates/umm-malloc)
[![docs.rs](https://docs.rs/umm-malloc/badge.svg)](https://docs.rs/umm-malloc)
![CI](https://github.com/mattico/umm-malloc-rs/workflows/Rust/badge.svg)

Provides a global allocator using the [umm_malloc][0] library.

`umm_malloc` is a small and simple memory allocator designed for embedded systems.
It is able to allocate and free quickly with limited memory fragmentation.

## Limitations

All allocations from this allocator are aligned by 8 bytes.
Requesting a larger alignment is not implemented and will panic.

## Features

- `sync`
  Enables critical sections around the allocator internals, which must be implemented
  with the external functions `_umm_critical_entry()` and `_umm_critical_exit()`.

- `cortex-m`
  When combined with `sync`, provides an implementation of critical sections
  by disabling interrupts.

- `first-fit`
  Use the first available block for allocation, rather than search for a better fit.

## Future Work

`umm_malloc` has features for collecting metrics and detecting heap corruption, 
which could be exposed conveniently with cargo features.

An implementation of memalign could be added to `umm_malloc`.

[0]: https://github.com/rhempel/umm_malloc