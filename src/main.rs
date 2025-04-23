#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"] //设定测试时的入口函数名
use core::panic::PanicInfo;
use blog_os::{println};
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    blog_os::init();
    // 内核中页表的存储方式：
    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    // 分页功能启动时，直接访问物理内存是禁止的，否则程序就很容易侵入其他程序的内存，可以构建一个指向物理地址的虚拟页
    println!("4级页表地址： {:?}", level_4_page_table.start_address());
    let ptr = 0x2031b2 as *mut u8;//测试异常的虚拟地址
    unsafe {
        let x = *ptr;
        println!("读取正常:{}",x);
    }
    unsafe {*ptr = 42;}
    println!("写入失败，page fault，因为该页设置了只读权限");
    #[cfg(test)] //条件编译，在运行cargo test时执行test_main代码
    test_main();
    println!("It did not crash!");
    blog_os::hlt_loop();
}
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("main.rs中panic：{}", info);
    blog_os::hlt_loop();
}