// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

use iota_sdk::{
    client::{
        api::GetAddressesBuilder,
        constants::{IOTA_COIN_TYPE, SHIMMER_COIN_TYPE, SHIMMER_TESTNET_BECH32_HRP},
        secret::{stronghold::StrongholdSecretManager, SecretManager},
        storage::StorageProvider,
        stronghold::{Error as StrongholdError, StrongholdAdapter},
        Error as ClientError,
    },
    wallet::{ClientOptions, Error as WalletError, Wallet},
};

use crate::wallet::common::{setup, tear_down, NODE_LOCAL};

#[cfg(feature = "stronghold")]
#[tokio::test]
async fn stronghold_snapshot_v2_v3_migration() {
    let storage_path = "test-storage/stronghold_snapshot_v2_v3_migration";
    setup(storage_path).unwrap();

    let error = StrongholdSecretManager::builder()
        .password("current_password")
        .build("./tests/wallet/fixtures/v2.stronghold");

    assert!(matches!(
        error,
        Err(StrongholdError::UnsupportedSnapshotVersion { found, expected }) if found == 2 && expected == 3
    ));

    StrongholdAdapter::migrate_v2_to_v3(
        "./tests/wallet/fixtures/v2.stronghold",
        "current_password",
        Some("./tests/wallet/fixtures/v3.stronghold"),
        Some("new_password"),
    )
    .unwrap();

    let stronghold_secret_manager = SecretManager::Stronghold(
        StrongholdSecretManager::builder()
            .password("new_password")
            .build("./tests/wallet/fixtures/v3.stronghold")
            .unwrap(),
    );

    let addresses = GetAddressesBuilder::new(&stronghold_secret_manager)
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

    let restore_manager = Wallet::builder()
        .with_storage_path("test-storage/stronghold_snapshot_v2_v3_migration")
        .with_secret_manager(stronghold_secret_manager)
        .with_client_options(ClientOptions::new().with_node(NODE_LOCAL).unwrap())
        // Build with a different coin type, to check if it gets replaced by the one from the backup
        .with_coin_type(IOTA_COIN_TYPE)
        .finish()
        .await
        .unwrap();

    // restore with ignore_if_coin_type_mismatch: Some(true) to not overwrite the coin type
    let error = restore_manager
        .restore_backup(
            PathBuf::from("./tests/wallet/fixtures/v3.stronghold"),
            "wrong_password".to_string(),
            Some(false),
        )
        .await;

    match error {
        Err(WalletError::Client(err)) => {
            assert!(matches!(
                *err,
                ClientError::Stronghold(StrongholdError::InvalidPassword)
            ));
        }
        _ => panic!("unexpected error"),
    }

    std::fs::remove_file("./tests/wallet/fixtures/v3.stronghold").unwrap();
    tear_down(storage_path).unwrap();
}

#[cfg(feature = "stronghold")]
#[tokio::test]
async fn stronghold_snapshot_v2_v3_migration_same_path() {
    let storage_path = "test-storage/stronghold_snapshot_v2_v3_migration_same_path";
    setup(storage_path).unwrap();

    std::fs::copy(
        "./tests/wallet/fixtures/v2.stronghold",
        "./tests/wallet/fixtures/v2-copy.stronghold",
    )
    .unwrap();

    let error = StrongholdSecretManager::builder()
        .password("current_password")
        .build("./tests/wallet/fixtures/v2-copy.stronghold");

    assert!(matches!(
        error,
        Err(StrongholdError::UnsupportedSnapshotVersion { found, expected }) if found == 2 && expected == 3
    ));

    StrongholdAdapter::migrate_v2_to_v3(
        "./tests/wallet/fixtures/v2-copy.stronghold",
        "current_password",
        Some("./tests/wallet/fixtures/v2-copy.stronghold"),
        Some("new_password"),
    )
    .unwrap();

    SecretManager::Stronghold(
        StrongholdSecretManager::builder()
            .password("new_password")
            .build("./tests/wallet/fixtures/v2-copy.stronghold")
            .unwrap(),
    );

    std::fs::remove_file("./tests/wallet/fixtures/v2-copy.stronghold").unwrap();
    tear_down(storage_path).unwrap();
}

#[cfg(feature = "stronghold")]
#[tokio::test]
async fn stronghold_snapshot_with_data_v2_v3_migration() {
    let error = StrongholdSecretManager::builder()
        .password("current_password")
        .build("./tests/wallet/fixtures/v2_backup.stronghold");

    assert!(matches!(
        error,
        Err(StrongholdError::UnsupportedSnapshotVersion { found, expected }) if found == 2 && expected == 3
    ));

    StrongholdAdapter::migrate_v2_to_v3(
        "./tests/wallet/fixtures/v2_backup.stronghold",
        "current_password",
        Some("./tests/wallet/fixtures/v3WalletBackup.stronghold"),
        Some("new_password"),
    )
    .unwrap();

    let mut stronghold_secret_manager = StrongholdSecretManager::builder()
        .password("new_password")
        .build("./tests/wallet/fixtures/v3WalletBackup.stronghold")
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
            "rms1qr6n2qn5ps6l9r23l875suc0zzn9ztms5jkfhmst238exc3zl0vd5vkzhpy",
            "rms1qq0pqye5wy99lmu3lny9vus7zr8gn3sptstkd0wq66aed5st94g5clekuw4",
            "rms1qrj7c2rn4ac5kp2nqzyqssgqlsn9vz6hlyduuy7qgn9q4cw8gdrxc03mn0z",
            "rms1qrfycvex0h666r3h5nqm3vq3jcvvfgj3nlsq0nwzss4zn5rxcrv9kxdwuh0",
            "rms1qp24knzulhemnxhj9yvwryaya88e47x3kfatkrjcwsw9vuta3tu2vm9ve33",
            "rms1qzmeyc8tq6uff73pca98t7e7esaxflhmw5ml7ecqw7qsahp0knu2sfgrrhl",
            "rms1qpt29vjuj26wu5xdkq5ywclsh5jjrt8rtd354u2j9xyhjkslh7ngksye5zv",
            "rms1qqmm5ngpxkwcg2fc25c5xe8k8l5txknndmm44ygj6fg2sjhrtrjwz6rkx2g",
            "rms1qq6fjt9kgtu5lpetmmv9j7j7m9zwps8vm8y49ju5e3v3gtm974cgzsv2408",
            "rms1qry8euv504maz2a7mvqlzfv2uy5v6f3r6n6rt3tap8tfan96exgpupgn0cy"
        ]
    );

    std::fs::remove_file("./tests/wallet/fixtures/v3WalletBackup.stronghold").unwrap();
}
