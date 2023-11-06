// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::semantic::{SemanticValidationContext, TransactionFailureReason};

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
