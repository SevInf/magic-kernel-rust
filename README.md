# Magic Kernel for Rust

[![Crates.io Version](https://img.shields.io/crates/v/magic-kernel)](https://crates.io/crates/magic-kernel)
[![docs.rs](https://img.shields.io/docsrs/magic-kernel)](https://docs.rs/magic-kernel/latest/magic_kernel/)



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