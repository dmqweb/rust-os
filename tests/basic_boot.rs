#![no_std] //集成测试都是单独的可执行文件，需要再次声明
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(dead_code,unused_variables,unused)]
use core::panic::PanicInfo;
#[unsafe(no_mangle)] // 编译时保持函数名
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}
fn test_runner(tests: &[&dyn Fn()]) {
    unimplemented!();
}
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}