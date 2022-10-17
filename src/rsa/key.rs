use num_bigint::BigUint;
use num_traits::One;

use super::algorithm::{new_prime, inverse, find_coprime};

use crate::cypher::{Encryptor, Decryptor};

#[derive(Default)]
pub struct RsaPrivateKey {
    pub(crate) public_component: RsaPublicKey,
    d: BigUint,
}

#[derive(Default, Clone)]
pub struct RsaPublicKey {
    n: BigUint,
    e: BigUint,
    bits: usize,
}

impl RsaPrivateKey {
    #[allow(dead_code)]
    pub fn new(bits: usize) -> Option<Self> {
        let one = BigUint::one();
        let p = new_prime(bits);
        let q = new_prime(bits);
        let n = &p * &q;
        let phi = (&p - &one) * (&q - &one);
        let e = match find_coprime(&phi) {
            Some(res) => res,
            None => return None,
        };
        let d = inverse(&phi, &e);
        let public_component = RsaPublicKey {n, e, bits};
        Some(Self {public_component, d})
    }

    pub fn new_with_print(bits: usize) -> Option<Self> {
        let one = BigUint::one();
        let p = new_prime(bits);
        println!("p: {p}");
        let q = new_prime(bits);
        println!("q: {q}");
        let n = &p * &q;
        println!("n: {n}");
        let phi = (&p - &one) * (&q - &one);
        println!("phi(n): {phi}");
        let e = match find_coprime(&phi) {
            Some(res) => res,
            None => return None,
        };
        println!("e: {e}");
        let d = inverse(&phi, &e);
        println!("d: {d}");
        let public_component = RsaPublicKey {n, e, bits};
        Some(Self {public_component, d})
    }
}

impl RsaPublicKey {
    pub fn new(private_key: &RsaPrivateKey) -> Self {
        private_key.public_component.clone()
    }
}

impl Encryptor for RsaPublicKey {
    fn encrypt(&self, plain: &String) -> Vec<u8> {
        let step = self.bits * 2 / 8 - 1;

        let plain = align_string(plain, step);
        let bytes = plain.as_bytes();

        let mut enc = Vec::with_capacity(plain.len());
        for i in (0..plain.len()).step_by(step) {
            let mut msg = BigUint::from_bytes_be(&bytes[i..i+step]);
            msg = msg.modpow(&self.e, &self.n);
            enc.append(&mut align_vec(&msg.to_bytes_le(), step + 1))
        };
        enc
    }
}

impl Decryptor for RsaPrivateKey {
    fn decrypt(&self, cypher: &Vec<u8>) -> String {
        let step = self.public_component.bits * 2 / 8;

        let mut plain = String::new();
        for i in (0..cypher.len()).step_by(step) {
            let mut msg = BigUint::from_bytes_le(&cypher[i..i+step]);
            msg = msg.modpow(&self.d, &self.public_component.n);

            let mut bytes = msg.to_bytes_le();
            bytes.reverse();
            let bytes = bytes.into_iter().filter(|x| x > &0).collect();

            plain.push_str(
                &String::from_utf8(bytes)
                .ok().expect("Malicious Cypher")
            );
        }
        plain
    }
}

fn align_string(plain: &String, step: usize) -> String {
    let mut plain = plain.clone();
    let left = (step - plain.len() % step) % step;
    for _ in 0..left {
        plain.push(0 as char)
    }
    plain
}

fn align_vec(cypher: &Vec<u8>, step: usize) -> Vec<u8> {
    let mut cypher = cypher.clone();
    let left = (step - cypher.len() % step) % step;
    for _ in 0..left {
        cypher.push(0u8);
    }
    cypher
}
