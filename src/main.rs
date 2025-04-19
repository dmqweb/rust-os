#[!no_std] //禁用标准库
// 禁用标准库之后，需要添加编译器在panic时应该调用的函数
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info:&PanicInfo)->!{
    loop {}
}
fn main(){
}