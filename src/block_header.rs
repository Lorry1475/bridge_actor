use ethabi::encode;
use ethabi::ethereum_types::U256;
use ethabi::Token;

#[derive(Debug, Clone)]
pub struct Ticket {
    pub vrf_proof: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct BlockHeader {
    pub miner: String,
    pub height: u64,
    pub time_stamp: u64,
    pub ticket: Ticket,
    pub parents: Vec<String>,
    pub parent_sign: Vec<u8>,
    pub parent_message_receipts: String,
    pub tx_hash: Vec<Vec<u8>>,
    pub validated: bool,
}

impl BlockHeader {
    pub fn encode(&self) -> Vec<u8> {
        let parents = self
            .parents
            .iter()
            .map(|x| Token::String(x.clone()))
            .collect();
        let tx_hash = self
            .tx_hash
            .iter()
            .map(|x| Token::Bytes(x.clone()))
            .collect();
        encode(&vec![Token::Tuple(vec![
            Token::String(self.miner.clone()),
            Token::Uint(self.height.into()),
            Token::Uint(self.time_stamp.into()),
            Token::Tuple(vec![Token::Bytes(self.ticket.vrf_proof.clone())]),
            Token::Array(parents),
            Token::Bytes(self.parent_sign.clone()),
            Token::String(self.parent_message_receipts.clone()),
            Token::Array(tx_hash),
            Token::Bool(self.validated),
        ])])
    }
}
