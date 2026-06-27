#![no_std] #![no_main] #![feature(decl_macro)]

#[allow(unused)] #[macro_use] extern crate nk;

nk::util::meta!("km-init");

#[cfg(not(test))] nk::macros::panic_handler![];

#[unsafe(no_mangle)]
pub extern "C" fn _start() {
    // greetings
    panic!("Hey there!");
    // nk::exit!();
}
