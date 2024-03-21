// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{
        api::{transaction_builder::transition::Transitions, PreparedTransactionData},
        secret::SecretManage,
        ClientError,
    },
    types::block::output::{
        feature::{BlockIssuerKey, BlockIssuerKeySource, Ed25519PublicKeyHashBlockIssuerKey},
        AccountId, OutputId,
    },
    wallet::{
        operations::transaction::{TransactionOptions, TransactionWithMetadata},
        Wallet, WalletError,
    },
};

impl<S: 'static + SecretManage> Wallet<S>
where
    WalletError: From<S::Error>,
    ClientError: From<S::Error>,
{
    /// Transitions an implicit account to an account.
    pub async fn implicit_account_transition(
        &self,
        output_id: &OutputId,
        key_source: impl Into<BlockIssuerKeySource> + Send,
    ) -> Result<TransactionWithMetadata, WalletError> {
        let issuer_id = AccountId::from(output_id);

        self.sign_and_submit_transaction(
            self.prepare_implicit_account_transition(output_id, key_source).await?,
            TransactionOptions {
                issuer_id: Some(issuer_id),
                ..Default::default()
            },
        )
        .await
    }

    /// Prepares to transition an implicit account to an account.
    pub async fn prepare_implicit_account_transition(
        &self,
        output_id: &OutputId,
        key_source: impl Into<BlockIssuerKeySource> + Send,
    ) -> Result<PreparedTransactionData, WalletError>
    where
        WalletError: From<S::Error>,
    {
        let ed25519_address = {
            let wallet_ledger = self.ledger().await;
            let implicit_account_data = wallet_ledger
                .unspent_outputs
                .get(output_id)
                .ok_or(WalletError::ImplicitAccountNotFound)?;
            let implicit_account = if implicit_account_data.output.is_implicit_account() {
                implicit_account_data.output.as_basic()
            } else {
                return Err(WalletError::ImplicitAccountNotFound);
            };
            *implicit_account
                .address()
                .as_implicit_account_creation()
                .ed25519_address()
        };

        let block_issuer_key = BlockIssuerKey::from(match key_source.into() {
            BlockIssuerKeySource::ImplicitAccountAddress => Ed25519PublicKeyHashBlockIssuerKey::new(*ed25519_address),
            BlockIssuerKeySource::PublicKey(public_key) => {
                Ed25519PublicKeyHashBlockIssuerKey::from_public_key(public_key)
            }
            BlockIssuerKeySource::Bip44Path(bip_path) => Ed25519PublicKeyHashBlockIssuerKey::from_public_key(
                self.secret_manager
                    .read()
                    .await
                    .generate_ed25519_public_keys(
                        bip_path.coin_type,
                        bip_path.account,
                        bip_path.address_index..bip_path.address_index + 1,
                        None,
                    )
                    .await?[0],
            ),
        });

        let transaction_options = TransactionOptions {
            required_inputs: [*output_id].into(),
            issuer_id: Some(AccountId::from(output_id)),
            transitions: Some(Transitions::new().add_implicit_account(*output_id, block_issuer_key)),
            ..Default::default()
        };

        self.prepare_send_outputs(None, transaction_options).await
    }
}
