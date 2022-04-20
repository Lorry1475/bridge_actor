use crate::lorry_address::LorryAddress;
use ethabi::encode;
use ethabi::ethereum_types::U256;
use ethabi::Token;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use fvm_shared::bigint::BigUint;
use ic_cdk::export::candid::Nat;
use sha256::digest_bytes;

#[derive(Clone, Debug, PartialEq)]
pub struct H256(pub [u8; 32]);

#[derive(Clone, Debug)]
pub struct LegacyTx {
    pub from: LorryAddress,
    pub to: LorryAddress,
    pub value: Nat,
    pub from_asset: LorryAddress,
    pub to_asset: LorryAddress,
    pub nonce: Nat,
    pub hash: H256,
}

impl LegacyTx {
    pub fn hash(&self) -> Result<Vec<u8>, hex::FromHexError> {
        let encode_data = self.encode();
        let hash = digest_bytes(&encode_data);
        hex::decode(hash)
    }

    pub fn encode(&self) -> Vec<u8> {
        encode(&vec![Token::Tuple(vec![
            Token::Tuple(vec![
                Token::String(self.from.account.clone()),
                Token::Uint(self.from.network_id.into()),
            ]),
            Token::Tuple(vec![
                Token::String(self.to.account.clone()),
                Token::Uint(self.to.network_id.into()),
            ]),
            Token::Uint(U256::from_big_endian(&self.value.0.to_bytes_be())),
            Token::Tuple(vec![
                Token::String(self.from_asset.account.clone()),
                Token::Uint(self.from_asset.network_id.into()),
            ]),
            Token::Tuple(vec![
                Token::String(self.to_asset.account.clone()),
                Token::Uint(self.to_asset.network_id.into()),
            ]),
            Token::Uint(U256::from_big_endian(&self.nonce.0.to_bytes_be())),
            Token::Bytes(self.hash.0.to_vec()),
        ])])
    }
}

pub struct LegacyTxs {
    pub txs: Vec<LegacyTx>,
}

impl LegacyTxs {
    pub fn new(txs: Vec<LegacyTx>) -> Self {
        Self { txs: txs }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut txs: Vec<Token> = vec![];
        for i in self.txs.iter() {
            txs.push(Token::Bytes(i.encode()));
        }

        encode(&vec![Token::Tuple(vec![Token::Array(txs)])])
    }
}
