#![no_std]

#![feature(naked_functions, asm_sym, asm_const)]
#![feature(linkage)]

extern crate riscv;
// extern crate riscv_rt;

use spin::{Once, Mutex};


pub struct PortableCH32V307Impl;



impl interface::PortIFSystemSetup for PortableCH32V307Impl {
    fn system_init(&self) {
        let peripherals = unsafe{ch32v3::Peripherals::steal()};
        peripherals.RCC.apb2pcenr.modify(|_,w| w.iopden().set_bit());
        peripherals.GPIOD.cfghr.modify(|_, w| unsafe{w.mode14().bits(0x03).cnf14().bits(0x01)});

    }
}

impl interface::PortIFConsoleIO for PortableCH32V307Impl {
    #[inline]
    fn console_getchar(&self) -> u8 {
        return 0;
    }

    #[inline]
    fn console_putchar(&self, c: u8) {
        
    }
}

impl interface::PortIFTimer for PortableCH32V307Impl {

}

pub enum PinList {
    Led = 1
}

impl core::convert::Into<usize> for PinList {
    fn into(self) -> usize {
        return self as usize
    }
}

impl interface::PortIFGPIO for PortableCH32V307Impl {
    type PinList = PinList;
    fn write_pin(&self, pin_id: Self::PinList, value: bool) {
        let peripherals = unsafe{ch32v3::Peripherals::steal()};
        match pin_id {
            PinList::Led => peripherals.GPIOD.outdr.modify(|_,w| w.odr14().bit(value)),
        };
    }
 
}

impl interface::PortInterface for PortableCH32V307Impl{}

impl PortableCH32V307Impl {
    pub fn new() -> Self {
        return PortableCH32V307Impl{}
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