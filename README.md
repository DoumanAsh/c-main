# c-main

![Rust](https://github.com/DoumanAsh/c-main/workflows/Rust/badge.svg?branch=master)
[![Crates.io](https://img.shields.io/crates/v/c-main.svg)](https://crates.io/crates/c-main)
[![Documentation](https://docs.rs/c-main/badge.svg)](https://docs.rs/crate/c-main/)

Utility crate providing c-main style arguments by using C main function.

## Usage

```rust
#![no_main]

#[no_mangle]
pub fn rust_main(args: c_main::Args) -> isize {
    for arg in args.into_iter().skip(1) {
        println!("arg={:?}", arg);
    }
    0
}
```
