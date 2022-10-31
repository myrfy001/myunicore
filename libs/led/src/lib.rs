#![no_std]

use core::{
    fmt::{Arguments, Write},
    str::FromStr,
    mem::MaybeUninit,
};


use port_impl_export::{PortInterface, PortIFConsoleIO, PortIFTimer, PortIFGPIO};
use port_impl_export::PortableImpl;

static mut PORT_IMPL: MaybeUninit<&'static PortableImpl> = MaybeUninit::uninit();

struct Led {}


pub fn init(imp: &'static PortableImpl) {
    unsafe {PORT_IMPL.write(imp)};
}



pub fn led_on() {
    let x = unsafe{PORT_IMPL.assume_init()};
    x.write_pin(<PortableImpl as PortIFGPIO>::PinList::Led, true);

}

pub fn led_off() {
    let x = unsafe{PORT_IMPL.assume_init()};
    x.write_pin(<PortableImpl as PortIFGPIO>::PinList::Led, false);
}