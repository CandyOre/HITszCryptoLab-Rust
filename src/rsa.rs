use crate::demo::Demo;
use crate::cypher::{Encryptor, Decryptor};

mod key;
mod algorithm;

use key::{RsaPrivateKey, RsaPublicKey};

#[derive(Default)]
pub struct Rsa {
    bits: usize,
    private_key: RsaPrivateKey,
    public_key: RsaPublicKey,
}

impl Rsa {
    pub fn new() -> Self {
        let mut new: Self = Default::default();
        new.bits = 1024;
        new
    }
}

impl Demo for Rsa {
    fn get_name(&self) -> String {
        "RSA".to_string()
    }

    fn acquire_key(&mut self) {
        self.private_key = RsaPrivateKey::new_with_print(self.bits)
            .expect("RSA Demo: Error generating private key");
        self.public_key = RsaPublicKey::new(&self.private_key);
    }

    fn get_encryptor(&self) -> &dyn Encryptor {
        &self.public_key
    }

    fn get_decryptor(&self) -> &dyn Decryptor {
        &self.private_key
    }
}
