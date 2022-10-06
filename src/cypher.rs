pub trait Encryptor {
    fn encrypt(plain: &String) -> Vec<u8>;
}

pub trait Decryptor {
    fn decrypt(cypher: &Vec<u8>) -> String;
}