#![no_std]
#![feature(linkage)]


use port_impl_export::{PortableImpl, PortIFSystemSetup};
use stdio;

#[linkage = "weak"]
#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    loop {}
}

static mut PORTABLE_IMPL: PortableImpl = PortableImpl{};


#[linkage = "weak"]
#[no_mangle]
pub fn main() {}


#[no_mangle]
pub fn rust_main() {
    unsafe {PORTABLE_IMPL.system_init()};
    stdio::init(unsafe {&PORTABLE_IMPL});
    main();
}