// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage},
    types::{
        api::plugins::participation::types::{Participation, ParticipationEventId, Participations, PARTICIPATION_TAG},
        block::{
            output::{
                feature::{MetadataFeature, TagFeature},
                BasicOutputBuilder, Feature,
            },
            payload::TaggedDataPayload,
        },
    },
    wallet::{
        account::{types::Transaction, Account, TransactionOptions},
        Result,
    },
};

impl<S: 'static + SecretManage> Account<S>
where
    crate::wallet::Error: From<S::Error>,
{
    /// Casts a given number of votes for a given (voting) event.
    ///
    /// If voting for other events, continues voting for them.
    /// Removes metadata for any event that has expired (uses event IDs to get cached event information, checks event
    /// milestones in there against latest network milestone).
    /// If already voting for this event, overwrites existing output metadata.
    /// If existing voting output(s) do NOT have enough funds (or don't exist), throws an error.
    /// If exceeds output metadata limit, throws an error (although better if automatically handled, but has UX
    /// implications).
    /// If event has expired, throws an error (do NOT remove previous votes).
    ///
    /// This is an add OR update function, not just add.
    /// This should use regular client options, NOT specific node for the event.
    pub async fn vote(
        &self,
        event_id: impl Into<Option<ParticipationEventId>> + Send,
        answers: impl Into<Option<Vec<u8>>> + Send,
    ) -> Result<Transaction> {
        let prepared = self.prepare_vote(event_id, answers).await?;

        self.sign_and_submit_transaction(prepared, None).await
    }

    /// Prepares the transaction for
    /// [Account::vote()](crate::wallet::Account::vote).
    pub async fn prepare_vote(
        &self,
        event_id: impl Into<Option<ParticipationEventId>> + Send,
        answers: impl Into<Option<Vec<u8>>> + Send,
    ) -> Result<PreparedTransactionData> {
        let event_id = event_id.into();
        let answers = answers.into();
        if let Some(event_id) = event_id {
            let event_status = self.get_participation_event_status(&event_id).await?;

            // Checks if voting event is still running.
            if event_status.status() == "ended" {
                return Err(crate::wallet::Error::Voting(format!("event {event_id} already ended")));
            }
        }

        let voting_output = self
            .get_voting_output()
            .await?
            .ok_or_else(|| crate::wallet::Error::Voting("No unspent voting output found".to_string()))?;
        let output = voting_output.output.as_basic();

        // Updates or creates participation.
        let participation_bytes = match output.features().metadata() {
            Some(metadata) => {
                let mut participations = Participations::from_bytes(&mut metadata.data())?;

                // Removes ended participations.
                self.remove_ended_participation_events(&mut participations).await?;

                if let Some(event_id) = event_id {
                    participations.add_or_replace(Participation {
                        event_id,
                        answers: answers.unwrap_or_default(),
                    });
                }

                participations
            }
            None => {
                if let Some(event_id) = event_id {
                    Participations {
                        participations: vec![Participation {
                            event_id,
                            answers: answers.unwrap_or_default(),
                        }],
                    }
                } else {
                    return Err(crate::wallet::Error::Voting("No event to vote for".to_string()));
                }
            }
        }
        .to_bytes()?;

        let new_output = BasicOutputBuilder::from(output)
            .with_features([
                Feature::Tag(TagFeature::new(PARTICIPATION_TAG)?),
                Feature::Metadata(MetadataFeature::new(participation_bytes.clone())?),
            ])
            .finish_output(self.client().get_token_supply().await?)?;

        self.prepare_transaction(
            [new_output],
            Some(TransactionOptions {
                // Only use previous voting output as input.
                custom_inputs: Some(vec![voting_output.output_id]),
                mandatory_inputs: Some(vec![voting_output.output_id]),
                tagged_data_payload: Some(TaggedDataPayload::new(
                    PARTICIPATION_TAG.as_bytes().to_vec(),
                    participation_bytes,
                )?),
                ..Default::default()
            }),
        )
        .await
    }

    /// Removes metadata corresponding to a given (voting) event ID from the voting output if it contains it.
    ///
    /// If voting for other events, continues voting for them.
    /// Removes metadata for any event that has expired (use event IDs to get cached event information, checks event
    /// milestones in there against latest network milestone).
    /// If NOT already voting for this event, throws an error.
    pub async fn stop_participating(&self, event_id: ParticipationEventId) -> Result<Transaction> {
        let prepared = self.prepare_stop_participating(event_id).await?;

        self.sign_and_submit_transaction(prepared, None).await
    }

    /// Prepares the transaction for
    /// [Account::stop_participating()](crate::wallet::Account::stop_participating).
    pub async fn prepare_stop_participating(&self, event_id: ParticipationEventId) -> Result<PreparedTransactionData> {
        let voting_output = self
            .get_voting_output()
            .await?
            .ok_or_else(|| crate::wallet::Error::Voting("No unspent voting output found".to_string()))?;
        let output = voting_output.output.as_basic();

        // Removes participation.
        let participation_bytes = match output.features().metadata() {
            Some(metadata) => {
                let mut participations = Participations::from_bytes(&mut metadata.data())?;

                let length_before = participations.participations.len();

                participations.remove(&event_id);

                if length_before == participations.participations.len() {
                    return Err(crate::wallet::Error::Voting(format!(
                        "currently not participating for {event_id}"
                    )));
                }

                // Removes ended participations.
                self.remove_ended_participation_events(&mut participations).await?;

                participations
            }
            None => {
                return Err(crate::wallet::Error::Voting(format!(
                    "currently not participating for {event_id}"
                )));
            }
        }
        .to_bytes()?;

        let new_output = BasicOutputBuilder::from(output)
            .with_features([
                Feature::Tag(TagFeature::new(PARTICIPATION_TAG)?),
                Feature::Metadata(MetadataFeature::new(participation_bytes.clone())?),
            ])
            .finish_output(self.client().get_token_supply().await?)?;

        self.prepare_transaction(
            [new_output],
            Some(TransactionOptions {
                // Only use previous voting output as input.
                custom_inputs: Some(vec![voting_output.output_id]),
                mandatory_inputs: Some(vec![voting_output.output_id]),
                tagged_data_payload: Some(TaggedDataPayload::new(
                    PARTICIPATION_TAG.as_bytes().to_vec(),
                    participation_bytes,
                )?),
                ..Default::default()
            }),
        )
        .await
    }
}
