#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
use bootloader::{entry_point, BootInfo};
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

entry_point!(kernel_main);
#[no_mangle]
//boot_info can be skipped since the x86_64 convention passes it in a cpu register instead of in stack
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::memory::translate_addr;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    rust_os::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let addresses = [
        0xb8000,                          // VGA buffer
        0x201008,                         // a code page
        0x0100_0020_1a10,                 // a stack page
        boot_info.physical_memory_offset, //virtual address pointing to the start of the physical memory (i.e. physical memory address 0)
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = unsafe { translate_addr(virt, phys_mem_offset) };
        println!("{:?} -> {:?}", virt, phys);
    }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    rust_os::hlt_loop();
}
