// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::secret::SecretManage,
    types::block::output::{
        feature::{BlockIssuerFeature, BlockIssuerKey, BlockIssuerKeys, Ed25519BlockIssuerKey},
        unlock_condition::{AddressUnlockCondition, UnlockCondition},
        AccountId, AccountOutput, BasicOutput, FoundryId, MinimumOutputAmount, NativeTokensBuilder, Output, OutputId,
    },
    wallet::{
        operations::transaction::{TransactionOptions, TransactionWithMetadata},
        Result, Wallet,
    },
};

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    async fn implicit_account_transition(&self, output_id: &OutputId) -> Result<TransactionWithMetadata> {
        let implicit_account_data = self.data().await.unspent_outputs.get(output_id).cloned();

        let implicit_account = if let Some(implicit_account_data) = &implicit_account_data {
            if implicit_account_data.output.is_implicit_account() {
                implicit_account_data.output.as_basic()
            } else {
                todo!()
            }
        } else {
            todo!()
        };

        // [BlockIssuerFeature::new(
        //     0,
        //     BlockIssuerKeys::from_vec([BlockIssuerKey::from(Ed25519BlockIssuerKey::from(
        //         implicit_account.address().as_implicit_account_creation(),
        //     ))]

        let account = AccountOutput::build_with_amount(implicit_account.amount(), AccountId::from(output_id))
            .with_mana(implicit_account.mana())
            .with_unlock_conditions([AddressUnlockCondition::from(implicit_account.address().clone())])
            .finish_output()?;
        // .with_features()?,

        let transaction_options = TransactionOptions {
            custom_inputs: Some(vec![*output_id]),
            ..Default::default()
        };

        let prepared_transaction = self
            .prepare_transaction(vec![account], transaction_options.clone())
            .await?;

        self.sign_and_submit_transaction(prepared_transaction, transaction_options)
            .await
    }
}
