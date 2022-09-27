#[macro_use]
extern crate lazy_static;
extern crate num_traits;

mod console;
mod demo;
mod aes;

use demo::Demo;
use aes::Aes;

fn main() {
    let mut aes = Aes::new();
    aes.start_demo();
}
