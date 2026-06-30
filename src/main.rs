#![no_std] #![no_main]

#[macro_use] extern crate nk;

meta! { "km-init" }

// type KeTest = fn();

// nk::Ke! { KeTest or || () }

#[allow(non_snake_case)]
unsafe extern "Rust" fn KeTest() { panic!("Failed to link KeTest") }

entry! {
    info!("Hey there!");
    info!("KeTest   = {:p}, * {:p}", KeTest as *const fn(), *unsafe { (KeTest as *const *const ()).as_ref_unchecked() });
    info!("KeMonLog = {:p}, * {:p}", nk::KeMonLog, *unsafe { (nk::KeMonLog as *const *const ()).as_ref_unchecked() });

    trace!("#");

    nk::KeExecYield();

    trace!("~");

    unsafe { KeTest(); }

    trace!("*");

    let y = nk::KeVtDeviceNew("kbd0");

    trace!("@");
    
    if let Some(x) = y {
        let _ = x;
        info!("Success: created KeDevice `kbd0`! :: {:p}", core::ptr::addr_of!(x));
    } else {
        error!("Failed to create KeDevice `kbd0`!");
    }

    trace!("$");
    
    exit!(1488);
}
