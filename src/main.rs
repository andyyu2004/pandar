#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(pandar::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use pandar::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hello world 2");
    #[cfg(test)]
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    pandar::panic_handler(info)
}
