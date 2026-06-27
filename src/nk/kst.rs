pub type KeSymbolHandle = usize;

pub type KeSymbolGuard = [usize; 2];

#[repr(C)]
pub struct KeSymbol {
    mprc        : *const    core::sync::atomic::AtomicUsize ,
    rc          :           core::sync::atomic::AtomicUsize ,
    ptr         : *const    ()                              ,
    poisonous   :           core::sync::atomic::AtomicBool  ,
    _pad        :           [u8; 3]                         ,
    id          :           u64                             ,
}

#[repr(C)]
pub struct KeSysTab {
    pub link            :   fn(u64) ->  Option<KeSymbolHandle>,
    pub link_guard      :   fn(&KeSymbolHandle) -> KeSymbolGuard,
    pub link_guard_get  :   fn(&KeSymbolGuard) -> &fn(),
    pub export          :   fn(u64, &'static fn()) -> Option<KeSymbol>,
    pub suicide         :   fn(i32) -> !,
    pub log             :   fn(u8, &'static str, &'static str, u32, &core::fmt::Arguments) -> (),
    pub panic           :   fn(&core::panic::PanicInfo) -> !,
    pub alloc           :   fn(core::alloc::Layout) -> *mut u8,
    pub free            :   fn(*mut u8, core::alloc::Layout) -> (),
}

pub struct KeSysTabPtr(pub *const KeSysTab);

unsafe impl Sync for KeSysTabPtr {}

#[unsafe(no_mangle)]
pub static SYSTAB: KeSysTabPtr = KeSysTabPtr(core::ptr::null());

pub macro Ke ( $n:ident $( $arg:expr ),* ) {
    (
        unsafe {
            SYSTAB
                .0
                .as_ref_unchecked()
        }.$n
    )
    (
        $(
            $arg
        ),*
    )
}
