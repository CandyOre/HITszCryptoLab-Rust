mod console;
mod demo;
mod aes;

use demo::Demo;
use aes::Aes;

fn main() {
    Aes::start_demo();
}
