#![no_std] #![no_main] #![crate_type = "dylib"]
#[macro_use] extern crate nk;

meta! { "km-init" }

type KeTest = fn();

nk::Ke! { KeTest or || () }

entry! {
    info!("Hey there! KeTest = {:p}", KeTest);

    KeTest();
    
    if let Some(x) = nk::KeVtDeviceNew("kbd0") {
        info!("Success: created KeDevice `kbd0`! :: {:p}", core::ptr::addr_of!(x));
    } else {
        error!("Failed to create KeDevice `kbd0`!");
    }
    
    exit!(1488);
}
