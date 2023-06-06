#![no_std]

use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};
use student_nft_io::*;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[metawasm]
pub mod metafns {
    pub type State = <ContractMetadata as Metadata>::State;

    pub fn get_tokens(state: State) -> Vec<(NftId, Nft)> {
        state.tokens
    }

    pub fn get_owners(state: State) -> Vec<(ActorId, NftId)> {
        state.owners
    }

    pub fn get_admins(state: State) -> Vec<ActorId> {
        state.admins
    }

    pub fn get_collection(state: State) -> Collection {
        state.collection
    }
}
