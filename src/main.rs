#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod uart;
use uart::{uart_init, uart_print};

#[no_mangle]
#[link_section = "._start"]
pub extern "C" fn _start() -> ! {
    uart_init();
    uart_print("Hello World!");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
