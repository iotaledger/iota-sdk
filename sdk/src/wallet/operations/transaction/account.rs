// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage},
    types::block::{
        address::Address,
        output::{
            feature::{BlockIssuerFeature, BlockIssuerKey, BlockIssuerKeys, Ed25519BlockIssuerKey},
            unlock_condition::AddressUnlockCondition,
            AccountId, AccountOutput, OutputId,
        },
    },
    wallet::{
        operations::transaction::{TransactionOptions, TransactionWithMetadata},
        Error, Result, Wallet,
    },
};

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    /// Transitions an implicit account to an account.
    pub async fn implicit_account_transition(&self, output_id: &OutputId) -> Result<TransactionWithMetadata> {
        self.sign_and_submit_transaction(self.prepare_implicit_account_transition(output_id).await?, None)
            .await
    }

    /// Prepares to transition an implicit account to an account.
    pub async fn prepare_implicit_account_transition(&self, output_id: &OutputId) -> Result<PreparedTransactionData> {
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

        let public_key = if let Some(bip_path) = self.bip_path().await {
            self.secret_manager
                .read()
                .await
                .generate_ed25519_public_keys(
                    bip_path.coin_type,
                    bip_path.account,
                    bip_path.address_index..bip_path.address_index + 1,
                    None,
                )
                .await?[0]
        } else {
            // TODO https://github.com/iotaledger/iota-sdk/issues/1666
            todo!()
        };

        let account = AccountOutput::build_with_amount(implicit_account.amount(), AccountId::from(output_id))
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

        let transaction_options = TransactionOptions {
            custom_inputs: Some(vec![*output_id]),
            ..Default::default()
        };

        self.prepare_transaction(vec![account], transaction_options.clone())
            .await
    }
}
