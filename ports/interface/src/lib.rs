
#![no_std]


pub trait PortIFConsoleIO {
    fn console_getchar(&self) -> u8;
    fn console_putchar(&self, c: u8);
    #[inline]
    fn console_put_str(&self, str: &str) {
        for c in str.bytes() {
            self.console_putchar(c);
        }
    }
}

pub trait PortIFTimer {

}

pub trait PortInterface: 
    PortIFConsoleIO + 
    PortIFTimer
    {}