use core::ptr;
const GPIO_BASE: usize = 0xFE200000;  // GPIO base address
const GPFSEL1: usize = GPIO_BASE + 0x04; // GPIO Function Select 1

pub fn gpio_set_alt_func(pin: u32, alt: u32) {
    let register = GPFSEL1;
    let shift = (pin % 10) * 3;
    
    unsafe {
        let gpfsel = register as *mut u32;
        let value = ptr::read_volatile(gpfsel);
        ptr::write_volatile(gpfsel, (value & !(0b111 << shift)) | (alt << shift));
    }
}

// Set GPIO14 and GPIO15 to ALT0 (UART)
pub fn gpio_init_uart() {
    gpio_set_alt_func(14, 0b100);  // GPIO14 -> TXD0
    gpio_set_alt_func(15, 0b100);  // GPIO15 -> RXD0
}