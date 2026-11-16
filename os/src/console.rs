use crate::sbi::console_putchar;
use core::fmt::{self, Write};

// 定义标准输出结构体
struct Stdout;

// 实现 Write Trait
impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

// 格式化输出函数
pub fn print(args: fmt::Arguments<'_>) {
    let mut stdout = Stdout;
    stdout.write_fmt(args).unwrap();
}

// 实现 print! 宏
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::console::print(format_args!($($arg)*))
    };
}

// 实现 println! 宏
#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::print!(concat!($fmt, "\n") $(, $arg)*)
    };
}
