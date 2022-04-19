use crate::block_header::BlockHeader;
use ethabi::encode;
use ethabi::ethereum_types::U256;
use ethabi::Token;
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
