pub struct ModNamePtr(pub *const &'static str);

unsafe impl Sync for ModNamePtr {}

#[macro_export]
macro_rules! IMPORT {
    ($($name:ident: $ty:ty = $def:expr $(;)?)*) => {
        $(
            #[unsafe(no_mangle)] pub static $name: $ty = $def;
        )*
    }
}

#[macro_export]
macro_rules! Mod {
    ($x:expr) => {
        #[allow(unused)]
        #[macro_use]
        extern crate alloc;

        static __MODNAME: &str = $x;

        // #[unsafe(no_mangle)] pub static MODNAME: $crate::nk::misc::ModNamePtr = $crate::nk::misc::ModNamePtr(core::ptr::addr_of!(__MODNAME));
        crate::IMPORT! { MODNAME: $crate::nk::misc::ModNamePtr = $crate::nk::misc::ModNamePtr(core::ptr::addr_of!(__MODNAME)) }

        pub macro mod_ident() {
            $x
        }
    }
}

#[macro_export]
macro_rules! hash {
    ($s:expr) => {
        {
            const fn fnv1a64(data: &[u8]) -> u64 {
                const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
                const FNV_PRIME: u64 = 0x100000001b3;
                let mut hash = FNV_OFFSET_BASIS;
                let mut i = 0;
                while i < data.len() {
                    hash ^= data[i] as u64;
                    hash = hash.wrapping_mul(FNV_PRIME);
                    i += 1;
                }
                hash
            }
            fnv1a64($s)
        }
    }
}
