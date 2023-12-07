// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::signatures::ed25519::PublicKey;

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage},
    types::block::{
        address::Address,
        context_input::{BlockIssuanceCreditContextInput, CommitmentContextInput},
        output::{
            feature::{BlockIssuerFeature, BlockIssuerKey, BlockIssuerKeys, Ed25519BlockIssuerKey},
            unlock_condition::AddressUnlockCondition,
            AccountId, AccountOutput, OutputId,
        },
        payload::signed_transaction::{TransactionCapabilities, TransactionCapabilityFlag},
    },
    wallet::{
        core::SecretData,
        operations::transaction::{TransactionOptions, TransactionWithMetadata},
        Error, Result, Wallet,
    },
};

impl<S: SecretManage> Wallet<SecretData<S>> {
    /// Transitions an implicit account to an account.
    pub async fn implicit_account_transition(
        &self,
        output_id: &OutputId,
        key_source: impl Into<Option<BlockIssuerKeySource<S::GenerationOptions>>> + Send,
    ) -> Result<TransactionWithMetadata> {
        let issuer_id = AccountId::from(output_id);

        self.sign_and_submit_transaction(
            self.prepare_implicit_account_transition(output_id, key_source).await?,
            issuer_id,
            None,
        )
        .await
    }

    /// Prepares to transition an implicit account to an account.
    pub async fn prepare_implicit_account_transition(
        &self,
        output_id: &OutputId,
        key_source: impl Into<Option<BlockIssuerKeySource<S::GenerationOptions>>> + Send,
    ) -> Result<PreparedTransactionData> {
        let implicit_account_data = self.data().await.unspent_outputs.get(output_id).cloned();

        let implicit_account = if let Some(implicit_account_data) = &implicit_account_data {
            if implicit_account_data.output.is_implicit_account() {
                implicit_account_data.output.as_basic()
            } else {
                return Err(Error::ImplicitAccountNotFound);
            }
        } else {
            return Err(Error::ImplicitAccountNotFound);
        };

        let key_source = match key_source.into() {
            Some(key_source) => key_source,
            None => BlockIssuerKeySource::Options(self.public_key_options().clone()),
        };

        let public_key = match key_source {
            BlockIssuerKeySource::Key(public_key) => public_key,
            BlockIssuerKeySource::Options(options) => self.secret_manager().read().await.generate(&options).await?,
        };

        let account_id = AccountId::from(output_id);
        let account = AccountOutput::build_with_amount(implicit_account.amount(), account_id)
            .with_mana(implicit_account.mana())
            .with_unlock_conditions([AddressUnlockCondition::from(Address::from(
                *implicit_account
                    .address()
                    .as_implicit_account_creation()
                    .ed25519_address(),
            ))])
            .with_features([BlockIssuerFeature::new(
                u32::MAX,
                BlockIssuerKeys::from_vec(vec![BlockIssuerKey::from(Ed25519BlockIssuerKey::from(public_key))])?,
            )?])
            .finish_output()?;

        // TODO https://github.com/iotaledger/iota-sdk/issues/1740
        let issuance = self.client().get_issuance().await?;

        // TODO remove when https://github.com/iotaledger/iota-sdk/issues/1744 is done
        let mut capabilities = TransactionCapabilities::default();
        capabilities.add_capability(TransactionCapabilityFlag::BurnMana);

        let transaction_options = TransactionOptions {
            context_inputs: Some(vec![
                CommitmentContextInput::new(issuance.latest_commitment.id()).into(),
                BlockIssuanceCreditContextInput::new(account_id).into(),
            ]),
            custom_inputs: Some(vec![*output_id]),
            capabilities: Some(capabilities),
            ..Default::default()
        };

        self.prepare_transaction(vec![account], transaction_options.clone())
            .await
    }
}

pub enum BlockIssuerKeySource<O> {
    Key(PublicKey),
    Options(O),
}
