// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::{
    client::secret::SecretManage,
    types::block::{
        address::{Address, Bech32Address},
        output::{
            feature::{IssuerFeature, MetadataFeature, SenderFeature, TagFeature},
            unlock_condition::{
                AddressUnlockCondition, ExpirationUnlockCondition, StorageDepositReturnUnlockCondition,
                TimelockUnlockCondition,
            },
            BasicOutputBuilder, MinimumStorageDepositBasicOutput, NativeToken, NftId, NftOutputBuilder, Output, Rent,
            RentStructure, UnlockCondition,
        },
        slot::SlotIndex,
        Error,
    },
    wallet::account::{
        operations::transaction::RemainderValueStrategy, types::OutputData, Account, TransactionOptions,
    },
};

impl<S: 'static + SecretManage> Account<S>
where
    crate::wallet::Error: From<S::Error>,
{
    /// Prepare a basic or NFT output for sending
    /// If the amount is below the minimum required storage deposit, by default the remaining amount will automatically
    /// be added with a StorageDepositReturn UnlockCondition, when setting the ReturnStrategy to `gift`, the full
    /// minimum required storage deposit will be sent to the recipient.
    /// When the assets contain an nft_id, the data from the existing nft output will be used, just with the address
    /// unlock conditions replaced
    pub async fn prepare_output(
        &self,
        params: OutputParams,
        transaction_options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<Output> {
        log::debug!("[OUTPUT] prepare_output {params:?}");
        let transaction_options = transaction_options.into();
        let token_supply = self.client().get_token_supply().await?;

        self.client().bech32_hrp_matches(params.recipient_address.hrp()).await?;

        let rent_structure = self.client().get_rent_structure().await?;

        let nft_id = params.assets.as_ref().and_then(|a| a.nft_id);

        let (mut first_output_builder, existing_nft_output_data) = self
            .create_initial_output_builder(params.recipient_address, nft_id, rent_structure)
            .await?;

        if let Some(assets) = &params.assets {
            if let Some(native_tokens) = &assets.native_tokens {
                first_output_builder = first_output_builder.with_native_tokens(native_tokens.clone());
            }
        }

        if let Some(features) = params.features {
            if let Some(tag) = features.tag {
                first_output_builder = first_output_builder.add_feature(TagFeature::new(
                    prefix_hex::decode::<Vec<u8>>(tag).map_err(|_| Error::InvalidField("tag"))?,
                )?);
            }

            if let Some(metadata) = features.metadata {
                first_output_builder = first_output_builder.add_feature(MetadataFeature::new(
                    prefix_hex::decode::<Vec<u8>>(metadata).map_err(|_| Error::InvalidField("metadata"))?,
                )?);
            }

            if let Some(sender) = features.sender {
                first_output_builder = first_output_builder.add_feature(SenderFeature::new(sender))
            }

            if let Some(issuer) = features.issuer {
                if let OutputBuilder::Basic(_) = first_output_builder {
                    return Err(crate::wallet::Error::MissingParameter("nft_id"));
                }
                first_output_builder = first_output_builder.add_immutable_feature(IssuerFeature::new(issuer));
            }
        }

        if let Some(unlocks) = params.unlocks {
            if let Some(expiration_slot_index) = unlocks.expiration_slot_index {
                let remainder_address = self.get_remainder_address(transaction_options.clone()).await?;

                first_output_builder = first_output_builder.add_unlock_condition(ExpirationUnlockCondition::new(
                    remainder_address,
                    expiration_slot_index,
                )?);
            }
            if let Some(timelock_slot_index) = unlocks.timelock_slot_index {
                first_output_builder =
                    first_output_builder.add_unlock_condition(TimelockUnlockCondition::new(timelock_slot_index)?);
            }
        }

        // Build output with minimum required storage deposit so we can use the amount in the next step
        let first_output = first_output_builder
            .with_minimum_storage_deposit(rent_structure)
            .finish_output(token_supply)?;

        let mut second_output_builder = if nft_id.is_some() {
            OutputBuilder::Nft(NftOutputBuilder::from(first_output.as_nft()))
        } else {
            OutputBuilder::Basic(BasicOutputBuilder::from(first_output.as_basic()))
        };

        let min_storage_deposit_basic_output =
            MinimumStorageDepositBasicOutput::new(rent_structure, token_supply).finish()?;

        if params.amount > min_storage_deposit_basic_output {
            second_output_builder = second_output_builder.with_amount(params.amount);
        }

        let return_strategy = params
            .storage_deposit
            .clone()
            .unwrap_or_default()
            .return_strategy
            .unwrap_or_default();
        let remainder_address = self.get_remainder_address(transaction_options).await?;
        if params.amount < min_storage_deposit_basic_output {
            if return_strategy == ReturnStrategy::Gift {
                second_output_builder = second_output_builder.with_amount(min_storage_deposit_basic_output);
            }
            if return_strategy == ReturnStrategy::Return {
                second_output_builder =
                    second_output_builder.add_unlock_condition(StorageDepositReturnUnlockCondition::new(
                        remainder_address,
                        // Return minimum storage deposit
                        min_storage_deposit_basic_output,
                        token_supply,
                    )?);

                // Update output amount, so recipient still gets the provided amount
                let new_amount = params.amount + min_storage_deposit_basic_output;
                // new_amount could be not enough because we added the storage deposit return unlock condition, so we
                // need to check the min required storage deposit again
                let min_storage_deposit_new_amount = second_output_builder
                    .clone()
                    .with_minimum_storage_deposit(rent_structure)
                    .finish_output(token_supply)?
                    .amount();

                if new_amount < min_storage_deposit_new_amount {
                    let additional_required_amount = min_storage_deposit_new_amount - new_amount;
                    second_output_builder = second_output_builder.with_amount(new_amount + additional_required_amount);
                    // Add the additional amount to the SDR
                    second_output_builder =
                        second_output_builder.replace_unlock_condition(StorageDepositReturnUnlockCondition::new(
                            remainder_address,
                            // Return minimum storage deposit
                            min_storage_deposit_basic_output + additional_required_amount,
                            token_supply,
                        )?);
                } else {
                    // new_amount is enough
                    second_output_builder = second_output_builder.with_amount(new_amount);
                }
            }
        }

        let third_output = second_output_builder.clone().finish_output(token_supply)?;
        let mut final_amount = third_output.amount();
        // Now we have to make sure that our output also works with our available balance, without leaving <
        // min_storage_deposit_basic_output for a remainder (if not 0)
        let mut available_base_coin = self.balance().await?.base_coin.available;
        // If we're sending an existing NFT, its minimum required storage deposit is not part of the available base_coin
        // balance, so we add it here
        if let Some(existing_nft_output_data) = existing_nft_output_data {
            available_base_coin += existing_nft_output_data.output.rent_cost(&rent_structure);
        }

        if final_amount > available_base_coin {
            return Err(crate::wallet::Error::InsufficientFunds {
                available: available_base_coin,
                required: final_amount,
            });
        }
        if final_amount == available_base_coin {
            return Ok(third_output);
        }

        if final_amount < available_base_coin {
            let remaining_balance = available_base_coin - final_amount;
            if remaining_balance < min_storage_deposit_basic_output {
                // not enough for remainder
                if params
                    .storage_deposit
                    .unwrap_or_default()
                    .use_excess_if_low
                    .unwrap_or_default()
                {
                    // add remaining amount
                    final_amount += remaining_balance;
                    second_output_builder = second_output_builder.with_amount(final_amount);

                    if let Some(sdr) = third_output
                        .unlock_conditions()
                        .expect("basic and nft outputs have unlock conditions")
                        .storage_deposit_return()
                    {
                        // create a new sdr unlock_condition with the updated amount and replace it
                        let new_sdr_amount = sdr.amount() + remaining_balance;
                        second_output_builder =
                            second_output_builder.replace_unlock_condition(StorageDepositReturnUnlockCondition::new(
                                remainder_address,
                                // Return minimum storage deposit
                                new_sdr_amount,
                                token_supply,
                            )?);
                    }
                } else {
                    // Would leave dust behind, so return what's required for a remainder
                    return Err(crate::wallet::Error::InsufficientFunds {
                        available: available_base_coin,
                        required: available_base_coin + min_storage_deposit_basic_output - remaining_balance,
                    });
                }
            }
        }

        Ok(second_output_builder.finish_output(token_supply)?)
    }

    // Create the initial output builder for prepare_output()
    async fn create_initial_output_builder(
        &self,
        recipient_address: Bech32Address,
        nft_id: Option<NftId>,
        rent_structure: RentStructure,
    ) -> crate::wallet::Result<(OutputBuilder, Option<OutputData>)> {
        let (mut first_output_builder, existing_nft_output_data) = if let Some(nft_id) = &nft_id {
            if nft_id.is_null() {
                // Mint a new NFT output
                (
                    OutputBuilder::Nft(NftOutputBuilder::new_with_minimum_storage_deposit(
                        rent_structure,
                        *nft_id,
                    )),
                    None,
                )
            } else {
                // Transition an existing NFT output
                let unspent_nft_output = self.unspent_nft_output(nft_id).await?;

                // Find nft output from the inputs
                let mut first_output_builder = if let Some(nft_output_data) = &unspent_nft_output {
                    let nft_output = nft_output_data.output.as_nft();
                    NftOutputBuilder::from(nft_output).with_nft_id(*nft_id)
                } else {
                    return Err(crate::wallet::Error::NftNotFoundInUnspentOutputs);
                };
                // Remove potentially existing features and unlock conditions.
                first_output_builder = first_output_builder.clear_features();
                first_output_builder = first_output_builder.clear_unlock_conditions();
                (OutputBuilder::Nft(first_output_builder), unspent_nft_output)
            }
        } else {
            (
                OutputBuilder::Basic(BasicOutputBuilder::new_with_minimum_storage_deposit(rent_structure)),
                None,
            )
        };

        // Set new address unlock condition
        first_output_builder =
            first_output_builder.add_unlock_condition(AddressUnlockCondition::new(recipient_address));
        Ok((first_output_builder, existing_nft_output_data))
    }

    // Get a remainder address based on transaction_options or use the first account address
    async fn get_remainder_address(
        &self,
        transaction_options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<Address> {
        let transaction_options = transaction_options.into();

        let remainder_address = match &transaction_options {
            Some(options) => {
                match &options.remainder_value_strategy {
                    RemainderValueStrategy::ReuseAddress => {
                        // select_inputs will select an address from the inputs if it's none
                        None
                    }
                    RemainderValueStrategy::ChangeAddress => {
                        let remainder_address = self.generate_remainder_address().await?;
                        Some(remainder_address.address().inner)
                    }
                    RemainderValueStrategy::CustomAddress(address) => Some(address.address().inner),
                }
            }
            None => None,
        };
        let remainder_address = match remainder_address {
            Some(address) => address,
            None => {
                self.addresses()
                    .await?
                    .first()
                    .ok_or(crate::wallet::Error::FailedToGetRemainder)?
                    .address()
                    .inner
            }
        };
        Ok(remainder_address)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputParams {
    pub recipient_address: Bech32Address,
    #[serde(with = "crate::utils::serde::string")]
    pub amount: u64,
    #[serde(default)]
    pub assets: Option<Assets>,
    #[serde(default)]
    pub features: Option<Features>,
    #[serde(default)]
    pub unlocks: Option<Unlocks>,
    #[serde(default)]
    pub storage_deposit: Option<StorageDeposit>,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assets {
    pub native_tokens: Option<Vec<NativeToken>>,
    pub nft_id: Option<NftId>,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Features {
    pub tag: Option<String>,
    pub metadata: Option<String>,
    pub issuer: Option<Bech32Address>,
    pub sender: Option<Bech32Address>,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Unlocks {
    pub expiration_slot_index: Option<SlotIndex>,
    pub timelock_slot_index: Option<SlotIndex>,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageDeposit {
    pub return_strategy: Option<ReturnStrategy>,
    // If account has 2 Mi, min storage deposit is 1 Mi and one wants to send 1.5 Mi, it wouldn't be possible with a
    // 0.5 Mi remainder. To still send a transaction, the 0.5 can be added to the output automatically, if set to true
    pub use_excess_if_low: Option<bool>,
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ReturnStrategy {
    // A storage deposit return unlock condition will be added with the required minimum storage deposit
    #[default]
    Return,
    // The recipient address will get the additional amount to reach the minimum storage deposit gifted
    Gift,
}

#[derive(Clone)]
enum OutputBuilder {
    Basic(BasicOutputBuilder),
    Nft(NftOutputBuilder),
}

impl OutputBuilder {
    fn add_feature(mut self, feature: impl Into<crate::types::block::output::Feature>) -> Self {
        match self {
            Self::Basic(b) => self = Self::Basic(b.add_feature(feature)),
            Self::Nft(b) => self = Self::Nft(b.add_feature(feature)),
        }
        self
    }
    fn add_immutable_feature(mut self, feature: impl Into<crate::types::block::output::Feature>) -> Self {
        match self {
            Self::Basic(_) => { // Basic outputs can't have immutable features
            }
            Self::Nft(b) => {
                self = Self::Nft(b.add_immutable_feature(feature));
            }
        }
        self
    }
    fn add_unlock_condition(mut self, unlock_condition: impl Into<UnlockCondition>) -> Self {
        match self {
            Self::Basic(b) => {
                self = Self::Basic(b.add_unlock_condition(unlock_condition));
            }
            Self::Nft(b) => {
                self = Self::Nft(b.add_unlock_condition(unlock_condition));
            }
        }
        self
    }
    fn replace_unlock_condition(mut self, unlock_condition: impl Into<UnlockCondition>) -> Self {
        match self {
            Self::Basic(b) => {
                self = Self::Basic(b.replace_unlock_condition(unlock_condition));
            }
            Self::Nft(b) => {
                self = Self::Nft(b.replace_unlock_condition(unlock_condition));
            }
        }
        self
    }
    fn with_amount(mut self, amount: u64) -> Self {
        match self {
            Self::Basic(b) => {
                self = Self::Basic(b.with_amount(amount));
            }
            Self::Nft(b) => {
                self = Self::Nft(b.with_amount(amount));
            }
        }
        self
    }
    fn with_minimum_storage_deposit(mut self, rent_structure: RentStructure) -> Self {
        match self {
            Self::Basic(b) => {
                self = Self::Basic(b.with_minimum_storage_deposit(rent_structure));
            }
            Self::Nft(b) => {
                self = Self::Nft(b.with_minimum_storage_deposit(rent_structure));
            }
        }
        self
    }
    fn with_native_tokens(mut self, native_tokens: impl IntoIterator<Item = NativeToken>) -> Self {
        match self {
            Self::Basic(b) => {
                self = Self::Basic(b.with_native_tokens(native_tokens));
            }
            Self::Nft(b) => {
                self = Self::Nft(b.with_native_tokens(native_tokens));
            }
        }
        self
    }
    fn finish_output(self, token_supply: u64) -> Result<Output, crate::types::block::Error> {
        match self {
            Self::Basic(b) => b.finish_output(token_supply),
            Self::Nft(b) => b.finish_output(token_supply),
        }
    }
}
