// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use iota_sdk::wallet::{message_interface::dtos::AccountDto, wallet::Wallet};
#[cfg(feature = "stronghold")]
use zeroize::Zeroize;

use super::account::call_account_method_internal;
use crate::{method::WalletMethod, response::Response, Result};

/// Call a wallet method.
pub(crate) async fn call_wallet_method_internal(wallet: &Wallet, method: WalletMethod) -> Result<Response> {
    let response = match method {
        WalletMethod::CreateAccount { alias, bech32_hrp } => {
            let mut builder = wallet.create_account();

            if let Some(alias) = alias {
                builder = builder.with_alias(alias);
            }

            if let Some(bech32_hrp) = bech32_hrp {
                builder = builder.with_bech32_hrp(bech32_hrp);
            }

            match builder.finish().await {
                Ok(account) => {
                    let account = account.read().await;
                    Response::Account(AccountDto::from(&*account))
                }
                Err(e) => return Err(e.into()),
            }
        }
        WalletMethod::GetAccount { account_id } => {
            let account = wallet.get_account(account_id.clone()).await?;
            let account = account.read().await;
            Response::Account(AccountDto::from(&*account))
        }
        WalletMethod::GetAccountIndexes => {
            let accounts = wallet.get_accounts().await?;
            let mut account_indexes = Vec::new();
            for account in accounts.iter() {
                account_indexes.push(*account.read().await.index());
            }
            Response::AccountIndexes(account_indexes)
        }
        WalletMethod::GetAccounts => {
            let accounts = wallet.get_accounts().await?;
            let mut accoun_dtos = Vec::new();
            for account in accounts {
                let account = account.read().await;
                accoun_dtos.push(AccountDto::from(&*account));
            }
            Response::Accounts(accoun_dtos)
        }
        WalletMethod::CallAccountMethod { account_id, method } => {
            let account = wallet.get_account(account_id).await?;
            call_account_method_internal(&account, method).await?
        }
        #[cfg(feature = "stronghold")]
        WalletMethod::Backup { destination, password } => {
            wallet.backup(destination, password).await?;
            Response::Ok
        }
        #[cfg(feature = "stronghold")]
        WalletMethod::ChangeStrongholdPassword {
            mut current_password,
            mut new_password,
        } => {
            wallet
                .change_stronghold_password(&current_password, &new_password)
                .await?;
            current_password.zeroize();
            new_password.zeroize();
            Response::Ok
        }
        #[cfg(feature = "stronghold")]
        WalletMethod::ClearStrongholdPassword => {
            wallet.clear_stronghold_password().await?;
            Response::Ok
        }
        #[cfg(feature = "stronghold")]
        WalletMethod::IsStrongholdPasswordAvailable => {
            let is_available = wallet.is_stronghold_password_available().await?;
            Response::Bool(is_available)
        }
        WalletMethod::RecoverAccounts {
            account_start_index,
            account_gap_limit,
            address_gap_limit,
            sync_options,
        } => {
            let accounts = wallet
                .recover_accounts(account_start_index, account_gap_limit, address_gap_limit, sync_options)
                .await?;
            let mut account_dtos = Vec::new();
            for account in accounts {
                let account = account.read().await;
                account_dtos.push(AccountDto::from(&*account));
            }
            Response::Accounts(account_dtos)
        }
        WalletMethod::RemoveLatestAccount => {
            wallet.remove_latest_account().await?;
            Response::Ok
        }
        #[cfg(feature = "stronghold")]
        WalletMethod::RestoreBackup {
            source,
            password,
            ignore_if_coin_type_mismatch,
        } => {
            wallet
                .restore_backup(source, password, ignore_if_coin_type_mismatch)
                .await?;
            Response::Ok
        }
        WalletMethod::SetClientOptions { client_options } => {
            wallet.set_client_options(*client_options).await?;
            Response::Ok
        }
        #[cfg(feature = "ledger_nano")]
        WalletMethod::GetLedgerNanoStatus => {
            let ledger_nano_status = wallet.get_ledger_nano_status().await?;
            Response::LedgerNanoStatus(ledger_nano_status)
        }
        WalletMethod::GenerateAddress {
            account_index,
            internal,
            address_index,
            options,
            bech32_hrp,
        } => {
            let address = wallet
                .generate_address(account_index, internal, address_index, options)
                .await?;

            let bech32_hrp = match bech32_hrp {
                Some(bech32_hrp) => bech32_hrp,
                None => wallet.get_bech32_hrp().await?,
            };

            Response::Bech32Address(address.to_bech32(bech32_hrp))
        }
        #[cfg(feature = "stronghold")]
        WalletMethod::SetStrongholdPassword { mut password } => {
            wallet.set_stronghold_password(&password).await?;
            password.zeroize();
            Response::Ok
        }
        #[cfg(feature = "stronghold")]
        WalletMethod::SetStrongholdPasswordClearInterval {
            interval_in_milliseconds,
        } => {
            let duration = interval_in_milliseconds.map(Duration::from_millis);
            wallet.set_stronghold_password_clear_interval(duration).await?;
            Response::Ok
        }
        #[cfg(feature = "stronghold")]
        WalletMethod::StoreMnemonic { mnemonic } => {
            wallet.store_mnemonic(mnemonic).await?;
            Response::Ok
        }
        WalletMethod::StartBackgroundSync {
            options,
            interval_in_milliseconds,
        } => {
            let duration = interval_in_milliseconds.map(Duration::from_millis);
            wallet.start_background_syncing(options, duration).await?;
            Response::Ok
        }
        WalletMethod::StopBackgroundSync => {
            wallet.stop_background_syncing().await?;
            Response::Ok
        }
        #[cfg(feature = "events")]
        WalletMethod::EmitTestEvent { event } => {
            wallet.emit_test_event(event.clone()).await?;
            Response::Ok
        }
        #[cfg(feature = "events")]
        WalletMethod::ClearListeners { event_types } => {
            wallet.clear_listeners(event_types).await;
            Response::Ok
        }
        WalletMethod::UpdateNodeAuth { url, auth } => {
            wallet.update_node_auth(url, auth).await?;
            Response::Ok
        }
    };
    Ok(response)
}
