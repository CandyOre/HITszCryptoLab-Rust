use crate::aes::consts::RCON;

use super::row::Row;
use super::block::Block;

#[derive(Default)]
pub struct RoundKey {
    pub w: Vec<Row>,
    pub k: Vec<Block>,
}

impl RoundKey {

    pub fn new(key: &String) -> Self {
        let w = RoundKey::new_w(key);
        let k = RoundKey::new_k(&w);
        RoundKey { w, k }
    }

    fn new_w(key: &String) -> Vec<Row> {
        assert_eq!(key.len(), 16);

        let mut w = Vec::new();
        w.resize(44, Default::default());

        for i in 0..4 {
            w[i] = Row::from_iter(key[i*4..i*4+4].to_string().chars()).unwrap();
        }

        for i in 1..=10 {
            w[i*4] = w[i*4-4] ^ (w[i*4-1].rotate().sub() ^ RCON[i-1]);
            for j in 1..4 {
                w[i*4+j] = w[i*4+j-4] ^ w[i*4+j-1];
            }
        }

        w
    }

    fn new_k(w: &Vec<Row>) -> Vec<Block> {
        let mut k = Vec::new();
        k.resize(11, Default::default());

        for i in 0..=10 {
            k[i] = Block::from_column(w[i*4..i*4+4].to_vec()).unwrap();
        }

        k
    }
}

impl std::ops::Index<usize> for RoundKey {
    type Output = Block;

    fn index(&self, index: usize) -> &Self::Output {
        &self.k[index]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::IOHelper;

    #[test]
    fn test_key_1() {
        let round_key = RoundKey::new(&"securitysecurity".to_string());
        IOHelper::print_with_newline(
            IOHelper::make_char_hex(
                round_key.w.clone(),
                8,
            ),
            4
        );
        println!("");
    }

    #[test]
    fn test_key_2() {
        let round_key = RoundKey::new(
            &[6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5 as u8]
            .into_iter().map(|x| x as char).collect()
        );
        IOHelper::print_with_newline(
            IOHelper::make_char_hex(
                round_key.w.clone(),
                8,
            ),
            4
        );
        println!("");
    }
}