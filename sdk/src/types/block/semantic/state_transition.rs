// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{
    output::{AccountOutput, AnchorOutput, ChainId, DelegationOutput, FoundryOutput, NftOutput, Output, TokenScheme},
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
    IssuerNotUnlocked,
    MissingAccountForFoundry,
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
}

impl From<TransactionFailureReason> for StateTransitionError {
    fn from(error: TransactionFailureReason) -> Self {
        Self::TransactionFailure(error)
    }
}

///
pub trait StateTransitionVerifier {
    ///
    fn creation(next_state: &Self, context: &SemanticValidationContext<'_>) -> Result<(), StateTransitionError>;

    ///
    fn transition(
        current_state: &Self,
        next_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), StateTransitionError>;

    ///
    fn destruction(current_state: &Self, context: &SemanticValidationContext<'_>) -> Result<(), StateTransitionError>;
}

impl SemanticValidationContext<'_> {
    ///
    pub fn verify_state_transition(
        &self,
        current_state: Option<&Output>,
        next_state: Option<&Output>,
    ) -> Result<(), StateTransitionError> {
        match (current_state, next_state) {
            // Creations.
            (None, Some(Output::Account(next_state))) => AccountOutput::creation(next_state, self),
            (None, Some(Output::Foundry(next_state))) => FoundryOutput::creation(next_state, self),
            (None, Some(Output::Nft(next_state))) => NftOutput::creation(next_state, self),
            (None, Some(Output::Delegation(next_state))) => DelegationOutput::creation(next_state, self),

            // Transitions.
            (Some(Output::Basic(current_state)), Some(Output::Account(_next_state))) => {
                if !current_state.is_implicit_account() {
                    Err(StateTransitionError::UnsupportedStateTransition)
                } else {
                    // TODO https://github.com/iotaledger/iota-sdk/issues/1664
                    Ok(())
                }
            }
            (Some(Output::Account(current_state)), Some(Output::Account(next_state))) => {
                AccountOutput::transition(current_state, next_state, self)
            }
            (Some(Output::Foundry(current_state)), Some(Output::Foundry(next_state))) => {
                FoundryOutput::transition(current_state, next_state, self)
            }
            (Some(Output::Nft(current_state)), Some(Output::Nft(next_state))) => {
                NftOutput::transition(current_state, next_state, self)
            }
            (Some(Output::Delegation(current_state)), Some(Output::Delegation(next_state))) => {
                DelegationOutput::transition(current_state, next_state, self)
            }

            // Destructions.
            (Some(Output::Account(current_state)), None) => AccountOutput::destruction(current_state, self),
            (Some(Output::Foundry(current_state)), None) => FoundryOutput::destruction(current_state, self),
            (Some(Output::Nft(current_state)), None) => NftOutput::destruction(current_state, self),
            (Some(Output::Delegation(current_state)), None) => DelegationOutput::destruction(current_state, self),

            // Unsupported.
            _ => Err(StateTransitionError::UnsupportedStateTransition),
        }
    }
}

impl StateTransitionVerifier for AccountOutput {
    fn creation(next_state: &Self, context: &SemanticValidationContext<'_>) -> Result<(), StateTransitionError> {
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
        current_state: &Self,
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

    fn destruction(_current_state: &Self, context: &SemanticValidationContext<'_>) -> Result<(), StateTransitionError> {
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
    fn creation(next_state: &Self, context: &SemanticValidationContext<'_>) -> Result<(), StateTransitionError> {
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
        current_state: &Self,
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

    fn destruction(_current_state: &Self, context: &SemanticValidationContext<'_>) -> Result<(), StateTransitionError> {
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
    fn creation(next_state: &Self, context: &SemanticValidationContext<'_>) -> Result<(), StateTransitionError> {
        let account_chain_id = ChainId::from(*next_state.account_address().account_id());

        if let (Some(Output::Account(input_account)), Some(Output::Account(output_account))) = (
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
        current_state: &Self,
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

    fn destruction(current_state: &Self, context: &SemanticValidationContext<'_>) -> Result<(), StateTransitionError> {
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
    fn creation(next_state: &Self, context: &SemanticValidationContext<'_>) -> Result<(), StateTransitionError> {
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
        current_state: &Self,
        next_state: &Self,
        _context: &SemanticValidationContext<'_>,
    ) -> Result<(), StateTransitionError> {
        Self::transition_inner(current_state, next_state)
    }

    fn destruction(_current_state: &Self, context: &SemanticValidationContext<'_>) -> Result<(), StateTransitionError> {
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
    fn creation(next_state: &Self, _context: &SemanticValidationContext<'_>) -> Result<(), StateTransitionError> {
        if !next_state.delegation_id().is_null() {
            return Err(StateTransitionError::NonZeroCreatedId);
        }

        if next_state.amount() != next_state.delegated_amount() {
            return Err(StateTransitionError::InvalidDelegatedAmount);
        }

        if next_state.end_epoch() != 0 {
            return Err(StateTransitionError::NonZeroDelegationEndEpoch);
        }

        Ok(())
    }

    fn transition(
        current_state: &Self,
        next_state: &Self,
        _context: &SemanticValidationContext<'_>,
    ) -> Result<(), StateTransitionError> {
        Self::transition_inner(current_state, next_state)
    }

    fn destruction(
        _current_state: &Self,
        _context: &SemanticValidationContext<'_>,
    ) -> Result<(), StateTransitionError> {
        // TODO handle mana rewards
        Ok(())
    }
}
