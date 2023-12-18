// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{boxed::Box, string::ToString, vec::Vec};
use core::{fmt, ops::RangeInclusive};

use derive_more::{AsRef, Deref, Display, From};
use iterator_sorted::is_unique_sorted;
use packable::{bounded::BoundedU8, prefix::BoxedSlicePrefix, Packable};

use crate::types::block::{address::Address, output::StorageScore, Error};

pub(crate) type WeightedAddressCount =
    BoundedU8<{ *MultiAddress::ADDRESSES_COUNT.start() }, { *MultiAddress::ADDRESSES_COUNT.end() }>;

/// An address with an assigned weight.
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
    pub fn new(address: Address, weight: u8) -> Result<Self, Error> {
        verify_address::<true>(&address)?;
        verify_weight::<true>(&weight)?;

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

fn verify_address<const VERIFY: bool>(address: &Address) -> Result<(), Error> {
    if VERIFY
        && !matches!(
            address,
            Address::Ed25519(_) | Address::Account(_) | Address::Nft(_) | Address::Anchor(_)
        )
    {
        Err(Error::InvalidAddressKind(address.kind()))
    } else {
        Ok(())
    }
}

fn verify_weight<const VERIFY: bool>(weight: &u8) -> Result<(), Error> {
    if VERIFY && *weight == 0 {
        Err(Error::InvalidAddressWeight(*weight))
    } else {
        Ok(())
    }
}

/// An address that consists of addresses with weights and a threshold value.
/// The Multi Address can be unlocked if the cumulative weight of all unlocked addresses is equal to or exceeds the
/// threshold.
#[derive(Clone, Debug, Deref, Eq, PartialEq, Ord, PartialOrd, Hash, Packable)]
#[packable(unpack_error = Error)]
#[packable(verify_with = verify_multi_address)]
pub struct MultiAddress {
    /// The weighted unlocked addresses.
    #[deref]
    #[packable(verify_with = verify_addresses)]
    #[packable(unpack_error_with = |e| e.unwrap_item_err_or_else(|p| Error::InvalidWeightedAddressCount(p.into())))]
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
    pub fn new(addresses: impl IntoIterator<Item = WeightedAddress>, threshold: u16) -> Result<Self, Error> {
        let mut addresses = addresses.into_iter().collect::<Box<[_]>>();

        addresses.sort_by(|a, b| a.address().cmp(b.address()));

        verify_threshold::<true>(&threshold)?;

        let addresses = BoxedSlicePrefix::<WeightedAddress, WeightedAddressCount>::try_from(addresses)
            .map_err(Error::InvalidWeightedAddressCount)?;

        let multi_address = Self { addresses, threshold };

        verify_multi_address::<true>(&multi_address)?;

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
}

fn verify_addresses<const VERIFY: bool>(addresses: &[WeightedAddress]) -> Result<(), Error> {
    if VERIFY && !is_unique_sorted(addresses.iter().map(WeightedAddress::address)) {
        Err(Error::WeightedAddressesNotUniqueSorted)
    } else {
        Ok(())
    }
}

fn verify_threshold<const VERIFY: bool>(threshold: &u16) -> Result<(), Error> {
    if VERIFY && *threshold == 0 {
        Err(Error::InvalidMultiAddressThreshold(*threshold))
    } else {
        Ok(())
    }
}

fn verify_multi_address<const VERIFY: bool>(address: &MultiAddress) -> Result<(), Error> {
    if VERIFY {
        let cumulative_weight = address.iter().map(|address| address.weight as u16).sum::<u16>();

        if cumulative_weight < address.threshold {
            return Err(Error::InvalidMultiAddressCumulativeWeight {
                cumulative_weight,
                threshold: address.threshold,
            });
        }
    }
    Ok(())
}

impl StorageScore for MultiAddress {}

impl fmt::Display for MultiAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.addresses()
                .iter()
                .map(|address| address.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
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
        type Error = Error;

        fn try_from(value: MultiAddressDto) -> Result<Self, Self::Error> {
            Self::new(value.addresses, value.threshold)
        }
    }

    crate::impl_serde_typed_dto!(MultiAddress, MultiAddressDto, "multi address");
}
