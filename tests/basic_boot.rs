#![no_std] //集成测试都是单独的可执行文件，需要再次声明
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(dead_code,unused_variables,unused)]
use core::panic::PanicInfo;
use blog_os::{exit_qemu, QemuExitCode};

#[unsafe(no_mangle)] // 编译时保持函数名
pub extern "C" fn _start(){
    test_main();
    exit_qemu(QemuExitCode::Success);//运行后退出qemu
    // loop {}
}
fn test_runner(tests: &[&dyn Fn()]) {
    // unimplemented!();//标记尚未实现的代码，用于占据未实现的功能
}
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    panic(info);
    loop {}
}