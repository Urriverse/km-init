#![no_main]
#![no_std]

#[unsafe(no_mangle)]
pub fn _start() {}

#[panic_handler]
fn _panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
