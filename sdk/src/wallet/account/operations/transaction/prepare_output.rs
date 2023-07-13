// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::cmp::Ordering;

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
            BasicOutputBuilder, MinimumStorageDepositBasicOutput, NativeToken, NftId, NftOutput, NftOutputBuilder,
            Output, Rent,
        },
        Error,
    },
    wallet::account::{operations::transaction::RemainderValueStrategy, Account, FilterOptions, TransactionOptions},
};

impl<S: 'static + SecretManage> Account<S>
where
    crate::wallet::Error: From<S::Error>,
{
    /// Prepare an output for sending
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

        if let Some(assets) = &params.assets {
            if let Some(nft_id) = assets.nft_id {
                return self.prepare_nft_output(params, transaction_options, nft_id).await;
            }
        }
        let rent_structure = self.client().get_rent_structure().await?;

        // We start building with minimum storage deposit, so we know the minimum required amount and can later replace
        // it, if needed
        let mut first_output_builder = BasicOutputBuilder::new_with_minimum_storage_deposit(rent_structure)
            .add_unlock_condition(AddressUnlockCondition::new(params.recipient_address));

        if let Some(assets) = params.assets {
            if let Some(native_tokens) = assets.native_tokens {
                first_output_builder = first_output_builder.with_native_tokens(native_tokens);
            }
        }

        if let Some(features) = params.features {
            if let Some(_issuer) = features.issuer {
                return Err(crate::wallet::Error::MissingParameter("nft_id"));
            }

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
        }

        if let Some(unlocks) = params.unlocks {
            if let Some(expiration_unix_time) = unlocks.expiration_unix_time {
                let remainder_address = self.get_remainder_address(transaction_options.clone()).await?;

                first_output_builder = first_output_builder
                    .add_unlock_condition(ExpirationUnlockCondition::new(remainder_address, expiration_unix_time)?);
            }
            if let Some(timelock_unix_time) = unlocks.timelock_unix_time {
                first_output_builder =
                    first_output_builder.add_unlock_condition(TimelockUnlockCondition::new(timelock_unix_time)?);
            }
        }

        let first_output = first_output_builder.finish(token_supply)?;

        let mut second_output_builder = BasicOutputBuilder::from(&first_output);

        // Update the amount
        match params.amount.cmp(&first_output.amount()) {
            Ordering::Greater | Ordering::Equal => {
                // if it's larger than the minimum storage deposit, just replace it
                second_output_builder = second_output_builder.with_amount(params.amount);
            }
            Ordering::Less => {
                let storage_deposit = params.storage_deposit.unwrap_or_default();
                // Gift return strategy doesn't need a change, since the amount is already the minimum storage
                // deposit
                if storage_deposit.return_strategy.unwrap_or_default() == ReturnStrategy::Return {
                    let remainder_address = self.get_remainder_address(transaction_options).await?;

                    // Calculate the minimum storage deposit to be returned
                    let min_storage_deposit_return_amount =
                        MinimumStorageDepositBasicOutput::new(rent_structure, token_supply).finish()?;

                    second_output_builder =
                        second_output_builder.add_unlock_condition(StorageDepositReturnUnlockCondition::new(
                            remainder_address,
                            // Return minimum storage deposit
                            min_storage_deposit_return_amount,
                            token_supply,
                        )?);
                }

                // Check if the remainder balance wouldn't leave dust behind, which wouldn't allow the creation of this
                // output. If that's the case, this remaining amount will be added to the output, to still allow sending
                // it.
                if storage_deposit.use_excess_if_low.unwrap_or_default() {
                    let balance = self.balance().await?;
                    if balance.base_coin.available.cmp(&first_output.amount()) == Ordering::Greater {
                        let balance_minus_output = balance.base_coin.available - first_output.amount();
                        // Calculate the amount for a basic output
                        let minimum_required_storage_deposit =
                            MinimumStorageDepositBasicOutput::new(rent_structure, token_supply).finish()?;

                        if balance_minus_output < minimum_required_storage_deposit {
                            second_output_builder =
                                second_output_builder.with_amount(first_output.amount() + balance_minus_output);
                        }
                    }
                }
            }
        }

        let second_output = second_output_builder.finish(token_supply)?;

        let required_storage_deposit = Output::Basic(second_output.clone()).rent_cost(&rent_structure);

        let mut third_output_builder = BasicOutputBuilder::from(&second_output);

        // We might have added more unlock conditions, so we check the minimum storage deposit again and update the
        // amounts if needed
        if second_output.amount() < required_storage_deposit {
            let mut new_sdr_amount = required_storage_deposit - params.amount;
            let minimum_storage_deposit =
                MinimumStorageDepositBasicOutput::new(rent_structure, token_supply).finish()?;
            let mut final_output_amount = required_storage_deposit;
            if required_storage_deposit < params.amount + minimum_storage_deposit {
                // return amount must be >= minimum_storage_deposit
                new_sdr_amount = minimum_storage_deposit;

                // increase the output amount by the additional required amount for the SDR
                final_output_amount += minimum_storage_deposit - (required_storage_deposit - params.amount);
            }
            third_output_builder = third_output_builder.with_amount(final_output_amount);

            // add newly added amount also to the storage deposit return unlock condition, if that was added
            if let Some(sdr) = second_output.unlock_conditions().storage_deposit_return() {
                // create a new sdr unlock_condition with the updated amount and replace it
                third_output_builder = third_output_builder.replace_unlock_condition(
                    StorageDepositReturnUnlockCondition::new(*sdr.return_address(), new_sdr_amount, token_supply)?,
                );
            }
        }

        // Build and return the final output
        Ok(third_output_builder.finish_output(token_supply)?)
    }

    /// Prepare an nft output
    async fn prepare_nft_output(
        &self,
        params: OutputParams,
        transaction_options: impl Into<Option<TransactionOptions>> + Send,
        nft_id: NftId,
    ) -> crate::wallet::Result<Output> {
        log::debug!("[OUTPUT] prepare_nft_output {params:?}");

        let transaction_options = transaction_options.into();

        let token_supply = self.client().get_token_supply().await?;
        let rent_structure = self.client().get_rent_structure().await?;
        let unspent_nft_outputs = self
            .unspent_outputs(Some(FilterOptions {
                output_types: Some(vec![NftOutput::KIND]),
                ..Default::default()
            }))
            .await?;

        // Find nft output from the inputs
        let mut first_output_builder = if let Some(nft_output_data) = unspent_nft_outputs.iter().find(|o| {
            if let Output::Nft(nft_output) = &o.output {
                nft_id == nft_output.nft_id_non_null(&o.output_id)
            } else {
                false
            }
        }) {
            if let Output::Nft(nft_output) = &nft_output_data.output {
                NftOutputBuilder::from(nft_output).with_nft_id(nft_output.nft_id_non_null(&nft_output_data.output_id))
            } else {
                unreachable!("We checked before if it's an nft output")
            }
        } else if nft_id.is_null() {
            NftOutputBuilder::new_with_minimum_storage_deposit(rent_structure, nft_id)
        } else {
            return Err(crate::wallet::Error::NftNotFoundInUnspentOutputs);
        };

        // Remove potentially existing features.
        first_output_builder = first_output_builder.clear_features();

        // Set new address unlock condition
        first_output_builder =
            first_output_builder.with_unlock_conditions([AddressUnlockCondition::new(params.recipient_address)]);

        if let Some(assets) = params.assets {
            if let Some(native_tokens) = assets.native_tokens {
                first_output_builder = first_output_builder.with_native_tokens(native_tokens);
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
                first_output_builder = first_output_builder.add_immutable_feature(IssuerFeature::new(issuer));
            }
        }

        if let Some(unlocks) = params.unlocks {
            if let Some(expiration_unix_time) = unlocks.expiration_unix_time {
                let remainder_address = self.get_remainder_address(transaction_options.clone()).await?;

                first_output_builder = first_output_builder
                    .add_unlock_condition(ExpirationUnlockCondition::new(remainder_address, expiration_unix_time)?);
            }
            if let Some(timelock_unix_time) = unlocks.timelock_unix_time {
                first_output_builder =
                    first_output_builder.add_unlock_condition(TimelockUnlockCondition::new(timelock_unix_time)?);
            }
        }

        let first_output = first_output_builder
            .with_minimum_storage_deposit(rent_structure)
            .finish(token_supply)?;

        let mut second_output_builder = NftOutputBuilder::from(&first_output);

        // Update the amount
        match params.amount.cmp(&first_output.amount()) {
            Ordering::Greater | Ordering::Equal => {
                // if it's larger than the minimum storage deposit, just replace it
                second_output_builder = second_output_builder.with_amount(params.amount);
            }
            Ordering::Less => {
                let storage_deposit = params.storage_deposit.unwrap_or_default();
                // Gift return strategy doesn't need a change, since the amount is already the minimum storage
                // deposit
                if storage_deposit.return_strategy.unwrap_or_default() == ReturnStrategy::Return {
                    let remainder_address = self.get_remainder_address(transaction_options).await?;

                    // Calculate the amount to be returned
                    let min_storage_deposit_return_amount =
                        MinimumStorageDepositBasicOutput::new(rent_structure, token_supply).finish()?;

                    second_output_builder =
                        second_output_builder.add_unlock_condition(StorageDepositReturnUnlockCondition::new(
                            remainder_address,
                            // Return minimum storage deposit
                            min_storage_deposit_return_amount,
                            token_supply,
                        )?);
                }

                // Check if the remaining balance wouldn't leave dust behind, which wouldn't allow the creation of this
                // output. If that's the case, this remaining amount will be added to the output, to still allow sending
                // it.
                if storage_deposit.use_excess_if_low.unwrap_or_default() {
                    let balance = self.balance().await?;
                    if balance.base_coin.available.cmp(&first_output.amount()) == Ordering::Greater {
                        let balance_minus_output = balance.base_coin.available - first_output.amount();
                        // Calculate the amount for a basic output
                        let minimum_required_storage_deposit =
                            MinimumStorageDepositBasicOutput::new(rent_structure, token_supply).finish()?;

                        if balance_minus_output < minimum_required_storage_deposit {
                            second_output_builder =
                                second_output_builder.with_amount(first_output.amount() + balance_minus_output);
                        }
                    }
                }
            }
        }

        let second_output = second_output_builder.finish(token_supply)?;

        let required_storage_deposit = Output::Nft(second_output.clone()).rent_cost(&rent_structure);

        let mut third_output_builder = NftOutputBuilder::from(&second_output);

        // We might have added more unlock conditions, so we check the minimum storage deposit again and update the
        // amounts if needed
        if second_output.amount() < required_storage_deposit {
            let mut new_sdr_amount = required_storage_deposit - params.amount;
            let minimum_storage_deposit =
                MinimumStorageDepositBasicOutput::new(rent_structure, token_supply).finish()?;
            let mut final_output_amount = required_storage_deposit;
            if required_storage_deposit < params.amount + minimum_storage_deposit {
                // return amount must be >= minimum_storage_deposit
                new_sdr_amount = minimum_storage_deposit;

                // increase the output amount by the additional required amount for the SDR
                final_output_amount += minimum_storage_deposit - (required_storage_deposit - params.amount);
            }
            third_output_builder = third_output_builder.with_amount(final_output_amount);

            // add newly added amount also to the storage deposit return unlock condition, if that was added
            if let Some(sdr) = second_output.unlock_conditions().storage_deposit_return() {
                // create a new sdr unlock_condition with the updated amount and replace it
                third_output_builder = third_output_builder.replace_unlock_condition(
                    StorageDepositReturnUnlockCondition::new(*sdr.return_address(), new_sdr_amount, token_supply)?,
                );
            }
        }

        // Build and return the final output
        Ok(third_output_builder.finish_output(token_supply)?)
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
    pub expiration_unix_time: Option<u32>,
    pub timelock_unix_time: Option<u32>,
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
pub enum ReturnStrategy {
    // A storage deposit return unlock condition will be added with the required minimum storage deposit
    #[default]
    Return,
    // The recipient address will get the additional amount to reach the minimum storage deposit gifted
    Gift,
}
