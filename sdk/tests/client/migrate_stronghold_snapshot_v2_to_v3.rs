// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::client::{
    api::GetAddressesBuilder,
    constants::{SHIMMER_COIN_TYPE, SHIMMER_TESTNET_BECH32_HRP},
    secret::{stronghold::StrongholdSecretManager, SecretManager},
    storage::StorageProvider,
    stronghold::StrongholdAdapter,
    Error,
};

#[cfg(feature = "stronghold")]
#[tokio::test]
async fn stronghold_snapshot_v2_v3_migration() {
    let error = StrongholdSecretManager::builder()
        .password("current_password")
        .build("./tests/fixtures/v2.stronghold");

    assert!(matches!(
        error,
        Err(Error::StrongholdUnsupportedSnapshotVersion { found, expected }) if found == 2 && expected == 3
    ));

    StrongholdAdapter::migrate_v2_to_v3(
        "./tests/fixtures/v2.stronghold",
        "current_password",
        Some("./tests/fixtures/v3.stronghold"),
        Some("new_password"),
    )
    .unwrap();

    let stronghold_secret_manager = StrongholdSecretManager::builder()
        .password("new_password")
        .build("./tests/fixtures/v3.stronghold")
        .unwrap();

    let addresses = GetAddressesBuilder::new(&SecretManager::Stronghold(stronghold_secret_manager))
        .with_bech32_hrp(SHIMMER_TESTNET_BECH32_HRP)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .with_account_index(0)
        .with_range(0..10)
        .finish()
        .await
        .unwrap();

    // These addresses have been generated with the same snapshot file but with a branch supporting v2 as the current
    // branch only supports v3.
    assert_eq!(
        addresses,
        [
            "rms1qrut5ajyfrtgjs325kd9chwfwyyy2z3fewy4vgy0vvdtf2pr8prg5u3zwjn",
            "rms1qqhvvur9xfj6yhgsxfa4f8xst7vz9zxeu3vcxds8mh4a6jlpteq9xrajhtf",
            "rms1qr47gz3xxjqpjrwd0yu5glhqrth6w0t08npney8000ust2lcw2r92j5a8rt",
            "rms1qz0dzxdax3g0fkz8sz5newd8ctc40ge3lj9ascj7wp7as52n2pjl64capk2",
            "rms1qryrtqe4y3secxy2vuj6fqmwzmxu7uwxed57h2k4ll469j6jtsw2y3wup6s",
            "rms1qz0v4tgthfnrx7u9tvqrt4za94a4s6uvg9yp46fg8musjk467fd0xvleq0a",
            "rms1qqs75c7mumdzjl3fhg6pg60daec95f3frfysx46hh87vud7xcr9xjhzlcxe",
            "rms1qpz3s73aus2m9sv9pxjxqskg2l5fq9v2ye4ed5xuyageq5kgpg6qkz6dllz",
            "rms1qz805rfsr0sc7aa3kvlv2z5pa5xyp3jla63p06tkqlx39dzctf6awf78c3m",
            "rms1qz8vuyjn0w05jragnca7k3eazxtqhjz4yg8q52ghcrpvr6k3xfn6grfrn2l",
        ]
    );

    std::fs::remove_file("./tests/fixtures/v3.stronghold").unwrap();
}

#[cfg(feature = "stronghold")]
#[tokio::test]
async fn stronghold_snapshot_with_data_v2_v3_migration() {
    let error = StrongholdSecretManager::builder()
        .password("current_password")
        .build("./tests/fixtures/v2_backup.stronghold");

    assert!(matches!(
        error,
        Err(Error::StrongholdUnsupportedSnapshotVersion { found, expected }) if found == 2 && expected == 3
    ));

    StrongholdAdapter::migrate_v2_to_v3(
        "./tests/fixtures/v2_backup.stronghold",
        "current_password",
        Some("./tests/fixtures/v3WalletBackup.stronghold"),
        Some("new_password"),
    )
    .unwrap();

    let mut stronghold_secret_manager = StrongholdSecretManager::builder()
        .password("new_password")
        .build("./tests/fixtures/v3WalletBackup.stronghold")
        .unwrap();

    let coin_type_bytes = stronghold_secret_manager
        .get("coin_type".as_bytes())
        .await
        .unwrap()
        .expect("missing data");
    let coin_type = u32::from_le_bytes(coin_type_bytes.try_into().expect("invalid coin_type"));
    assert_eq!(coin_type, SHIMMER_COIN_TYPE);

    let addresses = GetAddressesBuilder::new(&SecretManager::Stronghold(stronghold_secret_manager))
        .with_bech32_hrp(SHIMMER_TESTNET_BECH32_HRP)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .with_account_index(0)
        .with_range(0..10)
        .finish()
        .await
        .unwrap();

    // mnemonic: endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because
    // joke dream slender leisure group reason prepare broken river
    assert_eq!(
        addresses,
        [
            "rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy",
            "rms1qzzk86qv30l4e85ljtccxa0ruy8y7u8zn2dle3g8dv2tl2m4cu227a7n2wj",
            "rms1qqjrtmslm36l3lyd8086w9qdxd9mudu2z2qyywlltpycn2ftxsdmu9ulj47",
            "rms1qzz75j5h7vd5melxwersz49jja36m2vvnawedu0l6rlhg70ylzqp52lx8zf",
            "rms1qzvs2rvq5ef79vhuel354mnvzfz049gyyf808zmjculuatt56u92vc4v4p3",
            "rms1qpk0dj5lmv5r8d64f3x0qwx3jccredwa42xscdreqma73f3dxymaqdy645t",
            "rms1qqlp6nurrz459jvvvuhv6t965v6cmlata57kf6auv6955uyp77uyw0egzgn",
            "rms1qq2tu523zqzjxgdl69au8a9yewmd9ztctssv40dsv7lfd0tp36w4xdfzcyr",
            "rms1qpw904c253cc9yn2dkhxx89u6r5j9vzezfatchrlugrkcvvhed3hckx6h2u",
            "rms1qpa646cegkqsx9eht793nmux2vjwa63jrenqe87xq8j5wysfhu4l28k4ep9"
        ]
    );

    std::fs::remove_file("./tests/fixtures/v3WalletBackup.stronghold").unwrap();
}
