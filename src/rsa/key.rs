use crate::cypher::{Encryptor, Decryptor};

#[derive(Default)]
pub struct RsaPrivateKey {

}

#[derive(Default)]
pub struct RsaPublicKey {

}

impl RsaPrivateKey {
    pub fn new(bits: usize) -> Option<Self> {
        todo!()
    }
}

impl RsaPublicKey {
    pub fn new(private_key: &RsaPrivateKey) -> Self {
        todo!()
    }
}

impl Encryptor for RsaPublicKey {
    fn encrypt(&self, plain: &String) -> Vec<u8> {
        todo!()
    }
}

impl Decryptor for RsaPrivateKey {
    fn decrypt(&self, cypher: &Vec<u8>) -> String {
        todo!()
    }
}

