// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::semantic::ValidationContext;

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
    IssuerNotUnlocked,
    MissingAccountForFoundry,
    MutatedFieldWithoutRights,
    MutatedImmutableField,
    NonMonotonicallyIncreasingNativeTokens,
    NonZeroCreatedId,
    NonZeroCreatedFoundryCounter,
    NonZeroCreatedStateIndex,
    UnsortedCreatedFoundries,
    UnsupportedStateIndexOperation { current_state: u32, next_state: u32 },
    UnsupportedStateTransition,
}

///
pub trait StateTransitionVerifier {
    ///
    fn creation(next_state: &Self, context: &ValidationContext<'_>) -> Result<(), StateTransitionError>;

    ///
    fn transition(
        current_state: &Self,
        next_state: &Self,
        context: &ValidationContext<'_>,
    ) -> Result<(), StateTransitionError>;

    ///
    fn destruction(current_state: &Self, context: &ValidationContext<'_>) -> Result<(), StateTransitionError>;
}
