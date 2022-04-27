use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use ic_eth_recover::types::Address;
#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
pub struct EthAsset {
    pub address: Address,
    pub symbol: String,
    pub decimal: u8,
}
