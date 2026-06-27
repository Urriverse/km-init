#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions)]
extern "C" fn _start() -> ! {
    crate::main();
    panic!("Module finished.");
}
