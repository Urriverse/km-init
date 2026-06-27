#![no_std] #![no_main] #![feature(decl_macro)]
#[allow(unused)] #[macro_use] extern crate nk;

meta!
{
    "km-init"
}

entry! {
    panic!("Hey there!");
}
