// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use zeroize::Zeroize;

use crate::{
    client::{constants::SHIMMER_TESTNET_BECH32_HRP, utils, Client, NodeInfoWrapper},
    message_interface::{message::WalletMessage, message_handler::Result, panic::convert_panics, response::Response},
    wallet::{account_manager::AccountManager, message_interface::dtos::AccountDto},
};

impl AccountManager {
    /// Handle a message.
    pub(crate) async fn handle_message(&self, message: WalletMessage) -> Result<Response> {
        match message {
            WalletMessage::CreateAccount { alias, bech32_hrp } => {
                let mut builder = self.create_account();

                if let Some(alias) = alias {
                    builder = builder.with_alias(alias);
                }

                if let Some(bech32_hrp) = bech32_hrp {
                    builder = builder.with_bech32_hrp(bech32_hrp);
                }

                match builder.finish().await {
                    Ok(account_handle) => {
                        let account = account_handle.read().await;
                        Ok(Response::Account(AccountDto::from(&*account)))
                    }
                    Err(e) => Err(e.into()),
                }
            }
            WalletMessage::GetAccount { account_id } => {
                let account_handle = self.get_account(account_id.clone()).await?;
                let account = account_handle.read().await;
                Ok(Response::Account(AccountDto::from(&*account)))
            }
            WalletMessage::GetAccountIndexes => {
                let accounts = self.get_accounts().await?;
                let mut account_indexes = Vec::new();
                for account in accounts.iter() {
                    account_indexes.push(*account.read().await.index());
                }
                Ok(Response::AccountIndexes(account_indexes))
            }
            WalletMessage::GetAccounts => {
                let account_handles = self.get_accounts().await?;
                let mut accounts = Vec::new();
                for account_handle in account_handles {
                    let account = account_handle.read().await;
                    accounts.push(AccountDto::from(&*account));
                }
                Ok(Response::Accounts(accounts))
            }
            WalletMessage::CallAccountMethod { account_id, method } => {
                let account_handle = self.get_account(account_id).await?;
                account_handle.handle_message(method).await
            }
            #[cfg(feature = "stronghold")]
            WalletMessage::Backup { destination, password } => {
                self.backup(destination, password).await?;
                Ok(Response::Ok)
            }
            #[cfg(feature = "stronghold")]
            WalletMessage::ChangeStrongholdPassword {
                mut current_password,
                mut new_password,
            } => {
                self.change_stronghold_password(&current_password, &new_password)
                    .await?;
                current_password.zeroize();
                new_password.zeroize();
                Ok(Response::Ok)
            }
            #[cfg(feature = "stronghold")]
            WalletMessage::ClearStrongholdPassword => {
                self.clear_stronghold_password().await?;
                Ok(Response::Ok)
            }
            #[cfg(feature = "stronghold")]
            WalletMessage::IsStrongholdPasswordAvailable => {
                let is_available = self.is_stronghold_password_available().await?;
                Ok(Response::Bool(is_available))
            }
            WalletMessage::RecoverAccounts {
                account_start_index,
                account_gap_limit,
                address_gap_limit,
                sync_options,
            } => {
                let account_handles = self
                    .recover_accounts(account_start_index, account_gap_limit, address_gap_limit, sync_options)
                    .await?;
                let mut accounts = Vec::new();
                for account_handle in account_handles {
                    let account = account_handle.read().await;
                    accounts.push(AccountDto::from(&*account));
                }
                Ok(Response::Accounts(accounts))
            }
            WalletMessage::RemoveLatestAccount => {
                self.remove_latest_account().await?;
                Ok(Response::Ok)
            }
            #[cfg(feature = "stronghold")]
            WalletMessage::RestoreBackup {
                source,
                password,
                ignore_if_coin_type_mismatch,
            } => {
                self.restore_backup(source, password, ignore_if_coin_type_mismatch)
                    .await?;
                Ok(Response::Ok)
            }
            WalletMessage::GenerateMnemonic => convert_panics(|| {
                self.generate_mnemonic()
                    .map(Response::GeneratedMnemonic)
                    .map_err(Into::into)
            }),
            WalletMessage::VerifyMnemonic { mut mnemonic } => convert_panics(|| {
                self.verify_mnemonic(&mnemonic)?;
                mnemonic.zeroize();
                Ok(Response::Ok)
            }),
            WalletMessage::SetClientOptions { client_options } => {
                self.set_client_options(*client_options).await?;
                Ok(Response::Ok)
            }
            #[cfg(feature = "ledger_nano")]
            WalletMessage::GetLedgerNanoStatus => {
                let ledger_nano_status = self.get_ledger_nano_status().await?;
                Ok(Response::LedgerNanoStatus(ledger_nano_status))
            }
            WalletMessage::GenerateAddress {
                account_index,
                internal,
                address_index,
                options,
                bech32_hrp,
            } => {
                let address = self
                    .generate_address(account_index, internal, address_index, options)
                    .await?;

                let bech32_hrp = match bech32_hrp {
                    Some(bech32_hrp) => bech32_hrp,
                    None => self.get_bech32_hrp().await?,
                };

                Ok(Response::Bech32Address(address.to_bech32(bech32_hrp)))
            }
            WalletMessage::GetNodeInfo { url, auth } => match url {
                Some(url) => {
                    let node_info = Client::get_node_info(&url, auth).await?;
                    Ok(Response::NodeInfoWrapper(NodeInfoWrapper { node_info, url }))
                }
                None => self
                    .get_node_info()
                    .await
                    .map(Response::NodeInfoWrapper)
                    .map_err(Into::into),
            },
            #[cfg(feature = "stronghold")]
            WalletMessage::SetStrongholdPassword { mut password } => {
                self.set_stronghold_password(&password).await?;
                password.zeroize();
                Ok(Response::Ok)
            }
            #[cfg(feature = "stronghold")]
            WalletMessage::SetStrongholdPasswordClearInterval {
                interval_in_milliseconds,
            } => {
                let duration = interval_in_milliseconds.map(Duration::from_millis);
                self.set_stronghold_password_clear_interval(duration).await?;
                Ok(Response::Ok)
            }
            #[cfg(feature = "stronghold")]
            WalletMessage::StoreMnemonic { mnemonic } => {
                self.store_mnemonic(mnemonic).await?;
                Ok(Response::Ok)
            }
            WalletMessage::StartBackgroundSync {
                options,
                interval_in_milliseconds,
            } => {
                let duration = interval_in_milliseconds.map(Duration::from_millis);
                self.start_background_syncing(options, duration).await?;
                Ok(Response::Ok)
            }
            WalletMessage::StopBackgroundSync => {
                self.stop_background_syncing().await?;
                Ok(Response::Ok)
            }
            #[cfg(feature = "events")]
            WalletMessage::EmitTestEvent { event } => {
                self.emit_test_event(event.clone()).await?;
                Ok(Response::Ok)
            }
            WalletMessage::Bech32ToHex { bech32_address } => {
                convert_panics(|| Ok(Response::HexAddress(utils::bech32_to_hex(&bech32_address)?)))
            }
            WalletMessage::HexToBech32 { hex, bech32_hrp } => {
                let bech32_hrp = match bech32_hrp {
                    Some(bech32_hrp) => bech32_hrp,
                    None => match self.get_node_info().await {
                        Ok(node_info_wrapper) => node_info_wrapper.node_info.protocol.bech32_hrp,
                        Err(_) => SHIMMER_TESTNET_BECH32_HRP.into(),
                    },
                };

                Ok(Response::Bech32Address(utils::hex_to_bech32(&hex, &bech32_hrp)?))
            }
            #[cfg(feature = "events")]
            WalletMessage::ClearListeners { event_types } => {
                self.clear_listeners(event_types).await;
                Ok(Response::Ok)
            }
            WalletMessage::UpdateNodeAuth { url, auth } => {
                self.update_node_auth(url, auth).await?;
                Ok(Response::Ok)
            }
        }
    }
}
