#![no_main]
#![no_std]

// #[allow(unused)] #[macro_use] pub extern crate nanokit;

// allocator!{}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() {
    loop {
        //
    }
}

#[cfg(not(test))]
#[panic_handler]
fn _ph(_: &core::panic::PanicInfo) -> ! {
    loop {
        //
    }
}
