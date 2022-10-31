
#![no_std]

pub trait PortIFSystemSetup {
    fn system_init(&self);
}

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

pub trait PortIFGPIO {
    type PinList: core::convert::Into<usize>;
    fn write_pin(&self, pin_id: Self::PinList, value: bool);
}

pub trait PortInterface:
    PortIFSystemSetup +
    PortIFConsoleIO + 
    PortIFTimer + 
    PortIFGPIO
    {}