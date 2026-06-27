#![no_main]  #![no_std]     // required by environment
#![feature(decl_macro)]     // required by nanokit

pub mod nk; // include nanokit

Mod! { "km-init" }  // declare module name (this macro also adds custom prelude)

// entry point
fn main() -> i32 {
    warn!("Hey there!");
    10
}
