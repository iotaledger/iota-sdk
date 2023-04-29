// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod parameters;

use alloc::{boxed::Box, collections::BTreeSet, vec::Vec};

use derive_more::{Deref, From};
use iterator_sorted::is_unique_sorted;
use packable::{bounded::BoundedU8, prefix::BoxedSlicePrefix, Packable};

pub(crate) use self::parameters::BinaryParametersLength;
pub use self::parameters::ParametersMilestoneOption;
use crate::types::block::{protocol::ProtocolParameters, Error};

///
#[derive(Clone, Debug, Eq, PartialEq, From, Packable)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", content = "data")
)]
#[packable(unpack_error = Error)]
#[packable(tag_type = u8, with_error = Error::InvalidMilestoneOptionKind)]
#[packable(unpack_visitor = ProtocolParameters)]
pub enum MilestoneOption {
    /// A parameters milestone option.
    #[packable(tag = ParametersMilestoneOption::KIND)]
    Parameters(ParametersMilestoneOption),
}

impl MilestoneOption {
    /// Return the milestone option kind of a [`MilestoneOption`].
    pub fn kind(&self) -> u8 {
        match self {
            Self::Parameters(_) => ParametersMilestoneOption::KIND,
        }
    }
}

impl PartialOrd for MilestoneOption {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.kind().partial_cmp(&other.kind())
    }
}
impl Ord for MilestoneOption {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub(crate) type MilestoneOptionCount = BoundedU8<0, { MilestoneOptions::COUNT_MAX }>;

///
#[derive(Clone, Debug, Eq, PartialEq, Deref, Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[packable(unpack_error = Error, with = |e| e.unwrap_item_err_or_else(|p| Error::InvalidMilestoneOptionCount(p.into())))]
#[packable(unpack_visitor = ProtocolParameters)]
pub struct MilestoneOptions(
    #[packable(verify_with = verify_unique_sorted_packable)] BoxedSlicePrefix<MilestoneOption, MilestoneOptionCount>,
);

impl TryFrom<Vec<MilestoneOption>> for MilestoneOptions {
    type Error = Error;

    #[inline(always)]
    fn try_from(milestone_options: Vec<MilestoneOption>) -> Result<Self, Self::Error> {
        Self::from_vec(milestone_options)
    }
}

impl TryFrom<BTreeSet<MilestoneOption>> for MilestoneOptions {
    type Error = Error;

    #[inline(always)]
    fn try_from(milestone_options: BTreeSet<MilestoneOption>) -> Result<Self, Self::Error> {
        Self::from_set(milestone_options)
    }
}

impl IntoIterator for MilestoneOptions {
    type Item = MilestoneOption;
    type IntoIter = alloc::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::from(Into::<Box<[MilestoneOption]>>::into(self.0)).into_iter()
    }
}

impl MilestoneOptions {
    ///
    pub const COUNT_MAX: u8 = 2;

    /// Creates a new [`MilestoneOptions`] from a vec.
    pub fn from_vec(milestone_options: Vec<MilestoneOption>) -> Result<Self, Error> {
        let mut milestone_options =
            BoxedSlicePrefix::<MilestoneOption, MilestoneOptionCount>::try_from(milestone_options.into_boxed_slice())
                .map_err(Error::InvalidMilestoneOptionCount)?;

        milestone_options.sort_by_key(MilestoneOption::kind);
        // Sort is obviously fine now but uniqueness still needs to be checked.
        verify_unique_sorted::<true>(&milestone_options)?;

        Ok(Self(milestone_options))
    }

    /// Creates a new [`MilestoneOptions`] from an ordered set.
    pub fn from_set(milestone_options: BTreeSet<MilestoneOption>) -> Result<Self, Error> {
        Ok(Self(
            milestone_options
                .into_iter()
                .collect::<Box<[_]>>()
                .try_into()
                .map_err(Error::InvalidMilestoneOptionCount)?,
        ))
    }

    /// Gets a reference to a [`MilestoneOption`] from a milestone option kind, if any.
    #[inline(always)]
    pub fn get(&self, key: u8) -> Option<&MilestoneOption> {
        self.0
            .binary_search_by_key(&key, MilestoneOption::kind)
            // PANIC: indexation is fine since the index has been found.
            .map(|index| &self.0[index])
            .ok()
    }

    /// Gets a reference to a [`ParametersMilestoneOption`], if any.
    pub fn parameters(&self) -> Option<&ParametersMilestoneOption> {
        if let Some(MilestoneOption::Parameters(parameters)) = self.get(ParametersMilestoneOption::KIND) {
            Some(parameters)
        } else {
            None
        }
    }
}

#[inline]
fn verify_unique_sorted<const VERIFY: bool>(milestone_options: &[MilestoneOption]) -> Result<(), Error> {
    if VERIFY && !is_unique_sorted(milestone_options.iter().map(MilestoneOption::kind)) {
        Err(Error::MilestoneOptionsNotUniqueSorted)
    } else {
        Ok(())
    }
}

#[inline]
fn verify_unique_sorted_packable<const VERIFY: bool>(
    milestone_options: &[MilestoneOption],
    _visitor: &ProtocolParameters,
) -> Result<(), Error> {
    verify_unique_sorted::<VERIFY>(milestone_options)
}

#[allow(missing_docs)]
pub mod dto {
    use alloc::format;

    use serde::{Deserialize, Serialize, Serializer};
    use serde_json::Value;

    pub use self::parameters::dto::ParametersMilestoneOptionDto;
    use super::*;
    use crate::types::block::Error;

    #[derive(Clone, Debug, Eq, PartialEq, From)]
    pub enum MilestoneOptionDto {
        /// A parameters milestone option.
        Parameters(ParametersMilestoneOptionDto),
    }

    impl<'de> Deserialize<'de> for MilestoneOptionDto {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let value = Value::deserialize(d)?;
            Ok(
                match value
                    .get("type")
                    .and_then(Value::as_u64)
                    .ok_or_else(|| serde::de::Error::custom("invalid milestone option type"))?
                    as u8
                {
                    ParametersMilestoneOption::KIND => {
                        Self::Parameters(ParametersMilestoneOptionDto::deserialize(value).map_err(|e| {
                            serde::de::Error::custom(format!("cannot deserialize parameters milestone option: {e}"))
                        })?)
                    }
                    _ => return Err(serde::de::Error::custom("invalid milestone option type")),
                },
            )
        }
    }

    impl Serialize for MilestoneOptionDto {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            #[derive(Serialize)]
            #[serde(untagged)]
            enum MilestoneOptionDto_<'a> {
                T2(&'a ParametersMilestoneOptionDto),
            }
            #[derive(Serialize)]
            struct TypedMilestoneOption<'a> {
                #[serde(flatten)]
                milestone_option: MilestoneOptionDto_<'a>,
            }
            let milestone_option = match self {
                Self::Parameters(o) => TypedMilestoneOption {
                    milestone_option: MilestoneOptionDto_::T2(o),
                },
            };
            milestone_option.serialize(serializer)
        }
    }

    impl From<&MilestoneOption> for MilestoneOptionDto {
        fn from(value: &MilestoneOption) -> Self {
            match value {
                MilestoneOption::Parameters(v) => Self::Parameters(v.into()),
            }
        }
    }

    impl MilestoneOption {
        pub fn try_from_dto(value: &MilestoneOptionDto) -> Result<Self, Error> {
            Ok(match value {
                MilestoneOptionDto::Parameters(v) => Self::Parameters(v.try_into()?),
            })
        }

        pub fn try_from_dto_unverified(value: &MilestoneOptionDto) -> Result<Self, Error> {
            Ok(match value {
                MilestoneOptionDto::Parameters(v) => Self::Parameters(v.try_into()?),
            })
        }
    }

    impl MilestoneOptionDto {
        /// Returns the milestone option kind of a [`MilestoneOptionDto`].
        pub fn kind(&self) -> u8 {
            match self {
                Self::Parameters(_) => ParametersMilestoneOption::KIND,
            }
        }
    }
}
