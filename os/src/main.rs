#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use core::fmt::{self, Write};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// RISC-V Linux 系统调用编号
const SYSCALL_EXIT: usize = 93;
const SYSCALL_WRITE: usize = 64;

// 通用系统调用函数
fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut a0 = args[0];

    unsafe {
        asm!(
            "ecall",
            inlateout("a0") a0,
            in("a1") args[1],
            in("a2") args[2],
            in("a7") id,
            options(nostack)
        );
    }

    a0 as isize
}

// 退出系统调用
pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

// 输出系统调用
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(
        SYSCALL_WRITE,
        [fd, buffer.as_ptr() as usize, buffer.len()]
    )
}

// 实现 Write Trait
struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let ret = sys_write(1, s.as_bytes());
        if ret == s.len() as isize {
            Ok(())
        } else {
            Err(fmt::Error)
        }
    }
}

// 封装格式化输出函数
pub fn print(args: fmt::Arguments<'_>) {
    let mut stdout = Stdout;
    stdout.write_fmt(args).unwrap();
}

// 实现 print! 宏
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::print(format_args!($($arg)*))
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

// 程序入口
#[unsafe(no_mangle)]
extern "C" fn _start() {
    println!("Hello, world!");
    println!("Student ID: {}", 24301009);
    sys_exit(9);
}
