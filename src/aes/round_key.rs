use super::state::Block;

lazy_static! {
    pub static ref ROUNDKEY: RoundKey = RoundKey::new();
}

pub struct RoundKey {
    w: Vec<u8>,
    k: Vec<Block>,
}

impl RoundKey {
    pub fn get_w(&self, pos: usize) -> Option<u8> {
        if pos < 44 {
            Some(ROUNDKEY.w[pos])
        } else {
            None
        }
    }
}

impl RoundKey {
    fn new() -> Self {
        let w = RoundKey::new_w();
        let k = RoundKey::new_k(&w);
        RoundKey { w, k }
    }

    fn new_w() -> Vec<u8> {
        let mut w = Vec::new();
        w.resize(44, 0);
        unimplemented!();
        w
    }

    fn new_k(w: &Vec<u8>) -> Vec<Block> {
        let mut k = Vec::new();
        k.resize(11, Block::new());
        unimplemented!();
        k
    }
}