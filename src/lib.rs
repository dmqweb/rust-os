#![no_std] //分割main.rs中公共部分为库，提供给主逻辑和tests使用
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)] //x86-interrupt特性不稳定，需要手动启用
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;
pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;
pub mod memory;
pub trait Testable {
    fn run(&self) -> ();
}
#[cfg(test)]//测试模式下的入口点
use bootloader::{entry_point, BootInfo};
#[cfg(test)]
entry_point!(test_kernel_main);
#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    // like before
    init();
    test_main();
    hlt_loop();
}
impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("lib.rs中run函数：{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}
pub fn test_panic_handler(info: &PanicInfo) -> ! {//格式化错误信息
    serial_println!("[test_panic_handler失败]\n");
    serial_println!("错误信息: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}
#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    init();
    #[cfg(test)]
    test_main();
    hlt_loop();
}
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info) //测试double fault（当panic时再次panic就会触发）
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
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
pub fn init() {//main.rs、lib.rs及单元测试共享的初始化逻辑
    gdt::init();
    interrupts::init_idt();
    // exit_qemu(QemuExitCode::Success);//退出qemu,不然会触发panic，和test_panic_handler导致double fault
    unsafe { interrupts::PICS.lock().initialize() };//进行PIC初始化
    x86_64::instructions::interrupts::enable();//启用中断
}
pub fn hlt_loop() -> ! {//fix:当前cpu会高速运转，QEMU的CPU占用率高达100%
    loop {
        x86_64::instructions::hlt();//hlt指令可以让CPU在下一个中断触发之前休息一下，进入休眠状态节省能源
    }
}