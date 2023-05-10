// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::{
    client::api::PreparedTransactionData,
    types::block::{
        address::Bech32Address,
        output::{
            feature::{IssuerFeature, MetadataFeature, SenderFeature, TagFeature},
            unlock_condition::AddressUnlockCondition,
            NftId, NftOutputBuilder,
        },
        Error as BlockError,
    },
    wallet::{
        account::{operations::transaction::Transaction, Account, TransactionOptions},
        Error as WalletError,
    },
};

/// Address and NFT for `send_nft()`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MintNftParams {
    /// Bech32 encoded address to which the NFT will be minted. Default will use the
    /// first address of the account.
    pub address: Option<Bech32Address>,
    /// NFT sender feature.
    pub sender: Option<Bech32Address>,
    /// NFT metadata feature.
    pub metadata: Option<Vec<u8>>,
    /// NFT tag feature.
    pub tag: Option<Vec<u8>>,
    /// NFT issuer feature.
    pub issuer: Option<Bech32Address>,
    /// NFT immutable metadata feature.
    pub immutable_metadata: Option<Vec<u8>>,
}

/// Dto for MintNftParams.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MintNftParamsDto {
    /// Bech32 encoded address to which the NFT will be minted. Default will use the
    /// first address of the account.
    pub address: Option<Bech32Address>,
    /// NFT sender feature, bech32 encoded address.
    pub sender: Option<Bech32Address>,
    /// NFT metadata feature, hex encoded bytes.
    pub metadata: Option<String>,
    /// NFT tag feature, hex encoded bytes.
    pub tag: Option<String>,
    /// NFT issuer feature, bech32 encoded address.
    pub issuer: Option<Bech32Address>,
    /// Immutable NFT metadata, hex encoded bytes.
    pub immutable_metadata: Option<String>,
}

impl TryFrom<&MintNftParamsDto> for MintNftParams {
    type Error = crate::wallet::Error;

    fn try_from(value: &MintNftParamsDto) -> crate::wallet::Result<Self> {
        Ok(Self {
            address: value.address,
            sender: value.sender,
            metadata: match &value.metadata {
                Some(metadata) => Some(prefix_hex::decode(metadata).map_err(|_| BlockError::InvalidField("metadata"))?),
                None => None,
            },
            tag: match &value.tag {
                Some(tag) => Some(prefix_hex::decode(tag).map_err(|_| BlockError::InvalidField("tag"))?),
                None => None,
            },
            issuer: value.issuer,
            immutable_metadata: match &value.immutable_metadata {
                Some(metadata) => {
                    Some(prefix_hex::decode(metadata).map_err(|_| BlockError::InvalidField("immutable_metadata"))?)
                }
                None => None,
            },
        })
    }
}

impl Account {
    /// Function to mint nfts.
    /// Calls [Account.send()](crate::account::Account.send) internally, the options can define the
    /// RemainderValueStrategy or custom inputs.
    /// Address needs to be Bech32 encoded
    /// ```ignore
    /// let nft_id: [u8; 38] =
    ///     prefix_hex::decode("08e68f7616cd4948efebc6a77c4f93aed770ac53860100000000000000000000000000000000")?
    ///         .try_into()
    ///         .unwrap();
    /// let params = vec![MintNftParams {
    ///     address: Some("rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu".to_string()),
    ///     sender: None,
    ///     metadata: Some(b"some nft metadata".to_vec()),
    ///     tag: None,
    ///     issuer: None,
    ///     immutable_metadata: Some(b"some immutable nft metadata".to_vec()),
    /// }];
    ///
    /// let transaction = account.mint_nfts(params, None).await?;
    /// println!(
    ///     "Transaction sent: {}/transaction/{}",
    ///     std::env::var("EXPLORER_URL").unwrap(),
    ///     transaction.transaction_id,
    /// );
    /// ```
    pub async fn mint_nfts(
        &self,
        params: Vec<MintNftParams>,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<Transaction> {
        let prepared_transaction = self.prepare_mint_nfts(params, options).await?;
        self.sign_and_submit_transaction(prepared_transaction).await
    }

    /// Function to prepare the transaction for
    /// [Account.mint_nfts()](crate::account::Account.mint_nfts)
    async fn prepare_mint_nfts(
        &self,
        params: Vec<MintNftParams>,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<PreparedTransactionData> {
        log::debug!("[TRANSACTION] prepare_mint_nfts");
        let rent_structure = self.client.get_rent_structure().await?;
        let token_supply = self.client.get_token_supply().await?;
        let account_addresses = self.addresses().await?;
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
                    self.client.bech32_hrp_matches(address.hrp()).await?;
                    address.inner
                }
                // todo other error message
                None => {
                    account_addresses
                        .first()
                        .ok_or(WalletError::FailedToGetRemainder)?
                        .address
                        .inner
                }
            };

            // NftId needs to be set to 0 for the creation
            let mut nft_builder = NftOutputBuilder::new_with_minimum_storage_deposit(rent_structure, NftId::null())
                // Address which will own the nft
                .add_unlock_condition(AddressUnlockCondition::new(address));

            if let Some(sender) = sender {
                nft_builder = nft_builder.add_feature(SenderFeature::new(sender));
            }

            if let Some(metadata) = metadata {
                nft_builder = nft_builder.add_feature(MetadataFeature::new(metadata)?);
            }

            if let Some(tag) = tag {
                nft_builder = nft_builder.add_feature(TagFeature::new(tag)?);
            }

            if let Some(issuer) = issuer {
                nft_builder = nft_builder.add_immutable_feature(IssuerFeature::new(issuer));
            }

            if let Some(immutable_metadata) = immutable_metadata {
                nft_builder = nft_builder.add_immutable_feature(MetadataFeature::new(immutable_metadata)?);
            }

            outputs.push(nft_builder.finish_output(token_supply)?);
        }

        self.prepare_transaction(outputs, options).await
    }
}
