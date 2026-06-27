use core::{ptr::addr_of, sync::atomic::Ordering::Relaxed};

use crate::nk::{Ke, ketypes::sym::{KeSymbol, KeSymbolHandle}, kst::GSTAB};

pub fn export<F>(id: u64, f: &'static F) -> Option<KeSymbol> {
    if size_of::<F>() != 0 {
        panic!("Invalid upcast: size mismatch");
    }

    Ke!(cprc_inc);

    return GSTAB!()
        .write()
        .insert(
            id,
            KeSymbol::new(
                id,
                Ke!(cprc_ref),
                addr_of!(*f) as *const ()
            )
        )
    ;
}

pub fn link(id: u64) -> Option<KeSymbolHandle> {
    if let Some(sym) = GSTAB!().read().get(&id) {
        if !sym.poisonous.load(Relaxed) {
            return Some(KeSymbolHandle {
                hold: addr_of!(*sym),
            });
        }
    }
    None
}
