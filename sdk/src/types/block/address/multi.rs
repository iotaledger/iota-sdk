// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::vec;
use core::{fmt, ops::RangeInclusive, str::FromStr};

use derive_more::{AsRef, Display, From};
use packable::{bounded::BoundedU8, prefix::BoxedSlicePrefix, Packable};

use crate::types::block::{address::Address, Error};

pub(crate) type WeightedAddressCount =
    BoundedU8<{ *MultiAddress::ADDRESSES_COUNT.start() }, { *MultiAddress::ADDRESSES_COUNT.end() }>;

// context_inputs: BoxedSlicePrefix<ContextInput, ContextInputCount>,

/// An address with an assigned weight.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, AsRef, Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WeightedAddress {
    #[packable(verify_with = verify_address)]
    address: Address,
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
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Packable)]
#[packable(unpack_error = Error)]
pub struct MultiAddress {
    // #[packable(verify_with = verify_context_inputs_packable)]
    #[packable(unpack_error_with = |e| e.unwrap_item_err_or_else(|p| Error::InvalidWeightedAddressCount(p.into())))]
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
        Ok(Self {
            addresses: BoxedSlicePrefix::<WeightedAddress, WeightedAddressCount>::try_from(
                addresses.into_boxed_slice(),
            )
            .map_err(Error::InvalidWeightedAddressCount)?,
            threshold,
        })
    }

    // /// Returns the [`AccountId`] of an [`MultiAddress`].
    // #[inline(always)]
    // pub fn account_id(&self) -> &AccountId {
    //     &self.0
    // }

    // /// Consumes an [`MultiAddress`] and returns its [`AccountId`].
    // #[inline(always)]
    // pub fn into_account_id(self) -> AccountId {
    //     self.0
    // }
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
