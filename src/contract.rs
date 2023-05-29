use gstd::{errors::Result as GstdResult, msg, prelude::*, ActorId, MessageId};
use hashbrown::{HashMap, HashSet};
use student_nft_io::{
    Collection, Nft, NftId, StudentNFTAction, StudentNFTEvent, StudentNFTInit, StudentNFTState,
    UpdateAdminCommand,
};

#[derive(Debug)]
struct NFTContract {
    pub tokens: HashMap<NftId, Nft>,
    pub owners: HashMap<ActorId, NftId>,
    pub admins: HashSet<ActorId>,
    pub collection: Collection,
    pub nonce: NftId,
}

impl From<&NFTContract> for StudentNFTState {
    fn from(value: &NFTContract) -> Self {
        StudentNFTState {
            tokens: value.tokens.iter().map(|(k, v)| (*k, v.clone())).collect(),
            owners: value.owners.iter().map(|(k, v)| (*k, *v)).collect(),
            admins: value.admins.iter().copied().collect(),
            collection: value.collection.clone(),
            nonce: value.nonce,
        }
    }
}

static mut NFT_CONTRACT: Option<NFTContract> = None;

fn process_mint(
    nft_contract: &mut NFTContract,
    user: &ActorId,
    name: String,
    description: String,
    media_url: String,
    attrib_url: String,
) -> StudentNFTEvent {
    let Some(next_nft_nonce) = nft_contract.nonce.checked_add(1) else {
        return StudentNFTEvent::Error("Math overflow.".to_owned());
    };

    if nft_contract.owners.contains_key(user) {
        return StudentNFTEvent::Error("User already has student nft.".to_owned());
    }

    nft_contract.owners.insert(*user, next_nft_nonce);
    nft_contract.nonce = next_nft_nonce;
    nft_contract.tokens.insert(
        next_nft_nonce,
        Nft {
            owner: *user,
            name,
            description,
            media_url,
            attrib_url,
        },
    );

    StudentNFTEvent::Minted {
        user: *user,
        id: next_nft_nonce,
    }
}

fn process_update_metadata(
    nft_contract: &mut NFTContract,
    caller: &ActorId,
    nft_id: NftId,
    maybe_media_url: Option<String>,
    maybe_attrib_url: Option<String>,
) -> StudentNFTEvent {
    if nft_contract.admins.contains(caller) {
        let Some(nft) = nft_contract.tokens.get_mut(&nft_id) else {
            return StudentNFTEvent::Error("Invalid nft id.".to_owned());
        };

        if maybe_media_url.is_none() && maybe_attrib_url.is_none() {
            return StudentNFTEvent::Error("Metadata is empty.".to_owned());
        }

        if let Some(media_url) = maybe_media_url.clone() {
            nft.media_url = media_url;
        }

        if let Some(attrib_url) = maybe_attrib_url.clone() {
            nft.attrib_url = attrib_url;
        }

        StudentNFTEvent::MetadataUpdated {
            admin: *caller,
            nft_id,
            new_media_url: maybe_media_url,
            new_attrib_url: maybe_attrib_url,
        }
    } else {
        StudentNFTEvent::Error("Only admin can update metadata.".to_owned())
    }
}

fn process_update_admin(
    nft_contract: &mut NFTContract,
    caller: &ActorId,
    admin: ActorId,
    command: UpdateAdminCommand,
) -> StudentNFTEvent {
    if nft_contract.admins.contains(caller) {
        match command {
            UpdateAdminCommand::Add => nft_contract.admins.insert(admin),
            UpdateAdminCommand::Remove => nft_contract.admins.remove(&admin),
        };

        StudentNFTEvent::AdminUpdated { admin, command }
    } else {
        StudentNFTEvent::Error("Only admin can update other admins.".to_owned())
    }
}

#[no_mangle]
extern "C" fn init() {
    let init: StudentNFTInit = msg::load().expect("Unable to decode `StudentNFTInit`.");
    let mut admins = HashSet::new();

    admins.insert(msg::source());

    unsafe {
        NFT_CONTRACT = Some(NFTContract {
            tokens: HashMap::new(),
            owners: HashMap::new(),
            admins,
            collection: init.collection,
            nonce: 0,
        })
    };
}

#[no_mangle]
extern "C" fn handle() {
    let action: StudentNFTAction = msg::load().expect("Could not load `StudentNFTAction`.");
    let nft_contract: &mut NFTContract = unsafe {
        NFT_CONTRACT
            .as_mut()
            .expect("Unexpected uninitialized `NFTContract`.")
    };

    let user = msg::source();

    let result = match action {
        StudentNFTAction::Mint {
            name,
            description,
            media_url,
            attrib_url,
        } => process_mint(
            nft_contract,
            &user,
            name,
            description,
            media_url,
            attrib_url,
        ),
        StudentNFTAction::UpdateMetadata {
            nft_id,
            maybe_media_url,
            maybe_attrib_url,
        } => process_update_metadata(
            nft_contract,
            &user,
            nft_id,
            maybe_media_url,
            maybe_attrib_url,
        ),
        StudentNFTAction::UpdateAdmin { admin, command } => {
            process_update_admin(nft_contract, &user, admin, command)
        }
    };

    reply(result).expect("Failed to encode or reply with `StudentNFTEvent`.");
}

#[no_mangle]
extern "C" fn state() {
    reply(unsafe {
        let nft_contract = NFT_CONTRACT
            .as_ref()
            .expect("Uninitialized `NFTContract` state.");
        let student_nft_state: StudentNFTState = nft_contract.into();
        student_nft_state
    })
    .expect("Failed to share state.");
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    reply(metahash).expect("Failed to encode or reply from `metahash()`.");
}

fn reply(payload: impl Encode) -> GstdResult<MessageId> {
    msg::reply(payload, 0)
}
