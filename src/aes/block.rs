use super::row::Row;

#[derive(Clone, Default)]
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