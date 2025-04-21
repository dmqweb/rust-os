#![no_std] //禁用标准库
#![no_main] //覆盖默认入口点，因为没有调用它的底层运行时
#![feature(custom_test_frameworks)] //由于rust测试框架属于标准库，启动自定义测试框架功能
#![test_runner(crate::test_runner)] //自定义测试运行器
#![reexport_test_harness_main = "test_main"] //测试入口改为test_main函数
mod vga_buffer;
mod serial;
use core::fmt::Write;
// 禁用标准库之后，需要添加编译器在panic时应该调用的函数
use core::panic::PanicInfo;
#[cfg(test)]//条件编译、使测试模式下使用不同的panic处理方式
#[panic_handler]//panic发生时调用的函数
fn panic(info:&PanicInfo) -> ! {
    serial_println!("[failed]\n");//使用serial_println让失败的test显示在命令行中
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
static HELLO:&[u8] = b"Hello World";
#[unsafe(no_mangle)]//确保函数名不被编译器改变
pub extern "C" fn _start() ->!{//类unix操作系统以_start作为入口名称
    println!("hello world{}","!");
    #[cfg(test)] //条件编译
    test_main();
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {//测试运行器
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);//运行完测试之后退出qemu，防止无限递归
}
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        //在oxf4处创建一个新的端口，写入状态码
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
pub trait Testable {
    fn run(&self) -> ();
}
impl<T> Testable for T where T: Fn() {
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}