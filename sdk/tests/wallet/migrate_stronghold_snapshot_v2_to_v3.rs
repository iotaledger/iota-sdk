// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

use iota_sdk::{
    client::{
        api::GetAddressesOptions,
        constants::{IOTA_COIN_TYPE, SHIMMER_COIN_TYPE, SHIMMER_TESTNET_BECH32_HRP},
        secret::{stronghold::StrongholdSecretManager, SecretManager},
        storage::StorageAdapter,
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
        .password("current_password".to_owned())
        .build("./tests/wallet/fixtures/v2.stronghold");

    assert!(matches!(
        error,
        Err(StrongholdError::UnsupportedSnapshotVersion { found, expected }) if found == 2 && expected == 3
    ));

    StrongholdAdapter::migrate_snapshot_v2_to_v3(
        "./tests/wallet/fixtures/v2.stronghold",
        "current_password".to_owned().into(),
        PBKDF_SALT,
        PBKDF_ITER,
        Some("./tests/wallet/fixtures/v3.stronghold"),
        Some("new_password".to_owned().into()),
    )
    .unwrap();

    let stronghold_secret_manager = SecretManager::Stronghold(
        StrongholdSecretManager::builder()
            .password("new_password".to_owned())
            .build("./tests/wallet/fixtures/v3.stronghold")
            .unwrap(),
    );

    let addresses = stronghold_secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_bech32_hrp(SHIMMER_TESTNET_BECH32_HRP)
                .with_coin_type(SHIMMER_COIN_TYPE)
                .with_account_index(0)
                .with_range(0..10),
        )
        .await
        .unwrap();

    // mnemonic: winter spend artefact viable cigar pink easy charge ranch license coyote cage brass mushroom repair
    // game attack peanut glad rather cart obey famous chat
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
            "wrong_password".to_owned(),
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
        .password("current_password".to_owned())
        .build("./tests/wallet/fixtures/v2-copy.stronghold");

    assert!(matches!(
        error,
        Err(StrongholdError::UnsupportedSnapshotVersion { found, expected }) if found == 2 && expected == 3
    ));

    StrongholdAdapter::migrate_snapshot_v2_to_v3(
        "./tests/wallet/fixtures/v2-copy.stronghold",
        "current_password".to_owned().into(),
        PBKDF_SALT,
        PBKDF_ITER,
        Some("./tests/wallet/fixtures/v2-copy.stronghold"),
        Some("new_password".to_owned().into()),
    )
    .unwrap();

    StrongholdSecretManager::builder()
        .password("new_password".to_owned())
        .build("./tests/wallet/fixtures/v2-copy.stronghold")
        .unwrap();

    std::fs::remove_file("./tests/wallet/fixtures/v2-copy.stronghold").unwrap();
}

#[cfg(feature = "stronghold")]
#[tokio::test]
async fn stronghold_snapshot_v2_v3_migration_with_backup() {
    let error = StrongholdSecretManager::builder()
        .password("current_password".to_owned())
        .build("./tests/wallet/fixtures/v2_with_backup.stronghold");

    assert!(matches!(
        error,
        Err(StrongholdError::UnsupportedSnapshotVersion { found, expected }) if found == 2 && expected == 3
    ));

    StrongholdAdapter::migrate_snapshot_v2_to_v3(
        "./tests/wallet/fixtures/v2_with_backup.stronghold",
        "current_password".to_owned().into(),
        PBKDF_SALT,
        PBKDF_ITER,
        Some("./tests/wallet/fixtures/v3_with_backup.stronghold"),
        Some("new_password".to_owned().into()),
    )
    .unwrap();

    let stronghold_secret_manager = StrongholdSecretManager::builder()
        .password("new_password".to_owned())
        .build("./tests/wallet/fixtures/v3_with_backup.stronghold")
        .unwrap();

    let coin_type_bytes = stronghold_secret_manager
        .get_bytes("coin_type")
        .await
        .unwrap()
        .expect("missing data");
    let coin_type = u32::from_le_bytes(coin_type_bytes.try_into().expect("invalid coin_type"));
    assert_eq!(coin_type, SHIMMER_COIN_TYPE);

    let addresses = SecretManager::Stronghold(stronghold_secret_manager)
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_bech32_hrp(SHIMMER_TESTNET_BECH32_HRP)
                .with_coin_type(SHIMMER_COIN_TYPE)
                .with_account_index(0)
                .with_range(0..10),
        )
        .await
        .unwrap();

    // mnemonic: brisk egg allow van merge process chest type dove bomb proud purity monitor snap load verb utility
    // hungry cube coast fetch pioneer gadget credit
    assert_eq!(
        addresses,
        [
            "rms1qza3rek2ffhxtfjpaswfsc9hkekj7j5lrmzkp5fmr2wzlnz56hhvskcs4mz",
            "rms1qqjpevw6d7spdzsmfrrzdna64fpdqh89jme8q4g47ek4l0kz3m5eqersz6g",
            "rms1qqgc7rpa4u0uf4e085yap8ksz7jsrdqhy6saqszyt8etxleyph02spcdtsf",
            "rms1qqv0pspup06eszwf9ne7xccxkx2eks6x5h8528cgxmnu382qnay7u8mkdfh",
            "rms1qpqts58s8z6a0t3rs7z2q7qzr38h847rj3urc7esyqa0sdescg2sx553dct",
            "rms1qrquh2afd0sx0sg26hamuksdm5sntzs0c903aptrv0lsfhehc0etg58j9wq",
            "rms1qzzkwr6edw0pr25jzey7zmh5hkka4q4cqvvk5yhlgcg2ga7k8hzk2t0va50",
            "rms1qp9mt8elk7x32npvvdtxnmdtt5n4wxe28zrwhc9hnyrr6jpsgp7dx4zp2nn",
            "rms1qpt9gpycwmqy5ywrup8tmgpvrvxspqz7c9u9erk9qrwq72rk95y567p7j5z",
            "rms1qqee8vjh3pqehpm5p53s45y4e7f5kusxnadt35hqyp5vvkrf8e3z2rrd3t9"
        ]
    );

    std::fs::remove_file("./tests/wallet/fixtures/v3_with_backup.stronghold").unwrap();
}
