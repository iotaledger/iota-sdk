// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::collections::BTreeMap;
use core::{convert::Infallible, fmt};

use hashbrown::{HashMap, HashSet};
use primitive_types::U256;

use crate::types::block::{
    address::Address,
    output::{ChainId, FoundryId, InputsCommitment, NativeTokens, Output, OutputId, TokenId},
    payload::transaction::{RegularTransactionEssence, TransactionEssence, TransactionId},
    unlock::Unlocks,
    Error,
};

/// Errors related to ledger types.
#[derive(Debug)]
pub enum TransactionFailureError {
    /// Invalid transaction failure reason byte.
    InvalidReason(u8),
}

impl fmt::Display for TransactionFailureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidReason(byte) => write!(f, "invalid transaction failure reason byte {byte}"),
        }
    }
}

impl From<Infallible> for TransactionFailureError {
    fn from(err: Infallible) -> Self {
        match err {}
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TransactionFailureError {}

/// Describes the reason of a transaction failure.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, packable::Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[packable(unpack_error = TransactionFailureError)]
#[packable(tag_type = u8, with_error = TransactionFailureError::InvalidReason)]
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
    /// The created amount does not match the consumed amount.
    CreatedConsumedAmountMismatch = 5,
    /// The unlock signature is invalid.
    InvalidSignature = 6,
    /// The configured timelock is not yet expired.
    TimelockNotExpired = 7,
    /// The given native tokens are invalid.
    InvalidNativeTokens = 8,
    /// The return amount in a transaction is not fulfilled by the output side.
    StorageDepositReturnUnfulfilled = 9,
    /// An invalid unlock was used.
    InvalidUnlock = 10,
    /// The inputs commitments do not match.
    InputsCommitmentsMismatch = 11,
    /// The sender was not verified.
    UnverifiedSender = 12,
    /// The chain state transition is invalid.
    InvalidChainStateTransition = 13,
    /// The referenced input is created after transaction issuing time.
    InvalidTransactionIssuingTime = 14,
    /// The mana amount is invalid.
    InvalidManaAmount = 15,
    /// The Block Issuance Credits amount is invalid.
    InvalidBlockIssuanceCreditsAmount = 16,
    /// Reward Input is invalid.
    InvalidRewardInput = 17,
    /// Commitment Input is invalid.
    InvalidCommitmentInput = 18,
    /// Staking Feature is not provided in account output when claiming rewards.
    MissingStakingFeature = 19,
    /// Failed to claim staking reward.
    FailedToClaimStakingReward = 20,
    /// Failed to claim delegation reward.
    FailedToClaimDelegationReward = 21,
    /// The semantic validation failed for a reason not covered by the previous variants.
    SemanticValidationFailed = 255,
}

impl TryFrom<u8> for TransactionFailureReason {
    type Error = TransactionFailureError;

    fn try_from(c: u8) -> Result<Self, Self::Error> {
        Ok(match c {
            1 => Self::InputUtxoAlreadySpent,
            2 => Self::ConflictingWithAnotherTx,
            3 => Self::InvalidReferencedUtxo,
            4 => Self::InvalidTransaction,
            5 => Self::CreatedConsumedAmountMismatch,
            6 => Self::InvalidSignature,
            7 => Self::TimelockNotExpired,
            8 => Self::InvalidNativeTokens,
            9 => Self::StorageDepositReturnUnfulfilled,
            10 => Self::InvalidUnlock,
            11 => Self::InputsCommitmentsMismatch,
            12 => Self::UnverifiedSender,
            13 => Self::InvalidChainStateTransition,
            14 => Self::InvalidTransactionIssuingTime,
            15 => Self::InvalidManaAmount,
            16 => Self::InvalidBlockIssuanceCreditsAmount,
            17 => Self::InvalidRewardInput,
            18 => Self::InvalidCommitmentInput,
            19 => Self::MissingStakingFeature,
            20 => Self::FailedToClaimStakingReward,
            21 => Self::FailedToClaimDelegationReward,
            255 => Self::SemanticValidationFailed,
            x => return Err(Self::Error::InvalidReason(x)),
        })
    }
}

///
pub struct ValidationContext<'a> {
    ///
    pub essence: &'a RegularTransactionEssence,
    ///
    pub essence_hash: [u8; 32],
    ///
    pub inputs_commitment: InputsCommitment,
    ///
    pub unlocks: &'a Unlocks,
    ///
    pub milestone_timestamp: u32,
    ///
    pub input_amount: u64,
    ///
    pub input_native_tokens: BTreeMap<TokenId, U256>,
    ///
    pub input_chains: HashMap<ChainId, &'a Output>,
    ///
    pub output_amount: u64,
    ///
    pub output_native_tokens: BTreeMap<TokenId, U256>,
    ///
    pub output_chains: HashMap<ChainId, &'a Output>,
    ///
    pub unlocked_addresses: HashSet<Address>,
    ///
    pub storage_deposit_returns: HashMap<Address, u64>,
    ///
    pub simple_deposits: HashMap<Address, u64>,
}

impl<'a> ValidationContext<'a> {
    ///
    pub fn new(
        transaction_id: &TransactionId,
        essence: &'a RegularTransactionEssence,
        inputs: impl Iterator<Item = (&'a OutputId, &'a Output)> + Clone,
        unlocks: &'a Unlocks,
        milestone_timestamp: u32,
    ) -> Self {
        Self {
            essence,
            unlocks,
            essence_hash: TransactionEssence::from(essence.clone()).hash(),
            inputs_commitment: InputsCommitment::new(inputs.clone().map(|(_, output)| output)),
            milestone_timestamp,
            input_amount: 0,
            input_native_tokens: BTreeMap::<TokenId, U256>::new(),
            input_chains: inputs
                .filter_map(|(output_id, input)| {
                    input
                        .chain_id()
                        .map(|chain_id| (chain_id.or_from_output_id(output_id), input))
                })
                .collect(),
            output_amount: 0,
            output_native_tokens: BTreeMap::<TokenId, U256>::new(),
            output_chains: essence
                .outputs()
                .iter()
                .enumerate()
                .filter_map(|(index, output)| {
                    output.chain_id().map(|chain_id| {
                        (
                            chain_id.or_from_output_id(&OutputId::new(*transaction_id, index as u16).unwrap()),
                            output,
                        )
                    })
                })
                .collect(),
            unlocked_addresses: HashSet::new(),
            storage_deposit_returns: HashMap::new(),
            simple_deposits: HashMap::new(),
        }
    }
}

///
pub fn semantic_validation(
    mut context: ValidationContext<'_>,
    inputs: &[(&OutputId, &Output)],
    unlocks: &Unlocks,
) -> Result<Option<TransactionFailureReason>, Error> {
    // Validation of the inputs commitment.
    if context.essence.inputs_commitment() != &context.inputs_commitment {
        return Ok(Some(TransactionFailureReason::InputsCommitmentsMismatch));
    }

    // Validation of inputs.
    for ((output_id, consumed_output), unlock) in inputs.iter().zip(unlocks.iter()) {
        let (conflict, amount, consumed_native_tokens, unlock_conditions) = match consumed_output {
            Output::Basic(output) => (
                output.unlock(output_id, unlock, inputs, &mut context),
                output.amount(),
                Some(output.native_tokens()),
                output.unlock_conditions(),
            ),
            Output::Account(output) => (
                output.unlock(output_id, unlock, inputs, &mut context),
                output.amount(),
                Some(output.native_tokens()),
                output.unlock_conditions(),
            ),
            Output::Foundry(output) => (
                output.unlock(output_id, unlock, inputs, &mut context),
                output.amount(),
                Some(output.native_tokens()),
                output.unlock_conditions(),
            ),
            Output::Nft(output) => (
                output.unlock(output_id, unlock, inputs, &mut context),
                output.amount(),
                Some(output.native_tokens()),
                output.unlock_conditions(),
            ),
            Output::Delegation(output) => (
                output.unlock(output_id, unlock, inputs, &mut context),
                output.amount(),
                None,
                output.unlock_conditions(),
            ),
        };

        if let Err(conflict) = conflict {
            return Ok(Some(conflict));
        }

        if unlock_conditions.is_time_locked(context.milestone_timestamp) {
            return Ok(Some(TransactionFailureReason::TimelockNotExpired));
        }

        if !unlock_conditions.is_expired(context.milestone_timestamp) {
            if let Some(storage_deposit_return) = unlock_conditions.storage_deposit_return() {
                let amount = context
                    .storage_deposit_returns
                    .entry(*storage_deposit_return.return_address())
                    .or_default();

                *amount = amount
                    .checked_add(storage_deposit_return.amount())
                    .ok_or(Error::StorageDepositReturnOverflow)?;
            }
        }

        context.input_amount = context
            .input_amount
            .checked_add(amount)
            .ok_or(Error::ConsumedAmountOverflow)?;

        if let Some(consumed_native_tokens) = consumed_native_tokens {
            for native_token in consumed_native_tokens.iter() {
                let native_token_amount = context.input_native_tokens.entry(*native_token.token_id()).or_default();

                *native_token_amount = native_token_amount
                    .checked_add(native_token.amount())
                    .ok_or(Error::ConsumedNativeTokensAmountOverflow)?;
            }
        }
    }

    // Validation of outputs.
    for created_output in context.essence.outputs() {
        let (amount, created_native_tokens, features) = match created_output {
            Output::Basic(output) => {
                if let Some(address) = output.simple_deposit_address() {
                    let amount = context.simple_deposits.entry(*address).or_default();

                    *amount = amount
                        .checked_add(output.amount())
                        .ok_or(Error::CreatedAmountOverflow)?;
                }

                (output.amount(), Some(output.native_tokens()), Some(output.features()))
            }
            Output::Account(output) => (output.amount(), Some(output.native_tokens()), Some(output.features())),
            Output::Foundry(output) => (output.amount(), Some(output.native_tokens()), Some(output.features())),
            Output::Nft(output) => (output.amount(), Some(output.native_tokens()), Some(output.features())),
            Output::Delegation(output) => (output.amount(), None, None),
        };

        if let Some(sender) = features.and_then(|f| f.sender()) {
            if !context.unlocked_addresses.contains(sender.address()) {
                return Ok(Some(TransactionFailureReason::UnverifiedSender));
            }
        }

        context.output_amount = context
            .output_amount
            .checked_add(amount)
            .ok_or(Error::CreatedAmountOverflow)?;

        if let Some(created_native_tokens) = created_native_tokens {
            for native_token in created_native_tokens.iter() {
                let native_token_amount = context
                    .output_native_tokens
                    .entry(*native_token.token_id())
                    .or_default();

                *native_token_amount = native_token_amount
                    .checked_add(native_token.amount())
                    .ok_or(Error::CreatedNativeTokensAmountOverflow)?;
            }
        }
    }

    // Validation of storage deposit returns.
    for (return_address, return_amount) in context.storage_deposit_returns.iter() {
        if let Some(deposit_amount) = context.simple_deposits.get(return_address) {
            if deposit_amount < return_amount {
                return Ok(Some(TransactionFailureReason::StorageDepositReturnUnfulfilled));
            }
        } else {
            return Ok(Some(TransactionFailureReason::StorageDepositReturnUnfulfilled));
        }
    }

    // Validation of amounts.
    if context.input_amount != context.output_amount {
        return Ok(Some(TransactionFailureReason::CreatedConsumedAmountMismatch));
    }

    let mut native_token_ids = HashSet::new();

    // Validation of input native tokens.
    for (token_id, _input_amount) in context.input_native_tokens.iter() {
        native_token_ids.insert(token_id);
    }

    // Validation of output native tokens.
    for (token_id, output_amount) in context.output_native_tokens.iter() {
        let input_amount = context.input_native_tokens.get(token_id).copied().unwrap_or_default();

        if output_amount > &input_amount
            && !context
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
    for (chain_id, current_state) in context.input_chains.iter() {
        if Output::verify_state_transition(
            Some(current_state),
            context.output_chains.get(chain_id).map(core::ops::Deref::deref),
            &context,
        )
        .is_err()
        {
            return Ok(Some(TransactionFailureReason::InvalidChainStateTransition));
        }
    }

    // Validation of state creations.
    for (chain_id, next_state) in context.output_chains.iter() {
        if context.input_chains.get(chain_id).is_none()
            && Output::verify_state_transition(None, Some(next_state), &context).is_err()
        {
            return Ok(Some(TransactionFailureReason::InvalidChainStateTransition));
        }
    }

    Ok(None)
}
