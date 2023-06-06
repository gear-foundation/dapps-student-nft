mod utils_gclient;

use gclient::GearApi;

#[ignore]
#[tokio::test]
async fn success() -> gclient::Result<()> {
    let api = GearApi::dev().await?;

    let student_nft = utils_gclient::common::init(&api, "TST", "Test gNFT").await?;

    let state = utils_gclient::student_nft::get_state(&api, &student_nft).await?;
    assert!(state.tokens.is_empty());
    assert_eq!(state.nonce, 0);
    assert!(!state.admins.is_empty());
    assert!(state.owners.is_empty());

    utils_gclient::student_nft::mint(&api, &student_nft, "", "", "", "", false).await?;

    let state = utils_gclient::student_nft::get_state(&api, &student_nft).await?;
    assert!(!state.tokens.is_empty());
    assert_eq!(state.nonce, 1);
    assert!(!state.owners.is_empty());

    Ok(())
}
