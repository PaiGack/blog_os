#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点

use core::panic::PanicInfo;

// panic 时调用该函数
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // 不重整函数名
pub extern "C" fn _start() -> ! {
    // 因为编译器会寻找名称为 _start 的函数，所以该函数为入口点

    // VGA 字符缓冲区地址为 0x8000
    // 将 0x8000 转化为裸指针
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        // 所有的裸指针内存操作都被一个 unsafe 语句块包围
        unsafe {
            // offset 偏移裸指针
            // 写入每个字节和对应的颜色
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // 0xb 代表淡青色
        }
    }
    loop {}
}
