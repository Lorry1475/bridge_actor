use crate::block_header::BlockHeader;
use ethabi::encode;
use ethabi::ethereum_types::U256;
use ethabi::Token;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Vec<u8>>,
}

impl Block {
    pub fn encode(&self) -> Vec<u8> {
        let header = self.header.encode();
        let mut txs: Vec<Token> = vec![];
        for i in self.transactions.iter() {
            txs.push(Token::Bytes(i.clone()));
        }
        encode(&vec![Token::Tuple(vec![
            Token::Bytes(header),
            Token::Array(txs),
        ])])
    }
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, Default)]
pub struct BlockReceiptState {
    block_confirmation: HashMap<u64, bool>,
    block_height: u64,
}

impl BlockReceiptState {
    pub fn block_state(&self, block_height: u64) -> bool {
        *self.block_confirmation.get(&block_height).unwrap_or(&false)
    }
    pub fn block_height(&self) -> u64 {
        self.block_height
    }
    pub fn excute_receipt(&mut self, block_height: u64) -> Result<(), &str> {
        if self.block_state(block_height) {
            return Err("secondary confirmation");
        }
        self.block_confirmation.insert(block_height, true);
        Ok(())
    }
    pub fn update_block_height(&mut self, block_height: u64) {
        if self.block_height < block_height {
            self.block_height = block_height;
        }
    }
}
