mod utils;

use gstd::prelude::*;
use gtest::{Program, System};
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

    student_nft.update_metadata(utils::ADMIN, 1, Some("1".to_owned()), None, false);
    let state = student_nft.get_state();
    assert_eq!(state.tokens[0].1.media_url, "1".to_owned());

    student_nft.update_metadata(utils::ADMIN, 1, None, Some("2".to_owned()), false);
    let state = student_nft.get_state();
    assert_eq!(state.tokens[0].1.attrib_url, "2".to_owned());
}

#[test]
fn fail_metadata_is_empty() {
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

    student_nft.update_metadata(utils::ADMIN, 1, None, None, true);
}

#[test]
fn fail_invalid_nft_id() {
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

    student_nft.update_metadata(utils::ADMIN, 1337, Some("1".to_owned()), None, true);
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

    student_nft.update_metadata(user, 1, Some("1".to_owned()), None, true);
}
