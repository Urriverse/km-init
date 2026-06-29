#![no_std] #![no_main]
#[macro_use] extern crate nk;

meta! { "km-init" }

entry! {
    info!("Hey there!");
    
    if let Some(x) = nk::KeVtDeviceNew("kbd0") {
        info!("Success: created KeDevice `kbd0`! :: {:?}", x);
    } else {
        error!("Failed to create KeDevice `kbd0`!");
    }
    
    exit!(1488);
}
