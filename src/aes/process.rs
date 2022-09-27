use std::ops::Mul;

use crate::console::io::IOHelper;

use super::round_key::RoundKey;
use super::block::Block;
use super::consts::{COLM, COLM_INV};

pub fn encrypt(round_key: &RoundKey, plain: &String) -> Vec<u8> {
    let plain = aligned(&plain);
    let mut enc = Vec::with_capacity(plain.len());
    for i in (0..plain.len()).step_by(16) {
        let mut plain_block = Block::from_column_iter(
            plain[i..i+16].to_string().chars()
        ).unwrap() ^ round_key.k[0];
        for j in 1..10 {
            plain_block = COLM.mul(plain_block.sub().shift()) ^ round_key.k[j];
        }
        plain_block = plain_block.sub().shift() ^ round_key.k[10];
        for e in plain_block.into_column_iter() {
            enc.push(e);
        }
    }
    enc
}

fn aligned(plain: &String) -> String {
    let mut plain = plain.clone();
    let left = (16 - plain.len() % 16) % 16;
    plain.push_str(&"x".repeat(left));
    plain
}