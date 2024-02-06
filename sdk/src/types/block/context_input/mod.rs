// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod block_issuance_credit;
mod commitment;
mod reward;

use core::{cmp::Ordering, ops::RangeInclusive};

use derive_more::{Deref, Display, From};
use iterator_sorted::is_unique_sorted_by;
use packable::{bounded::BoundedU16, prefix::BoxedSlicePrefix, Packable};

pub(crate) use self::reward::RewardContextInputIndex;
pub use self::{
    block_issuance_credit::BlockIssuanceCreditContextInput, commitment::CommitmentContextInput,
    reward::RewardContextInput,
};
use crate::types::block::{
    protocol::{WorkScore, WorkScoreParameters},
    Error,
};

/// The maximum number of context inputs of a transaction.
pub const CONTEXT_INPUT_COUNT_MAX: u16 = 128;
/// The range of valid numbers of context inputs of a transaction.
pub const CONTEXT_INPUT_COUNT_RANGE: RangeInclusive<u16> = 0..=CONTEXT_INPUT_COUNT_MAX; // [0..128]

/// A Context Input provides additional contextual information for the execution of a transaction, such as for different
/// functionality related to accounts, commitments, or Mana rewards. A Context Input does not need to be unlocked.
#[derive(Clone, Eq, Display, PartialEq, Hash, Ord, PartialOrd, From, packable::Packable)]
#[packable(unpack_error = Error)]
#[packable(tag_type = u8, with_error = Error::InvalidContextInputKind)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(untagged))]
pub enum ContextInput {
    /// A [`CommitmentContextInput`].
    #[packable(tag = CommitmentContextInput::KIND)]
    Commitment(CommitmentContextInput),
    /// A [`BlockIssuanceCreditContextInput`].
    #[packable(tag = BlockIssuanceCreditContextInput::KIND)]
    BlockIssuanceCredit(BlockIssuanceCreditContextInput),
    /// A [`RewardContextInput`].
    #[packable(tag = RewardContextInput::KIND)]
    Reward(RewardContextInput),
}

impl ContextInput {
    /// Returns the context input kind of a `ContextInput`.
    pub fn kind(&self) -> u8 {
        match self {
            Self::Commitment(_) => CommitmentContextInput::KIND,
            Self::BlockIssuanceCredit(_) => BlockIssuanceCreditContextInput::KIND,
            Self::Reward(_) => RewardContextInput::KIND,
        }
    }

    crate::def_is_as_opt!(ContextInput: Commitment, BlockIssuanceCredit, Reward);
}

impl WorkScore for ContextInput {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        match self {
            Self::Commitment(commitment) => commitment.work_score(params),
            Self::BlockIssuanceCredit(bic) => bic.work_score(params),
            Self::Reward(reward) => reward.work_score(params),
        }
    }
}

impl core::fmt::Debug for ContextInput {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Commitment(input) => input.fmt(f),
            Self::BlockIssuanceCredit(input) => input.fmt(f),
            Self::Reward(input) => input.fmt(f),
        }
    }
}

pub(crate) type ContextInputCount =
    BoundedU16<{ *CONTEXT_INPUT_COUNT_RANGE.start() }, { *CONTEXT_INPUT_COUNT_RANGE.end() }>;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deref, Packable)]
#[packable(unpack_error = Error, with = |e| e.unwrap_item_err_or_else(|p| Error::InvalidContextInputCount(p.into())))]
pub struct ContextInputs(
    #[packable(verify_with = verify_context_inputs_packable)] BoxedSlicePrefix<ContextInput, ContextInputCount>,
);

impl TryFrom<Vec<ContextInput>> for ContextInputs {
    type Error = Error;

    #[inline(always)]
    fn try_from(features: Vec<ContextInput>) -> Result<Self, Self::Error> {
        Self::from_vec(features)
    }
}

// impl IntoIterator for Features {
//     type Item = Feature;
//     type IntoIter = alloc::vec::IntoIter<Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//         Vec::from(Into::<Box<[Feature]>>::into(self.0)).into_iter()
//     }
// }

impl ContextInputs {
    /// Creates a new [`ContextInputs`] from a vec.
    pub fn from_vec(features: Vec<ContextInput>) -> Result<Self, Error> {
        let mut context_inputs =
            BoxedSlicePrefix::<ContextInput, ContextInputCount>::try_from(features.into_boxed_slice())
                .map_err(Error::InvalidContextInputCount)?;

        context_inputs.sort_by(context_inputs_cmp);
        // Sort is obviously fine now but uniqueness still needs to be checked.
        verify_context_inputs(&context_inputs)?;

        Ok(Self(context_inputs))
    }

    //     /// Gets a reference to a [`Feature`] from a feature kind, if any.
    //     #[inline(always)]
    //     pub fn get(&self, key: u8) -> Option<&Feature> {
    //         self.0
    //             .binary_search_by_key(&key, Feature::kind)
    //             // PANIC: indexation is fine since the index has been found.
    //             .map(|index| &self.0[index])
    //             .ok()
    //     }

    //     /// Gets a reference to a [`SenderFeature`], if any.
    //     pub fn sender(&self) -> Option<&SenderFeature> {
    //         self.get(SenderFeature::KIND).map(Feature::as_sender)
    //     }

    //     /// Gets a reference to a [`IssuerFeature`], if any.
    //     pub fn issuer(&self) -> Option<&IssuerFeature> {
    //         self.get(IssuerFeature::KIND).map(Feature::as_issuer)
    //     }

    //     /// Gets a reference to a [`MetadataFeature`], if any.
    //     pub fn metadata(&self) -> Option<&MetadataFeature> {
    //         self.get(MetadataFeature::KIND).map(Feature::as_metadata)
    //     }

    //     /// Gets a reference to a [`StateMetadataFeature`], if any.
    //     pub fn state_metadata(&self) -> Option<&StateMetadataFeature> {
    //         self.get(StateMetadataFeature::KIND).map(Feature::as_state_metadata)
    //     }

    //     /// Gets a reference to a [`TagFeature`], if any.
    //     pub fn tag(&self) -> Option<&TagFeature> {
    //         self.get(TagFeature::KIND).map(Feature::as_tag)
    //     }

    //     /// Gets a reference to a [`NativeTokenFeature`], if any.
    //     pub fn native_token(&self) -> Option<&NativeTokenFeature> {
    //         self.get(NativeTokenFeature::KIND).map(Feature::as_native_token)
    //     }

    //     /// Gets a reference to a [`BlockIssuerFeature`], if any.
    //     pub fn block_issuer(&self) -> Option<&BlockIssuerFeature> {
    //         self.get(BlockIssuerFeature::KIND).map(Feature::as_block_issuer)
    //     }

    //     /// Gets a reference to a [`StakingFeature`], if any.
    //     pub fn staking(&self) -> Option<&StakingFeature> {
    //         self.get(StakingFeature::KIND).map(Feature::as_staking)
    //     }
    // }

    // impl StorageScore for Features {
    //     fn storage_score(&self, params: StorageScoreParameters) -> u64 {
    //         self.iter().map(|f| f.storage_score(params)).sum::<u64>()
    //     }
    // }

    // #[inline]
    // fn verify_unique_sorted<const VERIFY: bool>(features: &[Feature]) -> Result<(), Error> {
    //     if VERIFY && !is_unique_sorted(features.iter().map(Feature::kind)) {
    //         Err(Error::FeaturesNotUniqueSorted)
    //     } else {
    //         Ok(())
    //     }
    // }

    // pub(crate) fn verify_allowed_features(features: &Features, allowed_features: FeatureFlags) -> Result<(), Error> {
    //     for (index, feature) in features.iter().enumerate() {
    //         if !allowed_features.contains(feature.flag()) {
    //             return Err(Error::UnallowedFeature {
    //                 index,
    //                 kind: feature.kind(),
    //             });
    //         }
    //     }

    //     Ok(())
}

fn verify_context_inputs_packable<const VERIFY: bool>(context_inputs: &[ContextInput]) -> Result<(), Error> {
    if VERIFY {
        verify_context_inputs(context_inputs)?;
    }
    Ok(())
}

fn context_inputs_cmp(a: &ContextInput, b: &ContextInput) -> Ordering {
    a.kind().cmp(&b.kind()).then_with(|| match (a, b) {
        (ContextInput::Commitment(_), ContextInput::Commitment(_)) => Ordering::Equal,
        (ContextInput::BlockIssuanceCredit(a), ContextInput::BlockIssuanceCredit(b)) => {
            a.account_id().cmp(b.account_id())
        }
        (ContextInput::Reward(a), ContextInput::Reward(b)) => a.index().cmp(&b.index()),
        // No need to evaluate all combinations as `then_with` is only called on Equal.
        _ => unreachable!(),
    })
}

fn verify_context_inputs(context_inputs: &[ContextInput]) -> Result<(), Error> {
    if !is_unique_sorted_by(context_inputs.iter(), |a, b| context_inputs_cmp(a, b)) {
        return Err(Error::ContextInputsNotUniqueSorted);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::ContextInput;

    #[test]
    fn commitment() {
        let commitment: ContextInput = serde_json::from_value(serde_json::json!(
            {
                "type": 0,
                "commitmentId": "0xedf5f572c58ddf4b4f9567d82bf96689cc68b730df796d822b4b9fb643f5efda4f9567d8"
            }
        ))
        .unwrap();
        assert!(commitment.is_commitment());
        assert_eq!(
            commitment.as_commitment().slot_commitment_id().to_string(),
            "0xedf5f572c58ddf4b4f9567d82bf96689cc68b730df796d822b4b9fb643f5efda4f9567d8"
        );

        // Test wrong type returns error.
        let commitment_deserialization_result: Result<ContextInput, _> = serde_json::from_value(serde_json::json!(
            {
                "type": 1,
                "commitmentId": "0xedf5f572c58ddf4b4f9567d82bf96689cc68b730df796d822b4b9fb643f5efda4f9567d8"
            }
        ));
        assert!(commitment_deserialization_result.is_err());
    }

    #[test]
    fn block_issuance_credit() {
        let bic: ContextInput = serde_json::from_value(serde_json::json!(
            {
                "type": 1,
                "accountId": "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649"
            }
        ))
        .unwrap();
        assert!(bic.is_block_issuance_credit());
        assert_eq!(
            bic.as_block_issuance_credit().account_id().to_string(),
            "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649"
        );

        // Test wrong type returns error.
        let bic_deserialization_result: Result<ContextInput, _> = serde_json::from_value(serde_json::json!(
            {
                "type": 2,
                "accountId": "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649"
            }
        ));
        assert!(bic_deserialization_result.is_err());
    }

    #[test]
    fn reward() {
        let reward: ContextInput = serde_json::from_value(serde_json::json!(
            {
                "type": 2,
                "index": 10
            }
        ))
        .unwrap();
        assert!(reward.is_reward());
        assert_eq!(reward.as_reward().index(), 10);

        // Test wrong type returns error.
        let reward_serialization_result: Result<ContextInput, _> = serde_json::from_value(serde_json::json!(
            {
                "type": 0,
                "index": 10
            }
        ));
        assert!(reward_serialization_result.is_err())
    }
}
