use crate::demo::Demo;
use crate::cypher::{Encryptor, Decryptor};

mod key;

use key::{RsaPrivateKey, RsaPublicKey};


#[derive(Default)]
pub struct Rsa {
    bits: usize,
    private_key: RsaPrivateKey,
    public_key: RsaPublicKey,
    plain: String,
    cypher: Vec<u8>,
    decoded: String,
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
        "RSA".to_string()
    }

    fn start_demo(&mut self) {
        self.private_key = RsaPrivateKey::new(self.bits).expect("RSA Demo: Error generating private key");
        self.public_key = RsaPublicKey::new(&self.private_key);
        self.cypher = self.public_key.encrypt(&self.plain);
        self.decoded = self.private_key.decrypt(&self.cypher);
    }
}