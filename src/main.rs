#![no_std] //禁用标准库
#![no_main] //覆盖默认入口点，因为没有调用它的底层运行时
mod vga_buffer;
// 禁用标准库之后，需要添加编译器在panic时应该调用的函数
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info:&PanicInfo)->!{
    loop {}
}
static HELLO:&[u8] = b"Hello World";
#[unsafe(no_mangle)]//确保函数名不被编译器改变
pub extern "C" fn _start() ->!{//类unix操作系统以_start作为入口名称
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    loop {}
}
// 运行命令：rustup target add thumbv7em-none-eabihf下载一个嵌入式ARM系统
// 运行cargo build --target thumbv7em-none-eabihf可以为此目标构建独立的可执行文件