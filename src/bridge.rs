use crate::block::{Block, BlockReceiptState};
use crate::block_header::{BlockHeader, Ticket};
use crate::bls::verify;
use crate::constant::{Base_Point, Public_Key, DOMAIN};
use crate::legacy_tx::{LegacyTx, LegacyTxs, H256};
use crate::lorry_address::LorryAddress;
use crate::types::EthAsset;
use crate::uint256::Uint256;
use crate::util::digest_message;
use cid::multihash::Code;
use cid::Cid;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use fvm_ipld_encoding::{to_vec, CborStore, DAG_CBOR};
use fvm_shared::address::Address;
use fvm_shared::bigint::BigInt;
use fvm_shared::ActorID;
use ic_cdk::export::candid::Nat;
use sha256::digest_bytes;
use std::collections::HashMap;
use std::str::FromStr;
type FRC20 = String;
#[macro_use]
use crate::abort;
#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
pub struct BridgeManagement {
    pub asset: HashMap<FRC20, EthAsset>,
    pub balance: HashMap<ActorID, HashMap<FRC20, Uint256>>,
    pub block_receipt: BlockReceiptState,
}
impl BridgeManagement {
    pub fn default() -> Self {
        Self {
            asset: HashMap::new(),
            balance: HashMap::new(),
            block_receipt: BlockReceiptState::default(),
        }
    }

    pub fn load() -> Self {
        let root = match fvm_sdk::sself::root() {
            Ok(root) => root,
            Err(err) => abort!(USR_ILLEGAL_STATE, "failed to get root: {:?}", err),
        };

        match fvm_sdk::ipld::get(&root) {
            Err(err) => {
                abort!(USR_ILLEGAL_STATE, "failed to get root: {:?}", err)
            }
            Ok(data) => fvm_ipld_encoding::from_slice::<Self>(&data).unwrap(),
        }
    }

    pub fn save(&self) -> Cid {
        let serialized = match to_vec(self) {
            Ok(s) => s,
            Err(err) => abort!(USR_SERIALIZATION, "failed to serialize state: {:?}", err),
        };
        let serialized = serialized.as_slice();
        let cid = match fvm_sdk::ipld::put(Code::Blake2b256.into(), 32, DAG_CBOR, &serialized) {
            Ok(cid) => cid,
            Err(err) => abort!(USR_SERIALIZATION, "failed to store initial state: {:}", err),
        };
        if let Err(err) = fvm_sdk::sself::set_root(&cid) {
            abort!(USR_ILLEGAL_STATE, "failed to set root ciid: {:}", err);
        }
        cid
    }
    // Generate a test transaction
    fn mock_txs() -> LegacyTxs {
        let mut txs: Vec<LegacyTx> = Vec::new();

        let tx_hash =
            hex::decode("f62182c7b426051f02fdbd920bc1d97d5455002ae8e8a48dd7516eac8aded259")
                .unwrap();
        let tx_hash: [u8; 32] = tx_hash.try_into().unwrap();
        let from = LorryAddress {
            account: "f39fd6e51aad88f6f4ce6ab8827279cfffb92266".to_string(),
            network_id: 0u8,
        };
        let to = LorryAddress {
            account: "0".to_string(),
            network_id: 1u8,
        };
        let value: Nat = "45463472903480298342".parse().unwrap();
        let from_asset = LorryAddress {
            account: "f39fd6e51aad88f6f4ce6ab8827279cfffb92266".to_string(),
            network_id: 0u8,
        };
        let to_asset = LorryAddress {
            account: "t2q52nekezul32pypy7bs2o44uwztv5lo2nyhsnmy".to_string(),
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

        txs.push(tx);
        LegacyTxs { txs: txs }
    }

    // Generate a test block
    pub fn mock_submit_block(&mut self) -> bool {
        unsafe {
            let public_key = hex::decode("801daf546e570ea21864d375096f1e1b7ce5331c2ebcd99e2196a96005b4829219cbaf7b275f417efc2797415ee2d2c9099ad0583e33f47663044e619684c37ebaca64b504d563b0fb13412ce674843ce2b83ac5e053fe8246b08c54c68c79fd").unwrap();
            Public_Key = public_key.try_into().unwrap();
        }

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
        self.submit_block(block, txs, block_signature)
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

            {
                //mock_transactions test
                if result {
                    let txs = Self::mock_txs();
                    self.excute_transactions(&txs);
                }
            }

            result
        }
    }

    fn excute_transactions(&mut self, txs: &LegacyTxs) {
        for i in txs.txs.iter() {
            let account = i.to.account.parse::<u64>().unwrap();
            let amount = Uint256::from_bytes_be(&i.value.0.to_bytes_be());
            match self.balance.get_mut(&account) {
                None => {
                    let mut balance: HashMap<FRC20, Uint256> = HashMap::new();
                    balance.insert(i.to_asset.account.clone(), amount);

                    self.balance.insert(account, balance);
                }
                Some(balance) => match balance.get_mut(&i.to_asset.account) {
                    None => {
                        balance.insert(i.to_asset.account.clone(), amount);
                    }
                    Some(value) => {
                        *value = value.clone() + amount;
                    }
                },
            }
        }
    }

    pub fn balance_of(&self, account: ActorID, token: FRC20) -> Uint256 {
        let default_balance = Uint256::default();
        match self.balance.get(&account) {
            None => return default_balance,
            Some(balance) => {
                let balance = balance.get(&token).unwrap_or(&default_balance);
                balance.clone()
            }
        }
    }

    pub fn withdraw_asset(
        &mut self,
        sender: ActorID,
        token: &FRC20,
        amount: Uint256,
    ) -> Option<fvm_ipld_encoding::RawBytes> {
        match self.balance.get_mut(&sender) {
            None => {
                abort!(SYS_ASSERTION_FAILED, "account does not exist");
            }
            Some(asset) => match asset.get_mut(token) {
                None => {
                    abort!(SYS_ASSERTION_FAILED, "asset does not exist");
                }
                Some(balance) => {
                    if balance.clone() < amount {
                        abort!(SYS_ASSERTION_FAILED, "insufficient Balance");
                    }

                    let to = fvm_shared::address::Address::from_str(token).unwrap();
                    let method_num = 2u64;
                    let params = fvm_ipld_encoding::RawBytes::new(vec![]);
                    let token_amount = BigInt::parse_bytes(b"0", 10).unwrap();
                    {
                        // For the convenience of testing, first mint
                        fvm_sdk::send::send(&to, method_num, params.clone(), token_amount.clone());
                    }

                    let method_num = 6u64;
                    match fvm_sdk::send::send(&to, method_num, params, token_amount) {
                        Ok(_) => {
                            *balance = balance.clone() - amount;
                            None
                        }
                        Err(err) => {
                            abort!(SYS_ASSERTION_FAILED, "transfer failed");
                        }
                    }
                }
            },
        }
    }
}
