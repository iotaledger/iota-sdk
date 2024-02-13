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
    output::{feature::Features, AccountId, AnchorOutput, ChainId, FoundryId, Output, OutputId, TokenId},
    payload::signed_transaction::{Transaction, TransactionCapabilityFlag, TransactionId, TransactionSigningHash},
    protocol::ProtocolParameters,
    slot::SlotCommitmentId,
    unlock::Unlock,
    Error,
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
    pub(crate) mana_rewards: BTreeMap<OutputId, u64>,
    pub(crate) commitment_context_input: Option<SlotCommitmentId>,
    pub(crate) bic_context_inputs: HashSet<AccountId>,
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
    pub(crate) protocol_parameters: ProtocolParameters,
}

impl<'a> SemanticValidationContext<'a> {
    ///
    pub fn new(
        transaction: &'a Transaction,
        inputs: &'a [(&'a OutputId, &'a Output)],
        unlocks: Option<&'a [Unlock]>,
        mana_rewards: BTreeMap<OutputId, u64>,
        protocol_parameters: ProtocolParameters,
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
            commitment_context_input: None,
            bic_context_inputs: Default::default(),
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
    pub fn validate(mut self) -> Result<Option<TransactionFailureReason>, Error> {
        self.commitment_context_input = self
            .transaction
            .context_inputs()
            .commitment()
            .map(|c| c.slot_commitment_id());

        self.bic_context_inputs = self
            .transaction
            .context_inputs()
            .iter()
            .filter_map(|c| c.as_block_issuance_credit_opt().map(|bic| *bic.account_id()))
            .collect();

        for reward_context_input in self.transaction.context_inputs().rewards() {
            if let Some(output_id) = self.inputs.get(reward_context_input.index() as usize).map(|v| v.0) {
                self.reward_context_inputs.insert(*output_id, *reward_context_input);
            } else {
                return Ok(Some(TransactionFailureReason::RewardInputReferenceInvalid));
            }
        }

        // Validation of inputs.

        let mut has_implicit_account_creation_address = false;

        for (index, (output_id, consumed_output)) in self.inputs.iter().enumerate() {
            let (amount, consumed_native_token, unlock_conditions) = match consumed_output {
                Output::Basic(output) => (output.amount(), output.native_token(), output.unlock_conditions()),
                Output::Account(output) => {
                    if output.features().block_issuer().is_some() {
                        let account_id = output.account_id_non_null(&output_id);

                        if self.commitment_context_input.is_none() {
                            return Ok(Some(TransactionFailureReason::BlockIssuerCommitmentInputMissing));
                        }
                        if !self.bic_context_inputs.contains(&account_id) {
                            return Ok(Some(TransactionFailureReason::BlockIssuanceCreditInputMissing));
                        }
                        let entry = self.block_issuer_mana.entry(account_id).or_default();
                        entry.0 = entry
                            .0
                            .checked_add(consumed_output.available_mana(
                                &self.protocol_parameters,
                                output_id.transaction_id().slot_index(),
                                self.transaction.creation_slot(),
                            )?)
                            .ok_or(Error::ConsumedManaOverflow)?;
                    }

                    (output.amount(), None, output.unlock_conditions())
                }
                Output::Anchor(_) => return Err(Error::UnsupportedOutputKind(AnchorOutput::KIND)),
                Output::Foundry(output) => (output.amount(), output.native_token(), output.unlock_conditions()),
                Output::Nft(output) => (output.amount(), None, output.unlock_conditions()),
                Output::Delegation(output) => (output.amount(), None, output.unlock_conditions()),
            };

            if unlock_conditions.addresses().any(Address::is_implicit_account_creation) {
                if has_implicit_account_creation_address {
                    return Ok(Some(TransactionFailureReason::MultipleImplicitAccountCreationAddresses));
                } else {
                    has_implicit_account_creation_address = true;
                }
            }

            let commitment_slot_index = self.commitment_context_input.map(|c| c.slot_index());

            if let Some(timelock) = unlock_conditions.timelock() {
                if let Some(commitment_slot_index) = commitment_slot_index {
                    if timelock.is_timelocked(commitment_slot_index, self.protocol_parameters.min_committable_age()) {
                        return Ok(Some(TransactionFailureReason::TimelockNotExpired));
                    }
                } else {
                    return Ok(Some(TransactionFailureReason::TimelockCommitmentInputMissing));
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
                                    .ok_or(Error::StorageDepositReturnOverflow)?;
                            }
                        }
                        None => return Ok(Some(TransactionFailureReason::ExpirationNotUnlockable)),
                        _ => {}
                    }
                } else {
                    return Ok(Some(TransactionFailureReason::ExpirationCommitmentInputMissing));
                }
            }

            self.input_amount = self
                .input_amount
                .checked_add(amount)
                .ok_or(Error::ConsumedAmountOverflow)?;

            self.input_mana = self
                .input_mana
                .checked_add(consumed_output.available_mana(
                    &self.protocol_parameters,
                    output_id.transaction_id().slot_index(),
                    self.transaction.creation_slot(),
                )?)
                .ok_or(Error::ConsumedManaOverflow)?;

            if let Some(mana_rewards) = self.mana_rewards.get(*output_id) {
                self.input_mana
                    .checked_add(*mana_rewards)
                    .ok_or(Error::ConsumedManaOverflow)?;
            }

            if let Some(consumed_native_token) = consumed_native_token {
                let native_token_amount = self
                    .input_native_tokens
                    .entry(*consumed_native_token.token_id())
                    .or_default();

                *native_token_amount = native_token_amount
                    .checked_add(consumed_native_token.amount())
                    .ok_or(Error::ConsumedNativeTokensAmountOverflow)?;
            }

            if let Some(unlocks) = self.unlocks {
                if unlocks.len() != self.inputs.len() {
                    return Ok(Some(TransactionFailureReason::SemanticValidationFailed));
                }

                if let Err(conflict) = self.output_unlock(consumed_output, output_id, &unlocks[index]) {
                    return Ok(Some(conflict));
                }
            }
        }

        // Validation of outputs.
        for (index, created_output) in self.transaction.outputs().iter().enumerate() {
            let (amount, mana, created_native_token, features) = match created_output {
                Output::Basic(output) => {
                    if let Some(address) = output.simple_deposit_address() {
                        let amount = self.simple_deposits.entry(address.clone()).or_default();

                        *amount = amount
                            .checked_add(output.amount())
                            .ok_or(Error::CreatedAmountOverflow)?;
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

                        if self.commitment_context_input.is_none() {
                            return Ok(Some(TransactionFailureReason::BlockIssuerCommitmentInputMissing));
                        }
                        if !self.bic_context_inputs.contains(&account_id) {
                            return Ok(Some(TransactionFailureReason::BlockIssuanceCreditInputMissing));
                        }
                        let entry = self.block_issuer_mana.entry(account_id).or_default();

                        entry.1 = entry.1.checked_add(output.mana()).ok_or(Error::CreatedManaOverflow)?;

                        if let Some(allotment) = self.transaction.allotments().get(&account_id) {
                            entry.1 = entry
                                .1
                                .checked_add(allotment.mana())
                                .ok_or(Error::CreatedManaOverflow)?;
                        }
                    }

                    (output.amount(), output.mana(), None, Some(output.features()))
                }
                Output::Anchor(_) => return Err(Error::UnsupportedOutputKind(AnchorOutput::KIND)),
                Output::Foundry(output) => (output.amount(), 0, output.native_token(), Some(output.features())),
                Output::Nft(output) => (output.amount(), output.mana(), None, Some(output.features())),
                Output::Delegation(output) => (output.amount(), 0, None, None),
            };

            if let Some(sender) = features.and_then(Features::sender) {
                if !self.unlocked_addresses.contains(sender.address()) {
                    return Ok(Some(TransactionFailureReason::SenderFeatureNotUnlocked));
                }
            }

            // TODO: should be done only towards block issuers
            if let Some(unlock_conditions) = created_output.unlock_conditions() {
                if let (Some(address), Some(timelock)) = (unlock_conditions.address(), unlock_conditions.timelock()) {
                    if let Address::Account(account_address) = address.address() {
                        let past_bounded_slot_index = self
                            .protocol_parameters
                            .past_bounded_slot(self.commitment_context_input.unwrap());

                        if timelock.slot_index()
                            >= past_bounded_slot_index + self.protocol_parameters.max_committable_age()
                        {
                            let entry = self.block_issuer_mana.entry(*account_address.account_id()).or_default();
                            entry.1 = entry
                                .1
                                .checked_add(created_output.mana())
                                .ok_or(Error::CreatedAmountOverflow)?;
                        }
                    }
                }
            }

            self.output_amount = self
                .output_amount
                .checked_add(amount)
                .ok_or(Error::CreatedAmountOverflow)?;

            // Add stored mana
            self.output_mana = self.output_mana.checked_add(mana).ok_or(Error::CreatedManaOverflow)?;

            // Add allotted mana
            for mana_allotment in (*self.transaction.allotments()).iter() {
                self.output_mana = self
                    .output_mana
                    .checked_add(mana_allotment.mana())
                    .ok_or(Error::CreatedManaOverflow)?;
            }

            if let Some(created_native_token) = created_native_token {
                let native_token_amount = self
                    .output_native_tokens
                    .entry(*created_native_token.token_id())
                    .or_default();

                *native_token_amount = native_token_amount
                    .checked_add(created_native_token.amount())
                    // TODO should be a tx failure reason ?
                    .ok_or(Error::CreatedNativeTokensAmountOverflow)?;
            }
        }

        // Validation of storage deposit returns.
        for (return_address, return_amount) in self.storage_deposit_returns.iter() {
            if let Some(deposit_amount) = self.simple_deposits.get(return_address) {
                if deposit_amount < return_amount {
                    return Ok(Some(TransactionFailureReason::ReturnAmountNotFulFilled));
                }
            } else {
                return Ok(Some(TransactionFailureReason::ReturnAmountNotFulFilled));
            }
        }

        // Validation of amounts.
        if self.input_amount != self.output_amount {
            return Ok(Some(TransactionFailureReason::InputOutputBaseTokenMismatch));
        }

        if self.input_mana != self.output_mana {
            if self.input_mana > self.output_mana {
                if !self.transaction.has_capability(TransactionCapabilityFlag::BurnMana) {
                    return Ok(Some(TransactionFailureReason::CapabilitiesManaBurningNotAllowed));
                }
            } else {
                return Ok(Some(TransactionFailureReason::InputOutputManaMismatch));
            }
        }

        for (_, (account_input_mana, account_output_mana)) in &self.block_issuer_mana {
            if self.input_mana - account_input_mana < self.output_mana - account_output_mana {
                return Ok(Some(TransactionFailureReason::ManaMovedOffBlockIssuerAccount));
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
                return Ok(Some(TransactionFailureReason::NativeTokenSumUnbalanced));
            }
        }

        // Validation of state transitions and destructions.
        for (chain_id, current_state) in self.input_chains.iter() {
            if let Err(e) = self.verify_state_transition(
                Some(*current_state),
                self.output_chains.get(chain_id).map(|(id, o)| (id, *o)),
            ) {
                return Ok(Some(e));
            }
        }

        // Validation of state creations.
        for (chain_id, next_state) in self.output_chains.iter() {
            if self.input_chains.get(chain_id).is_none() {
                if let Err(e) = self.verify_state_transition(None, Some((&next_state.0, next_state.1))) {
                    return Ok(Some(e));
                }
            }
        }

        Ok(None)
    }
}
