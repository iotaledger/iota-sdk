// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::api::{input_selection::Burn, PreparedTransactionData},
    wallet::{operations::transaction::TransactionOptions, types::TransactionWithMetadata, Wallet},
};

pub(crate) mod melt_native_token;

impl Wallet {
    /// A generic function that can be used to burn native tokens, nfts, delegations, foundries and accounts.
    ///
    /// Note that burning **native tokens** doesn't require the foundry output which minted them, but will not increase
    /// the foundries `melted_tokens` field, which makes it impossible to destroy the foundry output. Therefore it's
    /// recommended to use melting, if the foundry output is available.
    pub async fn burn(
        &self,
        burn: impl Into<Burn> + Send,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<TransactionWithMetadata> {
        let options = options.into();
        let prepared = self.prepare_burn(burn, options.clone()).await?;

        self.sign_and_submit_transaction(prepared, None, options).await
    }

    /// A generic `prepare_burn()` function that can be used to prepare the burn of native tokens, nfts, delegations,
    /// foundries and accounts.
    ///
    /// Note that burning **native tokens** doesn't require the foundry output which minted them, but will not increase
    /// the foundries `melted_tokens` field, which makes it impossible to destroy the foundry output. Therefore it's
    /// recommended to use melting, if the foundry output is available.
    pub async fn prepare_burn(
        &self,
        burn: impl Into<Burn> + Send,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<PreparedTransactionData> {
        let mut options = options.into().unwrap_or_default();
        options.burn = Some(burn.into());

        // The empty list of outputs is used. Outputs will be generated by
        // the input selection algorithm based on the content of the [`Burn`] object.
        self.prepare_transaction([], options).await
    }
}
