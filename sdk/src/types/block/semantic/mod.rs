// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod error;
mod state_transition;
mod unlock;

use alloc::collections::BTreeMap;

use hashbrown::{HashMap, HashSet};
use primitive_types::U256;

pub use self::{
    error::TransactionFailureReason,
    state_transition::{StateTransitionError, StateTransitionVerifier},
};
use crate::types::block::{
    address::{Address, AddressCapabilityFlag},
    output::{AccountId, AnchorOutput, ChainId, FoundryId, NativeTokens, Output, OutputId, TokenId, UnlockCondition},
    payload::signed_transaction::{Transaction, TransactionCapabilityFlag, TransactionSigningHash},
    protocol::ProtocolParameters,
    unlock::Unlock,
    Error,
};

///
pub struct SemanticValidationContext<'a> {
    pub(crate) transaction: &'a Transaction,
    pub(crate) transaction_signing_hash: TransactionSigningHash,
    pub(crate) inputs: &'a [(&'a OutputId, &'a Output)],
    pub(crate) unlocks: Option<&'a [Unlock]>,
    pub(crate) input_amount: u64,
    pub(crate) input_mana: u64,
    pub(crate) input_native_tokens: BTreeMap<TokenId, U256>,
    pub(crate) input_chains: HashMap<ChainId, &'a Output>,
    pub(crate) output_amount: u64,
    pub(crate) output_mana: u64,
    pub(crate) output_native_tokens: BTreeMap<TokenId, U256>,
    pub(crate) output_chains: HashMap<ChainId, &'a Output>,
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
        protocol_parameters: ProtocolParameters,
    ) -> Self {
        let transaction_id = transaction.id();
        let input_chains = inputs
            .iter()
            .filter_map(|(output_id, input)| {
                if input.is_implicit_account() {
                    Some((ChainId::from(AccountId::from(*output_id)), *input))
                } else {
                    input
                        .chain_id()
                        .map(|chain_id| (chain_id.or_from_output_id(output_id), *input))
                }
            })
            .collect();
        let output_chains = transaction
            .outputs()
            .iter()
            .enumerate()
            .filter_map(|(index, output)| {
                output.chain_id().map(|chain_id| {
                    (
                        chain_id.or_from_output_id(&OutputId::new(transaction_id, index as u16)),
                        output,
                    )
                })
            })
            .collect();

        Self {
            transaction,
            transaction_signing_hash: transaction.signing_hash(),
            inputs,
            unlocks,
            input_amount: 0,
            input_mana: 0,
            input_native_tokens: BTreeMap::<TokenId, U256>::new(),
            input_chains,
            output_amount: 0,
            output_mana: 0,
            output_native_tokens: BTreeMap::<TokenId, U256>::new(),
            output_chains,
            unlocked_addresses: HashSet::new(),
            storage_deposit_returns: HashMap::new(),
            simple_deposits: HashMap::new(),
            protocol_parameters,
        }
    }

    ///
    pub fn validate(mut self) -> Result<Option<TransactionFailureReason>, Error> {
        // Validation of inputs.
        for (index, (output_id, consumed_output)) in self.inputs.iter().enumerate() {
            let (amount, mana, consumed_native_token, unlock_conditions) = match consumed_output {
                Output::Basic(output) => (
                    output.amount(),
                    output.mana(),
                    output.native_token(),
                    output.unlock_conditions(),
                ),
                Output::Account(output) => (output.amount(), output.mana(), None, output.unlock_conditions()),
                Output::Anchor(_) => return Err(Error::UnsupportedOutputKind(AnchorOutput::KIND)),
                Output::Foundry(output) => (output.amount(), 0, output.native_token(), output.unlock_conditions()),
                Output::Nft(output) => (output.amount(), output.mana(), None, output.unlock_conditions()),
                Output::Delegation(output) => (output.amount(), 0, None, output.unlock_conditions()),
            };

            let commitment_slot_index = self
                .transaction
                .context_inputs()
                .iter()
                .find_map(|c| c.as_commitment_opt().map(|c| c.slot_index()));

            if let Some(timelock) = unlock_conditions.timelock() {
                if let Some(commitment_slot_index) = commitment_slot_index {
                    if timelock.is_timelocked(commitment_slot_index, self.protocol_parameters.min_committable_age()) {
                        return Ok(Some(TransactionFailureReason::TimelockNotExpired));
                    }
                } else {
                    // Missing CommitmentContextInput
                    return Ok(Some(TransactionFailureReason::InvalidCommitmentContextInput));
                }
            }

            if let Some(expiration) = unlock_conditions.expiration() {
                if let Some(commitment_slot_index) = commitment_slot_index {
                    if expiration.is_expired(commitment_slot_index, self.protocol_parameters.committable_age_range())
                        == Some(false)
                    {
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
                } else {
                    // Missing CommitmentContextInput
                    return Ok(Some(TransactionFailureReason::InvalidCommitmentContextInput));
                }
            }

            self.input_amount = self
                .input_amount
                .checked_add(amount)
                .ok_or(Error::ConsumedAmountOverflow)?;

            self.input_mana = self.input_mana.checked_add(mana).ok_or(Error::ConsumedManaOverflow)?;

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
                    return Ok(Some(TransactionFailureReason::InvalidInputUnlock));
                }

                if let Err(conflict) = self.output_unlock(consumed_output, output_id, &unlocks[index]) {
                    return Ok(Some(conflict));
                }
            }
        }

        // Validation of outputs.
        for created_output in self.transaction.outputs() {
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
                Output::Account(output) => (output.amount(), output.mana(), None, Some(output.features())),
                Output::Anchor(_) => return Err(Error::UnsupportedOutputKind(AnchorOutput::KIND)),
                Output::Foundry(output) => (output.amount(), 0, output.native_token(), Some(output.features())),
                Output::Nft(output) => (output.amount(), output.mana(), None, Some(output.features())),
                Output::Delegation(output) => (output.amount(), 0, None, None),
            };

            if let Some(unlock_conditions) = created_output.unlock_conditions() {
                // Check the possibly restricted address-containing conditions
                let addresses = unlock_conditions
                    .iter()
                    .filter_map(|uc| match uc {
                        UnlockCondition::Address(uc) => Some(uc.address()),
                        UnlockCondition::Expiration(uc) => Some(uc.return_address()),
                        UnlockCondition::StateControllerAddress(uc) => Some(uc.address()),
                        UnlockCondition::GovernorAddress(uc) => Some(uc.address()),
                        _ => None,
                    })
                    .filter_map(Address::as_restricted_opt);
                for address in addresses {
                    if created_native_token.is_some()
                        && !address.has_capability(AddressCapabilityFlag::OutputsWithNativeTokens)
                    {
                        // TODO: add a variant https://github.com/iotaledger/iota-sdk/issues/1430
                        return Ok(Some(TransactionFailureReason::SemanticValidationFailed));
                    }

                    if mana > 0 && !address.has_capability(AddressCapabilityFlag::OutputsWithMana) {
                        // TODO: add a variant https://github.com/iotaledger/iota-sdk/issues/1430
                        return Ok(Some(TransactionFailureReason::SemanticValidationFailed));
                    }

                    if unlock_conditions.timelock().is_some()
                        && !address.has_capability(AddressCapabilityFlag::OutputsWithTimelock)
                    {
                        // TODO: add a variant https://github.com/iotaledger/iota-sdk/issues/1430
                        return Ok(Some(TransactionFailureReason::SemanticValidationFailed));
                    }

                    if unlock_conditions.expiration().is_some()
                        && !address.has_capability(AddressCapabilityFlag::OutputsWithExpiration)
                    {
                        // TODO: add a variant https://github.com/iotaledger/iota-sdk/issues/1430
                        return Ok(Some(TransactionFailureReason::SemanticValidationFailed));
                    }

                    if unlock_conditions.storage_deposit_return().is_some()
                        && !address.has_capability(AddressCapabilityFlag::OutputsWithStorageDepositReturn)
                    {
                        // TODO: add a variant https://github.com/iotaledger/iota-sdk/issues/1430
                        return Ok(Some(TransactionFailureReason::SemanticValidationFailed));
                    }

                    if match &created_output {
                        Output::Account(_) => !address.has_capability(AddressCapabilityFlag::AccountOutputs),
                        Output::Anchor(_) => !address.has_capability(AddressCapabilityFlag::AnchorOutputs),
                        Output::Nft(_) => !address.has_capability(AddressCapabilityFlag::NftOutputs),
                        Output::Delegation(_) => !address.has_capability(AddressCapabilityFlag::DelegationOutputs),
                        _ => false,
                    } {
                        // TODO: add a variant https://github.com/iotaledger/iota-sdk/issues/1430
                        return Ok(Some(TransactionFailureReason::SemanticValidationFailed));
                    }
                }
            }

            if let Some(sender) = features.and_then(|f| f.sender()) {
                if !self.unlocked_addresses.contains(sender.address()) {
                    return Ok(Some(TransactionFailureReason::SenderNotUnlocked));
                }
            }

            self.output_amount = self
                .output_amount
                .checked_add(amount)
                .ok_or(Error::CreatedAmountOverflow)?;

            self.output_mana = self.output_mana.checked_add(mana).ok_or(Error::CreatedManaOverflow)?;

            if let Some(created_native_token) = created_native_token {
                let native_token_amount = self
                    .output_native_tokens
                    .entry(*created_native_token.token_id())
                    .or_default();

                *native_token_amount = native_token_amount
                    .checked_add(created_native_token.amount())
                    .ok_or(Error::CreatedNativeTokensAmountOverflow)?;
            }
        }

        // Validation of storage deposit returns.
        for (return_address, return_amount) in self.storage_deposit_returns.iter() {
            if let Some(deposit_amount) = self.simple_deposits.get(return_address) {
                if deposit_amount < return_amount {
                    return Ok(Some(TransactionFailureReason::StorageDepositReturnUnfulfilled));
                }
            } else {
                return Ok(Some(TransactionFailureReason::StorageDepositReturnUnfulfilled));
            }
        }

        // Validation of amounts.
        if self.input_amount != self.output_amount {
            return Ok(Some(TransactionFailureReason::SumInputsOutputsAmountMismatch));
        }

        if self.input_mana > self.output_mana && !self.transaction.has_capability(TransactionCapabilityFlag::BurnMana) {
            // TODO: add a variant https://github.com/iotaledger/iota-sdk/issues/1430
            return Ok(Some(TransactionFailureReason::SemanticValidationFailed));
        }

        // Validation of input native tokens.
        let mut native_token_ids = self.input_native_tokens.keys().collect::<HashSet<_>>();

        // Validation of output native tokens.
        for (token_id, output_amount) in self.output_native_tokens.iter() {
            let input_amount = self.input_native_tokens.get(token_id).copied().unwrap_or_default();

            if output_amount > &input_amount
                && !self
                    .output_chains
                    .contains_key(&ChainId::from(FoundryId::from(*token_id)))
            {
                return Ok(Some(TransactionFailureReason::InvalidNativeTokens));
            }

            native_token_ids.insert(token_id);
        }

        if native_token_ids.len() > NativeTokens::COUNT_MAX as usize {
            return Ok(Some(TransactionFailureReason::InvalidNativeTokens));
        }

        // Validation of state transitions and destructions.
        for (chain_id, current_state) in self.input_chains.iter() {
            match self.verify_state_transition(
                Some(current_state),
                self.output_chains.get(chain_id).map(core::ops::Deref::deref),
            ) {
                Err(StateTransitionError::TransactionFailure(f)) => return Ok(Some(f)),
                Err(_) => {
                    return Ok(Some(TransactionFailureReason::InvalidChainStateTransition));
                }
                _ => {}
            }
        }

        // Validation of state creations.
        for (chain_id, next_state) in self.output_chains.iter() {
            if self.input_chains.get(chain_id).is_none() {
                match self.verify_state_transition(None, Some(next_state)) {
                    Err(StateTransitionError::TransactionFailure(f)) => return Ok(Some(f)),
                    Err(_) => {
                        return Ok(Some(TransactionFailureReason::InvalidChainStateTransition));
                    }
                    _ => {}
                }
            }
        }

        Ok(None)
    }
}
