// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::wallet::{MintNftParams, Result, SendNftParams, SendParams, TransactionOptions};
use pretty_assertions::assert_eq;

use crate::wallet::common::{make_wallet, request_funds, setup, tear_down};

// #[ignore]
// #[tokio::test]
// async fn send_amount() -> Result<()> {
//     let storage_path_0 = "test-storage/send_amount_0";
//     setup(storage_path_0)?;
//     let storage_path_1 = "test-storage/send_amount_1";
//     setup(storage_path_1)?;

//     let (wallet_0, secret_manager_0) = make_wallet(storage_path_0, None, None).await?;
//     request_funds(&wallet_0, 1).await?;

//     let (wallet_1, secret_manager_1) = make_wallet(storage_path_1, None, None).await?;

//     let amount = 1_000_000;
//     let tx = wallet_0
//         .send_with_params([SendParams::new(amount, wallet_1.address().await)?], None)
//         .await?;

//     wallet_0
//         .reissue_transaction_until_included(&tx.transaction_id, None, None)
//         .await?;

//     let balance = wallet_1.sync(&secret_manager_1,None).await.unwrap();
//     assert_eq!(balance.base_coin().available(), amount);

//     tear_down(storage_path)
// }

// #[ignore]
// #[tokio::test]
// async fn send_amount_127_outputs() -> Result<()> {
//     let storage_path_0 = "test-storage/send_amount_127_outputs_0";
//     setup(storage_path_0)?;
//     let storage_path_1 = "test-storage/send_amount_127_outputs_1";
//     setup(storage_path_1)?;

//     let (wallet_0, secret_manager_0) = make_wallet(storage_path_0, None, None).await?;
//     request_funds(&wallet_0, 1).await?;

//     let (wallet_1, secret_manager_1) = make_wallet(storage_path_1, None, None).await?;

//     let amount = 1_000_000;
//     let tx = wallet_0
//         .send_with_params(
//             vec![
//                 SendParams::new(
//                     amount,
//                     wallet_1.address().await,
//                 )?;
//                 // Only 127, because we need one remainder
//                 127
//             ],
//             None,
//         )
//         .await?;

//     wallet_0
//         .reissue_transaction_until_included(&tx.transaction_id, None, None)
//         .await?;

//     let balance = wallet_1.sync(&secret_manager_1,None).await.unwrap();
//     assert_eq!(balance.base_coin().available(), 127 * amount);

//     tear_down(storage_path)
// }

// #[ignore]
// #[tokio::test]
// async fn send_amount_custom_input() -> Result<()> {
//     let storage_path_0 = "test-storage/send_amount_custom_input_0";
//     setup(storage_path_0)?;
//     let storage_path_1 = "test-storage/send_amount_custom_input_1";
//     setup(storage_path_1)?;

//     let (wallet_0, secret_manager_0) = make_wallet(storage_path_0, None, None).await?;
//     request_funds(&wallet_0, 1).await?;

//     let (wallet_1, secret_manager_1) = make_wallet(storage_path_1, None, None).await?;

//     // Send 10 outputs to wallet_1
//     let amount = 1_000_000;
//     let tx = wallet_0
//         .send_with_params(
//             vec![SendParams::new(amount, wallet_1.first_address_bech32().await)?; 10],
//             None,
//         )
//         .await?;

//     wallet_0
//         .reissue_transaction_until_included(&tx.transaction_id, None, None)
//         .await?;

//     let balance = wallet_1.sync(&secret_manager_1,None).await.unwrap();
//     assert_eq!(balance.base_coin().available(), 10 * amount);

//     // Send back with custom provided input
//     let custom_input = &wallet_1.unspent_outputs(None).await?[5];
//     let tx = wallet_1
//         .send_with_params(
//             [SendParams::new(amount, wallet_0.first_address_bech32().await)?],
//             Some(TransactionOptions {
//                 custom_inputs: Some(vec![custom_input.output_id]),
//                 ..Default::default()
//             }),
//         )
//         .await?;

//     assert_eq!(tx.inputs.len(), 1);
//     assert_eq!(tx.inputs.first().unwrap().metadata.output_id(), &custom_input.output_id);

//     tear_down(storage_path)
// }

// #[ignore]
// #[tokio::test]
// async fn send_nft() -> Result<()> {
//     let storage_path_0 = "test-storage/send_nft_0";
//     setup(storage_path_0)?;
//     let storage_path_1 = "test-storage/send_nft_1";
//     setup(storage_path_1)?;

//     let (wallet_0, secret_manager_0) = make_wallet(storage_path_0, None, None).await?;
//     request_funds(&wallet_0, 2).await?;

//     let (wallet_1, secret_manager_1) = make_wallet(storage_path_1, None, None).await?;

//     let nft_options = [MintNftParams::new()
//         .with_address(wallet_0.address().await)
//         .with_metadata(b"some nft metadata".to_vec())
//         .with_immutable_metadata(b"some immutable nft metadata".to_vec())];

//     let transaction = wallet_0.mint_nfts(nft_options, None).await.unwrap();
//     wallet_0
//         .reissue_transaction_until_included(&transaction.transaction_id, None, None)
//         .await?;
//     let nft_id = *wallet_0.sync(&secret_manager_0,None).await?.nfts().first().unwrap();

//     // Send to wallet 1
//     let transaction = wallet_0
//         .send_nft(
//             [SendNftParams::new(wallet_1.address().await, nft_id)?],
//             None,
//         )
//         .await
//         .unwrap();
//     wallet_0
//         .reissue_transaction_until_included(&transaction.transaction_id, None, None)
//         .await?;

//     let balance = wallet_1.sync(&secret_manager_1,None).await?;
//     assert_eq!(balance.nfts().len(), 1);
//     assert_eq!(*balance.nfts().first().unwrap(), nft_id);

//     tear_down(storage_path)
// }

// #[ignore]
// #[tokio::test]
// async fn send_with_note() -> Result<()> {
//     let storage_path_0 = "test-storage/send_with_note_0";
//     setup(storage_path_0)?;
//     let storage_path_1 = "test-storage/send_with_note_1";
//     setup(storage_path_1)?;

//     let (wallet_0, secret_manager_0) = make_wallet(storage_path_0, None, None).await?;
//     request_funds(&wallet_0, 1).await?;

//     let (wallet_1, secret_manager_1) = make_wallet(storage_path_1, None, None).await?;

//     let amount = 1_000_000;
//     let tx = wallet_0
//         .send_with_params(
//             [SendParams::new(amount, wallet_1.address().await)?],
//             Some(TransactionOptions {
//                 note: Some(String::from("send_with_note")),
//                 ..Default::default()
//             }),
//         )
//         .await?;

//     assert_eq!(tx.note, Some(String::from("send_with_note")));

//     tear_down(storage_path)
// }

// #[ignore]
// #[tokio::test]
// async fn conflicting_transaction() -> Result<()> {
//     let storage_path_0 = "test-storage/conflicting_transaction_0";
//     let storage_path_1 = "test-storage/conflicting_transaction_1";
//     setup(storage_path_0)?;
//     setup(storage_path_1)?;

//     let mnemonic = iota_sdk::client::utils::generate_mnemonic()?;
//     // Create two wallets with the same mnemonic
//     let wallet_0 = make_wallet(storage_path_0, Some(mnemonic.clone()), None).await?;
//     request_funds(&wallet_0, 1).await?;
//     let wallet_1 = make_wallet(storage_path_1, Some(mnemonic), None).await?;

//     // Balance should be equal
//     assert_eq!(wallet_0.sync(&secret_manager_0,None).await?, wallet_1.sync(&secret_manager_1,None).await?);

//     // Send transaction without syncing again
//     let tx = wallet_0
//         .send_with_params(
//             [SendParams::new(
//                 1_000_000,
//                 wallet_0.address().await,
//             )?],
//             None,
//         )
//         .await?;
//     wallet_0
//         .reissue_transaction_until_included(&tx.transaction_id, None, None)
//         .await?;
//     // Second transaction will be conflicting
//     let tx = wallet_1
//         .send_with_params(
//             [SendParams::new(
//                 // Something in the transaction must be different than in the first one, otherwise it will be the
// same                 // one
//                 2_000_000,
//                 wallet_0.address().await,
//             )?],
//             None,
//         )
//         .await?;
//     // Should return an error since the tx is conflicting
//     match wallet_1
//         .reissue_transaction_until_included(&tx.transaction_id, None, None)
//         .await
//         .unwrap_err()
//     {
//         iota_sdk::wallet::Error::Client(client_error) => {
//             let iota_sdk::client::Error::TangleInclusion(_) = *client_error else {
//                 panic!("Expected TangleInclusion error");
//             };
//         }
//         _ => panic!("Expected TangleInclusion error"),
//     }

//     // After syncing the balance is still equal
//     assert_eq!(wallet_0.sync(&secret_manager_0,None).await?, wallet_1.sync(&secret_manager_1,None).await?);

//     let conflicting_tx = wallet_1.get_transaction(&tx.transaction_id).await.unwrap();
//     assert_eq!(
//         conflicting_tx.inclusion_state,
//         iota_sdk::wallet::types::InclusionState::Conflicting
//     );
//     // The conflicting tx is also removed from the pending txs
//     assert!(wallet_1.pending_transactions().await.is_empty());

//     tear_down(storage_path_0).ok();
//     tear_down(storage_path_1)
// }

// #[tokio::test]
// #[cfg(all(feature = "ledger_nano", feature = "events"))]
// #[ignore = "requires ledger nano instance"]
// async fn prepare_transaction_ledger() -> Result<()> {
//     use iota_sdk::wallet::events::{types::TransactionProgressEvent, WalletEvent, WalletEventType};

//     let storage_path_0 = "test-storage/wallet_address_generation_ledger_0";
//     setup(storage_path_0)?;
//     let storage_path_1 = "test-storage/wallet_address_generation_ledger_1";
//     setup(storage_path_1)?;

//     let wallet_0 = crate::wallet::common::make_ledger_nano_wallet(storage_path_0, None).await?;
//     request_funds(&wallet_0, 1).await?;

//     let (wallet_1, secret_manager_1) = make_wallet(storage_path_1, None, None).await?;

//     let amount = 1_000_000;

//     let (sender, mut receiver) = tokio::sync::mpsc::channel(1);

//     wallet
//         .listen([WalletEventType::TransactionProgress], move |event| {
//             if let WalletEvent::TransactionProgress(progress) = &event.event {
//                 if let TransactionProgressEvent::PreparedTransaction(data) = progress {
//                     sender
//                         .try_send(data.as_ref().clone())
//                         .expect("too many PreparedTransaction events");
//                 }
//             } else {
//                 panic!("expected TransactionProgress event")
//             }
//         })
//         .await;

//     let tx = wallet_0
//         .send_with_params([SendParams::new(amount, wallet_1.address().await)?], None)
//         .await?;

//     let data = receiver.recv().await.expect("never recieved event");
//     // TODO put it back
//     // assert_eq!(data.transaction, tx.payload.transaction().into());
//     for (sign, input) in data.inputs_data.iter().zip(tx.inputs) {
//         assert_eq!(sign.output, input.output);
//         assert_eq!(sign.output_metadata, input.metadata);
//     }

//     tear_down(storage_path)
// }
