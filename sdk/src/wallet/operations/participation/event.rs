// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use crate::{
    client::{node_manager::node::Node, secret::SecretManage, Client},
    types::api::plugins::participation::types::{
        ParticipationEventId, ParticipationEventStatus, ParticipationEventType,
    },
    wallet::{
        operations::participation::ParticipationEventWithNodes,
        types::participation::ParticipationEventRegistrationOptions, Wallet,
    },
};

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    /// Stores participation information for the given events locally and returns them all.
    ///
    /// This will NOT store the node url and auth inside the client options.
    pub async fn register_participation_events(
        &self,
        options: &ParticipationEventRegistrationOptions,
    ) -> Result<HashMap<ParticipationEventId, ParticipationEventWithNodes>, WalletError> {
        let client = Client::builder()
            .with_ignore_node_health()
            .with_node_auth(options.node.url.as_str(), options.node.auth.clone())?
            .finish()
            .await?;

        let events_to_register = match &options.events_to_register {
            Some(events_to_register_) => {
                if events_to_register_.is_empty() {
                    self.get_participation_event_ids(&options.node, Some(ParticipationEventType::Voting))
                        .await?
                } else {
                    events_to_register_.clone()
                }
            }
            None => {
                self.get_participation_event_ids(&options.node, Some(ParticipationEventType::Voting))
                    .await?
            }
        };

        let mut registered_participation_events = HashMap::new();
        for event_id in events_to_register {
            if let Some(events_to_ignore) = &options.events_to_ignore {
                if events_to_ignore.contains(&event_id) {
                    continue;
                }
            }

            let event_data = client.event(&event_id).await?;
            let event_with_node = ParticipationEventWithNodes {
                id: event_id,
                data: event_data,
                nodes: vec![options.node.clone()],
            };
            self.storage_manager()
                .insert_participation_event(event_with_node.clone())
                .await?;
            registered_participation_events.insert(event_id, event_with_node.clone());
        }

        Ok(registered_participation_events)
    }

    /// Removes a previously registered participation event from local storage.
    pub async fn deregister_participation_event(&self, id: &ParticipationEventId) -> Result<(), WalletError> {
        self.storage_manager().remove_participation_event(id).await?;
        Ok(())
    }

    /// Retrieves corresponding information for a participation event from local storage.
    pub async fn get_participation_event(
        &self,
        id: ParticipationEventId,
    ) -> Result<Option<ParticipationEventWithNodes>, WalletError> {
        Ok(self
            .storage_manager()
            .get_participation_events()
            .await?
            .get(&id)
            .cloned())
    }

    /// Retrieves information for all registered participation events.
    pub async fn get_participation_events(
        &self,
    ) -> Result<HashMap<ParticipationEventId, ParticipationEventWithNodes>, WalletError> {
        self.storage_manager().get_participation_events().await
    }

    /// Retrieves IDs of all events tracked by the client options node.
    pub async fn get_participation_event_ids(
        &self,
        node: &Node,
        event_type: Option<ParticipationEventType>,
    ) -> Result<Vec<ParticipationEventId>, WalletError> {
        let client = Client::builder()
            .with_ignore_node_health()
            .with_node_auth(node.url.as_str(), node.auth.clone())?
            .finish()
            .await?;
        Ok(client.events(event_type).await?.event_ids)
    }

    /// Retrieves the latest status of a given participation event.
    pub async fn get_participation_event_status(
        &self,
        id: &ParticipationEventId,
    ) -> Result<ParticipationEventStatus, WalletError> {
        Ok(self.get_client_for_event(id).await?.event_status(id, None).await?)
    }
}
