// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod account;
mod bech32;
mod ed25519;
mod implicit_account_creation;
mod nft;
mod restricted;

use derive_more::From;
use packable::Packable;

pub use self::{
    account::AccountAddress,
    bech32::{Bech32Address, Hrp},
    ed25519::Ed25519Address,
    implicit_account_creation::ImplicitAccountCreationAddress,
    nft::NftAddress,
    restricted::{CapabilityFlag, RestrictedAddress},
};
use crate::types::block::{
    output::{Output, OutputId},
    semantic::{TransactionFailureReason, ValidationContext},
    signature::Signature,
    unlock::Unlock,
    ConvertTo, Error,
};

/// A generic address supporting different address kinds.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, From, Packable)]
#[packable(tag_type = u8, with_error = Error::InvalidAddressKind)]
#[packable(unpack_error = Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(untagged))]
pub enum Address {
    /// An Ed25519 address.
    #[packable(tag = Ed25519Address::KIND)]
    Ed25519(Ed25519Address),
    /// A restricted Ed25519 address.
    #[packable(tag = RestrictedAddress::<Ed25519Address>::KIND)]
    RestrictedEd25519(RestrictedAddress<Ed25519Address>),
    /// An account address.
    #[packable(tag = AccountAddress::KIND)]
    Account(AccountAddress),
    /// A restricted account address.
    #[packable(tag = RestrictedAddress::<AccountAddress>::KIND)]
    RestrictedAccount(RestrictedAddress<AccountAddress>),
    /// An NFT address.
    #[packable(tag = NftAddress::KIND)]
    Nft(NftAddress),
    /// A restricted NFT address.
    #[packable(tag = RestrictedAddress::<NftAddress>::KIND)]
    RestrictedNft(RestrictedAddress<NftAddress>),
    /// An implicit account creation address.
    #[packable(tag = ImplicitAccountCreationAddress::KIND)]
    ImplicitAccountCreation(ImplicitAccountCreationAddress),
}

impl core::fmt::Debug for Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Ed25519(address) => address.fmt(f),
            Self::RestrictedEd25519(address) => address.fmt(f),
            Self::Account(address) => address.fmt(f),
            Self::RestrictedAccount(address) => address.fmt(f),
            Self::Nft(address) => address.fmt(f),
            Self::RestrictedNft(address) => address.fmt(f),
            Self::ImplicitAccountCreation(address) => address.fmt(f),
        }
    }
}

impl Address {
    /// Returns the address kind of an [`Address`].
    pub fn kind(&self) -> u8 {
        match self {
            Self::Ed25519(_) => Ed25519Address::KIND,
            Self::RestrictedEd25519(_) => RestrictedAddress::<Ed25519Address>::KIND,
            Self::Account(_) => AccountAddress::KIND,
            Self::RestrictedAccount(_) => RestrictedAddress::<AccountAddress>::KIND,
            Self::Nft(_) => NftAddress::KIND,
            Self::RestrictedNft(_) => RestrictedAddress::<NftAddress>::KIND,
            Self::ImplicitAccountCreation(_) => ImplicitAccountCreationAddress::KIND,
        }
    }

    /// Checks whether the address is an [`Ed25519Address`].
    pub fn is_ed25519(&self) -> bool {
        matches!(self, Self::Ed25519(_))
    }

    /// Gets the address as an actual [`Ed25519Address`].
    /// PANIC: do not call on a non-ed25519 address.
    pub fn as_ed25519(&self) -> &Ed25519Address {
        if let Self::Ed25519(address) = self {
            address
        } else {
            panic!("as_ed25519 called on a non-ed25519 address");
        }
    }

    /// Checks whether the address is an [`AccountAddress`].
    pub fn is_account(&self) -> bool {
        matches!(self, Self::Account(_))
    }

    /// Gets the address as an actual [`AccountAddress`].
    /// PANIC: do not call on a non-account address.
    pub fn as_account(&self) -> &AccountAddress {
        if let Self::Account(address) = self {
            address
        } else {
            panic!("as_account called on a non-account address");
        }
    }

    /// Checks whether the address is an [`NftAddress`].
    pub fn is_nft(&self) -> bool {
        matches!(self, Self::Nft(_))
    }

    /// Gets the address as an actual [`NftAddress`].
    /// PANIC: do not call on a non-nft address.
    pub fn as_nft(&self) -> &NftAddress {
        if let Self::Nft(address) = self {
            address
        } else {
            panic!("as_nft called on a non-nft address");
        }
    }

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
        inputs: &[(&OutputId, &Output)],
        context: &mut ValidationContext<'_>,
    ) -> Result<(), TransactionFailureReason> {
        match (self, unlock) {
            (Self::Ed25519(ed25519_address), Unlock::Signature(unlock)) => {
                if context.unlocked_addresses.contains(self) {
                    return Err(TransactionFailureReason::InvalidInputUnlock);
                }

                let Signature::Ed25519(signature) = unlock.signature();

                if signature.is_valid(&context.essence_hash, ed25519_address).is_err() {
                    return Err(TransactionFailureReason::InvalidUnlockBlockSignature);
                }

                context.unlocked_addresses.insert(*self);
            }
            (Self::Ed25519(_ed25519_address), Unlock::Reference(_unlock)) => {
                // TODO actually check that it was unlocked by the same signature.
                if !context.unlocked_addresses.contains(self) {
                    return Err(TransactionFailureReason::InvalidInputUnlock);
                }
            }
            (Self::Account(account_address), Unlock::Account(unlock)) => {
                // PANIC: indexing is fine as it is already syntactically verified that indexes reference below.
                if let (output_id, Output::Account(account_output)) = inputs[unlock.index() as usize] {
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
                if let (output_id, Output::Nft(nft_output)) = inputs[unlock.index() as usize] {
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
            _ => return Err(TransactionFailureReason::InvalidInputUnlock),
        }

        Ok(())
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
    /// Try to encode this address to a bech32 string with the given Human Readable Part as prefix.
    fn try_to_bech32(self, hrp: impl ConvertTo<Hrp>) -> Result<Bech32Address, Error> {
        Bech32Address::try_new(hrp, self)
    }

    /// Encodes this address to a bech32 string with the given Human Readable Part as prefix.
    fn to_bech32(self, hrp: Hrp) -> Bech32Address {
        Bech32Address::new(hrp, self)
    }

    /// Encodes this address to a bech32 string with the given Human Readable Part as prefix without checking
    /// validity.
    fn to_bech32_unchecked(self, hrp: impl ConvertTo<Hrp>) -> Bech32Address {
        Bech32Address::new(hrp.convert_unchecked(), self)
    }
}

impl From<&Self> for Address {
    fn from(value: &Self) -> Self {
        *value
    }
}

#[cfg(test)]
mod test {
    use crypto::{
        hashes::{blake2b::Blake2b256, Digest},
        signatures::ed25519::PublicKey,
    };

    use super::*;
    use crate::types::block::rand::address::rand_ed25519_address;

    #[test]
    fn capabilities() {
        let address = RestrictedAddress::new(rand_ed25519_address()).with_allowed_capabilities(0);
        let mut capabilities = address.allowed_capabilities().unwrap();
        assert!(!capabilities.has_capabilities(CapabilityFlag::NATIVE_TOKENS));
        capabilities.add_capabilities(CapabilityFlag::NATIVE_TOKENS);
        assert!(capabilities.has_capabilities(CapabilityFlag::NATIVE_TOKENS));

        assert!(!capabilities.has_capabilities(CapabilityFlag::MANA));
        capabilities.set_capabilities(CapabilityFlag::MANA | CapabilityFlag::DELEGATION_OUTPUTS);
        assert!(capabilities.has_capabilities(CapabilityFlag::MANA));
        assert!(capabilities.has_capabilities(CapabilityFlag::DELEGATION_OUTPUTS));
        assert!(!capabilities.has_capabilities(CapabilityFlag::NATIVE_TOKENS));
    }

    #[test]
    fn bech32() {
        // Data from https://github.com/iotaledger/tips-draft/blob/tip50/tips/TIP-0050/tip-0050.md#bech32-strings
        let address = Ed25519Address::new(
            Blake2b256::digest(
                PublicKey::try_from_bytes(
                    hex::decode("6f1581709bb7b1ef030d210db18e3b0ba1c776fba65d8cdaad05415142d189f8")
                        .unwrap()
                        .try_into()
                        .unwrap(),
                )
                .unwrap()
                .to_bytes(),
            )
            .try_into()
            .unwrap(),
        );
        assert_eq!(
            hex::encode(address),
            "efdc112efe262b304bcf379b26c31bad029f616ee3ec4aa6345a366e4c9e43a3"
        );
        assert_eq!(
            address.to_bech32_unchecked("iota").to_string(),
            "iota1qrhacyfwlcnzkvzteumekfkrrwks98mpdm37cj4xx3drvmjvnep6xqgyzyx"
        );
        let mut address = RestrictedAddress::new(address);
        assert_eq!(
            address.to_bech32_unchecked("iota").to_string(),
            "iota1q8hacyfwlcnzkvzteumekfkrrwks98mpdm37cj4xx3drvmjvnep6xqq7ar5ue"
        );
        address.set_allowed_capabilities(CapabilityFlag::ALL);
        assert_eq!(
            address.to_bech32_unchecked("iota").to_string(),
            "iota1q8hacyfwlcnzkvzteumekfkrrwks98mpdm37cj4xx3drvmjvnep6xq0l828jhc"
        );
        address.set_allowed_capabilities(CapabilityFlag::NONE);
        assert_eq!(
            address.to_bech32_unchecked("iota").to_string(),
            "iota1q8hacyfwlcnzkvzteumekfkrrwks98mpdm37cj4xx3drvmjvnep6xqq7ar5ue"
        );
        address.set_allowed_capabilities(
            CapabilityFlag::NFT_OUTPUTS | CapabilityFlag::ACCOUNT_OUTPUTS | CapabilityFlag::DELEGATION_OUTPUTS,
        );
        assert_eq!(
            address.to_bech32_unchecked("iota").to_string(),
            "iota1q8hacyfwlcnzkvzteumekfkrrwks98mpdm37cj4xx3drvmjvnep6xq0qdxukan"
        );
        let address = ImplicitAccountCreationAddress::from(*address.address());
        assert_eq!(
            address.to_bech32_unchecked("iota").to_string(),
            "iota1rrhacyfwlcnzkvzteumekfkrrwks98mpdm37cj4xx3drvmjvnep6xg4ad2d"
        );
    }
}
