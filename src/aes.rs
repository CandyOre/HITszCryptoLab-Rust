mod round_key;
mod row;
mod block;
mod process;
mod consts;

use crate::cypher::{Encryptor, Decryptor};
use round_key::RoundKey;

use crate::demo::Demo;
use crate::console::io::IOHelper;

#[derive(Default)]
pub struct Aes {
    key: String,
    round_key: RoundKey,
}

impl Aes {
    pub fn new() -> Self {
        Default::default()
    }

    fn acquire_round_key(&mut self) {
        println!("\nCalculating Round Key...");
        self.round_key = RoundKey::new(&self.key);
        IOHelper::print_with_newline(
            IOHelper::make_char_hex(
                self.round_key.w.clone(),
                8,
            ),
            4
        );
    }
}

impl Demo for Aes {
    fn get_name(&self) -> String {
        "Advanced Encryption Standard".to_string()
    }

    fn acquire_key(&mut self) {
        self.key = IOHelper::get_string_loop(
            16,
            "\nPlease input key (16 characters long): ".to_string(),
        );
        println!("Your key is {}.", self.key);
        self.acquire_round_key();
    }

    fn get_encryptor(&self) -> &dyn Encryptor {
        &self.round_key
    }

    fn get_decryptor(&self) -> &dyn Decryptor {
        &self.round_key
    }
}