#![no_std] //禁用标准库
#![no_main] //覆盖默认入口点，因为没有调用它的底层运行时
mod vga_buffer;
use core::fmt::Write;
// 禁用标准库之后，需要添加编译器在panic时应该调用的函数
use core::panic::PanicInfo;
#[panic_handler]//panic发生时调用的函数
fn panic(info:&PanicInfo)->!{
    println!("{}",info);
    loop {}
}
static HELLO:&[u8] = b"Hello World";
#[unsafe(no_mangle)]//确保函数名不被编译器改变
pub extern "C" fn _start() ->!{//类unix操作系统以_start作为入口名称
    println!("hello world{}","!");
    panic!("some panic info");
    loop {}
}