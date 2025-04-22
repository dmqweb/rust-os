#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"] //设定测试时的入口函数名
use core::panic::PanicInfo;
use blog_os::println;
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    blog_os::init();
    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42;
    // };
    #[cfg(test)] //条件编译，在运行cargo test时执行test_main代码
    test_main();
    println!("It did not crash!");
    loop {}
}
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}