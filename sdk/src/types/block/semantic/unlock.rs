// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{
    address::Address,
    output::{Output, OutputId},
    semantic::{SemanticValidationContext, TransactionFailureReason},
    signature::Signature,
    unlock::Unlock,
};

impl SemanticValidationContext<'_> {
    ///
    pub fn address_unlock(&mut self, address: &Address, unlock: &Unlock) -> Result<(), TransactionFailureReason> {
        match (address, unlock) {
            (Address::Ed25519(ed25519_address), Unlock::Signature(unlock)) => {
                if self.unlocked_addresses.contains(address) {
                    return Err(TransactionFailureReason::InvalidInputUnlock);
                }

                let Signature::Ed25519(signature) = unlock.signature();

                if signature
                    .is_valid(self.transaction_signing_hash.as_ref(), ed25519_address)
                    .is_err()
                {
                    return Err(TransactionFailureReason::InvalidUnlockBlockSignature);
                }

                self.unlocked_addresses.insert(address.clone());
            }
            (Address::Ed25519(_), Unlock::Reference(_)) => {
                // TODO actually check that it was unlocked by the same signature.
                if !self.unlocked_addresses.contains(address) {
                    return Err(TransactionFailureReason::InvalidInputUnlock);
                }
            }
            (Address::Account(account_address), Unlock::Account(unlock)) => {
                // PANIC: indexing is fine as it is already syntactically verified that indexes reference below.
                if let (output_id, Output::Account(account_output)) = self.inputs[unlock.index() as usize] {
                    if &account_output.account_id_non_null(output_id) != account_address.account_id() {
                        return Err(TransactionFailureReason::InvalidInputUnlock);
                    }
                    if !self.unlocked_addresses.contains(address) {
                        return Err(TransactionFailureReason::InvalidInputUnlock);
                    }
                } else {
                    return Err(TransactionFailureReason::InvalidInputUnlock);
                }
            }
            (Address::Nft(nft_address), Unlock::Nft(unlock)) => {
                // PANIC: indexing is fine as it is already syntactically verified that indexes reference below.
                if let (output_id, Output::Nft(nft_output)) = self.inputs[unlock.index() as usize] {
                    if &nft_output.nft_id_non_null(output_id) != nft_address.nft_id() {
                        return Err(TransactionFailureReason::InvalidInputUnlock);
                    }
                    if !self.unlocked_addresses.contains(address) {
                        return Err(TransactionFailureReason::InvalidInputUnlock);
                    }
                } else {
                    return Err(TransactionFailureReason::InvalidInputUnlock);
                }
            }
            // TODO maybe shouldn't be a semantic error but this function currently returns a TransactionFailureReason.
            (Address::Anchor(_), _) => return Err(TransactionFailureReason::SemanticValidationFailed),
            (Address::ImplicitAccountCreation(implicit_account_creation_address), _) => {
                return self.address_unlock(
                    &Address::from(*implicit_account_creation_address.ed25519_address()),
                    unlock,
                );
            }
            (Address::Multi(multi_address), Unlock::Multi(unlock)) => {
                if multi_address.len() != unlock.len() {
                    return Err(TransactionFailureReason::InvalidInputUnlock);
                }

                let mut cumulative_unlocked_weight = 0u16;

                for (address, unlock) in multi_address.addresses().iter().zip(unlock.unlocks()) {
                    if !unlock.is_empty() {
                        self.address_unlock(address, unlock)?;
                        cumulative_unlocked_weight += address.weight() as u16;
                    }
                }

                if cumulative_unlocked_weight < multi_address.threshold() {
                    return Err(TransactionFailureReason::InvalidInputUnlock);
                }
            }
            (Address::Restricted(restricted_address), _) => {
                return self.address_unlock(restricted_address.address(), unlock);
            }
            _ => return Err(TransactionFailureReason::InvalidInputUnlock),
        }

        Ok(())
    }

    pub fn output_unlock(
        &mut self,
        output: &Output,
        output_id: &OutputId,
        unlock: &Unlock,
    ) -> Result<(), TransactionFailureReason> {
        match output {
            Output::Basic(output) => {
                let slot_index = self
                    .transaction
                    .context_inputs()
                    .iter()
                    .find_map(|c| c.as_commitment_opt().map(|c| c.slot_index()));
                let locked_address = output
                    .unlock_conditions()
                    .locked_address(
                        output.address(),
                        slot_index,
                        self.protocol_parameters.committable_age_range(),
                    )
                    .map_err(|_| TransactionFailureReason::InvalidCommitmentContextInput)?
                    // because of expiration the input can't be unlocked at this time
                    .ok_or(TransactionFailureReason::SemanticValidationFailed)?;

                self.address_unlock(locked_address, unlock)?;
            }
            Output::Account(output) => {
                let locked_address = output
                    .unlock_conditions()
                    .locked_address(output.address(), None, self.protocol_parameters.committable_age_range())
                    // Safe to unwrap, AccountOutput can't have an expiration unlock condition.
                    .unwrap()
                    .unwrap();

                self.address_unlock(locked_address, unlock)?;

                self.unlocked_addresses
                    .insert(Address::from(output.account_id_non_null(output_id)));
            }
            Output::Anchor(_) => panic!(),
            // Output::Anchor(_) => return Err(Error::UnsupportedOutputKind(AnchorOutput::KIND)),
            Output::Foundry(output) => self.address_unlock(&Address::from(*output.account_address()), unlock)?,
            Output::Nft(output) => {
                let slot_index = self
                    .transaction
                    .context_inputs()
                    .iter()
                    .find_map(|c| c.as_commitment_opt().map(|c| c.slot_index()));
                let locked_address = output
                    .unlock_conditions()
                    .locked_address(
                        output.address(),
                        slot_index,
                        self.protocol_parameters.committable_age_range(),
                    )
                    .map_err(|_| TransactionFailureReason::InvalidCommitmentContextInput)?
                    // because of expiration the input can't be unlocked at this time
                    .ok_or(TransactionFailureReason::SemanticValidationFailed)?;

                self.address_unlock(locked_address, unlock)?;

                self.unlocked_addresses
                    .insert(Address::from(output.nft_id_non_null(output_id)));
            }
            Output::Delegation(output) => {
                let locked_address: &Address = output
                    .unlock_conditions()
                    .locked_address(output.address(), None, self.protocol_parameters.committable_age_range())
                    // Safe to unwrap, DelegationOutput can't have an expiration unlock condition.
                    .unwrap()
                    .unwrap();

                self.address_unlock(locked_address, unlock)?;
            }
        }

        Ok(())
    }
}
