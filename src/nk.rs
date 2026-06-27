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
    pub log:                fn(u8, &'static str, &'static str, u32, &core::fmt::Arguments) -> (),
    pub panic:              fn(&core::panic::PanicInfo) -> !,
}

pub struct Gall;

unsafe impl core::alloc::GlobalAlloc for Gall {
    unsafe fn alloc(&self, _layout: core::alloc::Layout) -> *mut u8 {
        0 as _
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        //
    }
}

struct KeSysTabPtr(pub *const KeSysTab);

unsafe impl Sync for KeSysTabPtr {}

#[unsafe(no_mangle)]
static SYSTAB: KeSysTabPtr = KeSysTabPtr(core::ptr::null());

pub macro Ke($n:ident $($arg:expr),*) { (unsafe { SYSTAB.0.as_ref_unchecked() }.$n)($($arg),*) }

#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions)]
pub(crate) extern "C" fn _start(st: &crate::nk::KeSysTab) {
    (st.log)(3, "km-init", file!(), line!(), &format_args!("I not wanna live!"));
    panic!("test panic");
    // Ke!(suicide);
}

#[cfg(not(test))]
#[panic_handler]
fn _ph(pi: &core::panic::PanicInfo) -> ! {
    Ke!(panic pi)
}
