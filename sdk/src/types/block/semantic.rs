// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::collections::BTreeMap;
use core::fmt;

use hashbrown::{HashMap, HashSet};
use primitive_types::U256;

use crate::types::block::{
    address::{Address, AddressCapabilityFlag},
    output::{
        AccountId, AnchorOutput, ChainId, FoundryId, NativeTokens, Output, OutputId, StateTransitionError, TokenId,
        UnlockCondition,
    },
    payload::signed_transaction::{Transaction, TransactionCapabilityFlag, TransactionId, TransactionSigningHash},
    unlock::Unlocks,
    Error,
};

/// Describes the reason of a transaction failure.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, packable::Packable)]
#[cfg_attr(feature = "serde", derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr))]
#[packable(unpack_error = Error)]
#[packable(tag_type = u8, with_error = Error::InvalidTransactionFailureReason)]
#[non_exhaustive]
pub enum TransactionFailureReason {
    /// The referenced UTXO was already spent.
    InputUtxoAlreadySpent = 1,
    /// The transaction is conflicting with another transaction. Conflicting specifically means a double spend
    /// situation that both transaction pass all validation rules, eventually losing one(s) should have this reason.
    ConflictingWithAnotherTx = 2,
    /// The referenced UTXO is invalid.
    InvalidReferencedUtxo = 3,
    /// The transaction is invalid.
    InvalidTransaction = 4,
    /// The sum of the inputs and output base token amount does not match.
    SumInputsOutputsAmountMismatch = 5,
    /// The unlock block signature is invalid.
    InvalidUnlockBlockSignature = 6,
    /// The configured timelock is not yet expired.
    TimelockNotExpired = 7,
    /// The given native tokens are invalid.
    InvalidNativeTokens = 8,
    /// The return amount in a transaction is not fulfilled by the output side.
    StorageDepositReturnUnfulfilled = 9,
    /// An input unlock was invalid.
    InvalidInputUnlock = 10,
    /// The output contains a Sender with an ident (address) which is not unlocked.
    SenderNotUnlocked = 11,
    /// The chain state transition is invalid.
    InvalidChainStateTransition = 12,
    /// The referenced input is created after transaction issuing time.
    InvalidTransactionIssuingTime = 13,
    /// The mana amount is invalid.
    InvalidManaAmount = 14,
    /// The Block Issuance Credits amount is invalid.
    InvalidBlockIssuanceCreditsAmount = 15,
    /// Reward Context Input is invalid.
    InvalidRewardContextInput = 16,
    /// Commitment Context Input is invalid.
    InvalidCommitmentContextInput = 17,
    /// Staking Feature is not provided in account output when claiming rewards.
    MissingStakingFeature = 18,
    /// Failed to claim staking reward.
    FailedToClaimStakingReward = 19,
    /// Failed to claim delegation reward.
    FailedToClaimDelegationReward = 20,
    /// Burning of native tokens is not allowed in the transaction capabilities.
    TransactionCapabilityNativeTokenBurningNotAllowed = 21,
    /// Burning of mana is not allowed in the transaction capabilities.
    TransactionCapabilityManaBurningNotAllowed = 22,
    /// Destruction of accounts is not allowed in the transaction capabilities.
    TransactionCapabilityAccountDestructionNotAllowed = 23,
    /// Destruction of anchors is not allowed in the transaction capabilities.
    TransactionCapabilityAnchorDestructionNotAllowed = 24,
    /// Destruction of foundries is not allowed in the transaction capabilities.
    TransactionCapabilityFoundryDestructionNotAllowed = 25,
    /// Destruction of nfts is not allowed in the transaction capabilities.
    TransactionCapabilityNftDestructionNotAllowed = 26,
    /// The semantic validation failed for a reason not covered by the previous variants.
    SemanticValidationFailed = 255,
}

impl fmt::Display for TransactionFailureReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputUtxoAlreadySpent => write!(f, "The referenced UTXO was already spent."),
            Self::ConflictingWithAnotherTx => write!(
                f,
                "The transaction is conflicting with another transaction. Conflicting specifically means a double spend situation that both transactions pass all validation rules, eventually losing one(s) should have this reason."
            ),
            Self::InvalidReferencedUtxo => write!(f, "The referenced UTXO is invalid."),
            Self::InvalidTransaction => write!(f, "The transaction is invalid."),
            Self::SumInputsOutputsAmountMismatch => {
                write!(f, "The sum of the inputs and output base token amount does not match.")
            }
            Self::InvalidUnlockBlockSignature => write!(f, "The unlock block signature is invalid."),
            Self::TimelockNotExpired => write!(f, "The configured timelock is not yet expired."),
            Self::InvalidNativeTokens => write!(f, "The given native tokens are invalid."),
            Self::StorageDepositReturnUnfulfilled => write!(
                f,
                "The return amount in a transaction is not fulfilled by the output side."
            ),
            Self::InvalidInputUnlock => write!(f, "An input unlock was invalid."),
            Self::SenderNotUnlocked => write!(
                f,
                "The output contains a Sender with an ident (address) which is not unlocked."
            ),
            Self::InvalidChainStateTransition => write!(f, "The chain state transition is invalid."),
            Self::InvalidTransactionIssuingTime => {
                write!(f, "The referenced input is created after transaction issuing time.")
            }
            Self::InvalidManaAmount => write!(f, "The mana amount is invalid."),
            Self::InvalidBlockIssuanceCreditsAmount => write!(f, "The Block Issuance Credits amount is invalid."),
            Self::InvalidRewardContextInput => write!(f, "Reward Context Input is invalid."),
            Self::InvalidCommitmentContextInput => write!(f, "Commitment Context Input is invalid."),
            Self::MissingStakingFeature => write!(
                f,
                "Staking Feature is not provided in account output when claiming rewards."
            ),
            Self::FailedToClaimStakingReward => write!(f, "Failed to claim staking reward."),
            Self::FailedToClaimDelegationReward => write!(f, "Failed to claim delegation reward."),
            Self::TransactionCapabilityNativeTokenBurningNotAllowed => write!(
                f,
                "Burning of native tokens is not allowed in the transaction capabilities."
            ),
            Self::TransactionCapabilityManaBurningNotAllowed => {
                write!(f, "Burning of mana is not allowed in the transaction capabilities.")
            }
            Self::TransactionCapabilityAccountDestructionNotAllowed => write!(
                f,
                "Destruction of accounts is not allowed in the transaction capabilities."
            ),
            Self::TransactionCapabilityAnchorDestructionNotAllowed => write!(
                f,
                "Destruction of anchors is not allowed in the transaction capabilities."
            ),
            Self::TransactionCapabilityFoundryDestructionNotAllowed => write!(
                f,
                "Destruction of foundries is not allowed in the transaction capabilities."
            ),
            Self::TransactionCapabilityNftDestructionNotAllowed => {
                write!(f, "Destruction of nfts is not allowed in the transaction capabilities.")
            }
            Self::SemanticValidationFailed => write!(
                f,
                "The semantic validation failed for a reason not covered by the previous variants."
            ),
        }
    }
}

impl TryFrom<u8> for TransactionFailureReason {
    type Error = Error;

    fn try_from(c: u8) -> Result<Self, Self::Error> {
        Ok(match c {
            1 => Self::InputUtxoAlreadySpent,
            2 => Self::ConflictingWithAnotherTx,
            3 => Self::InvalidReferencedUtxo,
            4 => Self::InvalidTransaction,
            5 => Self::SumInputsOutputsAmountMismatch,
            6 => Self::InvalidUnlockBlockSignature,
            7 => Self::TimelockNotExpired,
            8 => Self::InvalidNativeTokens,
            9 => Self::StorageDepositReturnUnfulfilled,
            10 => Self::InvalidInputUnlock,
            11 => Self::SenderNotUnlocked,
            12 => Self::InvalidChainStateTransition,
            13 => Self::InvalidTransactionIssuingTime,
            14 => Self::InvalidManaAmount,
            15 => Self::InvalidBlockIssuanceCreditsAmount,
            16 => Self::InvalidRewardContextInput,
            17 => Self::InvalidCommitmentContextInput,
            18 => Self::MissingStakingFeature,
            19 => Self::FailedToClaimStakingReward,
            20 => Self::FailedToClaimDelegationReward,
            21 => Self::TransactionCapabilityNativeTokenBurningNotAllowed,
            22 => Self::TransactionCapabilityManaBurningNotAllowed,
            23 => Self::TransactionCapabilityAccountDestructionNotAllowed,
            24 => Self::TransactionCapabilityAnchorDestructionNotAllowed,
            25 => Self::TransactionCapabilityFoundryDestructionNotAllowed,
            26 => Self::TransactionCapabilityNftDestructionNotAllowed,
            255 => Self::SemanticValidationFailed,
            x => return Err(Self::Error::InvalidTransactionFailureReason(x)),
        })
    }
}

///
pub struct SemanticValidationContext<'a> {
    pub(crate) transaction: &'a Transaction,
    pub(crate) transaction_signing_hash: TransactionSigningHash,
    pub(crate) inputs: &'a [(&'a OutputId, &'a Output)],
    pub(crate) unlocks: &'a Unlocks,
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
}

impl<'a> SemanticValidationContext<'a> {
    ///
    pub fn new(
        transaction: &'a Transaction,
        transaction_id: &TransactionId,
        inputs: &'a [(&'a OutputId, &'a Output)],
        unlocks: &'a Unlocks,
    ) -> Self {
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
                        chain_id.or_from_output_id(&OutputId::new(*transaction_id, index as u16)),
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
        }
    }

    ///
    pub fn validate(mut self) -> Result<Option<TransactionFailureReason>, Error> {
        // Validation of inputs.
        for ((output_id, consumed_output), unlock) in self.inputs.iter().zip(self.unlocks.iter()) {
            let (conflict, amount, mana, consumed_native_token, unlock_conditions) = match consumed_output {
                Output::Basic(output) => (
                    output.unlock(output_id, unlock, &mut self),
                    output.amount(),
                    output.mana(),
                    output.native_token(),
                    output.unlock_conditions(),
                ),
                Output::Account(output) => (
                    output.unlock(output_id, unlock, &mut self),
                    output.amount(),
                    output.mana(),
                    None,
                    output.unlock_conditions(),
                ),
                Output::Anchor(_) => return Err(Error::UnsupportedOutputKind(AnchorOutput::KIND)),
                Output::Foundry(output) => (
                    output.unlock(output_id, unlock, &mut self),
                    output.amount(),
                    0,
                    output.native_token(),
                    output.unlock_conditions(),
                ),
                Output::Nft(output) => (
                    output.unlock(output_id, unlock, &mut self),
                    output.amount(),
                    output.mana(),
                    None,
                    output.unlock_conditions(),
                ),
                Output::Delegation(output) => (
                    output.unlock(output_id, unlock, &mut self),
                    output.amount(),
                    0,
                    None,
                    output.unlock_conditions(),
                ),
            };

            if let Err(conflict) = conflict {
                return Ok(Some(conflict));
            }

            if unlock_conditions.is_time_locked(self.transaction.creation_slot()) {
                return Ok(Some(TransactionFailureReason::TimelockNotExpired));
            }

            if !unlock_conditions.is_expired(self.transaction.creation_slot()) {
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
            match Output::verify_state_transition(
                Some(current_state),
                self.output_chains.get(chain_id).map(core::ops::Deref::deref),
                &self,
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
                match Output::verify_state_transition(None, Some(next_state), &self) {
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
