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

const PBKDF_SALT: &str = "wallet.rs";
const PBKDF_ITER: u32 = 100;

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

    StrongholdAdapter::migrate_snapshot_v2_to_v3(
        "./tests/wallet/fixtures/v2.stronghold",
        "current_password",
        PBKDF_SALT,
        PBKDF_ITER,
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

    // mnemonic: winter spend artefact viable cigar pink easy charge ranch license coyote cage brass mushroom repair game attack peanut glad rather cart obey famous chat
    assert_eq!(
        addresses,
        [
            "rms1qrzyp87nqvcdctwrc7yzxjnwwetagffslhuknmey8t4fdf6552dnjxuaj3u",
            "rms1qpsvxm4q9p3xe4tkqjr04a64j0gxvhe0prt06vxwp0spkxfc8nr5gs28u0l",
            "rms1qqrt84z3dlhfy9wxa9whpn3xz9ugtspy80xwpu84p0cdszjc9vwr6d50m6k",
            "rms1qr57qle3rtj2kh5dtq8f00ys79cwa9dc3hq0hacd63aw0ngrx620vcctcza",
            "rms1qqkyhtt2lrqqpufcvydf6s7h8netyw0utuf0h458nafz26298wrespmrnyj",
            "rms1qz683r2zpl0qz355c3xlsrskke3563y9tn0s8u498zaxssr8ves0xq5p6c0",
            "rms1qrj4hszlpj6dnh3tpam5lwp0whgquj995ujsjvw0rxa5rt0sacrxxh4j9t7",
            "rms1qra52h296s4ty3x5np748xtruw52we63ardlp96v25yl9gzml7f7z8cvp9k",
            "rms1qqch88nnarx0czrdjee6v74ym08ruccr5w3wwxpk7nwjh3ll0dynxlnjtrw",
            "rms1qqrsl203x9wq29a2amcdszsps2lz7q20mqkh8t8vch0rz86pss9fwa8pjgx",
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
            None,
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

    StrongholdAdapter::migrate_snapshot_v2_to_v3(
        "./tests/wallet/fixtures/v2-copy.stronghold",
        "current_password",
        PBKDF_SALT,
        PBKDF_ITER,
        Some("./tests/wallet/fixtures/v2-copy.stronghold"),
        Some("new_password"),
    )
    .unwrap();

    StrongholdSecretManager::builder()
        .password("new_password")
        .build("./tests/wallet/fixtures/v2-copy.stronghold")
        .unwrap();

    std::fs::remove_file("./tests/wallet/fixtures/v2-copy.stronghold").unwrap();
}

#[cfg(feature = "stronghold")]
#[tokio::test]
async fn stronghold_snapshot_v2_v3_migration_with_backup() {
    let error = StrongholdSecretManager::builder()
        .password("current_password")
        .build("./tests/wallet/fixtures/v2_with_backup.stronghold");

    assert!(matches!(
        error,
        Err(StrongholdError::UnsupportedSnapshotVersion { found, expected }) if found == 2 && expected == 3
    ));

    StrongholdAdapter::migrate_snapshot_v2_to_v3(
        "./tests/wallet/fixtures/v2_with_backup.stronghold",
        "current_password",
        PBKDF_SALT,
        PBKDF_ITER,
        Some("./tests/wallet/fixtures/v3_with_backup.stronghold"),
        Some("new_password"),
    )
    .unwrap();

    let stronghold_secret_manager = StrongholdSecretManager::builder()
        .password("new_password")
        .build("./tests/wallet/fixtures/v3_with_backup.stronghold")
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

    std::fs::remove_file("./tests/wallet/fixtures/v3_with_backup.stronghold").unwrap();
}
