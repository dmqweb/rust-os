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
struct Buffer { //描述字符缓冲区
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
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
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
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
    fn new_line(&mut self) {/* TODO */}
}
pub fn print_something() {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };
    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");//ö是两个字节（utf-8特点9）
}