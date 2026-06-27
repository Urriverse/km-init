pub type KeSymbolHandle = usize;

pub type KeSymbolGuard = [usize; 2];

#[repr(C)]
pub struct KeSymbol {
    mprc: usize,
    rc: usize,
    ptr: usize,
    poisonous: u8,
    _pad: [u8; 3],
    id: u64,
}

#[repr(C)]
pub struct KeSysTab {
    pub link:               fn(u64) ->  Option<KeSymbolHandle>,
    pub link_guard:         fn(&KeSymbolHandle) -> KeSymbolGuard,
    pub link_guard_get:     fn(&KeSymbolGuard) -> &fn(),
    pub export:             fn(u64, &'static fn()) -> Option<KeSymbol>,
    pub suicide:            fn() -> !,
    pub log:                fn(u8, &'static str, &'static str, u32, *const ()) -> (),
}

pub struct Gall;

unsafe impl core::alloc::GlobalAlloc for Gall {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        0 as _
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        //
    }
}

pub macro nano() {
    #[unsafe(no_mangle)]
    #[allow(improper_ctypes_definitions)]
    pub(crate) extern "C" fn _start(st: &crate::nk::KeSysTab) {
        (st.suicide)()
    }
}
