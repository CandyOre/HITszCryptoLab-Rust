mod round_key;
mod state;
mod process;

use round_key::ROUNDKEY;

use crate::demo::Demo;
use crate::console::io::IOHelper;

pub struct Aes {
    key: String,
    plain: String,
    cypher: Vec<u8>,
}

impl Aes {
    pub fn new() -> Self {
        Aes {
            key: String::new(),
            plain: String::new(),
            cypher: Vec::new(),
        }
    }

    fn acquire_key(&mut self) {
        self.key = IOHelper::get_string_loop(
            16,
            "Please input key (16 characters long): ".to_string(),
        );
        println!("Your key is {}.", self.key);
    }

    fn acquire_plain(&mut self) {
        self.plain = IOHelper::get_string_loop(
            0,
            "Please input plain text: ".to_string(),
        )
    }

    fn show_round_key() {
        println!("Calculating Round Key...");
        unimplemented!();
    }

    fn acquire_cypher(&mut self) {
        println!("Encrypting...");
        let cypher = process::encrypt(&self.key, &self.plain);
        println!("The cypher text is: ");
        unimplemented!();
    }

    fn write_file(&self) {
        unimplemented!();
    }

    fn try_decrypt(&self) {
        unimplemented!();
    }

}

impl Demo for Aes {
    fn get_name(&self) -> String {
        "Advanced Encryption Standard".to_string()
    }

    fn start_demo(&mut self) {
        self.acquire_key();
        self.acquire_plain();
        Aes::show_round_key();
        self.acquire_cypher();
        self.write_file();
        self.try_decrypt();
    }
}