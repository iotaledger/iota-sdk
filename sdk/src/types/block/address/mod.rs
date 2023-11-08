// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod account;
mod anchor;
mod bech32;
mod ed25519;
mod implicit_account_creation;
mod multi;
mod nft;
mod restricted;

use alloc::boxed::Box;

use derive_more::{Display, From};
use packable::Packable;

pub(crate) use self::multi::WeightedAddressCount;
pub use self::{
    account::AccountAddress,
    anchor::AnchorAddress,
    bech32::{Bech32Address, Hrp},
    ed25519::Ed25519Address,
    implicit_account_creation::ImplicitAccountCreationAddress,
    multi::MultiAddress,
    nft::NftAddress,
    restricted::{AddressCapabilities, AddressCapabilityFlag, RestrictedAddress},
};
use crate::types::block::{
    output::{Output, StorageScore, StorageScoreParameters},
    semantic::{SemanticValidationContext, TransactionFailureReason},
    signature::Signature,
    unlock::Unlock,
    ConvertTo, Error,
};

/// A generic address supporting different address kinds.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, From, Display, Packable)]
#[packable(tag_type = u8, with_error = Error::InvalidAddressKind)]
#[packable(unpack_error = Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(untagged))]
pub enum Address {
    /// An Ed25519 address.
    #[packable(tag = Ed25519Address::KIND)]
    Ed25519(Ed25519Address),
    /// An account address.
    #[packable(tag = AccountAddress::KIND)]
    Account(AccountAddress),
    /// An NFT address.
    #[packable(tag = NftAddress::KIND)]
    Nft(NftAddress),
    /// An anchor address.
    #[packable(tag = AnchorAddress::KIND)]
    Anchor(AnchorAddress),
    /// An implicit account creation address.
    #[packable(tag = ImplicitAccountCreationAddress::KIND)]
    ImplicitAccountCreation(ImplicitAccountCreationAddress),
    /// A multi address.
    #[packable(tag = MultiAddress::KIND)]
    Multi(MultiAddress),
    /// An address with restricted capabilities.
    #[packable(tag = RestrictedAddress::KIND)]
    #[from(ignore)]
    Restricted(Box<RestrictedAddress>),
}

impl From<RestrictedAddress> for Address {
    fn from(value: RestrictedAddress) -> Self {
        Self::Restricted(value.into())
    }
}

impl core::fmt::Debug for Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Ed25519(address) => address.fmt(f),
            Self::Account(address) => address.fmt(f),
            Self::Nft(address) => address.fmt(f),
            Self::Anchor(address) => address.fmt(f),
            Self::ImplicitAccountCreation(address) => address.fmt(f),
            Self::Multi(address) => address.fmt(f),
            Self::Restricted(address) => address.fmt(f),
        }
    }
}

impl Address {
    /// Returns the address kind of an [`Address`].
    pub fn kind(&self) -> u8 {
        match self {
            Self::Ed25519(_) => Ed25519Address::KIND,
            Self::Account(_) => AccountAddress::KIND,
            Self::Nft(_) => NftAddress::KIND,
            Self::Anchor(_) => AnchorAddress::KIND,
            Self::ImplicitAccountCreation(_) => ImplicitAccountCreationAddress::KIND,
            Self::Multi(_) => MultiAddress::KIND,
            Self::Restricted(_) => RestrictedAddress::KIND,
        }
    }

    crate::def_is_as_opt!(Address: Ed25519, Account, Nft, Anchor, ImplicitAccountCreation, Multi, Restricted);

    /// Tries to create an [`Address`] from a bech32 encoded string.
    pub fn try_from_bech32(address: impl AsRef<str>) -> Result<Self, Error> {
        Bech32Address::try_from_str(address).map(|res| res.inner)
    }

    /// Checks if an string is a valid bech32 encoded address.
    #[must_use]
    pub fn is_valid_bech32(address: &str) -> bool {
        Self::try_from_bech32(address).is_ok()
    }

    ///
    pub fn unlock(
        &self,
        unlock: &Unlock,
        context: &mut SemanticValidationContext<'_>,
    ) -> Result<(), TransactionFailureReason> {
        match (self, unlock) {
            (Self::Ed25519(ed25519_address), Unlock::Signature(unlock)) => {
                if context.unlocked_addresses.contains(self) {
                    return Err(TransactionFailureReason::InvalidInputUnlock);
                }

                let Signature::Ed25519(signature) = unlock.signature();

                if signature
                    .is_valid(context.transaction_signing_hash.as_ref(), ed25519_address)
                    .is_err()
                {
                    return Err(TransactionFailureReason::InvalidUnlockBlockSignature);
                }

                context.unlocked_addresses.insert(self.clone());
            }
            (Self::Ed25519(_ed25519_address), Unlock::Reference(_unlock)) => {
                // TODO actually check that it was unlocked by the same signature.
                if !context.unlocked_addresses.contains(self) {
                    return Err(TransactionFailureReason::InvalidInputUnlock);
                }
            }
            (Self::Account(account_address), Unlock::Account(unlock)) => {
                // PANIC: indexing is fine as it is already syntactically verified that indexes reference below.
                if let (output_id, Output::Account(account_output)) = context.inputs[unlock.index() as usize] {
                    if &account_output.account_id_non_null(output_id) != account_address.account_id() {
                        return Err(TransactionFailureReason::InvalidInputUnlock);
                    }
                    if !context.unlocked_addresses.contains(self) {
                        return Err(TransactionFailureReason::InvalidInputUnlock);
                    }
                } else {
                    return Err(TransactionFailureReason::InvalidInputUnlock);
                }
            }
            (Self::Nft(nft_address), Unlock::Nft(unlock)) => {
                // PANIC: indexing is fine as it is already syntactically verified that indexes reference below.
                if let (output_id, Output::Nft(nft_output)) = context.inputs[unlock.index() as usize] {
                    if &nft_output.nft_id_non_null(output_id) != nft_address.nft_id() {
                        return Err(TransactionFailureReason::InvalidInputUnlock);
                    }
                    if !context.unlocked_addresses.contains(self) {
                        return Err(TransactionFailureReason::InvalidInputUnlock);
                    }
                } else {
                    return Err(TransactionFailureReason::InvalidInputUnlock);
                }
            }
            // TODO maybe shouldn't be a semantic error but this function currently returns a TransactionFailureReason.
            (Self::Anchor(_), _) => return Err(TransactionFailureReason::SemanticValidationFailed),
            _ => return Err(TransactionFailureReason::InvalidInputUnlock),
        }

        Ok(())
    }
}

impl StorageScore for Address {
    fn storage_score(&self, params: StorageScoreParameters) -> u64 {
        match self {
            Address::Ed25519(address) => address.storage_score(params),
            Address::Account(address) => address.storage_score(params),
            Address::Nft(address) => address.storage_score(params),
            Address::Anchor(address) => address.storage_score(params),
            Address::ImplicitAccountCreation(address) => address.storage_score(params),
            Address::Multi(address) => address.storage_score(params),
            Address::Restricted(address) => address.storage_score(params),
        }
    }
}

pub trait ToBech32Ext: Sized {
    /// Try to encode this address to a bech32 string with the given Human Readable Part as prefix.
    fn try_to_bech32(self, hrp: impl ConvertTo<Hrp>) -> Result<Bech32Address, Error>;

    /// Encodes this address to a bech32 string with the given Human Readable Part as prefix.
    fn to_bech32(self, hrp: Hrp) -> Bech32Address;

    /// Encodes this address to a bech32 string with the given Human Readable Part as prefix without checking
    /// validity.
    fn to_bech32_unchecked(self, hrp: impl ConvertTo<Hrp>) -> Bech32Address;
}

impl<T: Into<Address>> ToBech32Ext for T {
    fn try_to_bech32(self, hrp: impl ConvertTo<Hrp>) -> Result<Bech32Address, Error> {
        Bech32Address::try_new(hrp, self)
    }

    fn to_bech32(self, hrp: Hrp) -> Bech32Address {
        Bech32Address::new(hrp, self)
    }

    fn to_bech32_unchecked(self, hrp: impl ConvertTo<Hrp>) -> Bech32Address {
        Bech32Address::new(hrp.convert_unchecked(), self)
    }
}
