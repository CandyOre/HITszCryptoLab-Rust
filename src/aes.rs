mod round_key;
mod row;
mod block;
mod process;
mod consts;

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

    fn acquire_round_key(&mut self) {
        println!("Calculating Round Key...");
        self.round_key = RoundKey::new(&self.key);
        IOHelper::print_with_newline(
            IOHelper::make_char_hex(
                self.round_key.w.clone(),
                8,
            ),
            4
        );
    }

    fn acquire_cypher(&mut self) {
        println!("Encrypting...");
        self.cypher = process::encrypt(&self.round_key, &self.plain);
        println!("The cypher text is: ");
        IOHelper::print_with_newline(
            IOHelper::make_char_hex(self.cypher.clone(), 2),
            16
        );
    }

    fn write_file(&mut self) {
        self.save_path = IOHelper::get_string_loop(
            0,
            "Please input saved path: ".to_string(),
        );
        if let Err(_) = IOHelper::write_file(
            &self.save_path,
            &self.cypher,
        ) {
            println!("Error occur during writing file.");
            panic!("Aes Demo: write_file");
        }
    }

    fn try_decrypt(&self) {
        
    }

}

impl Demo for Aes {
    fn get_name(&self) -> String {
        "Advanced Encryption Standard".to_string()
    }

    fn start_demo(&mut self) {
        self.acquire_key();
        self.acquire_plain();
        self.acquire_round_key();
        self.acquire_cypher();
        self.write_file();
        self.try_decrypt();
    }
}