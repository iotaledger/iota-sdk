// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod call_account_method;
mod send_message;

#[cfg(feature = "stronghold")]
use std::path::PathBuf;

#[cfg(feature = "mqtt")]
use {
    crate::client::mqtt::{MqttPayload, Topic},
    crate::types::block::{
        payload::{dto::MilestonePayloadDto, milestone::option::dto::ReceiptMilestoneOptionDto},
        BlockDto,
    },
};

#[cfg(feature = "events")]
use crate::wallet::events::types::{Event, WalletEventType};
use crate::{
    client::Client,
    message_interface::response::Response,
    wallet::{account::types::AccountIdentifier, account_manager::AccountManager, message_interface::dtos::AccountDto},
};

/// Result type of the message interface.
pub type Result<T> = std::result::Result<T, super::error::MessageInterfaceError>;

/// The message handler.
pub struct MessageHandler {
    account_manager: AccountManager,
    client: Client,
}

impl MessageHandler {
    // TODO: not async and don't return Result or change params
    // TODO: make it possible to provide only Client or only AccountManager (without the other doing something unwanted
    // in the background like walletdb creation or node syncing)
    /// Creates a new instance of the message handler.
    pub async fn new(account_manager: AccountManager, client: Client) -> Result<Self> {
        let instance = Self {
            account_manager,
            client,
        };
        Ok(instance)
    }

    /// Listen to MQTT events
    #[cfg(feature = "mqtt")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mqtt")))]
    pub async fn listen_mqtt<F>(&self, topics: Vec<Topic>, handler: F)
    where
        F: Fn(String) + 'static + Clone + Send + Sync,
    {
        self.client
            .subscribe(topics, move |topic_event| {
                #[derive(Serialize)]
                struct MqttResponse {
                    topic: String,
                    payload: String,
                }
                // convert types to DTOs
                let payload = match &topic_event.payload {
                    MqttPayload::Json(val) => {
                        serde_json::to_string(&val).expect("failed to serialize MqttPayload::Json")
                    }
                    MqttPayload::Block(block) => {
                        serde_json::to_string(&BlockDto::from(block)).expect("failed to serialize MqttPayload::Block")
                    }
                    MqttPayload::MilestonePayload(ms) => serde_json::to_string(&MilestonePayloadDto::from(ms))
                        .expect("failed to serialize MqttPayload::MilestonePayload"),
                    MqttPayload::Receipt(receipt) => serde_json::to_string(&ReceiptMilestoneOptionDto::from(receipt))
                        .expect("failed to serialize MqttPayload::Receipt"),
                };
                let response = MqttResponse {
                    topic: topic_event.topic.clone(),
                    payload,
                };

                handler(serde_json::to_string(&response).expect("failed to serialize MQTT response"))
            })
            .await
            .expect("failed to listen to MQTT events");
    }

    /// Listen to wallet events, empty vec will listen to all events
    #[cfg(feature = "events")]
    #[cfg_attr(docsrs, doc(cfg(feature = "events")))]
    pub async fn listen_wallet_events<F>(&self, events: Vec<WalletEventType>, handler: F)
    where
        F: Fn(&Event) + 'static + Clone + Send + Sync,
    {
        self.account_manager.listen(events, handler).await;
    }

    #[cfg(feature = "stronghold")]
    async fn backup(&self, backup_path: PathBuf, stronghold_password: String) -> Result<Response> {
        self.account_manager.backup(backup_path, stronghold_password).await?;
        Ok(Response::Ok)
    }

    #[cfg(feature = "stronghold")]
    async fn restore_backup(
        &self,
        backup_path: PathBuf,
        stronghold_password: String,
        ignore_if_coin_type_mismatch: Option<bool>,
    ) -> Result<Response> {
        self.account_manager
            .restore_backup(backup_path, stronghold_password, ignore_if_coin_type_mismatch)
            .await?;
        Ok(Response::Ok)
    }

    /// The create account message handler.
    async fn create_account(&self, alias: Option<String>, bech32_hrp: Option<String>) -> Result<Response> {
        let mut builder = self.account_manager.create_account();

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

    async fn get_account(&self, account_id: &AccountIdentifier) -> Result<Response> {
        let account_handle = self.account_manager.get_account(account_id.clone()).await?;
        let account = account_handle.read().await;
        Ok(Response::Account(AccountDto::from(&*account)))
    }

    async fn get_accounts(&self) -> Result<Response> {
        let account_handles = self.account_manager.get_accounts().await?;
        let mut accounts = Vec::new();
        for account_handle in account_handles {
            let account = account_handle.read().await;
            accounts.push(AccountDto::from(&*account));
        }
        Ok(Response::Accounts(accounts))
    }
}

#[cfg(test)]
mod tests {
    use super::super::{panic::convert_async_panics, Response};

    #[tokio::test]
    async fn panic_to_response() {
        match convert_async_panics(|| async { panic!("rekt") }).await.unwrap() {
            Response::Panic(msg) => {
                assert!(msg.contains("rekt"));
            }
            response_type => panic!("Unexpected response type: {response_type:?}"),
        };
    }
}
