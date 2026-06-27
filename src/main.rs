#![no_main]  #![no_std]     // required by environment
#![feature(decl_macro)]     // required by nanokit

pub mod nk; // include nanokit

Mod! { "km-init" }  // declare module name (this macro also adds custom prelude)

fn example() {
    error!("unimplemented");
}

// entry point
fn main() -> i32 {
    // greetings
    warn!("Hey there!");

    // export symbol
    dlexport!  [ @ b"example" => example ];

    // link with symbol
    let sym = dlink![ @ b"example"; "Link failed" ];

    // use symbol
    dluse! [ sym => fn() ] ();

    10
}
