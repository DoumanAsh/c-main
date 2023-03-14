//! C main entry point with nice wrapper for arguments.
//!
//!```rust
//!#![no_main]
//!
//!#[no_mangle]
//!pub fn rust_main(args: c_main::Args) -> isize {
//!    for arg in args.into_iter().skip(1) {
//!        println!("arg={:?}", arg);
//!    }
//!    0
//!}
//!```

#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]
#![cfg_attr(rustfmt, rustfmt_skip)]

mod args;
pub use args::Args;

#[allow(unused)]
#[cold]
#[inline(never)]
unsafe fn invalid_cli_args_error() -> libc::c_int {
    libc::printf("Unable to parse C argv as utf-8 string\n\0".as_ptr() as _);
    255
}

///Converts C string to Rust's, verifying it is UTF-8
///
///It is UB to pass non-C string as it requires \0
pub unsafe fn c_str_to_rust(ptr: *const u8) -> Result<&'static str, core::str::Utf8Error> {
    let len = libc::strlen(ptr as *const i8);
    let parts = core::slice::from_raw_parts(ptr, len);
    core::str::from_utf8(parts)
}

///Converts C string to Rust's one assuming it is UTF-8
///
///It is UB to pass non-C string as it requires \0
pub unsafe fn c_str_to_rust_unchecked(ptr: *const u8) -> &'static str {
    let len = libc::strlen(ptr as *const i8);
    let parts = core::slice::from_raw_parts(ptr, len);
    core::str::from_utf8_unchecked(parts)
}

extern "Rust" {
    fn rust_main(args: args::Args) -> isize;
}

#[doc(hidden)]
#[cfg(not(test))]
#[no_mangle]
pub unsafe extern fn main(argc: libc::c_int, argv: *const *const u8) -> libc::c_int {
    match args::Args::new(argc as isize, argv) {
        Ok(args) => rust_main(args) as _,
        Err(_) => invalid_cli_args_error(),
    }
}
