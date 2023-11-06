// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::vec;
use core::{fmt, ops::RangeInclusive, str::FromStr};

use derive_more::{AsRef, Display, From};
use iterator_sorted::is_unique_sorted;
use packable::{
    bounded::BoundedU8,
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    prefix::BoxedSlicePrefix,
    unpacker::Unpacker,
    Packable,
};

use crate::types::block::{address::Address, Error};

pub(crate) type WeightedAddressCount =
    BoundedU8<{ *MultiAddress::ADDRESSES_COUNT.start() }, { *MultiAddress::ADDRESSES_COUNT.end() }>;

/// An address with an assigned weight.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, AsRef, Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WeightedAddress {
    /// The unlocked address.
    #[packable(verify_with = verify_address)]
    address: Address,
    /// The weight of the unlocked address.
    #[packable(verify_with = verify_weight)]
    weight: u8,
}

impl WeightedAddress {
    /// Creates a new [`WeightedAddress`].
    pub fn new(address: Address, weight: u8) -> Result<WeightedAddress, Error> {
        verify_address::<true>(&address, &())?;
        verify_weight::<true>(&weight, &())?;

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

fn verify_address<const VERIFY: bool>(address: &Address, _visitor: &()) -> Result<(), Error> {
    if VERIFY && !address.is_ed25519() && !address.is_account() && !address.is_nft() {
        return Err(Error::InvalidAddressKind(address.kind()));
    } else {
        Ok(())
    }
}

fn verify_weight<const VERIFY: bool>(weight: &u8, _visitor: &()) -> Result<(), Error> {
    if VERIFY && *weight == 0 {
        return Err(Error::InvalidAddressWeight(*weight));
    } else {
        Ok(())
    }
}

/// An address that consists of addresses with weights and a threshold value.
/// The Multi Address can be unlocked if the cumulative weight of all unlocked addresses is equal to or exceeds the
/// threshold.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MultiAddress {
    /// The weighted unlocked addresses.
    addresses: BoxedSlicePrefix<WeightedAddress, WeightedAddressCount>,
    /// The threshold that needs to be reached by the unlocked addresses in order to unlock the multi address.
    threshold: u16,
}

impl MultiAddress {
    /// The [`Address`](crate::types::block::address::Address) kind of an [`MultiAddress`].
    pub const KIND: u8 = 40;
    /// The allowed range of inner [`Address`]es.
    pub const ADDRESSES_COUNT: RangeInclusive<u8> = 1..=10;

    /// Creates a new [`MultiAddress`].
    #[inline(always)]
    pub fn new(addresses: Vec<WeightedAddress>, threshold: u16) -> Result<Self, Error> {
        verify_addresses::<true>(&addresses, &())?;
        verify_threshold::<true>(&threshold, &())?;

        let addresses =
            BoxedSlicePrefix::<WeightedAddress, WeightedAddressCount>::try_from(addresses.into_boxed_slice())
                .map_err(Error::InvalidWeightedAddressCount)?;

        verify_cumulative_weight::<true>(&addresses, &threshold, &())?;

        Ok(Self { addresses, threshold })
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

impl Packable for MultiAddress {
    type UnpackError = Error;
    type UnpackVisitor = ();

    #[inline]
    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.addresses.pack(packer)?;
        self.threshold.pack(packer)?;

        Ok(())
    }

    #[inline]
    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let addresses =
            BoxedSlicePrefix::<WeightedAddress, WeightedAddressCount>::unpack::<_, VERIFY>(unpacker, visitor)
                .map_packable_err(|e| e.unwrap_item_err_or_else(|e| Error::InvalidWeightedAddressCount(e.into())))?;

        verify_addresses::<VERIFY>(&addresses, &()).map_err(UnpackError::Packable)?;

        let threshold = u16::unpack::<_, VERIFY>(unpacker, visitor).coerce()?;

        verify_threshold::<VERIFY>(&threshold, &()).map_err(UnpackError::Packable)?;
        verify_cumulative_weight::<VERIFY>(&addresses, &threshold, &()).map_err(UnpackError::Packable)?;

        Ok(Self { addresses, threshold })
    }
}

fn verify_addresses<const VERIFY: bool>(addresses: &[WeightedAddress], _visitor: &()) -> Result<(), Error> {
    if VERIFY && !is_unique_sorted(addresses.iter().map(WeightedAddress::address)) {
        return Err(Error::WeightedAddressesNotUniqueSorted);
    } else {
        Ok(())
    }
}

fn verify_threshold<const VERIFY: bool>(threshold: &u16, _visitor: &()) -> Result<(), Error> {
    if VERIFY && *threshold == 0 {
        return Err(Error::InvalidAddressWeightThreshold(*threshold));
    } else {
        Ok(())
    }
}

fn verify_cumulative_weight<const VERIFY: bool>(
    addresses: &[WeightedAddress],
    threshold: &u16,
    _visitor: &(),
) -> Result<(), Error> {
    if VERIFY {
        let cumulative_weight = addresses.iter().map(|address| address.weight as u16).sum::<u16>();

        if cumulative_weight < *threshold {
            return Err(Error::InvalidCumulativeAddressWeight {
                cumulative_weight,
                threshold: *threshold,
            });
        }
    }
    Ok(())
}

// impl FromStr for MultiAddress {
//     type Err = Error;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Ok(Self::new(AccountId::from_str(s)?))
//     }
// }

impl fmt::Display for MultiAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!()
    }
}

// impl core::fmt::Debug for MultiAddress {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         write!(f, "MultiAddress({self})")
//     }
// }

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
