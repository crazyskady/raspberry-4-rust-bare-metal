#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[macro_use]
extern crate raspi4_boot;

const PERIPHERAL_BASE: u32 = 0xFE00_0000;

mod gpio;
mod uart;

/* 如果remove掉panic函数，会在编译时报没有panic_handler处理 */
/* 详细见：https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter1/6print-and-shutdown-based-on-sbi.html */
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

entry!(kernel_entry);

fn kernel_entry () -> ! {
    let uart = uart::MiniUart::new();

    uart.init();
    uart.puts("Hello, Steven!\n");

    loop {
        uart.send(uart.getc());
    }
}