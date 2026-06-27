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
    pub suicide:            fn(i32) -> !,
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

pub struct KeSysTabPtr(pub *const KeSysTab);

unsafe impl Sync for KeSysTabPtr {}

#[unsafe(no_mangle)]
static SYSTAB: KeSysTabPtr = KeSysTabPtr(core::ptr::null());

pub macro Ke($n:ident $($arg:expr),*) { (unsafe { $crate::nk::SYSTAB.0.as_ref_unchecked() }.$n)($($arg),*) }

#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions)]
pub(crate) extern "C" fn _start() {
    Ke!(suicide crate::main());
}

#[cfg(not(test))]
#[panic_handler]
fn _ph(pi: &core::panic::PanicInfo) -> ! {
    Ke!(panic pi)
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)+) => {{
        #[cfg(not(feature = "lowlog"))]
        Ke!(log 5, concat!(crate::mod_ident!(), "::", module_path!()), file!(), line!(), &format_args!($($arg)+));
    }};
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => {{
        #[cfg(not(feature = "lowlog"))]
        Ke!(log 4, concat!(crate::mod_ident!(), "::", module_path!()), file!(), line!(), &format_args!($($arg)+));
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => {{
        Ke!(log 3, concat!(crate::mod_ident!(), "::", module_path!()), file!(), line!(), &format_args!($($arg)+));
    }};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => {{
        Ke!(log 2, concat!(crate::mod_ident!(), "::", module_path!()), file!(), line!(), &format_args!($($arg)+));
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => {{
        Ke!(log 1, concat!(crate::mod_ident!(), "::", module_path!()), file!(), line!(), &format_args!($($arg)+));
    }};
}

#[macro_export]
macro_rules! panic_msg {
    ($($arg:tt)+) => {{
        Ke!(log 0, concat!(crate::mod_ident!(), "::", module_path!()), file!(), line!(), &format_args!($($arg)+));
    }};
}

#[macro_export]
macro_rules! hash {
    ($s:expr) => {{
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
    }};
}
