use std::{convert::TryInto, ops::Deref};

#[derive(PartialEq)]
pub struct BlockNumber(u32);

pub const InvalidBlockNumber: BlockNumber = BlockNumber(0xFFFFFFFF);
pub const MaxBlockNumber: BlockNumber = BlockNumber(0xFFFFFFFE);

impl BlockNumber {
    pub fn is_valid(&self) -> bool {
        InvalidBlockNumber != *self
    }
}

#[repr(C)]
#[derive(PartialEq)]
pub struct BlockId {
    bi_hi: u16,
    bi_lo: i16,
}

impl BlockId {
    pub fn new() -> Self {
        Self {
            bi_hi: 0,
            bi_lo: 0,
        }
    }

    pub fn set(&mut self, value: BlockNumber) -> &Self {
        self.bi_hi = (value.0 >> 16).try_into().unwrap();
        self.bi_lo = (value.0 & 0xFFFF).try_into().unwrap();
        self
    }

    pub fn copy(&self, other: &mut Self) -> &Self {
        other.bi_hi = self.bi_hi;
        other.bi_lo = self.bi_lo;
        self
    }

    pub fn get(&self) -> BlockNumber {
        BlockNumber((self.bi_hi as u32) << 16 | (self.bi_lo as u32))
    }
}