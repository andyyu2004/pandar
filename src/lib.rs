#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod serial;
pub mod vga;

use core::panic::PanicInfo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed  = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32)
    }
}

pub fn test_runner(tests: &[&dyn Test]) {
    serial_println!("running {} test(s)", tests.len());
    for test in tests {
        test.test();
    }
    exit_qemu(QemuExitCode::Success);
}

pub trait Test {
    fn test(&self);
}

impl<F: Fn()> Test for F {
    fn test(&self) {
        serial_print!("test {} ... ", core::any::type_name::<F>());
        self();
        serial_println!("ok");
    }
}

pub fn panic_handler(info: &PanicInfo) -> ! {
    serial_println!("{}", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    panic_handler(info)
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[test_case]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
