#![no_std]
#![no_main]

use core::panic::PanicInfo;

const GPIO_BASE: usize = 0xFE200000;  // GPIO base address for Raspberry Pi 5
const GPFSEL1: usize = GPIO_BASE + 0x04; // GPIO Function Select 1

const UART0_BASE: usize = 0xFE201000; // Base address for UART0
const UART_DR: usize = UART0_BASE + 0x00; // Data register
const UART_FR: usize = UART0_BASE + 0x18; // Flag register
const UART_IBRD: usize = UART0_BASE + 0x24; // Integer Baud rate divisor
const UART_FBRD: usize = UART0_BASE + 0x28; // Fractional Baud rate divisor
const UART_LCRH: usize = UART0_BASE + 0x2C; // Line control register
const UART_CR: usize = UART0_BASE + 0x30; // Control register

// UART flags
const FR_TXFF: u32 = 1 << 5;  // Transmit FIFO full

#[no_mangle]
pub extern "C" fn _start() -> ! {
    configure_uart_pins();  // Set up GPIO pins for UART
    uart_init();            // Initialize UART

    loop {
        uart_send(b'H');
        uart_send(b'e');
        uart_send(b'l');
        uart_send(b'l');
        uart_send(b'o');
        uart_send(b'\n');
        delay(1000000);     // Delay between sends
    }
}

pub fn delay(count: u32) {
    for _ in 0..count {
        unsafe { core::arch::asm!("nop"); }  // Simple delay
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub fn uart_init() {
    unsafe {
        // Disable UART while configuring
        core::ptr::write_volatile(UART_CR as *mut u32, 0x0);

        // Set Baud rate divisor to 115200 (assuming core_freq=250 MHz)
        core::ptr::write_volatile(UART_IBRD as *mut u32, 26); // Integer part
        core::ptr::write_volatile(UART_FBRD as *mut u32, 3);  // Fractional part

        // 8-bit word length, enable FIFO
        core::ptr::write_volatile(UART_LCRH as *mut u32, (1 << 4) | (1 << 5) | (1 << 6));

        // Enable UART, TX, and RX
        core::ptr::write_volatile(UART_CR as *mut u32, (1 << 0) | (1 << 8) | (1 << 9));
    }
}

pub fn uart_send(c: u8) {
    unsafe {
        // Wait until the UART is ready to transmit (TX FIFO is not full)
        while core::ptr::read_volatile(UART_FR as *mut u32) & FR_TXFF != 0 {}

        // Write the character to the UART data register
        core::ptr::write_volatile(UART_DR as *mut u32, c as u32);
    }
}

pub fn configure_uart_pins() {
    unsafe {
        let mut gpio_fsel1 = core::ptr::read_volatile(GPFSEL1 as *mut u32);

        // Clear bits 12-17 (for GPIO14 and GPIO15)
        gpio_fsel1 &= !(0b111 << 12);  // Clear GPIO14 bits
        gpio_fsel1 &= !(0b111 << 15);  // Clear GPIO15 bits

        // Set GPIO14 to ALT0 (UART0 TX) (100 in binary)
        gpio_fsel1 |= (0b100 << 12);

        // Set GPIO15 to ALT0 (UART0 RX) (100 in binary)
        gpio_fsel1 |= (0b100 << 15);

        // Write the modified value back to the GPFSEL1 register
        core::ptr::write_volatile(GPFSEL1 as *mut u32, gpio_fsel1);
    }
}