#![no_std] //禁用标准库
#![no_main] //覆盖默认入口点，因为没有调用它的底层运行时
// 禁用标准库之后，需要添加编译器在panic时应该调用的函数
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info:&PanicInfo)->!{
    loop {}
}
static HELLO:&[u8] = b"Hello World";
#[unsafe(no_mangle)]//确保函数名不被编译器改变
pub extern "C" fn _start() ->!{//类unix操作系统以_start作为入口名称
    // vga字符缓冲区允许操作系统或程序直接操作内存，直接控制显存中的字符和样式，而无需复杂的图形操作
    let vga_buffer = 0xb8000 as *mut u8;//vga缓冲区的地址,并转换为裸指针
    for (i,&byte) in HELLO.iter().enumerate(){//迭代字节
        unsafe {
            // 解引用并将字符串的每个字节赋值给裸指针偏移位置
            *vga_buffer.offset(i as isize * 2) = byte;
            // 添加前景色oxb表示谈青色
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {

    }
}
// 运行命令：rustup target add thumbv7em-none-eabihf下载一个嵌入式ARM系统
// 运行cargo build --target thumbv7em-none-eabihf可以为此目标构建独立的可执行文件