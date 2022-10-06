mod round_key;
mod row;
mod block;
mod process;
mod consts;

use crate::cypher::{Encryptor, Decryptor, CypherCommon};
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

    fn acquire_cypher(&mut self, common: &mut CypherCommon) {
        println!("\nEncrypting...");
        common.cypher = self.round_key.encrypt(&common.plain);
        println!("The cypher text is: ");
        IOHelper::print_with_newline(
            IOHelper::make_char_hex(common.cypher.clone(), 2),
            16
        );
    }

    fn decrypt_plain(&mut self, common: &mut CypherCommon) {
        println!("\nDecrypting...");
        common.plain = self.round_key.decrypt(&common.cypher);
        println!("The plain text as ASCII is:");
        IOHelper::print_with_newline(
            IOHelper::make_char_hex(
                common.plain.as_bytes(),
                2),
            16
        );
        println!("The plain text is \"{}\".", common.plain);
    }
}