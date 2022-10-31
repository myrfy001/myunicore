#![no_std]
#![no_main]

extern crate pre_main;
use led;

#[no_mangle]
fn main() {
    loop {
        led::led_on();
        for _ in 1..10000 {}
        led::led_off();
        for _ in 1..10000 {}
    }
}
