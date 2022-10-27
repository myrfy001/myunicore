#![no_std]
#![no_main]

use stdio::println;
extern crate pre_main;

#[no_mangle]
fn main() {
    println!("Hello, world!");
}
