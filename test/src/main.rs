#![no_main]

#[no_mangle]
pub fn rust_main(args: c_main::Args) -> isize {
    for arg in args.into_iter().skip(1) {
        println!("arg={:?}", arg);
    }
    0
}
