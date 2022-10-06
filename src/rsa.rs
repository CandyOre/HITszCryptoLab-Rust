use crate::demo::Demo;
use crate::cypher::{Encryptor, Decryptor, CypherCommon};

mod key;

use key::{RsaPrivateKey, RsaPublicKey};

use crate::console::io::IOHelper;

#[derive(Default)]
pub struct Rsa {
    bits: usize,
    private_key: RsaPrivateKey,
    public_key: RsaPublicKey,
}

impl Rsa {
    pub fn new() -> Self {
        let mut new: Self = Default::default();
        new.bits = 2048;
        new
    }
}

impl Demo for Rsa {
    fn get_name(&self) -> String {
        "RSA Demo".to_string()
    }

    fn acquire_key(&mut self) {
        self.private_key = RsaPrivateKey::new(self.bits)
            .expect("RSA Demo: Error generating private key");
        self.public_key = RsaPublicKey::new(&self.private_key);
    }

    fn acquire_cypher(&mut self, common: &mut CypherCommon) {
        println!("\nEncrypting...");
        common.cypher = self.public_key.encrypt(&common.plain);
        println!("The cypher text is: ");
        IOHelper::print_with_newline(
            IOHelper::make_char_hex(common.cypher.clone(), 2),
            16
        );
    }

    fn decrypt_plain(&mut self, common: &mut CypherCommon) {
        println!("\nDecrypting...");
        common.plain = self.private_key.decrypt(&common.cypher);
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