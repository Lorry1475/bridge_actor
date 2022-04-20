use crate::block::Block;
use crate::block_header::{BlockHeader, Ticket};
use crate::bls::verify;
use crate::constant::{Base_Point, Public_Key, DOMAIN};
use crate::legacy_tx::{LegacyTx, LegacyTxs, H256};
use crate::lorry_address::LorryAddress;
use crate::util::digest_message;
use ic_cdk::export::candid::Nat;
use sha256::digest_bytes;

pub struct BridgeManagement {}
impl BridgeManagement {
    pub fn new() -> Self {
        Self {}
    }
    pub fn mock_submit_block() -> bool {
        unsafe {
            let public_key = hex::decode("801daf546e570ea21864d375096f1e1b7ce5331c2ebcd99e2196a96005b4829219cbaf7b275f417efc2797415ee2d2c9099ad0583e33f47663044e619684c37ebaca64b504d563b0fb13412ce674843ce2b83ac5e053fe8246b08c54c68c79fd").unwrap();
            Public_Key = public_key.try_into().unwrap();
        }
        let mut bridge = BridgeManagement::new();
        let miner = "l00".to_string();
        let height = 0u64;
        let time_stamp = 0u64;
        let ticket = Ticket {
            vrf_proof: vec![0, 1, 2, 3, 4, 5],
        };
        let parents: Vec<String> =
            vec!["bafzbeigai3eoy2ccc7ybwjfz5r3rdxqrinwi4rwytly24tdbh6yk7zslrm".to_string()];
        let parent_sign: Vec<u8> = vec![0, 1, 3, 4, 5, 6, 7];
        let parent_message_receipts =
            "bafzbeigai3eoy2ccc7ybwjfz5r3rdxqrinwi4rwytly24tdbh6yk7zslrm".to_string();
        let txs_hash: Vec<Vec<u8>> = vec![vec![0, 1, 2, 3, 4, 5]];

        let block_header = BlockHeader {
            miner: miner,
            height: height,
            time_stamp: time_stamp,
            ticket: ticket,
            parents: parents,
            parent_sign: parent_sign,
            parent_message_receipts: parent_message_receipts,
            tx_hash: txs_hash,
            validated: false,
        };

        let tx_hash =
            hex::decode("f62182c7b426051f02fdbd920bc1d97d5455002ae8e8a48dd7516eac8aded259")
                .unwrap();
        let tx_hash: [u8; 32] = tx_hash.try_into().unwrap();
        let from = LorryAddress {
            account: "f39fd6e51aad88f6f4ce6ab8827279cfffb92266".to_string(),
            network_id: 0u8,
        };
        let to = LorryAddress {
            account: "rno2w-sqaaa-aaaaa-aaacq-cai".to_string(),
            network_id: 1u8,
        };
        let value: Nat = "1000000000000000000000".parse().unwrap();
        let from_asset = LorryAddress {
            account: "f39fd6e51aad88f6f4ce6ab8827279cfffb92266".to_string(),
            network_id: 0u8,
        };
        let to_asset = LorryAddress {
            account: "rno2w-sqaaa-aaaaa-aaacq-cai".to_string(),
            network_id: 1u8,
        };
        let nonce: Nat = "1".parse().unwrap();
        let tx = LegacyTx {
            from: from,
            to: to,
            value: value,
            from_asset: from_asset,
            to_asset: to_asset,
            nonce: nonce,
            hash: H256(tx_hash),
        };

        let txs = LegacyTxs { txs: vec![tx] };
        let block = Block {
            header: block_header,
            transactions: vec![],
        };
        let block_signature= hex::decode("8824cebfee103734f900b1578a2ad2205131682cdd27f7237eefaa62fdb79450e26e0735d5092f625b735f9fe04c6b4e").unwrap();
        bridge.submit_block(block, txs, block_signature)
    }

    pub fn submit_block(&mut self, block: Block, txs: LegacyTxs, block_signature: Vec<u8>) -> bool {
        let block_signature: [u8; 48] = block_signature.try_into().unwrap();
        let mut block = block;
        let mut block_txs: Vec<Vec<u8>> = vec![];
        for i in txs.txs.iter() {
            block_txs.push(i.encode());
        }
        block.transactions = block_txs;
        let mut abi_data = block.encode();
        unsafe {
            let point_base = hex::decode(Base_Point);
            let point_base: [u8; 96] = point_base.unwrap().try_into().unwrap();
            digest_message(&mut abi_data, 1);
            let abi_data_hash = hex::decode(digest_bytes(&abi_data)).unwrap();
            let result = verify(
                abi_data_hash,
                &block_signature,
                &Public_Key,
                &DOMAIN,
                &point_base,
            );
            let rs = format!("bls sign verify result:{:?}", result);
            println!("{}", rs);
            result
        }
    }
}
