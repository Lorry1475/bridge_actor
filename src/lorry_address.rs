use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
pub struct LorryAddress {
    pub account: String,
    pub network_id: u8,
}
