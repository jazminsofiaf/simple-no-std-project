#![no_main]
#![no_std]
#![feature(lang_items)]

mod logger;
use core::panic::PanicInfo;


#[no_mangle]
fn main() -> ! {
    let serial_port_logger = logger::SerialPortLogger::new();
    loop {
        serial_port_logger.delay();
    }
}
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}


#[lang = "eh_personality"]
extern "C" fn eh_personality() {}