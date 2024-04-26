// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{
    requirement::{account::is_account_with_id_non_null, foundry::is_foundry_with_id, nft::is_nft_with_id_non_null},
    TransactionBuilder, TransactionBuilderError,
};
use crate::{
    client::secret::types::InputSigningData,
    types::block::{
        address::Address,
        output::{
            feature::{BlockIssuerFeature, BlockIssuerKey, BlockIssuerKeys, StakingFeature},
            AccountId, AccountOutput, AccountOutputBuilder, AddressUnlockCondition, BasicOutput, FoundryOutput,
            FoundryOutputBuilder, NftOutput, NftOutputBuilder, Output, OutputId,
        },
        slot::EpochIndex,
    },
    utils::serde::string,
};

impl TransactionBuilder {
    /// Transitions an account input by creating a new account output if required.
    fn transition_account_input(
        &self,
        input: &AccountOutput,
        output_id: &OutputId,
    ) -> Result<Option<Output>, TransactionBuilderError> {
        let account_id = input.account_id_non_null(output_id);

        // Do not create an account output if the account input is to be burned.
        if self
            .burn
            .as_ref()
            .map(|burn| burn.accounts.contains(&account_id))
            .unwrap_or(false)
        {
            log::debug!("No transition of {output_id:?}/{account_id:?} as it needs to be burned");
            return Ok(None);
        }

        // Do not create an account output if it already exists.
        if self
            .non_remainder_outputs()
            .any(|output| is_account_with_id_non_null(output, &account_id))
        {
            log::debug!("No transition of {output_id:?}/{account_id:?} as output already exists");
            return Ok(None);
        }

        let mut highest_foundry_serial_number = 0;
        for output in self.non_remainder_outputs() {
            if let Output::Foundry(foundry) = output {
                if *foundry.account_address().account_id() == account_id {
                    highest_foundry_serial_number = u32::max(highest_foundry_serial_number, foundry.serial_number());
                }
            }
        }

        // Remove potential sender feature because it will not be needed anymore as it only needs to be verified once.
        let mut features = input
            .features()
            .iter()
            .filter(|feature| !feature.is_sender())
            .cloned()
            .collect::<Vec<_>>();

        if let Some(change) = self.transitions.as_ref().and_then(|t| t.accounts.get(&account_id)) {
            match change {
                AccountChange::BeginStaking {
                    staked_amount,
                    fixed_cost,
                    staking_period,
                } => {
                    if input.features().staking().is_some() {
                        return Err(TransactionBuilderError::AlreadyStaking(account_id));
                    }
                    let start_epoch = self.protocol_parameters.epoch_index_of(
                        self.protocol_parameters
                            .past_bounded_slot(self.latest_slot_commitment_id),
                    );
                    features.push(
                        StakingFeature::new(
                            *staked_amount,
                            *fixed_cost,
                            start_epoch,
                            staking_period
                                .map(|period| start_epoch + period)
                                .unwrap_or(EpochIndex(u32::MAX)),
                        )
                        .into(),
                    );
                }
                AccountChange::ExtendStaking { additional_epochs } => {
                    if let Some(feature) = features.iter_mut().find(|f| f.is_staking()) {
                        let future_bounded_epoch = self
                            .protocol_parameters
                            .future_bounded_epoch(self.latest_slot_commitment_id);
                        let staking_feature = feature.as_staking();
                        // Just extend the end epoch if it's still possible
                        if future_bounded_epoch <= staking_feature.end_epoch() {
                            *feature = StakingFeature::new(
                                staking_feature.staked_amount(),
                                staking_feature.fixed_cost(),
                                staking_feature.start_epoch(),
                                staking_feature.end_epoch().saturating_add(*additional_epochs),
                            )
                            .into();
                        // Otherwise, we'll have to claim the rewards
                        } else {
                            if *additional_epochs < self.protocol_parameters.staking_unbonding_period() {
                                return Err(TransactionBuilderError::StakingPeriodLessThanMin {
                                    additional_epochs: *additional_epochs,
                                    min: self.protocol_parameters.staking_unbonding_period(),
                                });
                            }
                            let past_bounded_epoch = self
                                .protocol_parameters
                                .past_bounded_epoch(self.latest_slot_commitment_id);
                            let end_epoch = past_bounded_epoch.saturating_add(*additional_epochs);
                            *feature = StakingFeature::new(
                                staking_feature.staked_amount(),
                                staking_feature.fixed_cost(),
                                past_bounded_epoch,
                                end_epoch,
                            )
                            .into();
                        }
                    } else {
                        return Err(TransactionBuilderError::NotStaking(account_id));
                    }
                }
                AccountChange::EndStaking => {
                    if input.features().staking().is_none() {
                        return Err(TransactionBuilderError::NotStaking(account_id));
                    }
                    features.retain(|f| !f.is_staking());
                }
                AccountChange::ModifyBlockIssuerKeys {
                    keys_to_add,
                    keys_to_remove,
                } => {
                    if let Some(feature) = features.iter_mut().find(|f| f.is_block_issuer()) {
                        let block_issuer_feature = feature.as_block_issuer();
                        let updated_keys = block_issuer_feature
                            .block_issuer_keys()
                            .iter()
                            .filter(|k| !keys_to_remove.contains(k))
                            .chain(keys_to_add)
                            .cloned()
                            .collect::<Vec<BlockIssuerKey>>();
                        *feature = BlockIssuerFeature::new(block_issuer_feature.expiry_slot(), updated_keys)?.into();
                    } else {
                        return Err(TransactionBuilderError::MissingBlockIssuerFeature(account_id));
                    }
                }
            }
        }

        let mut builder = AccountOutputBuilder::from(input)
            .with_minimum_amount(self.protocol_parameters.storage_score_parameters())
            .with_mana(0)
            .with_account_id(account_id)
            .with_foundry_counter(u32::max(highest_foundry_serial_number, input.foundry_counter()))
            .with_features(features);

        // Block issuers cannot move their mana elsewhere.
        if input.is_block_issuer() {
            if self.burn.as_ref().map_or(false, |b| b.generated_mana()) {
                builder = builder.with_mana(input.mana());
            } else {
                builder = builder.with_mana(input.available_mana(
                    &self.protocol_parameters,
                    output_id.transaction_id().slot_index(),
                    self.creation_slot,
                )?);
            }
        }

        let output = builder.finish_output()?;

        log::debug!("Automatic transition of {output_id:?}/{account_id:?}");

        Ok(Some(output))
    }

    fn transition_implicit_account_input(
        &self,
        input: &BasicOutput,
        output_id: &OutputId,
    ) -> Result<Option<Output>, TransactionBuilderError> {
        if let Some(block_issuer_key) = self
            .transitions
            .as_ref()
            .and_then(|t| t.implicit_accounts.get(output_id))
        {
            if !input.is_implicit_account() {
                return Err(TransactionBuilderError::TransitionNonImplicitAccount(*output_id));
            }
            let ed25519_address = *input.address().as_implicit_account_creation().ed25519_address();
            let account_id = AccountId::from(output_id);
            let account = AccountOutput::build_with_amount(input.amount(), account_id)
                .with_unlock_conditions([AddressUnlockCondition::from(Address::from(ed25519_address))])
                .with_features([BlockIssuerFeature::new(
                    u32::MAX,
                    BlockIssuerKeys::from_vec(vec![block_issuer_key.clone()])?,
                )?])
                .finish_output()?;
            Ok(Some(account))
        } else {
            Ok(None)
        }
    }

    /// Transitions an nft input by creating a new nft output if required.
    fn transition_nft_input(
        &self,
        input: &NftOutput,
        output_id: &OutputId,
    ) -> Result<Option<Output>, TransactionBuilderError> {
        let nft_id = input.nft_id_non_null(output_id);

        // Do not create an nft output if the nft input is to be burned.
        if self
            .burn
            .as_ref()
            .map(|burn| burn.nfts.contains(&nft_id))
            .unwrap_or(false)
        {
            log::debug!("No transition of {output_id:?}/{nft_id:?} as it needs to be burned");
            return Ok(None);
        }

        // Do not create an nft output if it already exists.
        if self
            .non_remainder_outputs()
            .any(|output| is_nft_with_id_non_null(output, &nft_id))
        {
            log::debug!("No transition of {output_id:?}/{nft_id:?} as output already exists");
            return Ok(None);
        }

        // Remove potential sender feature because it will not be needed anymore as it only needs to be verified once.
        let features = input.features().iter().filter(|feature| !feature.is_sender()).cloned();

        let output = NftOutputBuilder::from(input)
            .with_minimum_amount(self.protocol_parameters.storage_score_parameters())
            .with_mana(0)
            .with_nft_id(nft_id)
            .with_features(features)
            .finish_output()?;

        log::debug!("Automatic transition of {output_id:?}/{nft_id:?}");

        Ok(Some(output))
    }

    /// Transitions a foundry input by creating a new foundry output if required.
    fn transition_foundry_input(
        &self,
        input: &FoundryOutput,
        output_id: &OutputId,
    ) -> Result<Option<Output>, TransactionBuilderError> {
        let foundry_id = input.id();

        // Do not create a foundry output if the foundry input is to be burned.
        if self
            .burn
            .as_ref()
            .map(|burn| burn.foundries.contains(&foundry_id))
            .unwrap_or(false)
        {
            log::debug!("No transition of {output_id:?}/{foundry_id:?} as it needs to be burned");
            return Ok(None);
        }

        // Do not create a foundry output if it already exists.
        if self
            .non_remainder_outputs()
            .any(|output| is_foundry_with_id(output, &foundry_id))
        {
            log::debug!("No transition of {output_id:?}/{foundry_id:?} as output already exists");
            return Ok(None);
        }

        let output = FoundryOutputBuilder::from(input)
            .with_minimum_amount(self.protocol_parameters.storage_score_parameters())
            .finish_output()?;

        log::debug!("Automatic transition of {output_id:?}/{foundry_id:?}");

        Ok(Some(output))
    }

    /// Transitions an input by creating a new output if required.
    /// If no `account_transition` is provided, assumes a state transition.
    pub(crate) fn transition_input(&self, input: &InputSigningData) -> Result<Option<Output>, TransactionBuilderError> {
        match &input.output {
            Output::Account(account_input) => self.transition_account_input(account_input, input.output_id()),
            Output::Foundry(foundry_input) => self.transition_foundry_input(foundry_input, input.output_id()),
            Output::Nft(nft_input) => self.transition_nft_input(nft_input, input.output_id()),
            Output::Basic(basic_output) => self.transition_implicit_account_input(basic_output, input.output_id()),
            _ => Ok(None),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AccountChange {
    BeginStaking {
        /// The amount of tokens to stake.
        #[serde(with = "string")]
        staked_amount: u64,
        /// The fixed cost of the validator, which it receives as part of its Mana rewards.
        #[serde(with = "string")]
        fixed_cost: u64,
        /// The staking period (in epochs). Will default to the staking unbonding period.
        staking_period: Option<u32>,
    },
    ExtendStaking {
        additional_epochs: u32,
    },
    EndStaking,
    ModifyBlockIssuerKeys {
        /// The keys that will be added.
        keys_to_add: Vec<BlockIssuerKey>,
        /// The keys that will be removed.
        keys_to_remove: Vec<BlockIssuerKey>,
    },
}

/// A type to specify intended transitions.
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transitions {
    /// Implicit accounts to transition.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub(crate) implicit_accounts: HashMap<OutputId, BlockIssuerKey>,
    /// Accounts to transition.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub(crate) accounts: HashMap<AccountId, AccountChange>,
}

impl Transitions {
    /// Creates a new set of transitions.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an implicit account to transition.
    pub fn add_implicit_account(mut self, output_id: OutputId, block_issuer_key: BlockIssuerKey) -> Self {
        self.implicit_accounts.insert(output_id, block_issuer_key);
        self
    }

    /// Sets the implicit accounts to transition.
    pub fn set_implicit_accounts(mut self, implicit_accounts: HashMap<OutputId, BlockIssuerKey>) -> Self {
        self.implicit_accounts = implicit_accounts;
        self
    }

    /// Returns the implicit accounts to transition.
    pub fn implicit_accounts(&self) -> &HashMap<OutputId, BlockIssuerKey> {
        &self.implicit_accounts
    }

    /// Adds an account to transition.
    pub fn add_account(mut self, account_id: AccountId, change: AccountChange) -> Self {
        self.accounts.insert(account_id, change);
        self
    }

    /// Sets the accounts to transition.
    pub fn set_accounts(mut self, accounts: HashMap<AccountId, AccountChange>) -> Self {
        self.accounts = accounts;
        self
    }

    /// Returns the accounts to transition.
    pub fn accounts(&self) -> &HashMap<AccountId, AccountChange> {
        &self.accounts
    }
}
