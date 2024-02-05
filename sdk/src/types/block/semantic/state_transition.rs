// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{
    output::{
        AccountOutput, AnchorOutput, BasicOutput, ChainId, DelegationOutput, FoundryOutput, NftOutput, Output,
        OutputId, TokenScheme,
    },
    payload::signed_transaction::TransactionCapabilityFlag,
    semantic::{SemanticValidationContext, TransactionFailureReason},
};

///
#[allow(missing_docs)]
#[derive(Debug, Eq, PartialEq)]
pub enum StateTransitionError {
    InconsistentCreatedFoundriesCount,
    InconsistentFoundrySerialNumber,
    InconsistentNativeTokensFoundryCreation,
    InconsistentNativeTokensFoundryDestruction,
    InconsistentNativeTokensMint,
    InconsistentNativeTokensTransition,
    InconsistentNativeTokensMeltBurn,
    InvalidDelegatedAmount,
    InvalidBlockIssuerTransition,
    IssuerNotUnlocked,
    MissingAccountForFoundry,
    MissingCommitmentContextInput,
    MissingRewardInput,
    MutatedFieldWithoutRights,
    MutatedImmutableField,
    NonDelayedClaimingTransition,
    NonMonotonicallyIncreasingNativeTokens,
    NonZeroCreatedId,
    NonZeroCreatedFoundryCounter,
    NonZeroCreatedStateIndex,
    NonZeroDelegationEndEpoch,
    UnsortedCreatedFoundries,
    UnsupportedStateIndexOperation { current_state: u32, next_state: u32 },
    UnsupportedStateTransition,
    TransactionFailure(TransactionFailureReason),
    ZeroCreatedId,
}

impl From<TransactionFailureReason> for StateTransitionError {
    fn from(error: TransactionFailureReason) -> Self {
        Self::TransactionFailure(error)
    }
}

///
pub trait StateTransitionVerifier {
    ///
    fn creation(
        output_id: &OutputId,
        next_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), StateTransitionError>;

    ///
    fn transition(
        current_output_id: &OutputId,
        current_state: &Self,
        next_output_id: &OutputId,
        next_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), StateTransitionError>;

    ///
    fn destruction(
        output_id: &OutputId,
        current_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), StateTransitionError>;
}

impl SemanticValidationContext<'_> {
    ///
    pub fn verify_state_transition(
        &self,
        current_state: Option<(&OutputId, &Output)>,
        next_state: Option<(&OutputId, &Output)>,
    ) -> Result<(), StateTransitionError> {
        match (current_state, next_state) {
            // Creations.
            (None, Some((output_id, Output::Account(next_state)))) => {
                AccountOutput::creation(output_id, next_state, self)
            }
            (None, Some((output_id, Output::Foundry(next_state)))) => {
                FoundryOutput::creation(output_id, next_state, self)
            }
            (None, Some((output_id, Output::Nft(next_state)))) => NftOutput::creation(output_id, next_state, self),
            (None, Some((output_id, Output::Delegation(next_state)))) => {
                DelegationOutput::creation(output_id, next_state, self)
            }

            // Transitions.
            (
                Some((_current_output_id, Output::Basic(current_state))),
                Some((_next_output_id, Output::Account(next_state))),
            ) => {
                if current_state.is_implicit_account() {
                    BasicOutput::implicit_account_transition(current_state, next_state, self)
                } else {
                    Err(StateTransitionError::UnsupportedStateTransition)
                }
            }
            (
                Some((current_output_id, Output::Account(current_state))),
                Some((next_output_id, Output::Account(next_state))),
            ) => AccountOutput::transition(current_output_id, current_state, next_output_id, next_state, self),
            (
                Some((current_output_id, Output::Foundry(current_state))),
                Some((next_output_id, Output::Foundry(next_state))),
            ) => FoundryOutput::transition(current_output_id, current_state, next_output_id, next_state, self),
            (
                Some((current_output_id, Output::Nft(current_state))),
                Some((next_output_id, Output::Nft(next_state))),
            ) => NftOutput::transition(current_output_id, current_state, next_output_id, next_state, self),
            (
                Some((current_output_id, Output::Delegation(current_state))),
                Some((next_output_id, Output::Delegation(next_state))),
            ) => DelegationOutput::transition(current_output_id, current_state, next_output_id, next_state, self),

            // Destructions.
            (Some((output_id, Output::Account(current_state))), None) => {
                AccountOutput::destruction(output_id, current_state, self)
            }
            (Some((output_id, Output::Foundry(current_state))), None) => {
                FoundryOutput::destruction(output_id, current_state, self)
            }
            (Some((output_id, Output::Nft(current_state))), None) => {
                NftOutput::destruction(output_id, current_state, self)
            }
            (Some((output_id, Output::Delegation(current_state))), None) => {
                DelegationOutput::destruction(output_id, current_state, self)
            }

            // Unsupported.
            _ => Err(StateTransitionError::UnsupportedStateTransition),
        }
    }
}

impl BasicOutput {
    pub(crate) fn implicit_account_transition(
        _current_state: &Self,
        next_state: &AccountOutput,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), StateTransitionError> {
        if next_state.account_id().is_null() {
            return Err(StateTransitionError::ZeroCreatedId);
        }

        if let Some(_block_issuer) = next_state.features().block_issuer() {
            // TODO https://github.com/iotaledger/iota-sdk/issues/1853
            // The Account must have a Block Issuer Feature and it must pass semantic validation as if the implicit
            // account contained a Block Issuer Feature with its Expiry Slot set to the maximum value of
            // slot indices and the feature was transitioned.
        } else {
            return Err(StateTransitionError::InvalidBlockIssuerTransition);
        }

        if let Some(issuer) = next_state.immutable_features().issuer() {
            if !context.unlocked_addresses.contains(issuer.address()) {
                return Err(StateTransitionError::IssuerNotUnlocked);
            }
        }

        Ok(())
    }
}

impl StateTransitionVerifier for AccountOutput {
    fn creation(
        _output_id: &OutputId,
        next_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), StateTransitionError> {
        if !next_state.account_id().is_null() {
            return Err(StateTransitionError::NonZeroCreatedId);
        }

        if let Some(issuer) = next_state.immutable_features().issuer() {
            if !context.unlocked_addresses.contains(issuer.address()) {
                return Err(StateTransitionError::IssuerNotUnlocked);
            }
        }

        Ok(())
    }

    fn transition(
        _current_output_id: &OutputId,
        current_state: &Self,
        _next_output_id: &OutputId,
        next_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), StateTransitionError> {
        Self::transition_inner(
            current_state,
            next_state,
            &context.input_chains,
            context.transaction.outputs(),
        )
    }

    fn destruction(
        _output_id: &OutputId,
        _current_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), StateTransitionError> {
        if !context
            .transaction
            .has_capability(TransactionCapabilityFlag::DestroyAccountOutputs)
        {
            return Err(TransactionFailureReason::TransactionCapabilityAccountDestructionNotAllowed)?;
        }
        Ok(())
    }
}

impl StateTransitionVerifier for AnchorOutput {
    fn creation(
        _output_id: &OutputId,
        next_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), StateTransitionError> {
        if !next_state.anchor_id().is_null() {
            return Err(StateTransitionError::NonZeroCreatedId);
        }

        if let Some(issuer) = next_state.immutable_features().issuer() {
            if !context.unlocked_addresses.contains(issuer.address()) {
                return Err(StateTransitionError::IssuerNotUnlocked);
            }
        }

        Ok(())
    }

    fn transition(
        _current_output_id: &OutputId,
        current_state: &Self,
        _next_output_id: &OutputId,
        next_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), StateTransitionError> {
        Self::transition_inner(
            current_state,
            next_state,
            &context.input_chains,
            context.transaction.outputs(),
        )
    }

    fn destruction(
        _output_id: &OutputId,
        _current_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), StateTransitionError> {
        if !context
            .transaction
            .capabilities()
            .has_capability(TransactionCapabilityFlag::DestroyAnchorOutputs)
        {
            return Err(TransactionFailureReason::TransactionCapabilityAccountDestructionNotAllowed)?;
        }
        Ok(())
    }
}

impl StateTransitionVerifier for FoundryOutput {
    fn creation(
        _output_id: &OutputId,
        next_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), StateTransitionError> {
        let account_chain_id = ChainId::from(*next_state.account_address().account_id());

        if let (Some((_, Output::Account(input_account))), Some((_, Output::Account(output_account)))) = (
            context.input_chains.get(&account_chain_id),
            context.output_chains.get(&account_chain_id),
        ) {
            if input_account.foundry_counter() >= next_state.serial_number()
                || next_state.serial_number() > output_account.foundry_counter()
            {
                return Err(StateTransitionError::InconsistentFoundrySerialNumber);
            }
        } else {
            return Err(StateTransitionError::MissingAccountForFoundry);
        }

        let token_id = next_state.token_id();
        let output_tokens = context.output_native_tokens.get(&token_id).copied().unwrap_or_default();
        let TokenScheme::Simple(ref next_token_scheme) = next_state.token_scheme();

        // No native tokens should be referenced prior to the foundry creation.
        if context.input_native_tokens.contains_key(&token_id) {
            return Err(StateTransitionError::InconsistentNativeTokensFoundryCreation);
        }

        if output_tokens != next_token_scheme.minted_tokens() || !next_token_scheme.melted_tokens().is_zero() {
            return Err(StateTransitionError::InconsistentNativeTokensFoundryCreation);
        }

        Ok(())
    }

    fn transition(
        _current_output_id: &OutputId,
        current_state: &Self,
        _next_output_id: &OutputId,
        next_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), StateTransitionError> {
        Self::transition_inner(
            current_state,
            next_state,
            &context.input_native_tokens,
            &context.output_native_tokens,
            context.transaction.capabilities(),
        )
    }

    fn destruction(
        _output_id: &OutputId,
        current_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), StateTransitionError> {
        if !context
            .transaction
            .has_capability(TransactionCapabilityFlag::DestroyFoundryOutputs)
        {
            return Err(TransactionFailureReason::TransactionCapabilityFoundryDestructionNotAllowed)?;
        }

        let token_id = current_state.token_id();
        let input_tokens = context.input_native_tokens.get(&token_id).copied().unwrap_or_default();
        let TokenScheme::Simple(ref current_token_scheme) = current_state.token_scheme();

        // No native tokens should be referenced after the foundry destruction.
        if context.output_native_tokens.contains_key(&token_id) {
            return Err(StateTransitionError::InconsistentNativeTokensFoundryDestruction);
        }

        // This can't underflow as it is known that minted_tokens >= melted_tokens (syntactic rule).
        let minted_melted_diff = current_token_scheme.minted_tokens() - current_token_scheme.melted_tokens();

        if minted_melted_diff != input_tokens {
            return Err(StateTransitionError::InconsistentNativeTokensFoundryDestruction);
        }

        Ok(())
    }
}

impl StateTransitionVerifier for NftOutput {
    fn creation(
        _output_id: &OutputId,
        next_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), StateTransitionError> {
        if !next_state.nft_id().is_null() {
            return Err(StateTransitionError::NonZeroCreatedId);
        }

        if let Some(issuer) = next_state.immutable_features().issuer() {
            if !context.unlocked_addresses.contains(issuer.address()) {
                return Err(StateTransitionError::IssuerNotUnlocked);
            }
        }

        Ok(())
    }

    fn transition(
        _current_output_id: &OutputId,
        current_state: &Self,
        _next_output_id: &OutputId,
        next_state: &Self,
        _context: &SemanticValidationContext<'_>,
    ) -> Result<(), StateTransitionError> {
        Self::transition_inner(current_state, next_state)
    }

    fn destruction(
        _output_id: &OutputId,
        _current_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), StateTransitionError> {
        if !context
            .transaction
            .has_capability(TransactionCapabilityFlag::DestroyNftOutputs)
        {
            return Err(TransactionFailureReason::TransactionCapabilityNftDestructionNotAllowed)?;
        }
        Ok(())
    }
}

impl StateTransitionVerifier for DelegationOutput {
    fn creation(
        _output_id: &OutputId,
        next_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), StateTransitionError> {
        let protocol_parameters = &context.protocol_parameters;

        if !next_state.delegation_id().is_null() {
            return Err(StateTransitionError::NonZeroCreatedId);
        }

        if next_state.amount() != next_state.delegated_amount() {
            return Err(StateTransitionError::InvalidDelegatedAmount);
        }

        if next_state.end_epoch() != 0 {
            return Err(StateTransitionError::NonZeroDelegationEndEpoch);
        }

        let slot_commitment_id = context
            .commitment_context_input
            .map(|c| c.slot_commitment_id())
            .ok_or(StateTransitionError::MissingCommitmentContextInput)?;

        if next_state.start_epoch() != protocol_parameters.delegation_start_epoch(slot_commitment_id) {
            // TODO: specific tx failure reason https://github.com/iotaledger/iota-core/issues/679
            return Err(StateTransitionError::TransactionFailure(
                TransactionFailureReason::SemanticValidationFailed,
            ));
        }

        Ok(())
    }

    fn transition(
        _current_output_id: &OutputId,
        current_state: &Self,
        _next_output_id: &OutputId,
        next_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), StateTransitionError> {
        Self::transition_inner(current_state, next_state)?;

        let protocol_parameters = &context.protocol_parameters;

        let slot_commitment_id = context
            .commitment_context_input
            .map(|c| c.slot_commitment_id())
            .ok_or(StateTransitionError::MissingCommitmentContextInput)?;

        if next_state.end_epoch() != protocol_parameters.delegation_end_epoch(slot_commitment_id) {
            return Err(StateTransitionError::NonDelayedClaimingTransition);
        }

        Ok(())
    }

    fn destruction(
        output_id: &OutputId,
        _current_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), StateTransitionError> {
        // If a mana reward was provided but no reward context input exists
        if context.mana_rewards.get(output_id).is_some() && !context.reward_context_inputs.contains_key(output_id) {
            return Err(StateTransitionError::MissingRewardInput);
        }
        if context.commitment_context_input.is_none() {
            return Err(StateTransitionError::MissingCommitmentContextInput);
        }

        Ok(())
    }
}
