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

    fn acquire_cypher(&mut self) {
        println!("\nEncrypting...");
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
            "Please input save path: ".to_string(),
        );
        if let Err(e) = IOHelper::write_file(
            &self.save_path,
            &self.cypher,
        ) {
            println!("Error occur during writing file.");
            panic!("Aes Demo - write_file: {}", e.to_string());
        }
    }

    fn read_file(&mut self) -> bool {
        if "1" != IOHelper::get_string_loop(
            0, 
            "Input \"1\" to continue decypher demo: ".to_string()
        ) {
            false
        } else {
            self.save_path = IOHelper::get_string_loop(
                0,
                "Please input save path: ".to_string(),
            );
            match IOHelper::read_file(&self.save_path) {
                Ok(content) =>
                    self.cypher = content.to_vec(),
                Err(e) => {
                    println!("Error occur during reading file.");
                    panic!("Aes Demo - read_file: {}", e.to_string());
                }
            }
            true
        }
    }

    fn decrypt_plain(&mut self) {
        println!("\nDecrypting...");
        self.plain = process::decrypt(&self.round_key, &self.cypher);
        println!("The plain text as ASCII is:");
        IOHelper::print_with_newline(
            IOHelper::make_char_hex(
                self.plain.as_bytes(),
                2),
            16
        );
        println!("The plain text is \"{}\".", self.plain);
    }

    fn decrypt_write(&self) {
        if let Err(e) = IOHelper::write_file(
            &(self.save_path.clone() + ".txt"),
            &self.plain.as_bytes(),
        ) {
            println!("Error occur during writing file.");
            panic!("Aes Demo - write_file: {}", e.to_string());
        }
        println!("\nThe plain text is available at \"{}.txt\".", &self.save_path);
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
        if self.read_file() {
            self.decrypt_plain();
            self.decrypt_write();
        }
    }
}