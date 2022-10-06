#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;
extern crate num_traits;
extern crate num_bigint;

mod console;
mod demo;
mod cypher;
mod aes;
mod rsa;

use demo::Demo;
#[allow(unused_imports)]
use aes::Aes;
#[allow(unused_imports)]
use rsa::Rsa;

fn main() {
    let mut demo = Aes::new();
    demo.start_demo();
}
