#![no_std]
#![no_main]

#[macro_use]
extern crate nk;

meta! { "km-init" }

entry! {
    info!("Hey there!");
    // exit!(0);
    panic!("Ouch!");
}
