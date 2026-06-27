#![no_std] #![no_main] #![feature(decl_macro)]

#[allow(unused)] #[macro_use] extern crate nk;

meta!("km-init");

#[unsafe(no_mangle)]
pub fn module_start() {
    // greetings
    warn!("Hey there!");
    nk::exit!();
}
