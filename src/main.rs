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

use console::io::IOHelper;
use demo::Demo;
use aes::Aes;
use rsa::Rsa;

fn main() {
    loop {
        match IOHelper::get_string_loop(
            0,
            "\n".to_string()
            + "Enter a number to choose demo\n"
            + "  1 - AES\n"
            + "  2 - RSA (Default)\n"
            + "Your choice is: "
        ).parse::<usize>().unwrap_or(2) {
            1 => Aes::new().start_demo(),
            2
            |_ => Rsa::new().start_demo(),
        };
        if IOHelper::get_string_loop(
            0,
            "\nEnter 1 to choose another demo: ".to_string()
        ) != "1" {
            break;
        }
    }
}
