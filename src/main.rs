#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"] //设定测试时的入口函数名
use core::panic::PanicInfo;
use blog_os::{println};
use bootloader::{BootInfo, entry_point};
entry_point!(kernel_main);//entry_point宏为我们定义了真正的低级_start入口点，不需要手动no_mangle了
// #[unsafe(no_mangle)]
// pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
// }
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use blog_os::memory::active_level_4_table;
    use x86_64::VirtAddr;
    println!("Hello World{}", "!");
    blog_os::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };
    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
        use x86_64::structures::paging::PageTable;
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
            // 从entry中获取物理地址并覆盖
            let phys = entry.frame().unwrap().start_address();
            let virt = phys.as_u64() + boot_info.physical_memory_offset;
            let ptr = VirtAddr::new(virt).as_mut_ptr();
            let l3_table: &PageTable = unsafe { &*ptr };
            // 输出3级别页表中的非空entry
            for (i, entry) in l3_table.iter().enumerate() {
                if !entry.is_unused() {
                    println!("  L3 Entry {}: {:?}", i, entry);
                }
            }
        }
    }
    #[cfg(test)]
    test_main();
    println!("It did not crash!");
    blog_os::hlt_loop();

}
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("main.rs中panic：{}", info);
    blog_os::hlt_loop();
}