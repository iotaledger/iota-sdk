// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use iota_sdk::{types::block::address::ToBech32Ext, wallet::Wallet};

use super::wallet_command::call_wallet_command_method_internal;
use crate::{method::WalletMethod, response::Response, Result};

/// Call a wallet method.
pub(crate) async fn call_wallet_method_internal(wallet: &Wallet, method: WalletMethod) -> Result<Response> {
    let response = match method {
        WalletMethod::CallMethod { method } => call_wallet_command_method_internal(&wallet, method).await?,
        #[cfg(feature = "stronghold")]
        WalletMethod::Backup { destination, password } => {
            wallet.backup(destination, password).await?;
            Response::Ok
        }
        #[cfg(feature = "stronghold")]
        WalletMethod::ChangeStrongholdPassword {
            current_password,
            new_password,
        } => {
            wallet
                .change_stronghold_password(current_password, new_password)
                .await?;
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
        #[cfg(feature = "stronghold")]
        WalletMethod::RestoreBackup {
            source,
            password,
            ignore_if_coin_type_mismatch,
            ignore_if_bech32_mismatch,
        } => {
            wallet
                .restore_backup(
                    source,
                    password,
                    ignore_if_coin_type_mismatch,
                    ignore_if_bech32_mismatch,
                )
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
        WalletMethod::GenerateEd25519Address {
            account_index,
            address_index,
            options,
            bech32_hrp,
        } => {
            let address = wallet
                .generate_ed25519_address(account_index, address_index, options)
                .await?;

            let bech32_hrp = match bech32_hrp {
                Some(bech32_hrp) => bech32_hrp,
                None => *wallet.address().await.hrp(),
            };

            Response::Bech32Address(address.to_bech32(bech32_hrp))
        }
        #[cfg(feature = "stronghold")]
        WalletMethod::SetStrongholdPassword { password } => {
            wallet.set_stronghold_password(password).await?;
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
            wallet.store_mnemonic(mnemonic.into()).await?;
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
            wallet.emit_test_event(event.clone()).await;
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
