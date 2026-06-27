#[cfg(not(test))]
use super::Ke;

#[cfg(not(test))]
#[panic_handler]
fn _ph(pi: &core::panic::PanicInfo) -> ! {
    Ke!(panic pi)
}
