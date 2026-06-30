#![no_std] #![no_main]

#[macro_use] extern crate nk;

meta! { "km-init" }

entry! {
    info!("Hey there!");

    nk::KeExecYield();

    let y = nk::KeDevice::new("kbd0");
    
    if let Some(x) = y {
        let _ = x;
        info!("Success: created KeDevice `kbd0`! :: {:p}", core::ptr::addr_of!(x));
    } else {
        error!("Failed to create KeDevice `kbd0`!");
    }
    
    exit!(1488);
}
