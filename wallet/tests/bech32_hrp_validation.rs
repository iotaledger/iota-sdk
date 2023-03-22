// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod common;

use iota_client::Error as ClientError;
use iota_wallet::{account::OutputOptions, AddressWithAmount, Error, Result};

#[ignore]
#[tokio::test]
async fn bech32_hrp_send_amount() -> Result<()> {
    let storage_path = "test-storage/bech32_hrp_send_amount";
    common::setup(storage_path)?;

    let manager = common::make_manager(storage_path, None, None).await?;

    let account = manager.create_account().finish().await?;

    let error = account
        .send_amount(
            vec![AddressWithAmount {
                address: account.addresses().await?[0].address().as_ref().to_bech32("wronghrp"),
                amount: 1_000_000,
            }],
            None,
        )
        .await
        .unwrap_err();

    let bech32_hrp = account.client().get_bech32_hrp().await?;

    match error {
        Error::Client(error) => match *error {
            ClientError::InvalidBech32Hrp {
                provided_bech32_hrp,
                expected_bech32_hrp,
            } => {
                assert_eq!(provided_bech32_hrp, "wronghrp".to_string());
                assert_eq!(expected_bech32_hrp, bech32_hrp);
            }
            _ => panic!("expected InvalidBech32Hrp error variant"),
        },
        _ => panic!("expected Client error variant"),
    }

    common::tear_down(storage_path)
}

#[ignore]
#[tokio::test]
async fn bech32_hrp_prepare_output() -> Result<()> {
    let storage_path = "test-storage/bech32_hrp_prepare_output";
    common::setup(storage_path)?;

    let manager = common::make_manager(storage_path, None, None).await?;
    let account = manager.create_account().finish().await?;

    let error = account
        .prepare_output(
            OutputOptions {
                recipient_address: account.addresses().await?[0].address().as_ref().to_bech32("wronghrp"),
                amount: 1_000_000,
                assets: None,
                features: None,
                unlocks: None,
                storage_deposit: None,
            },
            None,
        )
        .await
        .unwrap_err();

    let bech32_hrp = account.client().get_bech32_hrp().await?;

    match error {
        Error::Client(error) => match *error {
            ClientError::InvalidBech32Hrp {
                provided_bech32_hrp,
                expected_bech32_hrp,
            } => {
                assert_eq!(provided_bech32_hrp, "wronghrp".to_string());
                assert_eq!(expected_bech32_hrp, bech32_hrp);
            }
            _ => panic!("expected InvalidBech32Hrp error variant"),
        },
        _ => panic!("expected Client error variant"),
    }

    common::tear_down(storage_path)
}
