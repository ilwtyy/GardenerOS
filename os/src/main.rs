#![no_std]
#![no_main]

mod console;
mod lang_items;
mod sbi;

use core::arch::global_asm;

// 引入汇编启动代码
global_asm!(include_str!("entry.asm"));

// 清空 .bss 段
fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }

    let start = sbss as *const () as usize;
    let end = ebss as *const () as usize;

    (start..end).for_each(|a| unsafe {
        (a as *mut u8).write_volatile(0);
    });
}

// Rust 内核主函数
#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    unsafe extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack();
        fn boot_stack_top();
    }

    clear_bss();

    println!("Hello, world!");
    println!("Student ID: {}", 24301009);

    println!(
        ".text [{:#x}, {:#x})",
        stext as *const () as usize,
        etext as *const () as usize
    );

    println!(
        ".rodata [{:#x}, {:#x})",
        srodata as *const () as usize,
        erodata as *const () as usize
    );

    println!(
        ".data [{:#x}, {:#x})",
        sdata as *const () as usize,
        edata as *const () as usize
    );

    println!(
        "boot_stack [{:#x}, {:#x})",
        boot_stack as *const () as usize,
        boot_stack_top as *const () as usize
    );

    println!(
        ".bss [{:#x}, {:#x})",
        sbss as *const () as usize,
        ebss as *const () as usize
    );

    println!("Hello, world!");

    panic!("Shutdown machine!");
}
