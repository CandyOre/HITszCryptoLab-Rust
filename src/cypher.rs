pub trait Encryptor {
    fn encrypt(&self, plain: &String) -> Vec<u8>;
}

pub trait Decryptor {
    fn decrypt(&self, cypher: &Vec<u8>) -> String;
}