#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rust_os::{memory::BootInfoFrameAllocator, println};
use x86_64::structures::paging::Size4KiB;

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

entry_point!(kernel_main);
#[no_mangle]
//boot_info can be skipped since the x86_64 convention passes it in a cpu register instead of in stack
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::memory;
    use x86_64::{
        structures::paging::{Page, Translate},
        VirtAddr,
    };

    println!("Hello World{}", "!");
    rust_os::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    rust_os::hlt_loop();
}
