// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use getset::Getters;
use serde::{Deserialize, Serialize};

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage},
    types::block::{
        address::Bech32Address,
        output::{
            feature::{IssuerFeature, MetadataFeature, SenderFeature, TagFeature},
            unlock_condition::AddressUnlockCondition,
            NftId, NftOutputBuilder,
        },
    },
    utils::ConvertTo,
    wallet::{
        operations::transaction::{TransactionOptions, TransactionWithMetadata},
        Wallet,
    },
};

/// Address and NFT for `send_nft()`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Getters)]
#[serde(rename_all = "camelCase")]
pub struct MintNftParams {
    /// Bech32 encoded address to which the NFT will be minted. Default will use the
    /// address of the wallet.
    #[getset(get = "pub")]
    address: Option<Bech32Address>,
    /// NFT sender feature.
    #[getset(get = "pub")]
    sender: Option<Bech32Address>,
    /// NFT metadata feature.
    #[getset(get = "pub")]
    metadata: Option<MetadataFeature>,
    /// NFT tag feature.
    #[getset(get = "pub")]
    #[serde(default, with = "crate::utils::serde::option_prefix_hex_bytes")]
    tag: Option<Vec<u8>>,
    /// NFT issuer feature.
    #[getset(get = "pub")]
    issuer: Option<Bech32Address>,
    /// NFT immutable metadata feature.
    #[getset(get = "pub")]
    immutable_metadata: Option<MetadataFeature>,
}

impl MintNftParams {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the address and try convert to [`Bech32Address`]
    pub fn try_with_address(mut self, address: impl ConvertTo<Bech32Address>) -> crate::wallet::Result<Self> {
        self.address = Some(address.convert()?);
        Ok(self)
    }

    /// Set the address
    pub fn with_address(mut self, address: impl Into<Option<Bech32Address>>) -> Self {
        self.address = address.into();
        self
    }

    /// Set the sender address and try convert to [`Bech32Address`]
    pub fn try_with_sender(mut self, sender: impl ConvertTo<Bech32Address>) -> crate::wallet::Result<Self> {
        self.sender = Some(sender.convert()?);
        Ok(self)
    }

    /// Set the sender address
    pub fn with_sender(mut self, sender: impl Into<Option<Bech32Address>>) -> Self {
        self.sender = sender.into();
        self
    }

    /// Set the metadata
    pub fn with_metadata(mut self, metadata: impl Into<Option<MetadataFeature>>) -> Self {
        self.metadata = metadata.into();
        self
    }

    /// Set the tag
    pub fn with_tag(mut self, tag: impl Into<Option<Vec<u8>>>) -> Self {
        self.tag = tag.into();
        self
    }

    /// Set the issuer address and try convert to [`Bech32Address`]
    pub fn try_with_issuer(mut self, issuer: impl ConvertTo<Bech32Address>) -> crate::wallet::Result<Self> {
        self.issuer = Some(issuer.convert()?);
        Ok(self)
    }

    /// Set the issuer address
    pub fn with_issuer(mut self, issuer: impl Into<Option<Bech32Address>>) -> Self {
        self.issuer = issuer.into();
        self
    }

    /// Set the immutable metadata
    pub fn with_immutable_metadata(mut self, immutable_metadata: impl Into<Option<MetadataFeature>>) -> Self {
        self.immutable_metadata = immutable_metadata.into();
        self
    }
}

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    /// Mints NFTs.
    ///
    /// Calls [Wallet::send_outputs()] internally. The options may define the remainder value strategy or custom inputs.
    /// Note that addresses need to be bech32-encoded.
    /// ```ignore
    /// let nft_id: [u8; 38] =
    ///     prefix_hex::decode("08e68f7616cd4948efebc6a77c4f93aed770ac53860100000000000000000000000000000000")?
    ///         .try_into()
    ///         .unwrap();
    /// let params = [MintNftParams::new()
    ///     try_with_address("rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu")?
    ///     with_metadata(b"some nft metadata".to_vec())
    ///     with_immutable_metadata(b"some immutable nft metadata".to_vec())
    /// ];
    ///
    /// let transaction = account.mint_nfts(params, None).await?;
    /// println!(
    ///     "Transaction sent: {}/transaction/{}",
    ///     std::env::var("EXPLORER_URL").unwrap(),
    ///     transaction.transaction_id,
    /// );
    /// ```
    pub async fn mint_nfts<I: IntoIterator<Item = MintNftParams> + Send>(
        &self,
        params: I,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<TransactionWithMetadata>
    where
        I::IntoIter: Send,
    {
        let options = options.into();
        let prepared_transaction = self.prepare_mint_nfts(params, options.clone()).await?;

        self.sign_and_submit_transaction(prepared_transaction, None, options)
            .await
    }

    /// Prepares the transaction for [Wallet::mint_nfts()].
    pub async fn prepare_mint_nfts<I: IntoIterator<Item = MintNftParams> + Send>(
        &self,
        params: I,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<PreparedTransactionData>
    where
        I::IntoIter: Send,
    {
        log::debug!("[TRANSACTION] prepare_mint_nfts");
        let storage_score_params = self.client().get_storage_score_parameters().await?;
        let wallet_address = self.address().await.into_inner();
        let mut outputs = Vec::new();

        for MintNftParams {
            address,
            sender,
            metadata,
            tag,
            issuer,
            immutable_metadata,
        } in params
        {
            let address = match address {
                Some(address) => {
                    self.client().bech32_hrp_matches(address.hrp()).await?;
                    address.inner().clone()
                }
                None => wallet_address.clone(),
            };

            // NftId needs to be set to 0 for the creation
            let mut nft_builder = NftOutputBuilder::new_with_minimum_amount(storage_score_params, NftId::null())
                // Address which will own the nft
                .add_unlock_condition(AddressUnlockCondition::new(address));

            if let Some(sender) = sender {
                nft_builder = nft_builder.add_feature(SenderFeature::new(sender));
            }

            if let Some(metadata) = metadata {
                nft_builder = nft_builder.add_feature(metadata);
            }

            if let Some(tag) = tag {
                nft_builder = nft_builder.add_feature(TagFeature::new(tag)?);
            }

            if let Some(issuer) = issuer {
                nft_builder = nft_builder.add_immutable_feature(IssuerFeature::new(issuer));
            }

            if let Some(immutable_metadata) = immutable_metadata {
                nft_builder = nft_builder.add_immutable_feature(immutable_metadata);
            }

            outputs.push(nft_builder.finish_output()?);
        }

        self.prepare_transaction(outputs, options).await
    }
}
