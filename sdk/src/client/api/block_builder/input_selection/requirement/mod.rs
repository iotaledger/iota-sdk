// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) mod account;
pub(crate) mod amount;
pub(crate) mod delegation;
pub(crate) mod ed25519;
pub(crate) mod foundry;
pub(crate) mod issuer;
pub(crate) mod mana;
pub(crate) mod native_tokens;
pub(crate) mod nft;
pub(crate) mod sender;

use self::{
    account::is_account_with_id_non_null, delegation::is_delegation_with_id_non_null, foundry::is_foundry_with_id,
    nft::is_nft_with_id_non_null,
};
use super::{Error, InputSelection};
use crate::{
    client::secret::types::InputSigningData,
    types::block::{
        address::Address,
        output::{AccountId, ChainId, DelegationId, Features, FoundryId, NftId, Output},
    },
};

/// A requirement, imposed by outputs, that needs to be resolved by selected inputs.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Requirement {
    /// Sender requirement.
    Sender(Address),
    /// Issuer requirement.
    Issuer(Address),
    /// Ed25519 requirement.
    Ed25519(Address),
    /// Foundry requirement.
    Foundry(FoundryId),
    /// Account requirement.
    Account(AccountId),
    /// Nft requirement.
    Nft(NftId),
    /// Delegation requirement.
    Delegation(DelegationId),
    /// Native tokens requirement.
    NativeTokens,
    /// Amount requirement.
    Amount,
    /// Mana requirement.
    Mana,
}

impl InputSelection {
    /// Fulfills a requirement by selecting the appropriate available inputs.
    /// Returns the selected inputs and an optional new requirement.
    pub(crate) fn fulfill_requirement(&mut self, requirement: Requirement) -> Result<Vec<InputSigningData>, Error> {
        log::debug!("Fulfilling requirement {requirement:?}");

        match requirement {
            Requirement::Sender(address) => self.fulfill_sender_requirement(&address),
            Requirement::Issuer(address) => self.fulfill_issuer_requirement(&address),
            Requirement::Ed25519(address) => self.fulfill_ed25519_requirement(&address),
            Requirement::Foundry(foundry_id) => self.fulfill_foundry_requirement(foundry_id),
            Requirement::Account(account_id) => self.fulfill_account_requirement(account_id),
            Requirement::Nft(nft_id) => self.fulfill_nft_requirement(nft_id),
            Requirement::Delegation(delegation_id) => self.fulfill_delegation_requirement(delegation_id),
            Requirement::NativeTokens => self.fulfill_native_tokens_requirement(),
            Requirement::Amount => self.fulfill_amount_requirement(),
            Requirement::Mana => self.fulfill_mana_requirement(),
        }
    }

    /// Gets requirements from outputs.
    pub(crate) fn outputs_requirements(&mut self) {
        let inputs = self.available_inputs.iter().chain(self.selected_inputs.iter());

        for output in &self.outputs {
            let is_created = match output {
                // Add an account requirement if the account output is transitioning and then required in the inputs.
                Output::Account(account_output) => {
                    let is_created = account_output.account_id().is_null();

                    if !is_created {
                        let requirement = Requirement::Account(*account_output.account_id());
                        log::debug!("Adding {requirement:?} from output");
                        self.requirements.push(requirement);
                    }

                    is_created
                }
                // Add a foundry requirement if the foundry output is transitioning and then required in the inputs.
                // Also add an account requirement since the associated account output needs to be transitioned.
                Output::Foundry(foundry_output) => {
                    // TODO add some tests
                    let is_created = !inputs.clone().any(|input| {
                        if let Output::Foundry(output) = &input.output {
                            output.id() == foundry_output.id()
                        } else {
                            false
                        }
                    });

                    if !is_created {
                        let requirement = Requirement::Foundry(foundry_output.id());
                        log::debug!("Adding {requirement:?} from output");
                        self.requirements.push(requirement);
                    }

                    let requirement = Requirement::Account(*foundry_output.account_address().account_id());
                    log::debug!("Adding {requirement:?} from output");
                    self.requirements.push(requirement);

                    is_created
                }
                // Add an nft requirement if the nft output is transitioning and then required in the inputs.
                Output::Nft(nft_output) => {
                    let is_created = nft_output.nft_id().is_null();

                    if !is_created {
                        let requirement = Requirement::Nft(*nft_output.nft_id());
                        log::debug!("Adding {requirement:?} from output");
                        self.requirements.push(requirement);
                    }

                    is_created
                }
                Output::Delegation(delegation_output) => {
                    let is_created = delegation_output.delegation_id().is_null();

                    if !is_created {
                        let requirement = Requirement::Delegation(*delegation_output.delegation_id());
                        log::debug!("Adding {requirement:?} from output");
                        self.requirements.push(requirement);
                    }

                    is_created
                }
                _ => false,
            };

            // Add a sender requirement if the sender feature is present.
            if let Some(sender) = output.features().and_then(Features::sender) {
                let requirement = Requirement::Sender(*sender.address());
                log::debug!("Adding {requirement:?} from output");
                self.requirements.push(requirement);
            }

            // Add an issuer requirement if the issuer feature is present and the chain output is created.
            if is_created {
                if let Some(issuer) = output.immutable_features().and_then(Features::issuer) {
                    let requirement = Requirement::Issuer(*issuer.address());
                    log::debug!("Adding {requirement:?} from output");
                    self.requirements.push(requirement);
                }
            }
        }
    }

    /// Gets requirements from burn.
    pub(crate) fn burn_requirements(&mut self) -> Result<(), Error> {
        if let Some(burn) = self.burn.as_ref() {
            for account_id in &burn.accounts {
                if self
                    .outputs
                    .iter()
                    .any(|output| is_account_with_id_non_null(output, account_id))
                {
                    return Err(Error::BurnAndTransition(ChainId::from(*account_id)));
                }

                let requirement = Requirement::Account(*account_id);
                log::debug!("Adding {requirement:?} from burn");
                self.requirements.push(requirement);
            }

            for foundry_id in &burn.foundries {
                if self.outputs.iter().any(|output| is_foundry_with_id(output, foundry_id)) {
                    return Err(Error::BurnAndTransition(ChainId::from(*foundry_id)));
                }

                let requirement = Requirement::Foundry(*foundry_id);
                log::debug!("Adding {requirement:?} from burn");
                self.requirements.push(requirement);
            }

            for nft_id in &burn.nfts {
                if self
                    .outputs
                    .iter()
                    .any(|output| is_nft_with_id_non_null(output, nft_id))
                {
                    return Err(Error::BurnAndTransition(ChainId::from(*nft_id)));
                }

                let requirement = Requirement::Nft(*nft_id);
                log::debug!("Adding {requirement:?} from burn");
                self.requirements.push(requirement);
            }

            for delegation_id in &burn.delegations {
                if self
                    .outputs
                    .iter()
                    .any(|output| is_delegation_with_id_non_null(output, delegation_id))
                {
                    return Err(Error::BurnAndTransition(ChainId::from(*delegation_id)));
                }

                let requirement = Requirement::Delegation(*delegation_id);
                log::debug!("Adding {requirement:?} from burn");
                self.requirements.push(requirement);
            }
        }

        Ok(())
    }
}
