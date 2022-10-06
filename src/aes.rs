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
    plain: String,
    round_key: RoundKey,
    cypher: Vec<u8>,
    save_path: String,
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

    fn start_demo(&mut self) {
        self.acquire_key();
        self.plain =  Aes::acquire_plain();
        self.acquire_cypher();
        Aes::write_file(&mut self.save_path, &self.cypher);
        if Aes::read_file(&mut self.save_path, &mut self.cypher) {
            self.decrypt_plain();
            Aes::decrypt_write(&self.save_path, &self.plain);
        }
    }

    fn acquire_key(&mut self) {
        self.key = IOHelper::get_string_loop(
            16,
            "\nPlease input key (16 characters long): ".to_string(),
        );
        println!("Your key is {}.", self.key);
        self.acquire_round_key();
    }

    fn acquire_cypher(&mut self) {
        println!("\nEncrypting...");
        self.cypher = self.round_key.encrypt(&self.plain);
        println!("The cypher text is: ");
        IOHelper::print_with_newline(
            IOHelper::make_char_hex(self.cypher.clone(), 2),
            16
        );
    }

    fn decrypt_plain(&mut self) {
        println!("\nDecrypting...");
        self.plain = self.round_key.decrypt(&self.cypher);
        println!("The plain text as ASCII is:");
        IOHelper::print_with_newline(
            IOHelper::make_char_hex(
                self.plain.as_bytes(),
                2),
            16
        );
        println!("The plain text is \"{}\".", self.plain);
    }
}