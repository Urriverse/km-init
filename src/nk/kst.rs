#[repr(C)]
pub struct KeSysTab {
    pub log             :   fn(u8, &'static str, &'static str, u32, &core::fmt::Arguments) -> (),
    pub panic           :   fn(&core::panic::PanicInfo) -> !,
    pub alloc           :   fn(core::alloc::Layout) -> *mut u8,
    pub free            :   fn(*mut u8, core::alloc::Layout) -> (),
}

crate::IMPORT! { pub SYSTAB: KeSysTab [KeSysTabPtr] = KeSysTabPtr(core::ptr::null()) }

pub macro Ke ( $n:ident $( $arg:expr ),* ) { ( unsafe { SYSTAB.0.as_ref_unchecked() }.$n )( $($arg),* ) }
