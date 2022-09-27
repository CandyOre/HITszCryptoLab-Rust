use std::fmt::LowerHex;

use super::consts::SBOX;

#[derive(Clone, Default)]
pub struct Block {
    content: [[u8; 4]; 4],
}

impl Block {
    pub fn from_vec(vec: Vec<u8>) -> Option<Block>
    {
        let mut block: Block = Default::default();
        for i in 0..3 {
            for j in 0..3 {
                block.content[j][i] = vec[i * 4 + j]
            }
        }
        Some(block)
    }
}

#[derive(Clone, Default, Copy)]
pub struct Row {
    content: u32,
}

impl Row {
    pub fn from_iter<T, E>(src: T) -> Option<Row>
    where
        T: IntoIterator<Item = E>,
        E: num_traits::AsPrimitive<u8>,
    {
        let mut sz = 0 as u32;
        let mut content = 0 as u32;
        for e in src.into_iter() {
            sz += 1;
            content = (e.as_() as u32) + (content << 8);
        }
        if sz == 4 {
            Some(Row::from(content))
        } else {
            None
        }
    }

    pub fn into_iter(&self) -> impl Iterator<Item = u8> {
        [
            (self.content & 0xff000000) >> 24,
            (self.content & 0x00ff0000) >> 16,
            (self.content & 0x0000ff00) >> 8,
            (self.content & 0x000000ff) >> 0,
        ].into_iter().map(|x| x as u8)
    }

    pub fn rotate(&mut self) -> Self {
        Row::from((self.content >> 24) + (self.content << 8))
    }

    pub fn sub(&mut self) -> Self {
        Self::from_iter(
            self.into_iter().map(|x| SBOX[x as usize])
        ).unwrap()
    }
}

impl<T> From<T> for Row
where
    T: num_traits::PrimInt
{
    fn from(e: T) -> Self {
        Row {content: e.to_u32().unwrap()}
    }
}

impl LowerHex for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.content.fmt(f)
    }
}

impl std::ops::BitXor<Row> for Row {
    type Output = Row;

    fn bitxor(self, rhs: Row) -> Self::Output {
        Row {content: self.content ^ rhs.content}
    }
}
