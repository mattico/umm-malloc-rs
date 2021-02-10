# umm-malloc-rs

Provides a global allocator using the [umm_malloc][0] library.

`umm_malloc` is a small and simple malloc designed for embedded systems.
It is able to allocate and free quickly with limited memory fragmentation.

## Limitations

All allocations form this allocator are aligned by 8 bytes.
Requesting a larger alignment is not implemented and will panic.

## Future Work

`umm_malloc` has features for collecting metrics and detecting heap corruption, 
which could be exposed conveniently with cargo features.

An implementation of memalign could be added to `umm_malloc`.

[0]: https://github.com/rhempel/umm_malloc