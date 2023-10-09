// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::collections::BTreeMap;
use core::fmt;

use hashbrown::{HashMap, HashSet};
use primitive_types::U256;

use crate::types::block::{
    address::Address,
    output::{ChainId, FoundryId, InputsCommitment, NativeTokens, Output, OutputId, TokenId},
    payload::transaction::{RegularTransactionEssence, TransactionEssence, TransactionId},
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
    /// The inputs commitment is invalid.
    InvalidInputsCommitment = 11,
    /// The output contains a Sender with an ident (address) which is not unlocked.
    SenderNotUnlocked = 12,
    /// The chain state transition is invalid.
    InvalidChainStateTransition = 13,
    /// The referenced input is created after transaction issuing time.
    InvalidTransactionIssuingTime = 14,
    /// The mana amount is invalid.
    InvalidManaAmount = 15,
    /// The Block Issuance Credits amount is invalid.
    InvalidBlockIssuanceCreditsAmount = 16,
    /// Reward Context Input is invalid.
    InvalidRewardContextInput = 17,
    /// Commitment Context Input is invalid.
    InvalidCommitmentContextInput = 18,
    /// Staking Feature is not provided in account output when claiming rewards.
    MissingStakingFeature = 19,
    /// Failed to claim staking reward.
    FailedToClaimStakingReward = 20,
    /// Failed to claim delegation reward.
    FailedToClaimDelegationReward = 21,
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
            Self::InvalidInputsCommitment => write!(f, "The inputs commitment is invalid."),
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
            11 => Self::InvalidInputsCommitment,
            12 => Self::SenderNotUnlocked,
            13 => Self::InvalidChainStateTransition,
            14 => Self::InvalidTransactionIssuingTime,
            15 => Self::InvalidManaAmount,
            16 => Self::InvalidBlockIssuanceCreditsAmount,
            17 => Self::InvalidRewardContextInput,
            18 => Self::InvalidCommitmentContextInput,
            19 => Self::MissingStakingFeature,
            20 => Self::FailedToClaimStakingReward,
            21 => Self::FailedToClaimDelegationReward,
            255 => Self::SemanticValidationFailed,
            x => return Err(Self::Error::InvalidTransactionFailureReason(x)),
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
    ) -> Self {
        Self {
            essence,
            unlocks,
            essence_hash: TransactionEssence::from(essence.clone()).hash(),
            inputs_commitment: InputsCommitment::new(inputs.clone().map(|(_, output)| output)),
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
        return Ok(Some(TransactionFailureReason::InvalidInputsCommitment));
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

        if unlock_conditions.is_time_locked(context.essence.creation_slot()) {
            return Ok(Some(TransactionFailureReason::TimelockNotExpired));
        }

        if !unlock_conditions.is_expired(context.essence.creation_slot()) {
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
                return Ok(Some(TransactionFailureReason::SenderNotUnlocked));
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
        return Ok(Some(TransactionFailureReason::SumInputsOutputsAmountMismatch));
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
