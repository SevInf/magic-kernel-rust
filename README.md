# Magic Kernel for Rust

Implementation of [Magic Kernel](https://johncostella.com/magic/) family of resizing algorithms.

Usage:

```rust
use magic_kernel::{magic_resize, Version};
magic_resize(
    &image,
    Version::MagicKernelSharp2021, // algorithm version to use
    Some(1500), // width
    Some(500), // height
);
```

`image` is expected to be an instance `magic_kernel::ImageF64`. Example of creating `ImageF64` from
[image crate](https://crates.io/crates/image) can be found in [examples directory](https://github.com/SevInf/magic-kernel-rust/tree/main/examples).