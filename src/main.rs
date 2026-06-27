#![no_main]
#![no_std]
#![allow(unused_features)]
#![feature(decl_macro)]

pub mod nk; use nk::*;

pub macro mod_ident() { "km-init" }

fn main() -> i32 {
    warn!("Hey there!");
    10
}
