mod utils;

use gstd::prelude::*;
use gtest::{Program, System};
use student_nft_io::UpdateAdminCommand;
use utils::student_nft::StudentNFTMock;

#[test]
fn success() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];

    let student_nft = Program::student_nft(&system, "TST".to_owned(), "Test gNFT".to_owned());
    student_nft.mint(
        user,
        "".to_owned(),
        "".to_owned(),
        "".to_owned(),
        "".to_owned(),
        false,
    );

    student_nft.update_admin(utils::ADMIN, user, UpdateAdminCommand::Add, false);
    let state = student_nft.get_state();
    assert_eq!(state.admins.len(), 2);

    student_nft.update_admin(user, utils::ADMIN, UpdateAdminCommand::Remove, false);
    let state = student_nft.get_state();
    assert_eq!(state.admins.len(), 1);
}

#[test]
fn fail_only_admin_can_update() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];

    let student_nft = Program::student_nft(&system, "TST".to_owned(), "Test gNFT".to_owned());
    student_nft.mint(
        user,
        "".to_owned(),
        "".to_owned(),
        "".to_owned(),
        "".to_owned(),
        false,
    );

    student_nft.update_admin(user, user, UpdateAdminCommand::Add, true);
}
