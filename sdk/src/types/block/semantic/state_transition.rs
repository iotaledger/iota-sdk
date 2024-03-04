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
pub trait StateTransitionVerifier {
    ///
    fn creation(
        output_id: &OutputId,
        next_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), TransactionFailureReason>;

    ///
    fn transition(
        current_output_id: &OutputId,
        current_state: &Self,
        next_output_id: &OutputId,
        next_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), TransactionFailureReason>;

    ///
    fn destruction(
        output_id: &OutputId,
        current_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), TransactionFailureReason>;
}

impl SemanticValidationContext<'_> {
    ///
    pub fn verify_state_transition(
        &self,
        current_state: Option<(&OutputId, &Output)>,
        next_state: Option<(&OutputId, &Output)>,
    ) -> Result<(), TransactionFailureReason> {
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
                    Err(TransactionFailureReason::SemanticValidationFailed)
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
            (Some((_output_id, Output::Basic(current_state))), None) => {
                if current_state.is_implicit_account() {
                    Err(TransactionFailureReason::ImplicitAccountDestructionDisallowed)
                } else {
                    Err(TransactionFailureReason::SemanticValidationFailed)
                }
            }
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
            _ => Err(TransactionFailureReason::SemanticValidationFailed),
        }
    }
}

impl BasicOutput {
    pub(crate) fn implicit_account_transition(
        _current_state: &Self,
        next_state: &AccountOutput,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), TransactionFailureReason> {
        if next_state.account_id().is_null() {
            return Err(TransactionFailureReason::ImplicitAccountDestructionDisallowed);
        }

        if let Some(_block_issuer) = next_state.features().block_issuer() {
            // TODO https://github.com/iotaledger/iota-sdk/issues/1853
            // The Account must have a Block Issuer Feature and it must pass semantic validation as if the implicit
            // account contained a Block Issuer Feature with its Expiry Slot set to the maximum value of
            // slot indices and the feature was transitioned.
        } else {
            return Err(TransactionFailureReason::BlockIssuerNotExpired);
        }

        if context.unlocks.is_some() {
            if let Some(issuer) = next_state.immutable_features().issuer() {
                if !context.unlocked_addresses.contains(issuer.address()) {
                    return Err(TransactionFailureReason::IssuerFeatureNotUnlocked);
                }
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
    ) -> Result<(), TransactionFailureReason> {
        if !next_state.account_id().is_null() {
            return Err(TransactionFailureReason::NewChainOutputHasNonZeroedId);
        }

        if let Some(block_issuer) = next_state.features().block_issuer() {
            let past_bounded_slot = context
                .protocol_parameters
                .past_bounded_slot(context.commitment_context_input.unwrap());

            if block_issuer.expiry_slot() < past_bounded_slot {
                return Err(TransactionFailureReason::BlockIssuerExpiryTooEarly);
            }
        }
        if let Some(staking) = next_state.features().staking() {
            let past_bounded_epoch = context
                .protocol_parameters
                .past_bounded_epoch(context.commitment_context_input.unwrap());

            if staking.start_epoch() != past_bounded_epoch {
                return Err(TransactionFailureReason::StakingStartEpochInvalid);
            }
            if staking.end_epoch() < past_bounded_epoch + context.protocol_parameters.staking_unbonding_period {
                return Err(TransactionFailureReason::StakingEndEpochTooEarly);
            }
        }

        if context.unlocks.is_some() {
            if let Some(issuer) = next_state.immutable_features().issuer() {
                if !context.unlocked_addresses.contains(issuer.address()) {
                    return Err(TransactionFailureReason::IssuerFeatureNotUnlocked);
                }
            }
        }

        Ok(())
    }

    fn transition(
        current_output_id: &OutputId,
        current_state: &Self,
        _next_output_id: &OutputId,
        next_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), TransactionFailureReason> {
        match (
            current_state.features().block_issuer(),
            next_state.features().block_issuer(),
        ) {
            (None, Some(block_issuer_output)) => {
                let past_bounded_slot = context
                    .protocol_parameters
                    .past_bounded_slot(context.commitment_context_input.unwrap());

                if block_issuer_output.expiry_slot() < past_bounded_slot {
                    return Err(TransactionFailureReason::BlockIssuerExpiryTooEarly);
                }
            }
            (Some(block_issuer_input), None) => {
                let commitment_index = context.commitment_context_input.unwrap();

                if block_issuer_input.expiry_slot() >= commitment_index.slot_index() {
                    return Err(TransactionFailureReason::BlockIssuerNotExpired);
                }
            }
            (Some(block_issuer_input), Some(block_issuer_output)) => {
                let commitment_index = context.commitment_context_input.unwrap();
                let past_bounded_slot = context.protocol_parameters.past_bounded_slot(commitment_index);

                if block_issuer_input.expiry_slot() >= commitment_index.slot_index() {
                    if block_issuer_input.expiry_slot() != block_issuer_output.expiry_slot()
                        && block_issuer_input.expiry_slot() < past_bounded_slot
                    {
                        return Err(TransactionFailureReason::BlockIssuerNotExpired);
                    }
                } else if block_issuer_output.expiry_slot() < past_bounded_slot {
                    return Err(TransactionFailureReason::BlockIssuerExpiryTooEarly);
                }
            }
            _ => {}
        }

        match (current_state.features().staking(), next_state.features().staking()) {
            (None, Some(staking_output)) => {
                let past_bounded_epoch = context
                    .protocol_parameters
                    .past_bounded_epoch(context.commitment_context_input.unwrap());

                if staking_output.start_epoch() != past_bounded_epoch {
                    return Err(TransactionFailureReason::StakingStartEpochInvalid);
                }
                if staking_output.end_epoch()
                    < past_bounded_epoch + context.protocol_parameters.staking_unbonding_period
                {
                    return Err(TransactionFailureReason::StakingEndEpochTooEarly);
                }
            }
            (Some(staking_input), None) => {
                let future_bounded_epoch = context
                    .protocol_parameters
                    .future_bounded_epoch(context.commitment_context_input.unwrap());

                if staking_input.end_epoch() >= future_bounded_epoch {
                    return Err(TransactionFailureReason::StakingFeatureRemovedBeforeUnbonding);
                } else if context
                    .mana_rewards
                    .as_ref()
                    .is_some_and(|r| !r.contains_key(current_output_id))
                    || !context.reward_context_inputs.contains_key(current_output_id)
                {
                    return Err(TransactionFailureReason::StakingRewardClaimingInvalid);
                }
            }
            (Some(staking_input), Some(staking_output)) => {
                let past_bounded_epoch = context
                    .protocol_parameters
                    .past_bounded_epoch(context.commitment_context_input.unwrap());
                let future_bounded_epoch = context
                    .protocol_parameters
                    .future_bounded_epoch(context.commitment_context_input.unwrap());

                if staking_input.end_epoch() >= future_bounded_epoch {
                    if staking_input.staked_amount() != staking_output.staked_amount()
                        || staking_input.start_epoch() != staking_output.start_epoch()
                        || staking_input.fixed_cost() != staking_output.fixed_cost()
                    {
                        return Err(TransactionFailureReason::StakingFeatureModifiedBeforeUnbonding);
                    }
                    if staking_input.end_epoch() != staking_output.end_epoch()
                        && staking_input.end_epoch()
                            < past_bounded_epoch + context.protocol_parameters.staking_unbonding_period
                    {
                        return Err(TransactionFailureReason::StakingEndEpochTooEarly);
                    }
                } else if (staking_input.staked_amount() != staking_output.staked_amount()
                    || staking_input.start_epoch() != staking_output.start_epoch()
                    || staking_input.fixed_cost() != staking_output.fixed_cost())
                    && (staking_input.start_epoch() != past_bounded_epoch
                        || staking_input.end_epoch()
                            < past_bounded_epoch + context.protocol_parameters.staking_unbonding_period
                        || context
                            .mana_rewards
                            .as_ref()
                            .is_some_and(|r| !r.contains_key(current_output_id))
                        || !context.reward_context_inputs.contains_key(current_output_id))
                {
                    return Err(TransactionFailureReason::StakingRewardClaimingInvalid);
                }
            }
            _ => {}
        }

        Self::transition_inner(
            current_state,
            next_state,
            &context.input_chains,
            context.transaction.outputs(),
        )
    }

    fn destruction(
        output_id: &OutputId,
        current_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), TransactionFailureReason> {
        if !context
            .transaction
            .has_capability(TransactionCapabilityFlag::DestroyAccountOutputs)
        {
            return Err(TransactionFailureReason::CapabilitiesAccountDestructionNotAllowed);
        }

        if let Some(block_issuer) = current_state.features().block_issuer() {
            if block_issuer.expiry_slot() >= context.commitment_context_input.unwrap().slot_index() {
                return Err(TransactionFailureReason::BlockIssuerNotExpired);
            }
        }
        if let Some(staking) = current_state.features().staking() {
            let future_bounded_epoch = context
                .protocol_parameters
                .future_bounded_epoch(context.commitment_context_input.unwrap());

            if staking.end_epoch() >= future_bounded_epoch {
                return Err(TransactionFailureReason::StakingFeatureRemovedBeforeUnbonding);
            } else if context
                .mana_rewards
                .as_ref()
                .is_some_and(|r| !r.contains_key(output_id))
                || !context.reward_context_inputs.contains_key(output_id)
            {
                return Err(TransactionFailureReason::StakingRewardClaimingInvalid);
            }
        }

        Ok(())
    }
}

impl StateTransitionVerifier for AnchorOutput {
    fn creation(
        _output_id: &OutputId,
        next_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), TransactionFailureReason> {
        if !next_state.anchor_id().is_null() {
            return Err(TransactionFailureReason::NewChainOutputHasNonZeroedId);
        }

        if context.unlocks.is_some() {
            if let Some(issuer) = next_state.immutable_features().issuer() {
                if !context.unlocked_addresses.contains(issuer.address()) {
                    return Err(TransactionFailureReason::IssuerFeatureNotUnlocked);
                }
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
    ) -> Result<(), TransactionFailureReason> {
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
    ) -> Result<(), TransactionFailureReason> {
        if !context
            .transaction
            .capabilities()
            .has_capability(TransactionCapabilityFlag::DestroyAnchorOutputs)
        {
            return Err(TransactionFailureReason::CapabilitiesAnchorDestructionNotAllowed);
        }

        Ok(())
    }
}

impl StateTransitionVerifier for FoundryOutput {
    fn creation(
        _output_id: &OutputId,
        next_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), TransactionFailureReason> {
        let account_chain_id = ChainId::from(*next_state.account_address().account_id());

        if let (Some((_, Output::Account(input_account))), Some((_, Output::Account(output_account)))) = (
            context.input_chains.get(&account_chain_id),
            context.output_chains.get(&account_chain_id),
        ) {
            if input_account.foundry_counter() >= next_state.serial_number()
                || next_state.serial_number() > output_account.foundry_counter()
            {
                return Err(TransactionFailureReason::FoundrySerialInvalid);
            }
        } else {
            return Err(TransactionFailureReason::FoundryTransitionWithoutAccount);
        }

        let token_id = next_state.token_id();
        let output_tokens = context.output_native_tokens.get(&token_id).copied().unwrap_or_default();
        let TokenScheme::Simple(ref next_token_scheme) = next_state.token_scheme();

        // No native tokens should be referenced prior to the foundry creation.
        if context.input_native_tokens.contains_key(&token_id) {
            return Err(TransactionFailureReason::NativeTokenSumUnbalanced);
        }

        if output_tokens != next_token_scheme.minted_tokens() || !next_token_scheme.melted_tokens().is_zero() {
            return Err(TransactionFailureReason::NativeTokenSumUnbalanced);
        }

        Ok(())
    }

    fn transition(
        _current_output_id: &OutputId,
        current_state: &Self,
        _next_output_id: &OutputId,
        next_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), TransactionFailureReason> {
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
    ) -> Result<(), TransactionFailureReason> {
        if !context
            .transaction
            .has_capability(TransactionCapabilityFlag::DestroyFoundryOutputs)
        {
            return Err(TransactionFailureReason::CapabilitiesFoundryDestructionNotAllowed);
        }

        let token_id = current_state.token_id();
        let input_tokens = context.input_native_tokens.get(&token_id).copied().unwrap_or_default();
        let TokenScheme::Simple(ref current_token_scheme) = current_state.token_scheme();

        // No native tokens should be referenced after the foundry destruction.
        if context.output_native_tokens.contains_key(&token_id) {
            return Err(TransactionFailureReason::NativeTokenSumUnbalanced);
        }

        // This can't underflow as it is known that minted_tokens >= melted_tokens (syntactic rule).
        let minted_melted_diff = current_token_scheme.minted_tokens() - current_token_scheme.melted_tokens();

        if minted_melted_diff != input_tokens {
            return Err(TransactionFailureReason::NativeTokenSumUnbalanced);
        }

        Ok(())
    }
}

impl StateTransitionVerifier for NftOutput {
    fn creation(
        _output_id: &OutputId,
        next_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), TransactionFailureReason> {
        if !next_state.nft_id().is_null() {
            return Err(TransactionFailureReason::NewChainOutputHasNonZeroedId);
        }

        if context.unlocks.is_some() {
            if let Some(issuer) = next_state.immutable_features().issuer() {
                if !context.unlocked_addresses.contains(issuer.address()) {
                    return Err(TransactionFailureReason::IssuerFeatureNotUnlocked);
                }
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
    ) -> Result<(), TransactionFailureReason> {
        Self::transition_inner(current_state, next_state)
    }

    fn destruction(
        _output_id: &OutputId,
        _current_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), TransactionFailureReason> {
        if !context
            .transaction
            .has_capability(TransactionCapabilityFlag::DestroyNftOutputs)
        {
            return Err(TransactionFailureReason::CapabilitiesNftDestructionNotAllowed);
        }

        Ok(())
    }
}

impl StateTransitionVerifier for DelegationOutput {
    fn creation(
        _output_id: &OutputId,
        next_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), TransactionFailureReason> {
        let protocol_parameters = &context.protocol_parameters;

        if !next_state.delegation_id().is_null() {
            return Err(TransactionFailureReason::NewChainOutputHasNonZeroedId);
        }

        if next_state.amount() != next_state.delegated_amount() {
            return Err(TransactionFailureReason::DelegationAmountMismatch);
        }

        if next_state.end_epoch() != 0 {
            return Err(TransactionFailureReason::DelegationEndEpochNotZero);
        }

        let slot_commitment_id = context
            .commitment_context_input
            .ok_or(TransactionFailureReason::DelegationCommitmentInputMissing)?;

        if next_state.start_epoch() != protocol_parameters.delegation_start_epoch(slot_commitment_id) {
            return Err(TransactionFailureReason::DelegationStartEpochInvalid);
        }

        Ok(())
    }

    fn transition(
        _current_output_id: &OutputId,
        current_state: &Self,
        _next_output_id: &OutputId,
        next_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), TransactionFailureReason> {
        Self::transition_inner(current_state, next_state)?;

        let protocol_parameters = &context.protocol_parameters;

        let slot_commitment_id = context
            .commitment_context_input
            .ok_or(TransactionFailureReason::DelegationCommitmentInputMissing)?;

        if next_state.end_epoch() != protocol_parameters.delegation_end_epoch(slot_commitment_id) {
            return Err(TransactionFailureReason::DelegationEndEpochInvalid);
        }

        Ok(())
    }

    fn destruction(
        output_id: &OutputId,
        _current_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), TransactionFailureReason> {
        if context
            .mana_rewards
            .as_ref()
            .is_some_and(|r| !r.contains_key(output_id))
            || !context.reward_context_inputs.contains_key(output_id)
        {
            return Err(TransactionFailureReason::DelegationRewardInputMissing);
        }

        if context.commitment_context_input.is_none() {
            return Err(TransactionFailureReason::DelegationCommitmentInputMissing);
        }

        Ok(())
    }
}
