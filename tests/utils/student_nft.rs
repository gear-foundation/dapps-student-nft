use super::ADMIN;
use gstd::prelude::*;
use gtest::{Program, System};
use student_nft_io::{
    Collection, NftId, StudentNFTAction, StudentNFTEvent, StudentNFTInit, StudentNFTState,
    UpdateAdminCommand,
};

pub trait StudentNFTMock {
    fn student_nft(system: &System, name: String, description: String) -> Program;
    fn mint(
        &self,
        from: u64,
        name: String,
        description: String,
        media_url: String,
        attrib_url: String,
        error: bool,
    );
    #[allow(clippy::too_many_arguments)]
    fn mint_to(
        &self,
        from: u64,
        user: u64,
        name: String,
        description: String,
        media_url: String,
        attrib_url: String,
        error: bool,
    );
    fn update_metadata(
        &self,
        from: u64,
        nft_id: NftId,
        maybe_media_url: Option<String>,
        maybe_attrib_url: Option<String>,
        error: bool,
    );
    fn update_admin(&self, from: u64, admin: u64, command: UpdateAdminCommand, error: bool);
    fn send_student_nft_tx(&self, from: u64, action: StudentNFTAction, error: bool);
    fn get_state(&self) -> StudentNFTState;
}

impl StudentNFTMock for Program<'_> {
    fn student_nft(system: &System, name: String, description: String) -> Program {
        let student_nft = Program::current(system);
        assert!(!student_nft
            .send(
                ADMIN,
                StudentNFTInit {
                    collection: Collection { name, description }
                }
            )
            .main_failed());

        student_nft
    }

    fn mint(
        &self,
        from: u64,
        name: String,
        description: String,
        media_url: String,
        attrib_url: String,
        error: bool,
    ) {
        self.send_student_nft_tx(
            from,
            StudentNFTAction::Mint {
                name,
                description,
                media_url,
                attrib_url,
            },
            error,
        );
    }

    fn mint_to(
        &self,
        from: u64,
        user: u64,
        name: String,
        description: String,
        media_url: String,
        attrib_url: String,
        error: bool,
    ) {
        self.send_student_nft_tx(
            from,
            StudentNFTAction::MintTo {
                user: user.into(),
                name,
                description,
                media_url,
                attrib_url,
            },
            error,
        );
    }

    fn update_metadata(
        &self,
        from: u64,
        nft_id: NftId,
        maybe_media_url: Option<String>,
        maybe_attrib_url: Option<String>,
        error: bool,
    ) {
        self.send_student_nft_tx(
            from,
            StudentNFTAction::UpdateMetadata {
                nft_id,
                maybe_media_url,
                maybe_attrib_url,
            },
            error,
        )
    }

    fn update_admin(&self, from: u64, admin: u64, command: UpdateAdminCommand, error: bool) {
        self.send_student_nft_tx(
            from,
            StudentNFTAction::UpdateAdmin {
                admin: admin.into(),
                command,
            },
            error,
        )
    }

    fn send_student_nft_tx(&self, from: u64, action: StudentNFTAction, error: bool) {
        let result = self.send(from, action);
        assert!(!result.main_failed());

        let maybe_error = result.log().iter().find_map(|log| {
            let mut payload = log.payload();
            if let Ok(StudentNFTEvent::Error(error)) = StudentNFTEvent::decode(&mut payload) {
                Some(error)
            } else {
                None
            }
        });

        assert_eq!(maybe_error.is_some(), error);
    }

    fn get_state(&self) -> StudentNFTState {
        self.read_state().expect("Unexpected invalid state.")
    }
}
