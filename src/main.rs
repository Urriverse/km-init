#![no_std] #![no_main] #[macro_use] extern crate nk; use ketypes::*;

// Import! {
//     pub fn ExecYield() where kernel 0.0 {
//         error!("ExecYield not provided");
//     }
// }

#[allow(non_snake_case)]
fn __stub_ExecYield() { error!("ExecYield not provided"); }

#[used]
#[allow(non_upper_case_globals)]
#[unsafe(export_name = concat!("KiExecYield"))]
static _ExecYield: Import = Import(__stub_ExecYield as *const (), 0);

#[allow(non_snake_case)]
#[inline(always)]
pub fn ExecYield() {
    (unsafe{core::mem::transmute::<_, fn ()>(_ExecYield.0 )})()
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
