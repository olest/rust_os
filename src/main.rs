#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// This function is called on panic.
/// This is a diverging function, which does not return
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
