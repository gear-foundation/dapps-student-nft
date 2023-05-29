#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};

pub type NftId = u128;

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = In<StudentNFTInit>;
    type Handle = InOut<StudentNFTAction, StudentNFTEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = StudentNFTState;
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct StudentNFTInit {
    pub collection: Collection,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum UpdateAdminCommand {
    Add,
    Remove,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum StudentNFTAction {
    Mint {
        name: String,
        description: String,
        media_url: String,
        attrib_url: String,
    },
    UpdateMetadata {
        nft_id: NftId,
        maybe_media_url: Option<String>,
        maybe_attrib_url: Option<String>,
    },
    UpdateAdmin {
        admin: ActorId,
        command: UpdateAdminCommand,
    },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum StudentNFTEvent {
    Minted {
        user: ActorId,
        id: NftId,
    },
    MetadataUpdated {
        admin: ActorId,
        nft_id: NftId,
        new_media_url: Option<String>,
        new_attrib_url: Option<String>,
    },
    AdminUpdated {
        admin: ActorId,
        command: UpdateAdminCommand,
    },
    Error(String),
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct StudentNFTState {
    pub tokens: Vec<(NftId, Nft)>,
    pub owners: Vec<(ActorId, NftId)>,
    pub admins: Vec<ActorId>,
    pub collection: Collection,
    pub nonce: NftId,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct Nft {
    pub owner: ActorId,
    pub name: String,
    pub description: String,
    pub media_url: String,
    pub attrib_url: String,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct Collection {
    pub name: String,
    pub description: String,
}
