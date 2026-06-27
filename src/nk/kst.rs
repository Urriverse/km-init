use core::sync::atomic::AtomicUsize;

use alloc::collections::btree_map::BTreeMap;
use crate::nk::{ketypes::{sym::{KeSymbol, KeSymbolGuard, KeSymbolHandle}, task::KeTaskId}, sync::RwLock};

#[repr(C)]
pub struct KeSysTab {
    pub link            :   fn(u64) -> Option<KeSymbolHandle>,
    pub link_guard      :   fn(&KeSymbolHandle) -> KeSymbolGuard,
    pub link_guard_get  :   fn(&KeSymbolGuard) -> &fn(),
    pub export          :   fn(u64, &'static fn()) -> Option<KeSymbol>,
    pub suicide         :   fn(i32) -> !,
    pub log             :   fn(u8, &'static str, &'static str, u32, &core::fmt::Arguments) -> (),
    pub panic           :   fn(&core::panic::PanicInfo) -> !,
    pub alloc           :   fn(core::alloc::Layout) -> *mut u8,
    pub free            :   fn(*mut u8, core::alloc::Layout) -> (),
    pub run_module      :   fn(elf: &[u8]) -> Result<KeTaskId, usize>,
    pub cprc_inc        :   fn() -> (),
    pub cprc_dec        :   fn() -> (),
    pub cprc_load       :   fn() -> usize,
    pub cprc_store      :   fn(usize) -> (),
    pub cprc_ref        :   fn() -> &'static AtomicUsize,

    pub gstab           :   &'static RwLock<BTreeMap<u64, KeSymbol>>,
}

#[repr(C)]
pub struct Test {}

pub macro GSTAB() { unsafe { SYSTAB.0.as_ref_unchecked() }.gstab }

crate::IMPORT! {
    pub SYSTAB: KeSysTab [KeSysTabPtr] = KeSysTabPtr(core::ptr::null());
    pub TEST: Test [TestPtr] = TestPtr(core::ptr::null());
}

pub macro Ke ( $n:ident $( $arg:expr ),* ) { ( unsafe { SYSTAB.0.as_ref_unchecked() }.$n )( $($arg),* ) }
