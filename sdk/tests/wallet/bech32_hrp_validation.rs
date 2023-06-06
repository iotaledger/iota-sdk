// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    client::Error as ClientError,
    types::block::address::{Bech32Address, ToBech32Ext},
    wallet::{account::OutputParams, Error, Result, SendAmountParams},
};

use crate::wallet::common::{make_wallet, setup, tear_down};

#[ignore]
#[tokio::test]
async fn bech32_hrp_send_amount() -> Result<()> {
    let storage_path = "test-storage/bech32_hrp_send_amount";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;

    let account = wallet.create_account().finish().await?;

    let error = account
        .send_amount(
            [SendAmountParams::new(
                Bech32Address::try_new("wronghrp", account.addresses().await?[0].address())?,
                1_000_000,
            )?],
            None,
        )
        .await
        .unwrap_err();

    let bech32_hrp = account.client().get_bech32_hrp().await?;

    match error {
        Error::Client(error) => match *error {
            ClientError::Bech32HrpMismatch { provided, expected } => {
                assert_eq!(provided, "wronghrp");
                assert_eq!(expected, bech32_hrp.to_string());
            }
            _ => panic!("expected InvalidBech32Hrp error variant"),
        },
        _ => panic!("expected Client error variant"),
    }

    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
async fn bech32_hrp_prepare_output() -> Result<()> {
    let storage_path = "test-storage/bech32_hrp_prepare_output";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;
    let account = wallet.create_account().finish().await?;

    let error = account
        .prepare_output(
            OutputParams {
                recipient_address: account.addresses().await?[0]
                    .address()
                    .as_ref()
                    .to_bech32_unchecked("wronghrp"),
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
            ClientError::Bech32HrpMismatch { provided, expected } => {
                assert_eq!(provided, "wronghrp");
                assert_eq!(expected, bech32_hrp.to_string());
            }
            _ => panic!("expected InvalidBech32Hrp error variant"),
        },
        _ => panic!("expected Client error variant"),
    }

    tear_down(storage_path)
}
