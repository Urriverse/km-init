use super::Ke;

#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions)]
extern "C" fn _start() -> ! {
    Ke!(suicide crate::main())
}
