#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
use core::panic::PanicInfo;
use rust_os::println;

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os::hlt_loop();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    rust_os::init();
    //fn stack_overflow() {
    //stack_overflow();
    //}
    //stack_overflow();
    //unsafe {
    //*(0xdeadbeef as *mut u64) = 42;
    //};
    //x86_64::instructions::interrupts::int3();
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    rust_os::hlt_loop();
}
