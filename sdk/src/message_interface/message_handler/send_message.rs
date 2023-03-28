// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::Client,
    message_interface::{
        message::{ClientMessage, WalletMessage},
        panic::convert_async_panics,
        response::Response,
    },
    wallet::account_manager::AccountManager,
};

impl Client {
    /// Send a message.
    pub async fn send_message(&self, message: ClientMessage) -> Response {
        match &message {
            // Don't log secrets
            ClientMessage::GenerateAddresses {
                secret_manager: _,
                options,
            } => {
                log::debug!("Response: GenerateAddresses{{ secret_manager: <omitted>, options: {options:?} }}")
            }
            ClientMessage::BuildAndPostBlock {
                secret_manager: _,
                options,
            } => {
                log::debug!("Response: BuildAndPostBlock{{ secret_manager: <omitted>, options: {options:?} }}")
            }
            ClientMessage::PrepareTransaction {
                secret_manager: _,
                options,
            } => {
                log::debug!("Response: PrepareTransaction{{ secret_manager: <omitted>, options: {options:?} }}")
            }
            ClientMessage::SignTransaction {
                secret_manager: _,
                prepared_transaction_data,
            } => {
                log::debug!(
                    "Response: SignTransaction{{ secret_manager: <omitted>, prepared_transaction_data: {prepared_transaction_data:?} }}"
                )
            }
            #[cfg(feature = "stronghold")]
            ClientMessage::StoreMnemonic { .. } => {
                log::debug!("Response: StoreMnemonic{{ <omitted> }}")
            }
            ClientMessage::ConsolidateFunds {
                secret_manager: _,
                generate_addresses_options,
            } => {
                log::debug!(
                    "Response: ConsolidateFunds{{ secret_manager: <omitted>, generate_addresses_options: {generate_addresses_options:?} }}"
                )
            }
            ClientMessage::MnemonicToHexSeed { .. } => {
                log::debug!("Response: MnemonicToHexSeed{{ <omitted> }}")
            }
            _ => log::debug!("Message: {:?}", message),
        }

        let result = convert_async_panics(|| async { self.handle_message(message).await }).await;

        let response = match result {
            Ok(r) => r,
            Err(e) => Response::Error(e),
        };

        match response {
            // Don't log secrets
            Response::GeneratedMnemonic { .. } => {
                log::debug!("Response: GeneratedMnemonic(<omitted>)")
            }
            Response::MnemonicHexSeed { .. } => {
                log::debug!("Response: MnemonicHexSeed(<omitted>)")
            }
            _ => log::debug!("Response: {:?}", response),
        }

        response
    }
}

impl AccountManager {
    /// Send a message.
    pub async fn send_message(&self, message: WalletMessage) -> Response {
        log::debug!("Message: {:?}", message);

        let result = convert_async_panics(|| async { self.handle_message(message).await }).await;

        let response = match result {
            Ok(r) => r,
            Err(e) => Response::Error(e),
        };

        match response {
            // Don't log secrets
            Response::GeneratedMnemonic { .. } => {
                log::debug!("Response: GeneratedMnemonic(<omitted>)")
            }
            Response::MnemonicHexSeed { .. } => {
                log::debug!("Response: MnemonicHexSeed(<omitted>)")
            }
            _ => log::debug!("Response: {:?}", response),
        }

        response
    }
}
