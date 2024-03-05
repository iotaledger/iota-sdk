// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage},
    types::{
        api::plugins::participation::types::{Participations, PARTICIPATION_TAG},
        block::{
            output::{
                feature::{MetadataFeature, TagFeature},
                unlock_condition::AddressUnlockCondition,
                BasicOutput, BasicOutputBuilder, Output,
            },
            payload::TaggedDataPayload,
        },
    },
    wallet::{operations::transaction::TransactionOptions, types::TransactionWithMetadata, Error, Result, Wallet},
};

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    /// Returns an account's total voting power (voting or NOT voting).
    pub async fn get_voting_power(&self) -> Result<u64, WalletError> {
        Ok(self
            .get_voting_output()
            .await?
            // If no voting output exists, return 0
            .map_or(0, |v| v.output.amount()))
    }

    /// Designates a given amount of tokens towards an account's "voting power" by creating a
    /// special output, which is really a basic one with some metadata.
    ///
    /// If not enough funds, throws an error.
    /// If voting, use voting output (should only ever have one unless more space for more votes is needed).
    /// This will stop voting in most cases (if there is a remainder), but the voting data isn't lost and calling `Vote`
    /// without parameters will revote. Removes metadata for any events that have expired (uses event IDs to get
    /// cached event information, checks event milestones in there against latest network milestone).
    /// Prioritizes consuming outputs that are designated for voting but don't have any metadata (only possible if user
    /// increases voting power then increases again immediately after).
    pub async fn increase_voting_power(&self, amount: u64) -> Result<TransactionWithMetadata, WalletError> {
        let prepared = self.prepare_increase_voting_power(amount).await?;

        self.sign_and_submit_transaction(prepared, None, None).await
    }

    /// Prepares the transaction for [Wallet::increase_voting_power()].
    pub async fn prepare_increase_voting_power(&self, amount: u64) -> Result<PreparedTransactionData, WalletError> {
        let (new_output, tx_options) = match self.get_voting_output().await? {
            Some(current_output_data) => {
                let output = current_output_data.output.as_basic();

                let new_amount = output.amount().checked_add(amount).ok_or(Error::InvalidVotingPower)?;

                let (new_output, tagged_data_payload) =
                    self.new_voting_output_and_tagged_data(output, new_amount).await?;

                (
                    new_output,
                    Some(TransactionOptions {
                        // Use the previous voting output and additionally other for the additional amount.
                        required_inputs: Some(vec![current_output_data.output_id]),
                        tagged_data_payload: Some(tagged_data_payload),
                        ..Default::default()
                    }),
                )
            }
            None => (
                BasicOutputBuilder::new_with_amount(amount)
                    .add_unlock_condition(AddressUnlockCondition::new(self.address().await))
                    .add_feature(TagFeature::new(PARTICIPATION_TAG)?)
                    .finish_output()?,
                None,
            ),
        };

        self.prepare_send_outputs([new_output], tx_options).await
    }

    /// Reduces an account's "voting power" by a given amount.
    /// This will stop voting, but the voting data isn't lost and calling `Vote` without parameters will revote.
    ///
    /// If amount is higher than actual voting power, throws an error.
    /// If voting and amount is equal to voting power, removes tagged data payload and output metadata.
    /// Removes metadata for any events that have expired (uses event IDs to get cached event information, checks event
    /// milestones in there against latest network milestone).
    /// Prioritizes consuming outputs that are designated for voting but don't have any metadata (only possible if user
    /// increases voting power then decreases immediately after).
    pub async fn decrease_voting_power(&self, amount: u64) -> Result<TransactionWithMetadata, WalletError> {
        let prepared = self.prepare_decrease_voting_power(amount).await?;

        self.sign_and_submit_transaction(prepared, None, None).await
    }

    /// Prepares the transaction for [Wallet::decrease_voting_power()].
    pub async fn prepare_decrease_voting_power(&self, amount: u64) -> Result<PreparedTransactionData, WalletError> {
        let current_output_data = self
            .get_voting_output()
            .await?
            .ok_or_else(|| crate::wallet::Error::Voting("No unspent voting output found".to_string()))?;
        let output = current_output_data.output.as_basic();

        // If the amount to decrease is the amount of the output, then we just remove the features.
        let (new_output, tagged_data_payload) = if amount == output.amount() {
            (BasicOutputBuilder::from(output).clear_features().finish_output()?, None)
        } else {
            let new_amount = output.amount().checked_sub(amount).ok_or(Error::InvalidVotingPower)?;

            let (new_output, tagged_data_payload) = self.new_voting_output_and_tagged_data(output, new_amount).await?;

            (new_output, Some(tagged_data_payload))
        };

        self.prepare_send_outputs(
            [new_output],
            Some(TransactionOptions {
                // Use the previous voting output and additionally others for possible additional required amount for
                // the remainder to reach the minimum required storage deposit.
                required_inputs: Some(vec![current_output_data.output_id]),
                tagged_data_payload,
                ..Default::default()
            }),
        )
        .await
    }

    async fn new_voting_output_and_tagged_data(
        &self,
        output: &BasicOutput,
        amount: u64,
    ) -> Result<(Output, TaggedDataPayload)> {
        let mut output_builder = BasicOutputBuilder::from(output).with_amount(amount);
        let mut participation_bytes = output.features().metadata().map(|m| m.data()).unwrap_or(&[]);

        let participation_bytes = if let Ok(mut participations) = Participations::from_bytes(&mut participation_bytes) {
            // Remove ended participations.
            self.remove_ended_participation_events(&mut participations).await?;

            let participation_bytes = participations.to_bytes()?;

            output_builder = output_builder.replace_feature(MetadataFeature::new(participation_bytes.clone())?);

            participation_bytes
        } else {
            Vec::new()
        };

        Ok((
            output_builder.finish_output()?,
            TaggedDataPayload::new(PARTICIPATION_TAG.as_bytes().to_vec(), participation_bytes.to_vec())?,
        ))
    }
}
