#![no_std]

#![feature(naked_functions, asm_sym, asm_const)]
#![feature(linkage)]

use spin::{Once, Mutex};
use uart_16550::MmioSerialPort;


pub struct PortableQemuImpl;

static UART0: Once<Mutex<MmioSerialPort>> = Once::new();

impl interface::PortIFConsoleIO for PortableQemuImpl {
    #[inline]
    fn console_getchar(&self) -> u8 {
        UART0.wait().lock().receive()
    }

    #[inline]
    fn console_putchar(&self, c: u8) {
        UART0.wait().lock().send(c)
    }
}

impl interface::PortIFTimer for PortableQemuImpl {

}

impl interface::PortInterface for PortableQemuImpl{}

impl PortableQemuImpl {
    pub fn new() -> Self {
        return PortableQemuImpl{}
    }

    pub fn init(&mut self) {
        UART0.call_once(|| Mutex::new(unsafe { MmioSerialPort::new(0x1000_0000) }));
    }
}



#[link_section = ".text.entry"]
#[no_mangle]
#[naked]
unsafe extern "C" fn _start() -> ! {
    const STACK_SIZE: usize = 4096;

    #[link_section = ".bss.uninit"]
    static mut STACK: [u8; STACK_SIZE] = [0u8; STACK_SIZE];

    core::arch::asm!(
        "la sp, {stack} + {stack_size}",
        "j  {main}",
        stack_size = const STACK_SIZE,
        stack      =   sym STACK,
        main       =   sym rust_main,
        options(noreturn),
    )
}

#[linkage = "weak"]
#[no_mangle]
extern "C" fn rust_main() {
    
}