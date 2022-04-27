mod abort;
mod block;
mod block_header;
mod blockstore;
mod bls;
mod bridge;
mod constant;
mod legacy_tx;
mod lorry_address;
mod types;
mod uint256;
mod util;
use crate::blockstore::Blockstore;
use bridge::BridgeManagement;
use cid::multihash::Code;
use cid::Cid;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use fvm_ipld_encoding::{to_vec, CborStore, RawBytes, DAG_CBOR};
use fvm_sdk as sdk;
use fvm_sdk::message::NO_DATA_BLOCK_ID;
use fvm_shared::ActorID;
use std::str::FromStr;
use types::EthAsset;

#[no_mangle]
pub fn invoke(id: u32) -> u32 {
    let ret: Option<RawBytes> = match sdk::message::method_number() {
        1 => constructor(),
        2 => submit_block(),
        3 => asset_management(),
        4 => balance_of(),
        5 => withdraw_asset(),
        _ => abort!(USR_UNHANDLED_MESSAGE, "unrecognized method"),
    };
    match ret {
        None => NO_DATA_BLOCK_ID,
        Some(v) => match sdk::ipld::put_block(DAG_CBOR, v.bytes()) {
            Ok(id) => id,
            Err(err) => abort!(USR_SERIALIZATION, "failed to store return value: {}", err),
        },
    }
}

pub fn constructor() -> Option<RawBytes> {
    let mut state = BridgeManagement::default();
    state.block_hight = 1u64;
    state.save();
    None
}

pub fn asset_management() -> Option<RawBytes> {
    let mut state = BridgeManagement::load();
    let frc20_address = "t2pxr3zwllj6cjaclzvqehkeppcooz4kmnj46unwq".to_string();

    let eth_address: ic_eth_recover::types::Address =
        "6F241f7dCDa951bdffB00000B4B11C361369bCac".parse().unwrap();
    let eth_asset = EthAsset {
        address: eth_address,
        symbol: "ETH".to_string(),
        decimal: 18,
    };

    state.asset.insert(frc20_address, eth_asset);
    state.save();
    None
}

pub fn submit_block() -> Option<RawBytes> {
    let mut state = BridgeManagement::load();
    let res = state.mock_submit_block();
    state.save();
    Some(RawBytes::new(
        format!("ver result :{:?}", res).as_bytes().to_vec(),
    ))
}

pub fn balance_of() -> Option<RawBytes> {
    let state = BridgeManagement::load();
    let account: ActorID = 0u64;
    let token = "t2q52nekezul32pypy7bs2o44uwztv5lo2nyhsnmy".to_string();
    let balance = state.balance_of(account, token);
    Some(RawBytes::new(balance.to_string().as_bytes().to_vec()))
}

pub fn withdraw_asset() -> Option<RawBytes> {
    let mut state = BridgeManagement::load();
    let mock_amount = fvm_shared::bigint::BigUint::parse_bytes(b"100000000000", 10).unwrap();
    let mock_amount = uint256::Uint256 {
        big_uint: mock_amount,
    };
    let token = "t2q52nekezul32pypy7bs2o44uwztv5lo2nyhsnmy".to_string();
    let sender: ActorID = 0u64;
    let balance = state.withdraw_asset(sender, &token, mock_amount);
    state.save();
    None
}

#[cfg(test)]
mod bridge_test {
    use super::*;
    #[test]
    fn submit_block_test() {}
}
