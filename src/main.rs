#![no_main]
#![no_std]
#![feature(decl_macro)]

pub mod nk;

nk::nano!{}

#[cfg(not(test))]
#[panic_handler]
fn _ph(_: &core::panic::PanicInfo) -> ! {
    loop {
        //
    }
}
