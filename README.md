# sequential-id-alloc

[![Crates.io](https://img.shields.io/crates/v/sequential-id-alloc)](https://crates.io/crates/sequential-id-alloc)
[![](https://docs.rs/sequential-id-alloc/badge.svg)](https://docs.rs/sequential-id-alloc)
[![License](https://img.shields.io/crates/l/sequential-id-alloc?color=informational&logo=mpl-2)](/LICENSE)

<!-- cargo-rdme start -->

A simple sequential ID allocator that guarantees sequential allocation.

This crate provides a macro to generate sequential ID allocators that differ from
traditional bitmap/slab allocators by not immediately reusing freed IDs. Instead,
freed IDs are only reused when the allocation pointer wraps around to them again.
This ensures IDs are allocated sequentially and predictably.

## Example

```rust
use sequential_id_alloc::sequential_id_alloc;

// Create an allocator for u8 IDs (0-255)
sequential_id_alloc!(MyIdAllocator, u8, 256, u8);

let mut allocator = MyIdAllocator::default();

// Allocate IDs sequentially
assert_eq!(allocator.alloc(), Some(0u8));
assert_eq!(allocator.alloc(), Some(1u8));
assert_eq!(allocator.alloc(), Some(2u8));

// Free an ID - it won't be reused immediately
allocator.dealloc(1u8);
assert_eq!(allocator.alloc(), Some(3u8)); // Gets 3, not 1

// Check if an ID is allocated
assert!(allocator.contains(0u8));
assert!(!allocator.contains(1u8));

// Get allocation statistics
assert_eq!(allocator.size(), 3); // Currently 3 IDs allocated
assert!(!allocator.is_full());   // Not all IDs are allocated
```

<!-- cargo-rdme end -->
