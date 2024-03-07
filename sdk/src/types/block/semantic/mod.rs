// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod error;
mod state_transition;
mod unlock;

use alloc::collections::BTreeMap;

use hashbrown::{HashMap, HashSet};
use primitive_types::U256;

pub use self::{error::TransactionFailureReason, state_transition::StateTransitionVerifier};
use crate::types::block::{
    address::Address,
    context_input::RewardContextInput,
    output::{feature::Features, AccountId, ChainId, FoundryId, Output, OutputId, TokenId},
    payload::signed_transaction::{Transaction, TransactionCapabilityFlag, TransactionId, TransactionSigningHash},
    protocol::ProtocolParameters,
    slot::SlotCommitmentId,
    unlock::Unlock,
};

///
pub struct SemanticValidationContext<'a> {
    pub(crate) transaction: &'a Transaction,
    pub(crate) transaction_id: TransactionId,
    pub(crate) transaction_signing_hash: TransactionSigningHash,
    pub(crate) inputs: &'a [(&'a OutputId, &'a Output)],
    pub(crate) unlocks: Option<&'a [Unlock]>,
    pub(crate) input_amount: u64,
    pub(crate) input_mana: u64,
    pub(crate) mana_rewards: Option<&'a BTreeMap<OutputId, u64>>,
    pub(crate) commitment_context_input: Option<SlotCommitmentId>,
    pub(crate) reward_context_inputs: HashMap<OutputId, RewardContextInput>,
    pub(crate) input_native_tokens: BTreeMap<TokenId, U256>,
    pub(crate) input_chains: HashMap<ChainId, (&'a OutputId, &'a Output)>,
    pub(crate) output_amount: u64,
    pub(crate) output_mana: u64,
    pub(crate) output_native_tokens: BTreeMap<TokenId, U256>,
    pub(crate) output_chains: HashMap<ChainId, (OutputId, &'a Output)>,
    pub(crate) block_issuer_mana: HashMap<AccountId, (u64, u64)>,
    pub(crate) unlocked_addresses: HashSet<Address>,
    pub(crate) storage_deposit_returns: HashMap<Address, u64>,
    pub(crate) simple_deposits: HashMap<Address, u64>,
    pub(crate) protocol_parameters: &'a ProtocolParameters,
}

impl<'a> SemanticValidationContext<'a> {
    ///
    pub fn new(
        transaction: &'a Transaction,
        inputs: &'a [(&'a OutputId, &'a Output)],
        unlocks: Option<&'a [Unlock]>,
        mana_rewards: Option<&'a BTreeMap<OutputId, u64>>,
        protocol_parameters: &'a ProtocolParameters,
    ) -> Self {
        let transaction_id = transaction.id();
        let input_chains = inputs
            .iter()
            .filter_map(|(output_id, input)| {
                if input.is_implicit_account() {
                    Some((ChainId::from(AccountId::from(*output_id)), (*output_id, *input)))
                } else {
                    input
                        .chain_id()
                        .map(|chain_id| (chain_id.or_from_output_id(output_id), (*output_id, *input)))
                }
            })
            .collect();
        let output_chains = transaction
            .outputs()
            .iter()
            .enumerate()
            .filter_map(|(index, output)| {
                output.chain_id().map(|chain_id| {
                    let output_id = OutputId::new(transaction_id, index as u16);
                    (chain_id.or_from_output_id(&output_id), (output_id, output))
                })
            })
            .collect();

        Self {
            transaction,
            transaction_id,
            transaction_signing_hash: transaction.signing_hash(),
            inputs,
            unlocks,
            input_amount: 0,
            input_mana: 0,
            mana_rewards,
            commitment_context_input: transaction
                .context_inputs()
                .commitment()
                .map(|c| c.slot_commitment_id()),
            reward_context_inputs: Default::default(),
            input_native_tokens: BTreeMap::<TokenId, U256>::new(),
            input_chains,
            output_amount: 0,
            output_mana: 0,
            output_native_tokens: BTreeMap::<TokenId, U256>::new(),
            output_chains,
            block_issuer_mana: HashMap::new(),
            unlocked_addresses: HashSet::new(),
            storage_deposit_returns: HashMap::new(),
            simple_deposits: HashMap::new(),
            protocol_parameters,
        }
    }

    ///
    pub fn validate(mut self) -> Result<(), TransactionFailureReason> {
        self.validate_reward_context_inputs()?;

        self.validate_inputs()?;

        self.validate_outputs()?;

        self.validate_storage_deposit_returns()?;

        self.validate_balances()?;

        self.validate_transitions()?;

        Ok(())
    }

    fn validate_reward_context_inputs(&mut self) -> Result<(), TransactionFailureReason> {
        for reward_context_input in self.transaction.context_inputs().rewards() {
            if let Some(output_id) = self.inputs.get(reward_context_input.index() as usize).map(|v| v.0) {
                self.reward_context_inputs.insert(*output_id, *reward_context_input);
            } else {
                return Err(TransactionFailureReason::RewardInputReferenceInvalid);
            }
        }
        Ok(())
    }

    fn validate_inputs(&mut self) -> Result<(), TransactionFailureReason> {
        let bic_context_inputs = self
            .transaction
            .context_inputs()
            .block_issuance_credits()
            .map(|bic| *bic.account_id())
            .collect::<HashSet<_>>();

        let mut has_implicit_account_creation_address = false;

        for (index, (output_id, consumed_output)) in self.inputs.iter().enumerate() {
            if output_id.transaction_id().slot_index() > self.transaction.creation_slot() {
                return Err(TransactionFailureReason::InputCreationAfterTxCreation);
            }

            let (amount, consumed_native_token, unlock_conditions) = match consumed_output {
                Output::Basic(output) => (output.amount(), output.native_token(), output.unlock_conditions()),
                Output::Account(output) => {
                    if output.features().block_issuer().is_some() {
                        let account_id = output.account_id_non_null(output_id);

                        if self.commitment_context_input.is_none() {
                            return Err(TransactionFailureReason::BlockIssuerCommitmentInputMissing);
                        }
                        if !bic_context_inputs.contains(&account_id) {
                            return Err(TransactionFailureReason::BlockIssuanceCreditInputMissing);
                        }
                        let entry = self.block_issuer_mana.entry(account_id).or_default();
                        entry.0 = entry
                            .0
                            .checked_add(
                                consumed_output
                                    .available_mana(
                                        self.protocol_parameters,
                                        output_id.transaction_id().slot_index(),
                                        self.transaction.creation_slot(),
                                    )
                                    // Unwrap is fine as we already checked both slot indices against each others.
                                    .unwrap(),
                            )
                            .ok_or(TransactionFailureReason::ManaOverflow)?;
                    }
                    if output.features().staking().is_some() && self.commitment_context_input.is_none() {
                        return Err(TransactionFailureReason::StakingCommitmentInputMissing);
                    }

                    (output.amount(), None, output.unlock_conditions())
                }
                Output::Anchor(_) => return Err(TransactionFailureReason::SemanticValidationFailed),
                Output::Foundry(output) => (output.amount(), output.native_token(), output.unlock_conditions()),
                Output::Nft(output) => (output.amount(), None, output.unlock_conditions()),
                Output::Delegation(output) => (output.amount(), None, output.unlock_conditions()),
            };

            if unlock_conditions.addresses().any(Address::is_implicit_account_creation) {
                if has_implicit_account_creation_address {
                    return Err(TransactionFailureReason::MultipleImplicitAccountCreationAddresses);
                } else {
                    has_implicit_account_creation_address = true;
                }
            }

            let commitment_slot_index = self.commitment_context_input.map(|c| c.slot_index());

            if let Some(timelock) = unlock_conditions.timelock() {
                if let Some(commitment_slot_index) = commitment_slot_index {
                    if timelock.is_timelocked(commitment_slot_index, self.protocol_parameters.min_committable_age()) {
                        return Err(TransactionFailureReason::TimelockNotExpired);
                    }
                } else {
                    return Err(TransactionFailureReason::TimelockCommitmentInputMissing);
                }
            }

            if let Some(expiration) = unlock_conditions.expiration() {
                if let Some(commitment_slot_index) = commitment_slot_index {
                    match expiration.is_expired(commitment_slot_index, self.protocol_parameters.committable_age_range())
                    {
                        Some(false) => {
                            if let Some(storage_deposit_return) = unlock_conditions.storage_deposit_return() {
                                let amount = self
                                    .storage_deposit_returns
                                    .entry(storage_deposit_return.return_address().clone())
                                    .or_default();

                                *amount = amount
                                    .checked_add(storage_deposit_return.amount())
                                    .ok_or(TransactionFailureReason::SemanticValidationFailed)?;
                            }
                        }
                        None => return Err(TransactionFailureReason::ExpirationNotUnlockable),
                        _ => {}
                    }
                } else {
                    return Err(TransactionFailureReason::ExpirationCommitmentInputMissing);
                }
            }

            self.input_amount = self
                .input_amount
                .checked_add(amount)
                .ok_or(TransactionFailureReason::SemanticValidationFailed)?;

            self.input_mana = self
                .input_mana
                .checked_add(
                    consumed_output
                        .available_mana(
                            self.protocol_parameters,
                            output_id.transaction_id().slot_index(),
                            self.transaction.creation_slot(),
                        )
                        // Unwrap is fine as we already checked both slot indices against each others.
                        .unwrap(),
                )
                .ok_or(TransactionFailureReason::ManaOverflow)?;

            if let Some(mana_rewards) = self.mana_rewards.as_ref().and_then(|r| r.get(*output_id)) {
                self.input_mana = self
                    .input_mana
                    .checked_add(*mana_rewards)
                    .ok_or(TransactionFailureReason::ManaOverflow)?;
            }

            if let Some(consumed_native_token) = consumed_native_token {
                let native_token_amount = self
                    .input_native_tokens
                    .entry(*consumed_native_token.token_id())
                    .or_default();

                *native_token_amount = native_token_amount
                    .checked_add(consumed_native_token.amount())
                    .ok_or(TransactionFailureReason::SemanticValidationFailed)?;
            }

            if let Some(unlocks) = self.unlocks {
                if unlocks.len() != self.inputs.len() {
                    return Err(TransactionFailureReason::SemanticValidationFailed);
                }

                self.output_unlock(consumed_output, output_id, &unlocks[index])?
            }
        }

        Ok(())
    }

    fn validate_outputs(&mut self) -> Result<(), TransactionFailureReason> {
        let bic_context_inputs = self
            .transaction
            .context_inputs()
            .block_issuance_credits()
            .map(|bic| *bic.account_id())
            .collect::<HashSet<_>>();

        // Add allotted mana
        for mana_allotment in self.transaction.allotments().iter() {
            self.output_mana = self
                .output_mana
                .checked_add(mana_allotment.mana())
                .ok_or(TransactionFailureReason::ManaOverflow)?;
        }

        for (index, created_output) in self.transaction.outputs().iter().enumerate() {
            let (amount, mana, created_native_token, features) = match created_output {
                Output::Basic(output) => {
                    if let Some(address) = output.simple_deposit_address() {
                        let amount = self.simple_deposits.entry(address.clone()).or_default();

                        *amount = amount
                            .checked_add(output.amount())
                            .ok_or(TransactionFailureReason::SemanticValidationFailed)?;
                    }

                    (
                        output.amount(),
                        output.mana(),
                        output.native_token(),
                        Some(output.features()),
                    )
                }
                Output::Account(output) => {
                    if output.features().block_issuer().is_some() {
                        let account_id = output.account_id_non_null(&OutputId::new(self.transaction_id, index as u16));

                        if !bic_context_inputs.contains(&account_id) {
                            return Err(TransactionFailureReason::BlockIssuanceCreditInputMissing);
                        }
                        let entry = self.block_issuer_mana.entry(account_id).or_default();

                        entry.1 = entry
                            .1
                            .checked_add(output.mana())
                            .ok_or(TransactionFailureReason::ManaOverflow)?;

                        if let Some(allotment) = self.transaction.allotments().get(&account_id) {
                            entry.1 = entry
                                .1
                                .checked_add(allotment.mana())
                                .ok_or(TransactionFailureReason::ManaOverflow)?;
                        }
                    }

                    (output.amount(), output.mana(), None, Some(output.features()))
                }
                Output::Anchor(_) => return Err(TransactionFailureReason::SemanticValidationFailed),
                Output::Foundry(output) => (output.amount(), 0, output.native_token(), Some(output.features())),
                Output::Nft(output) => (output.amount(), output.mana(), None, Some(output.features())),
                Output::Delegation(output) => (output.amount(), 0, None, None),
            };

            if self.unlocks.is_some() {
                if let Some(sender) = features.and_then(Features::sender) {
                    if !self.unlocked_addresses.contains(sender.address()) {
                        return Err(TransactionFailureReason::SenderFeatureNotUnlocked);
                    }
                }
            }

            if let Some(unlock_conditions) = created_output.unlock_conditions() {
                if let (Some(address), Some(timelock)) = (unlock_conditions.address(), unlock_conditions.timelock()) {
                    if let Address::Account(account_address) = address.address() {
                        if let Some(entry) = self.block_issuer_mana.get_mut(account_address.account_id()) {
                            if let Some(commitment_context_input) = self.commitment_context_input {
                                let past_bounded_slot =
                                    self.protocol_parameters.past_bounded_slot(commitment_context_input);

                                if timelock.slot_index()
                                    >= past_bounded_slot + self.protocol_parameters.max_committable_age()
                                {
                                    entry.1 = entry
                                        .1
                                        .checked_add(created_output.mana())
                                        .ok_or(TransactionFailureReason::SemanticValidationFailed)?;
                                }
                            } else {
                                return Err(TransactionFailureReason::BlockIssuerCommitmentInputMissing);
                            }
                        }
                    }
                }
            }

            self.output_amount = self
                .output_amount
                .checked_add(amount)
                .ok_or(TransactionFailureReason::SemanticValidationFailed)?;

            // Add stored mana
            self.output_mana = self
                .output_mana
                .checked_add(mana)
                .ok_or(TransactionFailureReason::ManaOverflow)?;

            if let Some(created_native_token) = created_native_token {
                let native_token_amount = self
                    .output_native_tokens
                    .entry(*created_native_token.token_id())
                    .or_default();

                *native_token_amount = native_token_amount
                    .checked_add(created_native_token.amount())
                    .ok_or(TransactionFailureReason::SemanticValidationFailed)?;
            }
        }
        Ok(())
    }

    fn validate_storage_deposit_returns(&mut self) -> Result<(), TransactionFailureReason> {
        for (return_address, return_amount) in self.storage_deposit_returns.iter() {
            if let Some(deposit_amount) = self.simple_deposits.get(return_address) {
                if deposit_amount < return_amount {
                    return Err(TransactionFailureReason::ReturnAmountNotFulFilled);
                }
            } else {
                return Err(TransactionFailureReason::ReturnAmountNotFulFilled);
            }
        }
        Ok(())
    }

    fn validate_balances(&mut self) -> Result<(), TransactionFailureReason> {
        // Validation of amounts
        if self.input_amount != self.output_amount {
            return Err(TransactionFailureReason::InputOutputBaseTokenMismatch);
        }

        if self.input_mana != self.output_mana {
            if self.input_mana > self.output_mana {
                if !self.transaction.has_capability(TransactionCapabilityFlag::BurnMana) {
                    return Err(TransactionFailureReason::CapabilitiesManaBurningNotAllowed);
                }
            } else if self.mana_rewards.is_some() || self.reward_context_inputs.is_empty() {
                return Err(TransactionFailureReason::InputOutputManaMismatch);
            }
        }

        for (account_input_mana, account_output_mana) in self.block_issuer_mana.values() {
            if self.input_mana - account_input_mana < self.output_mana - account_output_mana {
                return Err(TransactionFailureReason::ManaMovedOffBlockIssuerAccount);
            }
        }

        // Validation of output native tokens.
        for (token_id, output_amount) in self.output_native_tokens.iter() {
            let input_amount = self.input_native_tokens.get(token_id).copied().unwrap_or_default();

            if output_amount > &input_amount
                && !self
                    .output_chains
                    .contains_key(&ChainId::from(FoundryId::from(*token_id)))
            {
                return Err(TransactionFailureReason::NativeTokenSumUnbalanced);
            }
        }
        Ok(())
    }

    fn validate_transitions(&mut self) -> Result<(), TransactionFailureReason> {
        // Validation of state transitions and destructions.
        for (chain_id, current_state) in self.input_chains.iter() {
            self.verify_state_transition(
                Some(*current_state),
                self.output_chains.get(chain_id).map(|(id, o)| (id, *o)),
            )?;
        }

        // Validation of state creations.
        for (chain_id, next_state) in self.output_chains.iter() {
            if self.input_chains.get(chain_id).is_none() {
                self.verify_state_transition(None, Some((&next_state.0, next_state.1)))?;
            }
        }
        Ok(())
    }
}
