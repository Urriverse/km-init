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

#[macro_export]
macro_rules! dlexport {
    (@ $name:expr => $val:expr) => {
        $crate::nk::dl::export(hash!($name), &$val)
    };
    ($name:expr => $val:expr) => {
        $crate::nk::dl::export($name, $val)
    };
}

#[macro_export]
macro_rules! dlink {
    (@ $name:expr ; $err:expr) => {
        $crate::nk::dl::link(hash!($name)).expect($err)
    };
    ($name:expr ; $err:expr) => {
        $crate::nk::dl::link($name).expect($err)
    };
}

#[macro_export]
macro_rules! dluse {
    ($sym:expr => $ty:ty) => {
        ($sym.get().get::<$ty>())
    };
}
