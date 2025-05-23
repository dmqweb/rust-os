// #[should_panic]属性需要标准库支持，因此创建一个集成测试进行支持
#![no_std]
#![no_main]
use core::panic::PanicInfo;
use blog_os::{QemuExitCode, exit_qemu, serial_println, serial_print};
#[unsafe(no_mangle)]
pub extern "C" fn _start() {
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Success);
    loop{}
}
fn should_fail() {
    serial_print!("should_fail... ");
    // assert_eq!(0, 1);
}
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}