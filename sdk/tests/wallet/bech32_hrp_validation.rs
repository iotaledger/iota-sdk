// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    client::ClientError,
    types::block::address::{Bech32Address, ToBech32Ext},
    wallet::{OutputParams, SendParams, WalletError},
};
use pretty_assertions::assert_eq;

use crate::wallet::common::{make_wallet, setup, tear_down};

#[ignore]
#[tokio::test]
async fn bech32_hrp_send_amount() -> Result<(), Box<dyn std::error::Error>> {
    let storage_path = "test-storage/bech32_hrp_send_amount";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;

    let error = wallet
        .send_with_params(
            [SendParams::new(
                1_000_000,
                Bech32Address::try_new("wronghrp", wallet.address().await)?,
            )?],
            None,
        )
        .await
        .unwrap_err();

    let bech32_hrp = wallet.client().get_bech32_hrp().await?;

    match error {
        WalletError::Client(error) => match error {
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
async fn bech32_hrp_prepare_output() -> Result<(), Box<dyn std::error::Error>> {
    let storage_path = "test-storage/bech32_hrp_prepare_output";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;

    let error = wallet
        .prepare_output(
            OutputParams {
                recipient_address: wallet.address().await.to_bech32_unchecked("wronghrp"),
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

    let bech32_hrp = wallet.client().get_bech32_hrp().await?;

    match error {
        WalletError::Client(error) => match error {
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
