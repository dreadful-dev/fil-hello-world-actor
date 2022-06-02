mod blockstore;

use crate::blockstore::Blockstore;
use cid::multihash::Code;
use cid::Cid;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use fvm_ipld_encoding::{to_vec, CborStore, RawBytes, DAG_CBOR};
use fvm_sdk as sdk;
use fvm_sdk::NO_DATA_BLOCK_ID;
use fvm_shared::{ActorID};

use fvm_macro::StateObject;
use fvm_macro_derive::{StateObject, fvm_actor};

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

/// The state object.
#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, Default, StateObject)]
pub struct ComputeState {
    pub count: u64,
}

pub struct ComputeActor;

#[fvm_actor(state=ComputeState, dispatch="method_num")]
impl ComputeActor {
    /// The constructor populates the initial state.
    ///
    /// Method num 1. This is part of the Filecoin calling convention.
    /// InitActor#Exec will call the constructor on method_num = 1.

    pub fn constructor(_: RawBytes, state: ComputeState) -> Option<RawBytes> {
      // This constant should be part of the SDK.
      const INIT_ACTOR_ADDR: ActorID = 1;

      // Should add SDK sugar to perform ACL checks more succinctly.
      // i.e. the equivalent of the validate_* builtin-actors runtime methods.
      // https://github.com/filecoin-project/builtin-actors/blob/master/actors/runtime/src/runtime/fvm.rs#L110-L146
      if sdk::message::caller() != INIT_ACTOR_ADDR {
          abort!(USR_FORBIDDEN, "constructor invoked by non-init actor");
      }

      state.save();
      None
    }

    pub fn say_hello(_: RawBytes, mut state: ComputeState) -> Option<RawBytes> {
      state.count += 1;
      state.save();
  
      let ret = to_vec(format!("Hello world #{}!", &state.count).as_str());
      match ret {
          Ok(ret) => Some(RawBytes::new(ret)),
          Err(err) => {
              abort!(
                  USR_ILLEGAL_STATE,
                  "failed to serialize return value: {:?}",
                  err
              );
          }
      }
    }
}
