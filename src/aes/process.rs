use std::ops::Mul;

use crate::console::io::IOHelper;

use super::round_key::RoundKey;
use super::block::Block;
use super::consts::{COLM, COLM_INV};

pub fn encrypt(round_key: &RoundKey, plain: &String) -> Vec<u8> {
    let plain = align_string(&plain);
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

pub fn decrypt(round_key: &RoundKey, cypher: &Vec<u8>) -> String {
    let cypher = align_vec(cypher);
    let mut plain = String::new();
    for i in (0..cypher.len()).step_by(16) {
        let mut cypher_block = Block::from_column_iter(
            cypher[i..i+16].to_vec()
        ).unwrap() ^ round_key.k[10];
        for j in (1..10).rev() {
            cypher_block = COLM_INV.mul(cypher_block.shift_inv().sub_inv() ^ round_key.k[j]);
        }
        cypher_block = cypher_block.shift_inv().sub_inv() ^ round_key.k[0];
        for e in cypher_block.into_column_iter() {
            plain.push(e as char)
        }
    }
    plain
}

fn align_string(plain: &String) -> String {
    let mut plain = plain.clone();
    let left = (16 - plain.len() % 16) % 16;
    plain.push_str(&"x".repeat(left));
    plain
}

fn align_vec(cypher: &Vec<u8>) -> Vec<u8> {
    let mut cypher = cypher.clone();
    let left = (16 - cypher.len() & 16) % 16;
    for _ in 0..left {
        cypher.push(0u8);
    }
    cypher
}