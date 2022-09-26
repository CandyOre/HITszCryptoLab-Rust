mod console;
mod demo;
mod aes;

use demo::Demo;

fn main() {
    let my_demo = aes::Aes::new();
    my_demo.start_demo();
}
