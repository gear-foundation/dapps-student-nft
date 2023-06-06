mod utils;

use gstd::prelude::*;
use gtest::{Program, System};
use utils::student_nft::StudentNFTMock;

#[test]
fn success() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];
    let user_id = user.into();

    let student_nft = Program::student_nft(&system, "TST".to_owned(), "Test gNFT".to_owned());
    let state = student_nft.get_state();

    assert!(state.tokens.is_empty());
    assert!(state.owners.is_empty());
    assert!(!state.admins.is_empty());
    assert_eq!(state.nonce, 0);

    student_nft.mint(
        user,
        "".to_owned(),
        "".to_owned(),
        "".to_owned(),
        "".to_owned(),
        false,
    );

    let state = student_nft.get_state();

    assert!(!state.tokens.is_empty());
    assert!(!state.owners.is_empty());
    assert_eq!(state.nonce, 1);
    assert_eq!(state.owners[0], (user_id, 1));
}

#[test]
fn success_mint_to() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];

    let user_1 = utils::USERS[1];
    let user_1_id = user_1.into();

    let student_nft = Program::student_nft(&system, "TST".to_owned(), "Test gNFT".to_owned());
    let state = student_nft.get_state();

    assert!(state.tokens.is_empty());
    assert!(state.owners.is_empty());
    assert!(!state.admins.is_empty());
    assert_eq!(state.nonce, 0);

    student_nft.mint_to(
        user,
        user_1,
        "".to_owned(),
        "".to_owned(),
        "".to_owned(),
        "".to_owned(),
        false,
    );

    let state = student_nft.get_state();

    assert!(!state.tokens.is_empty());
    assert!(!state.owners.is_empty());
    assert_eq!(state.nonce, 1);
    assert_eq!(state.owners[0], (user_1_id, 1));
}

#[test]
fn fail_user_already_has_nft() {
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
    student_nft.mint(
        user,
        "".to_owned(),
        "".to_owned(),
        "".to_owned(),
        "".to_owned(),
        true,
    );
}
