use core::arch::asm;

pub struct SerialPort {
    port: u16,
}

impl SerialPort {
    pub const COM1: Self = Self { port: 0x3F8 };

    pub unsafe fn init(&self) {
        self.outb(1, 0x00); // Disable interrupts
        self.outb(3, 0x80); // Enable DLAB
        self.outb(0, 0x03); // Divisor low byte (38400 baud)
        self.outb(1, 0x00); // Divisor high byte
        self.outb(3, 0x03); // 8 bits, no parity, one stop bit
        self.outb(2, 0xC7); // Enable FIFO
        self.outb(4, 0x0B); // IRQs enabled, RTS/DSR set
    }

    pub unsafe fn write_byte(&self, b: u8) {
        // Wait for transmit holding register empty (bit 5 of Line Status Register)
        while (self.inb(5) & 0x20) == 0 {}
        self.outb(0, b);
    }

    pub fn write_str(&self, s: &str) {
        for b in s.bytes() {
            unsafe {
                if b == b'\n' {
                    self.write_byte(b'\r');
                }
                self.write_byte(b);
            }
        }
    }

    unsafe fn outb(&self, offset: u16, val: u8) {
        asm!("out dx, al", in("dx") self.port + offset, in("al") val, options(nomem, nostack, preserves_flags));
    }

    unsafe fn inb(&self, offset: u16) -> u8 {
        let val: u8;
        asm!("in al, dx", in("dx") self.port + offset, out("al") val, options(nomem, nostack, preserves_flags));
        val
    }
}
