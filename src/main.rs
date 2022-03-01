#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点

mod vga_buffer;

use core::panic::PanicInfo;

// panic 时调用该函数
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // 不重整函数名
             // 因为编译器会寻找名称为 _start 的函数，所以该函数为入口点

pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    println!("Hello World{}", "!");
    panic!("Some panic message");
    loop {}
}
