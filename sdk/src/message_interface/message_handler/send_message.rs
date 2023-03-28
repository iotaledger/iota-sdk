// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::message_interface::{
    message::{ClientMessage, Message},
    panic::convert_async_panics,
    response::Response,
    MessageHandler,
};

impl MessageHandler {
    /// Send a message.
    pub async fn send_message(&self, message: Message) -> Response {
        match &message {
            Message::Client(message) => {
                match message {
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
            }
            _ => log::debug!("Message: {:?}", message),
        }

        let result = convert_async_panics(|| async {
            match message {
                Message::Client(message) => self.client.handle_message(message).await,
                Message::Wallet(message) => self.account_manager.handle_message(message).await,
            }
        })
        .await;

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
