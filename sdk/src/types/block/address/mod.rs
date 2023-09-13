// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod account;
mod bech32;
mod ed25519;
mod nft;

use derive_more::{Deref, From};
use packable::{error::UnpackErrorExt, Packable};

use self::ed25519::ImplicitAccountCreationAddress;
pub use self::{
    account::AccountAddress,
    bech32::{Bech32Address, Hrp},
    ed25519::Ed25519Address,
    nft::NftAddress,
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
    #[packable(tag = Restricted::<Ed25519Address>::KIND)]
    RestrictedEd25519(Restricted<Ed25519Address>),
    /// An account address.
    #[packable(tag = AccountAddress::KIND)]
    Account(AccountAddress),
    /// A restricted account address.
    #[packable(tag = Restricted::<AccountAddress>::KIND)]
    RestrictedAccount(Restricted<AccountAddress>),
    /// An NFT address.
    #[packable(tag = NftAddress::KIND)]
    Nft(NftAddress),
    /// A restricted NFT address.
    #[packable(tag = Restricted::<NftAddress>::KIND)]
    RestrictedNft(Restricted<NftAddress>),
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
            Self::RestrictedEd25519(_) => Restricted::<Ed25519Address>::KIND,
            Self::Account(_) => AccountAddress::KIND,
            Self::RestrictedAccount(_) => Restricted::<AccountAddress>::KIND,
            Self::Nft(_) => NftAddress::KIND,
            Self::RestrictedNft(_) => Restricted::<NftAddress>::KIND,
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
                    return Err(TransactionFailureReason::InvalidUnlock);
                }

                let Signature::Ed25519(signature) = unlock.signature();

                if signature.is_valid(&context.essence_hash, ed25519_address).is_err() {
                    return Err(TransactionFailureReason::InvalidSignature);
                }

                context.unlocked_addresses.insert(*self);
            }
            (Self::Ed25519(_ed25519_address), Unlock::Reference(_unlock)) => {
                // TODO actually check that it was unlocked by the same signature.
                if !context.unlocked_addresses.contains(self) {
                    return Err(TransactionFailureReason::InvalidUnlock);
                }
            }
            (Self::Account(account_address), Unlock::Account(unlock)) => {
                // PANIC: indexing is fine as it is already syntactically verified that indexes reference below.
                if let (output_id, Output::Account(account_output)) = inputs[unlock.index() as usize] {
                    if &account_output.account_id_non_null(output_id) != account_address.account_id() {
                        return Err(TransactionFailureReason::InvalidUnlock);
                    }
                    if !context.unlocked_addresses.contains(self) {
                        return Err(TransactionFailureReason::InvalidUnlock);
                    }
                } else {
                    return Err(TransactionFailureReason::InvalidUnlock);
                }
            }
            (Self::Nft(nft_address), Unlock::Nft(unlock)) => {
                // PANIC: indexing is fine as it is already syntactically verified that indexes reference below.
                if let (output_id, Output::Nft(nft_output)) = inputs[unlock.index() as usize] {
                    if &nft_output.nft_id_non_null(output_id) != nft_address.nft_id() {
                        return Err(TransactionFailureReason::InvalidUnlock);
                    }
                    if !context.unlocked_addresses.contains(self) {
                        return Err(TransactionFailureReason::InvalidUnlock);
                    }
                } else {
                    return Err(TransactionFailureReason::InvalidUnlock);
                }
            }
            _ => return Err(TransactionFailureReason::InvalidUnlock),
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, getset::Getters)]
#[getset(get = "pub")]
pub struct Restricted<A> {
    address: A,
    allowed_capabilities: Option<Capabilities>,
}

impl<A> Restricted<A> {
    /// Creates a new [`Restricted`] address from the underlying type.
    #[inline(always)]
    pub fn new(address: A) -> Self {
        Self {
            address,
            allowed_capabilities: Default::default(),
        }
    }

    /// Sets the allowed capabilities flags.
    #[inline(always)]
    pub fn with_allowed_capabilities(mut self, allowed_capabilities: impl Into<Capabilities>) -> Self {
        self.allowed_capabilities.replace(allowed_capabilities.into());
        self
    }

    /// Sets the allowed capabilities flags.
    #[inline(always)]
    pub fn set_allowed_capabilities(&mut self, allowed_capabilities: impl Into<Capabilities>) -> &mut Self {
        self.allowed_capabilities.replace(allowed_capabilities.into());
        self
    }
}

impl<A> From<A> for Restricted<A> {
    fn from(value: A) -> Self {
        Self::new(value)
    }
}

impl<A: 'static + Packable> Packable for Restricted<A>
where
    Error: From<A::UnpackError>,
{
    type UnpackError = Error;
    type UnpackVisitor = A::UnpackVisitor;

    fn pack<P: packable::packer::Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.address.pack(packer)?;
        if self
            .allowed_capabilities
            .map(|c| c.0 != CapabilityFlag::NONE)
            .unwrap_or_default()
        {
            self.allowed_capabilities.pack(packer)?;
        } else {
            0_u8.pack(packer)?;
        }
        Ok(())
    }

    fn unpack<U: packable::unpacker::Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, packable::error::UnpackError<Self::UnpackError, U::Error>> {
        let address = A::unpack::<_, VERIFY>(unpacker, visitor).coerce()?;
        let allowed_capabilities_set = u8::unpack::<_, VERIFY>(unpacker, &()).coerce()? != 0;
        let allowed_capabilities = allowed_capabilities_set
            .then(|| Capabilities::unpack::<_, VERIFY>(unpacker, &()).coerce())
            .transpose()?;
        Ok(Self {
            address,
            allowed_capabilities,
        })
    }
}

pub struct CapabilityFlag;

impl CapabilityFlag {
    pub const NATIVE_TOKENS: u8 = 0b00000001;
    pub const MANA: u8 = 0b00000010;
    pub const TIMELOCKED_OUTPUTS: u8 = 0b00000100;
    pub const EXPIRING_OUTPUTS: u8 = 0b00001000;
    pub const STORAGE_DEPOSIT_OUTPUTS: u8 = 0b00010000;
    pub const ACCOUNT_OUTPUTS: u8 = 0b00100000;
    pub const NFT_OUTPUTS: u8 = 0b01000000;
    pub const DELEGATION_OUTPUTS: u8 = 0b10000000;
    pub const NONE: u8 = 0;
    pub const ALL: u8 = u8::MAX;
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, From, Deref, Packable)]
#[repr(transparent)]
pub struct Capabilities(u8);

impl Capabilities {
    pub fn with_capabilities(mut self, flags: impl Into<u8>) -> Self {
        self.0 |= flags.into();
        self
    }

    pub fn add_capabilities(&mut self, flags: impl Into<u8>) -> &mut Self {
        self.0 |= flags.into();
        self
    }

    pub fn set_capabilities(&mut self, flags: impl Into<u8>) -> &mut Self {
        self.0 = flags.into();
        self
    }

    pub fn has_capabilities(&self, flags: impl Into<u8>) -> bool {
        self.0 & flags.into() != 0
    }
}

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use serde::{Deserialize, Serialize};

    use crate::utils::serde::prefix_hex_bytes;

    /// Describes a restricted Ed25519 address.
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RestrictedDto<A> {
        #[serde(flatten)]
        pub address: A,
        // TODO: is this format right?
        #[serde(with = "prefix_hex_bytes")]
        pub allowed_capabilities: Vec<u8>,
    }

    impl<A> core::ops::Deref for RestrictedDto<A> {
        type Target = A;

        fn deref(&self) -> &Self::Target {
            &self.address
        }
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
        let address = Restricted::new(rand_ed25519_address()).with_allowed_capabilities(0);
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
        let mut address = Restricted::new(address);
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
