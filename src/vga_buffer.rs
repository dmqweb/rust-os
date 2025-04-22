use core::fmt;
use lazy_static::lazy_static;
use volatile::Volatile;
use spin::Mutex;

#[allow(dead_code)]
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
#[repr(u8)] //指定枚举的底层存储为u8类型
pub enum Color{
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);//包装完整的颜色代码字节，包含前景色和背景色
impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]//保证按顺序布局成员变量，从而正确映射内存片段
struct ScreenChar { //描述字符
    ascii_character: u8,
    color_code: ColorCode,
}
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
#[repr(transparent)]
struct Buffer { //描述字符缓冲区，使用Volatile避免写操作被rust编译器优化
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
//vga缓冲区允许操作系统或程序直接操作内存，直接控制显存中的字符和样式，而无需复杂的图形操作
pub struct Writer {//输出字符到屏幕
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}
impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let color_code = self.color_code;
                // 改为调用write以保证rust编译器不会优化这个写操作
                self.buffer.chars[row][col].write(ScreenChar{
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                //vga只支持ASCII和Code page 437的字节，但Rust字符串默认为UTF-8编码
                // 可以是能打印的 ASCII 码字节，也可以是换行符
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // 不包含在上述范围之内的字节
                _ => self.write_byte(0xfe),
            }
        }
    }
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {//写入BUFFER_WIDTH个空格以清空整行
            self.buffer.chars[row][col].write(blank);
        }
    }
}
impl fmt::Write for Writer {//为Writer类型实现write!函数宏
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
//常量求值器会在编译时处理，但还不能在编译时直接转换裸指针到变量的引用，所以使用lazy_static延迟静态变量的初始化
lazy_static! {
     pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {//单例模式
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        // oxb8000是vga缓冲区的地址，将其转为裸指针进行操作
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}
pub fn print_something() {
    use core::fmt::Write;
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };
    writer.write_byte(b'H');
    writer.write_string("ello! ");
    write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
}
#[macro_export]//暴露当前宏到根命名空间，需要注意暴露后导入时要从crate::println根模块导入
macro_rules! print { //定义print宏
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}
#[macro_export]
macro_rules! println { //定义println宏
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}
#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}
#[test_case]
fn test_println_output() {
    let s = "Some test string that fits on a single line";
    println!("{}", s);
    // for (i, c) in s.chars().enumerate() {
    //     let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
    //     assert_eq!(char::from(screen_char.ascii_character), c);
    // }
}