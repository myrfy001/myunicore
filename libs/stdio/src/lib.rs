#![no_std]

use core::{
    fmt::{Arguments, Write},
    str::FromStr,
    mem::MaybeUninit,
};


use port_impl_export::{PortInterface, PortIFConsoleIO, PortIFTimer};
use port_impl_export::PortableImpl;

static mut PORT_IMPL: MaybeUninit<&'static PortableImpl> = MaybeUninit::uninit();

struct Console {}


pub fn init(imp: &'static PortableImpl) {
    unsafe {PORT_IMPL.write(imp)};
}


impl Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let x = unsafe{PORT_IMPL.assume_init()};
        x.console_put_str(s);
        Ok(())
    }
}


#[inline]
pub fn _print(args: Arguments) {
    Console{}.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! println {
    ($($args:tt)*) => {
        $crate::_print(core::format_args!($($args)*))
    }
}