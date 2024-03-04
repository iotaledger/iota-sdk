// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{boxed::Box, collections::BTreeMap, vec::Vec};
use core::{fmt, ops::RangeInclusive};

use crypto::hashes::{blake2b::Blake2b256, Digest};
use derive_more::{AsRef, Deref, Display, From};
use iterator_sorted::is_unique_sorted;
use packable::{bounded::BoundedU8, prefix::BoxedSlicePrefix, Packable, PackableExt};

use crate::types::block::{
    address::{Address, AddressError},
    output::StorageScore,
};

/// An [`Address`] with an assigned weight.
#[derive(Clone, Debug, Display, Eq, PartialEq, Ord, PartialOrd, Hash, From, AsRef, Deref, Packable)]
#[display(fmt = "{address}")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WeightedAddress {
    /// The unlocked address.
    #[deref]
    #[packable(verify_with = verify_address)]
    address: Address,
    /// The weight of the unlocked address.
    #[packable(verify_with = verify_weight)]
    weight: u8,
}

impl WeightedAddress {
    /// Creates a new [`WeightedAddress`].
    pub fn new(address: impl Into<Address>, weight: u8) -> Result<Self, AddressError> {
        let address = address.into();

        verify_address(&address)?;
        verify_weight(&weight)?;

        Ok(Self { address, weight })
    }

    /// Returns the address of the [`WeightedAddress`].
    pub fn address(&self) -> &Address {
        &self.address
    }

    /// Returns the weight of the [`WeightedAddress`].
    pub fn weight(&self) -> u8 {
        self.weight
    }
}

fn verify_address(address: &Address) -> Result<(), AddressError> {
    if !matches!(
        address,
        Address::Ed25519(_) | Address::Account(_) | Address::Nft(_) | Address::Anchor(_)
    ) {
        Err(AddressError::Kind(address.kind()))
    } else {
        Ok(())
    }
}

fn verify_weight(weight: &u8) -> Result<(), AddressError> {
    if *weight == 0 {
        Err(AddressError::Weight(*weight))
    } else {
        Ok(())
    }
}

pub(crate) type WeightedAddressCount =
    BoundedU8<{ *MultiAddress::ADDRESSES_COUNT.start() }, { *MultiAddress::ADDRESSES_COUNT.end() }>;

/// An [`Address`] that consists of addresses with weights and a threshold value.
/// It can be unlocked if the cumulative weight of all unlocked addresses is equal to or exceeds the threshold.
#[derive(Clone, Debug, Deref, Eq, PartialEq, Ord, PartialOrd, Hash, Packable)]
#[packable(unpack_error = AddressError)]
#[packable(verify_with = verify_multi_address)]
pub struct MultiAddress {
    /// The weighted unlocked addresses.
    #[deref]
    #[packable(verify_with = verify_addresses)]
    #[packable(unpack_error_with = |e| e.unwrap_item_err_or_else(|p| AddressError::WeightedAddressCount(p.into())))]
    addresses: BoxedSlicePrefix<WeightedAddress, WeightedAddressCount>,
    /// The threshold that needs to be reached by the unlocked addresses in order to unlock the multi address.
    #[packable(verify_with = verify_threshold)]
    threshold: u16,
}

impl MultiAddress {
    /// The [`Address`] kind of a [`MultiAddress`].
    pub const KIND: u8 = 40;
    /// The allowed range of inner [`Address`]es.
    pub const ADDRESSES_COUNT: RangeInclusive<u8> = 2..=10;

    /// Creates a new [`MultiAddress`].
    #[inline(always)]
    pub fn new(addresses: impl IntoIterator<Item = WeightedAddress>, threshold: u16) -> Result<Self, AddressError> {
        // Using an intermediate BTreeMap to sort the addresses without having to repeatedly packing them.
        let addresses = addresses
            .into_iter()
            .map(|address| (address.address().pack_to_vec(), address))
            .collect::<BTreeMap<_, _>>()
            .into_values()
            .collect::<Box<[_]>>();

        verify_addresses(&addresses)?;
        verify_threshold(&threshold)?;

        let addresses = BoxedSlicePrefix::<WeightedAddress, WeightedAddressCount>::try_from(addresses)
            .map_err(AddressError::WeightedAddressCount)?;

        let multi_address = Self { addresses, threshold };

        verify_multi_address(&multi_address)?;

        Ok(multi_address)
    }

    /// Returns the addresses of a [`MultiAddress`].
    #[inline(always)]
    pub fn addresses(&self) -> &[WeightedAddress] {
        &self.addresses
    }

    /// Returns the threshold of a [`MultiAddress`].
    #[inline(always)]
    pub fn threshold(&self) -> u16 {
        self.threshold
    }

    /// Hash the [`MultiAddress`] with BLAKE2b-256.
    #[inline(always)]
    pub fn hash(&self) -> [u8; 32] {
        let mut digest = Blake2b256::new();

        digest.update([Self::KIND]);
        digest.update(self.pack_to_vec());

        digest.finalize().into()
    }
}

fn verify_addresses(addresses: &[WeightedAddress]) -> Result<(), AddressError> {
    if !is_unique_sorted(addresses.iter().map(|a| a.address.pack_to_vec())) {
        Err(AddressError::WeightedAddressesNotUniqueSorted)
    } else {
        Ok(())
    }
}

fn verify_threshold(threshold: &u16) -> Result<(), AddressError> {
    if *threshold == 0 {
        Err(AddressError::MultiAddressThreshold(*threshold))
    } else {
        Ok(())
    }
}

fn verify_multi_address(address: &MultiAddress) -> Result<(), AddressError> {
    let cumulative_weight = address.iter().map(|address| address.weight as u16).sum::<u16>();

    if cumulative_weight < address.threshold {
        return Err(AddressError::MultiAddressCumulativeWeight {
            cumulative_weight,
            threshold: address.threshold,
        });
    }

    Ok(())
}

impl StorageScore for MultiAddress {}

impl fmt::Display for MultiAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", prefix_hex::encode(self.hash()))
    }
}

#[cfg(feature = "serde")]
mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct MultiAddressDto {
        #[serde(rename = "type")]
        kind: u8,
        addresses: Vec<WeightedAddress>,
        threshold: u16,
    }

    impl From<&MultiAddress> for MultiAddressDto {
        fn from(value: &MultiAddress) -> Self {
            Self {
                kind: MultiAddress::KIND,
                addresses: value.addresses.to_vec(),
                threshold: value.threshold,
            }
        }
    }

    impl TryFrom<MultiAddressDto> for MultiAddress {
        type Error = AddressError;

        fn try_from(value: MultiAddressDto) -> Result<Self, Self::Error> {
            Self::new(value.addresses, value.threshold)
        }
    }

    crate::impl_serde_typed_dto!(MultiAddress, MultiAddressDto, "multi address");
}
