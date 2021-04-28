#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use pandar::{exit_qemu, serial_println, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("`should_panic` test did not panic");
    loop {}
}

fn should_fail() {
    assert!(false)
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("ok");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
