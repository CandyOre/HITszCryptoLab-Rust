use super::{
    row::Row,
    consts::{SBOX, SBOX_INV}
};

#[derive(Clone, Default, Copy)]
pub struct Block {
    content: [[u8; 4]; 4],
}

impl Block {
    pub fn from_column(vec: Vec<Row>) -> Option<Block>
    {
        if vec.len() != 4 {
            None
        } else {
            let mut block: Block = Default::default();
            for j in 0..4 {
                let mut iter = vec[j].into_iter();
                for i in 0..4 {
                    block.content[i][j] = iter.next().unwrap();
                }
            }
            Some(block)
        }
    }

    pub fn from_column_iter<T, E>(src: T) -> Option<Block>
    where
        T: IntoIterator<Item = E>,
        E: num_traits::AsPrimitive<u8>,
    {
        let mut sz = 0 as usize;
        let mut content: [u8; 16] = Default::default();
        for e in src.into_iter() {
            content[sz as usize] = e.as_();
            sz += 1;
        }
        if sz == 16 {
            Some(Self::from(content).transpose())
        } else {
            None
        }
    }

    pub fn transpose(&mut self) -> Self {
        for i in 0..4 {
            for j in 0..i {
                (self.content[i][j], self.content[j][i])
                    = (self.content[j][i], self.content[i][j]);
            }
        }
        *self
    }

    pub fn into_iter(&self) -> impl Iterator<Item = u8> {
        self.content.into_iter().flat_map(|x| x.into_iter())
    }

    pub fn into_column_iter(&self) -> impl Iterator<Item = u8> {
        self.clone().transpose().into_iter()
    }

    pub fn sub(&mut self) -> Self {
        Self::from_column_iter(
            self.into_column_iter().map(|x| SBOX[x as usize])
        ).unwrap()
    }

    pub fn sub_inv(&mut self) -> Self {
        Self::from_column_iter(
            self.into_column_iter().map(|x| SBOX_INV[x as usize])
        ).unwrap()
    }

    pub fn shift(&mut self) -> Self {
        self.content[1] = [
            self.content[1][1],
            self.content[1][2],
            self.content[1][3],
            self.content[1][0],
        ];
        self.content[2] = [
            self.content[2][2],
            self.content[2][3],
            self.content[2][0],
            self.content[2][1],
        ];
        self.content[3] = [
            self.content[3][3],
            self.content[3][0],
            self.content[3][1],
            self.content[3][2],
        ];
        *self
    }

    pub fn shift_inv(&mut self) -> Self {
        self.content[1] = [
            self.content[1][3],
            self.content[1][0],
            self.content[1][1],
            self.content[1][2],
        ];
        self.content[2] = [
            self.content[2][2],
            self.content[2][3],
            self.content[2][0],
            self.content[2][1],
        ];
        self.content[3] = [
            self.content[3][1],
            self.content[3][2],
            self.content[3][3],
            self.content[3][0],
        ];
        *self
    }
}

impl From<[u8; 16]> for Block {
    fn from(e: [u8; 16]) -> Self {
        let mut block: Block = Default::default();
        for i in 0..4 {
            for j in 0..4 {
                block.content[i][j] = e[i*4+j];
            }
        }
        block
    }
}

impl std::ops::Add<Block> for Block {
    type Output = Block;

    fn add(self, rhs: Block) -> Self::Output {
        let mut block: Block = Default::default();
        for i in 0..4 {
            for j in 0..4 {
                block.content[i][j] = self.content[i][j] ^ rhs.content[i][j];
            }
        }
        block
    }
}

fn galois_field_mul(a: u8, b: u8) -> u8 {
    let mut res = 0;
    let mut a = a;
    let mut b = b;
    while a > 0 {
        if a & 1u8 > 0 {
            res ^= b;
        }
        a >>= 1;
        let overflow = b & 0x80u8;
        b <<= 1;
        if overflow > 0 {
            b ^= 0x1b;
        }
    }
    res
}

impl std::ops::Mul<Block> for Block {
    type Output = Block;

    fn mul(self, rhs: Block) -> Self::Output {
        let mut block: Block = Default::default();
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    block.content[i][j] ^= galois_field_mul(
                        self.content[i][k],
                        rhs.content[k][j]
                    );
                }
            }
        }
        block
    }
}