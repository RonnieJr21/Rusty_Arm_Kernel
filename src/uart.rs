// GPIO 14 TXD GPIO 15 RXD ALT0 for both

use core::ptr;

const GPIO_BASE: usize = 0xFE200000; // base address for the gpio registers
const GPFSEL1: usize = GPIO_BASE + 0x04; // address for gpio function select - register 1
const ALTFX:usize = 0x4;


const UART0: usize = 0xFE201000;
const UART_DR: usize = UART0 + 0x00; // Data register
const UART_FR: usize = UART0 + 0x18; // Flag register
const UART_IBRD: usize = UART0 + 0x24; // Integer Buad Rate Divisor IBRD = [(UART clock) / (16 X Baud Rate)]
const UART_FBRD: usize = UART0 + 0x28; // Fractional Buad Rate Divisor IBRD = [(Baud Rate Divisor - IBRD) X 64 ]
const UART_LCRH: usize = UART0 + 0x2C; // Line control Register
const UART_CR: usize = UART0 + 0x30; // Control Register 
const UART_IMSC:usize = UART0 + 0x38; // Interrupt Mask Set/Clear

// UART Flags
const FR_TXFF: u32 = 1 << 5;  // Transmit FIFO full (cannot send more data until space frees up)
const FR_RXFE: u32 = 1 << 4;  // Receive FIFO empty (no data available to read)

pub fn configure_uart_pins() {
    unsafe {
        let mut gpio_fsel1 = ptr::read_volatile(GPFSEL1 as *mut u32);

        // Clear bits 12-17 (for GPIO14 and GPIO15)
        gpio_fsel1 &= !(0b111 << 12);  // Clear GPIO14 bits
        gpio_fsel1 &= !(0b111 << 15);  // Clear GPIO15 bits

        // Set GPIO14 to ALT0 (UART0 TX) (100 in binary)
        gpio_fsel1 |= (0b100 << 12);

        // Set GPIO15 to ALT0 (UART0 RX) (100 in binary)
        gpio_fsel1 |= (0b100 << 15);

        // Write the modified value back to the GPFSEL1 register
        ptr::write_volatile(GPFSEL1 as *mut u32, gpio_fsel1);
    }
}

pub fn uart_init() {
    unsafe{
        // Disable uart while configuring
        ptr::write_volatile(UART_CR as *mut u32, 0x0);


        // Configure BAUD Rate to 115200
        ptr::write_volatile(UART_IBRD as *mut u32, 135);
        ptr::write_volatile(UART_FBRD as *mut u32, 41);

         // 8-bit word length, enable FIFO
         ptr::write_volatile(UART_LCRH as *mut u32, (1 << 4) | (1 << 5) | (1 << 6));

         // Enable UART, TX, and RX
         ptr::write_volatile(UART_CR as *mut u32, (1 << 0) | (1 << 8) | (1 << 9));
     
    }
}

pub fn uart_send(c: u8) {
    unsafe {
        // Wait until the UART is ready to transmit (TX FIFO is not full)
        while ptr::read_volatile(UART_FR as *mut u32) & FR_TXFF != 0 {}

        // Write the character to the UART data register
        ptr::write_volatile(UART_DR as *mut u32, c as u32);
    }
}

pub fn uart_recv() -> u8 {
    unsafe {
        // Wait until the UART has received data (RX FIFO is not empty)
        while ptr::read_volatile(UART_FR as *mut u32) & FR_RXFE != 0 {}

        // Read the character from the UART data register
        ptr::read_volatile(UART_DR as *mut u32) as u8
    }
}

pub fn uart_read_string(buffer: &mut [u8], max_len: usize) -> usize {
    let mut index = 0;

    // Loop until newline or buffer is full
    while index < max_len {
        let byte = uart_recv();  // Receive a byte

        // Check for newline (end of string)
        if byte == b'\n' || byte == b'\r' {
            break;
        }

        // Store the received byte in the buffer
        buffer[index] = byte;
        index += 1;
    }

    // Optionally, null-terminate the string
    if index < max_len {
        buffer[index] = b'\0';  // Null-terminate the string (optional)
    }

    index  // Return the number of bytes read
}