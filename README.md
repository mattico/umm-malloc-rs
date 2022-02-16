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

- `first-fit`
  Use the first available block for allocation, rather than search for a better fit.

- `init-if-uninitialized`
  Adds checks to every malloc function which tries to initialize the heap (using the extern symbols for heap location)
  if it is not initialized.

- `hang-if-uninitialized`
  Adds checks to every malloc function which enters an infinite loop if the heap is not initialized.

### Global Allocator Critical Sections

Concurrent access to the global allocator is Undefined Behavior. Enable one of the following cargo features to configure
how access to the global allocator is controlled.

- `cortex-m-interrupt-critical-section`: interrupt-disabled critical section for ARM Cortex-M processors.
- `extern-critical-section`: Uses the extern functions `void _umm_critical_entry(uint32_t*)` and 
  `void _umm_critical_exit(uint32_t*)` to implement the global allocator critical sections. You MUST supply those
  functions via some other means. Note that critical sections may nest.
- `unsafe-no-critical-section`: no critical sections around the global allocator. You MUST prevent concurrent use
  of the global allcator to avoid Undefined Behavior.

## Future Work

`umm_malloc` has features for collecting metrics and detecting heap corruption, 
which could be exposed conveniently with cargo features.

An implementation of memalign could be added to `umm_malloc`.

[0]: https://github.com/rhempel/umm_malloc