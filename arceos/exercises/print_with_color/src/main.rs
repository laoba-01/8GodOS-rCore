#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";
#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
     println!("{GREEN}[WithColor]: Hello, Arceos!{RESET}");
}
