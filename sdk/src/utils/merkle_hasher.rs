// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::hashes::{Digest, Output};

/// Leaf domain separation prefix.
pub(crate) const LEAF_HASH_PREFIX: u8 = 0x00;
/// Node domain separation prefix.
pub(crate) const NODE_HASH_PREFIX: u8 = 0x01;

/// A Merkle hasher based on a digest function.
pub(crate) struct MerkleHasher;

impl MerkleHasher {
    /// Returns the digest of the empty hash.
    fn empty<D: Default + Digest>() -> Output<D> {
        D::digest([])
    }

    /// Returns the digest of a Merkle leaf.
    fn leaf<D: Default + Digest>(value: &impl AsRef<[u8]>) -> Output<D> {
        let mut hasher = D::default();

        hasher.update([LEAF_HASH_PREFIX]);
        hasher.update(value);
        hasher.finalize()
    }

    /// Returns the digest of a Merkle node.
    fn node<D: Default + Digest>(values: &[impl AsRef<[u8]>]) -> Output<D> {
        let mut hasher = D::default();
        let (left, right) = values.split_at(largest_power_of_two(values.len() as u32));

        hasher.update([NODE_HASH_PREFIX]);
        hasher.update(Self::digest::<D>(left));
        hasher.update(Self::digest::<D>(right));
        hasher.finalize()
    }

    /// Returns the digest of a list of hashes as an `Output<D>`.
    pub(crate) fn digest<D: Default + Digest>(value: &[impl AsRef<[u8]>]) -> Output<D> {
        match value {
            [] => Self::empty::<D>(),
            [leaf] => Self::leaf::<D>(leaf),
            _ => Self::node::<D>(value),
        }
    }
}

/// Computes the largest power of two less than or equal to `n`.
pub(crate) fn largest_power_of_two(n: u32) -> usize {
    debug_assert!(n > 1, "invalid input to `largest_power_of_two`");
    1 << (32 - (n - 1).leading_zeros() - 1)
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;

    use crypto::hashes::blake2b::Blake2b256;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::types::block::BlockId;

    #[test]
    fn tree() {
        let hashes = [
            "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c64900000000",
            "0x81855ad8681d0d86d1e91e00167939cb6694d2c422acd208a0072939487f699900000000",
            "0xeb9d18a44784045d87f3c67cf22746e995af5a25367951baa2ff6cd471c483f100000000",
            "0x5fb90badb37c5821b6d95526a41a9504680b4e7c8b763a1b1d49d4955c84862100000000",
            "0x6325253fec738dd7a9e28bf921119c160f0702448615bbda08313f6a8eb668d200000000",
            "0x0bf5059875921e668a5bdf2c7fc4844592d2572bcd0668d2d6c52f5054e2d08300000000",
            "0x6bf84c7174cb7476364cc3dbd968b0f7172ed85794bb358b0c3b525da1786f9f00000000",
        ]
        .iter()
        .map(|hash| BlockId::from_str(hash).unwrap())
        .collect::<Vec<_>>();

        let hash = MerkleHasher::digest::<Blake2b256>(&hashes).to_vec();

        assert_eq!(
            prefix_hex::encode(hash),
            "0x4a6ff2aca6a11554b6997cf91c31585d436235e7a45f6b4ea48648d6488f6726"
        )
    }
}
