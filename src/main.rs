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
    use rust_os::memory::active_layer_4_table;
    use x86_64::structures::paging::PageTable;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    rust_os::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_layer_4_table(phys_mem_offset) };
    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
            let phys = entry.frame().unwrap().start_address();
            let virt = phys.as_u64() + boot_info.physical_memory_offset;
            let ptr = VirtAddr::new(virt).as_mut_ptr();
            let l3_table: &PageTable = unsafe { &*ptr };
            for (i, entry) in l3_table.iter().enumerate() {
                if !entry.is_unused() {
                    println!(" L3 Entry {}: {:?}", i, entry);
                }
            }
        }
    }

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );

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
