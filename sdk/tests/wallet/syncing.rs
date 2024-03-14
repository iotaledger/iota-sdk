// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// use iota_sdk::{
//     types::block::output::{
//         unlock_condition::{AddressUnlockCondition, ExpirationUnlockCondition, StorageDepositReturnUnlockCondition},
//         AccountId, AccountOutputBuilder, BasicOutputBuilder, NftId, NftOutputBuilder, UnlockCondition,
//     },
//     wallet::{Result, SyncOptions},
// };
// use pretty_assertions::assert_eq;

// use crate::wallet::common::{make_wallet, request_funds, setup, tear_down};

// #[tokio::test]
// #[cfg(feature = "rocksdb")]
// async fn updated_default_sync_options() -> Result<(), WalletError> {
//     let storage_path = "test-storage/updated_default_sync_options";
//     setup(storage_path)?;

//     let default_sync = SyncOptions::default();

//     let wallet = make_wallet(storage_path, None, None).await?;

//     assert_eq!(default_sync, wallet.default_sync_options().await);

//     let custom_options = SyncOptions {
//         sync_only_most_basic_outputs: true,
//         ..Default::default()
//     };
//     wallet.set_default_sync_options(custom_options.clone()).await?;
//     assert_eq!(custom_options, wallet.default_sync_options().await);

//     drop(wallet);

//     let wallet = make_wallet(storage_path, None, None).await?;

//     assert_eq!(custom_options, wallet.default_sync_options().await);

//     tear_down(storage_path)
// }

// #[ignore]
// #[tokio::test]
// async fn sync_only_most_basic_outputs() -> Result<(), WalletError> {
//     let storage_path_0 = "test-storage/sync_only_most_basic_outputs_0";
//     setup(storage_path_0)?;
//     let storage_path_1 = "test-storage/sync_only_most_basic_outputs_1";
//     setup(storage_path_1)?;

//     let wallet_0 = create_wallet_with_funds(storage_path_0, None, None, 1).await?;
//     let wallet_1 = make_wallet(storage_path_1, None, None).await?;

//     let wallet_1_address = wallet_1.address().clone();

//     let token_supply = wallet_0.client().get_token_supply().await?;
//     // Only one basic output without further unlock conditions
//     let outputs = [
//         BasicOutputBuilder::new_with_amount(1_000_000)
//             .with_unlock_conditions([AddressUnlockCondition::new(wallet_1_address.clone())])
//             .finish_output(token_supply)?,
//         BasicOutputBuilder::new_with_amount(1_000_000)
//             .with_unlock_conditions([
//                 UnlockCondition::Address(AddressUnlockCondition::new(wallet_1_address.clone())),
//                 UnlockCondition::Expiration(ExpirationUnlockCondition::new(
//                     wallet_1_address.clone(),
//                     // Already expired
//                     wallet_0.client().get_slot_index().await? - 5000,
//                 )?),
//             ])
//             .finish_output(token_supply)?,
//         BasicOutputBuilder::new_with_amount(1_000_000)
//             .with_unlock_conditions([
//                 UnlockCondition::Address(AddressUnlockCondition::new(wallet_1_address.clone())),
//                 UnlockCondition::Expiration(ExpirationUnlockCondition::new(
//                     wallet_1_address.clone(),
//                     // Not expired
//                     wallet_0.client().get_slot_index().await? + 5000,
//                 )?),
//             ])
//             .finish_output(token_supply)?,
//         BasicOutputBuilder::new_with_amount(1_000_000)
//             .with_unlock_conditions([
//                 UnlockCondition::Address(AddressUnlockCondition::new(wallet_1_address.clone())),
//                 UnlockCondition::StorageDepositReturn(StorageDepositReturnUnlockCondition::new(
//                     wallet_1_address.clone(),
//                     1_000_000,
//                     token_supply,
//                 )?),
//             ])
//             .finish_output(token_supply)?,
//         NftOutputBuilder::new_with_amount(1_000_000, NftId::null())
//             .with_unlock_conditions([AddressUnlockCondition::new(wallet_1_address.clone())])
//             .finish_output(token_supply)?,
//         NftOutputBuilder::new_with_amount(1_000_000, NftId::null())
//             .with_unlock_conditions([
//                 UnlockCondition::Address(AddressUnlockCondition::new(wallet_1_address.clone())),
//                 UnlockCondition::Expiration(ExpirationUnlockCondition::new(
//                     wallet_1_address.clone(),
//                     wallet_0.client().get_slot_index().await? + 5000,
//                 )?),
//             ])
//             .finish_output(token_supply)?,
//         AccountOutputBuilder::new_with_amount(1_000_000, AccountId::null())
//             .with_unlock_conditions([UnlockCondition::Address(AddressUnlockCondition::new(
//                 wallet_1_address.clone(),
//             ))])
//             .finish_output(token_supply)?,
//     ];

//     let tx = wallet_0.send_outputs(outputs, None).await?;
//     wallet_0
//         .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
//         .await?;

//     // Sync with sync_only_most_basic_outputs: true, only the first output should be synced
//     let balance = wallet_1
//         .sync(Some(SyncOptions {
//             sync_only_most_basic_outputs: true,
//             ..Default::default()
//         }))
//         .await?;
//     assert_eq!(balance.potentially_locked_outputs().len(), 0);
//     assert_eq!(balance.nfts().len(), 0);
//     assert_eq!(balance.accounts().len(), 0);
//     let unspent_outputs = wallet_1.unspent_outputs(None).await?;
//     assert_eq!(unspent_outputs.len(), 1);
//     unspent_outputs.into_iter().for_each(|output_with_ext_metadata| {
//         assert!(output_with_ext_metadata.output.is_basic());
//         assert_eq!(output_with_ext_metadata.output.unlock_conditions().unwrap().len(), 1);
//         assert_eq!(
//             output_with_ext_metadata
//                 .output
//                 .unlock_conditions()
//                 .unwrap()
//                 .address()
//                 .unwrap()
//                 .address(),
//             wallet_1_address.as_ref()
//         );
//     });

//     tear_down(storage_path)
// }

// #[ignore]
// #[tokio::test]
// async fn sync_incoming_transactions() -> Result<(), WalletError> {
//     let storage_path_0 = "test-storage/sync_incoming_transactions_0";
//     setup(storage_path_0)?;
//     let storage_path_1 = "test-storage/sync_incoming_transactions_1";
//     setup(storage_path_1)?;

//     let wallet_0 = create_wallet_with_funds(storage_path_0, None, None, 1).await?;
//     let wallet_1 = make_wallet(storage_path_1, None, None).await?;

//     let wallet_1_address = wallet_1.address().clone();

//     let token_supply = wallet_0.client().get_token_supply().await?;

//     let outputs = [
//         BasicOutputBuilder::new_with_amount(750_000)
//             .with_unlock_conditions([AddressUnlockCondition::new(wallet_1_address.clone())])
//             .finish_output(token_supply)?,
//         BasicOutputBuilder::new_with_amount(250_000)
//             .with_unlock_conditions([AddressUnlockCondition::new(wallet_1_address)])
//             .finish_output(token_supply)?,
//     ];

//     let tx = wallet_0.send_outputs(outputs, None).await?;
//     wallet_0
//         .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
//         .await?;

//     wallet_1
//         .sync(Some(SyncOptions {
//             sync_incoming_transactions: true,
//             ..Default::default()
//         }))
//         .await?;
//     let incoming_transactions = wallet_1.incoming_transactions().await;
//     assert_eq!(incoming_transactions.len(), 1);
//     let incoming_tx = wallet_1.get_incoming_transaction(&tx.transaction_id).await.unwrap();
//     assert_eq!(incoming_tx.inputs.len(), 1);
//     let transaction = incoming_tx.payload.transaction();

//     // 2 created outputs plus remainder
//     assert_eq!(transaction.outputs().len(), 3);

//     tear_down(storage_path)
// }

// #[ignore]
// #[tokio::test]
// #[cfg(feature = "storage")]
// async fn background_syncing() -> Result<(), WalletError> {
//     let storage_path = "test-storage/background_syncing";
//     setup(storage_path)?;

//     let wallet = make_wallet(storage_path, None, None).await?;

//     wallet.start_background_syncing(None, None).await?;

//     iota_sdk::client::request_funds_from_faucet(
//         crate::wallet::common::FAUCET_URL,
//         &wallet.address().clone(),
//     )
//     .await?;

//     for _ in 0..30 {
//         tokio::time::sleep(std::time::Duration::from_secs(2)).await;
//         let balance = wallet.balance().await?;
//         if balance.base_coin().available() > 0 {
//             break;
//         }
//     }

//     // Balance should be != 0 without calling wallet.sync()
//     let balance = wallet.balance().await?;
//     if balance.base_coin().available() == 0 {
//         panic!("Faucet no longer wants to hand over coins or background syncing failed");
//     }

//     wallet.stop_background_syncing().await?;

//     tear_down(storage_path)
// }
