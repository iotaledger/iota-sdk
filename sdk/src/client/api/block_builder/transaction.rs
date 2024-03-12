// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Transaction preparation and signing

use crate::{
    client::api::{PreparedTransactionData, SignedTransactionData},
    types::block::{
        output::{Output, OutputId},
        protocol::ProtocolParameters,
        semantic::{SemanticValidationContext, TransactionFailureReason},
    },
};

impl PreparedTransactionData {
    /// Verifies the semantic of a prepared transaction.
    pub fn verify_semantic(&self, protocol_parameters: &ProtocolParameters) -> Result<(), TransactionFailureReason> {
        let inputs = self
            .inputs_data
            .iter()
            .map(|input| (input.output_id(), &input.output))
            .collect::<Vec<(&OutputId, &Output)>>();

        let context = SemanticValidationContext::new(
            &self.transaction,
            &inputs,
            None,
            Some(&self.mana_rewards),
            protocol_parameters,
        );

        context.validate()
    }
}

impl SignedTransactionData {
    /// Verifies the semantic of a prepared transaction.
    pub fn verify_semantic(&self, protocol_parameters: &ProtocolParameters) -> Result<(), TransactionFailureReason> {
        let inputs = self
            .inputs_data
            .iter()
            .map(|input| (input.output_id(), &input.output))
            .collect::<Vec<(&OutputId, &Output)>>();

        let context = SemanticValidationContext::new(
            self.payload.transaction(),
            &inputs,
            Some(self.payload.unlocks()),
            Some(&self.mana_rewards),
            protocol_parameters,
        );

        context.validate()
    }
}
