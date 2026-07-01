#![no_std] #![no_main] #[macro_use] extern crate nk; use ketypes::*;

Import! {
    pub fn ExecYield() where kernel 0.0 {
        error!("ExecYield not provided");
    }
}

entry! {
    mod "km-init";

    ExecYield();

    info!("Hey there!");

    let y = nk::Device::new("kbd0");
    
    if let Some(x) = y {
        let _ = x;
        info!("Success: created KeDevice `kbd0`! :: {:p}", core::ptr::addr_of!(x));
    } else {
        error!("Failed to create KeDevice `kbd0`!");
    }
    
    exit!(1488);
}
