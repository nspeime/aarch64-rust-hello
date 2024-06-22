use core::ptr;

const UARTBASE: usize = 0x0900_0000;
const UARTDR: usize = UARTBASE + 0x000;
const UARTFR: usize = UARTBASE + 0x018;
const UARTCR: usize = UARTBASE + 0x030;

// UART Control Register (UARTCR) bit definitions
const UARTEN: u32 = 1 << 0; // UART enable
const TXE: u32 = 1 << 8; // Transmit enable

// UART Flag Register (UARTFR) bit definitions
const TXFF: u32 = 1 << 5; // Transmit FIFO Full

pub fn uart_init() {
    unsafe {
        // Initialize the UART control register to enable UART and transmission
        ptr::write_volatile(UARTCR as *mut u32, UARTEN | TXE);
    }
}

pub fn uart_print(s: &str) {
    for byte in s.bytes() {
        unsafe {
            // Wait for the transmit FIFO to be empty
            while ptr::read_volatile(UARTFR as *const u32) & TXFF != 0 {}
            // Write the byte to the data register
            ptr::write_volatile(UARTDR as *mut u32, byte as u32);
        }
    }
}
