mod block;
mod block_header;
mod blockstore;
mod bls;
mod bridge;
mod constant;
mod legacy_tx;
mod lorry_address;
mod util;
use crate::blockstore::Blockstore;
use cid::multihash::Code;
use cid::Cid;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use fvm_ipld_encoding::{to_vec, CborStore, RawBytes, DAG_CBOR};
use fvm_sdk as sdk;
use fvm_sdk::message::NO_DATA_BLOCK_ID;
use fvm_shared::ActorID;

/// A macro to abort concisely.
/// This should be part of the SDK as it's very handy.
macro_rules! abort {
    ($code:ident, $msg:literal $(, $ex:expr)*) => {
        fvm_sdk::vm::abort(
            fvm_shared::error::ExitCode::$code.value(),
            Some(format!($msg, $($ex,)*).as_str()),
        )
    };
}

// The state object.
#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, Default)]
pub struct State {
    pub count: u64,
}

/// We should probably have a derive macro to mark an object as a state object,
/// and have load and save methods automatically generated for them as part of a
/// StateObject trait (i.e. impl StateObject for State).
impl State {
    pub fn load() -> Self {
        // First, load the current state root.
        let root = match sdk::sself::root() {
            Ok(root) => root,
            Err(err) => abort!(USR_ILLEGAL_STATE, "failed to get root: {:?}", err),
        };

        // Load the actor state from the state tree.
        match Blockstore.get_cbor::<Self>(&root) {
            Ok(Some(state)) => state,
            Ok(None) => abort!(USR_ILLEGAL_STATE, "state does not exist"),
            Err(err) => abort!(USR_ILLEGAL_STATE, "failed to get state: {}", err),
        }
    }

    pub fn save(&self) -> Cid {
        let serialized = match to_vec(self) {
            Ok(s) => s,
            Err(err) => abort!(USR_SERIALIZATION, "failed to serialize state: {:?}", err),
        };
        let cid = match sdk::ipld::put(Code::Blake2b256.into(), 32, DAG_CBOR, serialized.as_slice())
        {
            Ok(cid) => cid,
            Err(err) => abort!(USR_SERIALIZATION, "failed to store initial state: {:}", err),
        };
        if let Err(err) = sdk::sself::set_root(&cid) {
            abort!(USR_ILLEGAL_STATE, "failed to set root ciid: {:}", err);
        }
        cid
    }
}

#[no_mangle]
pub fn invoke(id: u32) -> u32 {
    let ret: Option<RawBytes> = match sdk::message::method_number() {
        1 => constructor(),
        2 => submit_block(),
        _ => abort!(USR_UNHANDLED_MESSAGE, "unrecognized method"),
    };
    match ret {
        // Ok(data) => match data {
        None => NO_DATA_BLOCK_ID,
        Some(v) => match sdk::ipld::put_block(DAG_CBOR, v.bytes()) {
            Ok(id) => id,
            Err(err) => abort!(USR_SERIALIZATION, "failed to store return value: {}", err),
        },
        // },
        // Err(msg) => {
        //     abort!(USR_SERIALIZATION, "failed to store return value: {}", msg)
        // }
    }
}

pub fn constructor() -> Option<RawBytes> {
    // This constant should be part of the SDK.
    const INIT_ACTOR_ADDR: ActorID = 1;

    // Should add SDK sugar to perform ACL checks more succinctly.
    // i.e. the equivalent of the validate_* builtin-actors runtime methods.
    // https://github.com/filecoin-project/builtin-actors/blob/master/actors/runtime/src/runtime/fvm.rs#L110-L146
    if sdk::message::caller() != INIT_ACTOR_ADDR {
        abort!(USR_FORBIDDEN, "constructor invoked by non-init actor");
    }

    let state = State::default();
    state.save();
    None
}

pub fn submit_block() -> Option<RawBytes> {
    let state = State::load();
    let res = bridge::BridgeManagement::mock_submit_block();
    Some(RawBytes::new(
        format!("ver result :{:?}", res).as_bytes().to_vec(),
    ))
}

#[cfg(test)]
mod bridge_test {
    use super::*;
    #[test]
    fn submit_block_test() {
        let rs = bridge::BridgeManagement::mock_submit_block();
        assert_eq!(rs, true);
    }
}
