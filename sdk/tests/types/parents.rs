// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::ops::Deref;
use std::collections::BTreeSet;

use iota_sdk::types::block::{
    parent::StrongParents,
    rand::block::{rand_block_id, rand_block_ids},
    BlockId, Error,
};
use packable::{error::UnpackError, prefix::VecPrefix, PackableExt};

#[test]
fn len() {
    for i in 1..=8 {
        assert_eq!(StrongParents::from_vec(rand_block_ids(i)).unwrap().len(), i);
    }
}

#[test]
fn new_valid_iter() {
    let inner = rand_block_ids(8);
    let parents = StrongParents::from_vec(inner.clone()).unwrap();

    let parents_vec = parents.iter().copied().collect::<Vec<BlockId>>();

    assert_eq!(inner, parents_vec[0..].to_vec());
}

#[test]
fn new_from_set() {
    let inner = rand_block_ids(8);
    let parents = StrongParents::from_set(BTreeSet::from_iter(inner.clone())).unwrap();

    assert_eq!(*parents.to_vec(), inner);
}

#[test]
fn new_valid_deref() {
    let inner = rand_block_ids(8);
    let parents = StrongParents::from_vec(inner.clone()).unwrap();

    assert_eq!(parents.deref(), &inner.into_boxed_slice());
}

#[test]
fn new_invalid_more_than_max() {
    let mut inner = vec![rand_block_id()];

    for _ in 0..8 {
        StrongParents::from_vec(inner.clone()).unwrap();
        inner.push(rand_block_id());
        inner.sort();
    }

    // TODO https://github.com/iotaledger/iota-sdk/issues/647
    // assert!(matches!(
    //     Parents::from_vec(inner),
    //     Err(Error::InvalidParentCount(TryIntoBoundedU8Error::Invalid(9)))
    // ));
    assert!(matches!(StrongParents::from_vec(inner), Err(Error::InvalidParentCount)));
}

#[test]
fn new_not_sorted() {
    let mut inner_1 = rand_block_ids(8);
    let inner_2 = inner_1.clone();
    inner_1.reverse();

    let parents = StrongParents::from_vec(inner_1).unwrap();

    assert_eq!(*parents.to_vec(), inner_2);
}

#[test]
fn new_not_unique() {
    let mut inner_1 = rand_block_ids(7);
    let inner_2 = inner_1.clone();
    inner_1.push(*inner_1.last().unwrap());

    let parents = StrongParents::from_vec(inner_1).unwrap();

    assert_eq!(*parents.to_vec(), inner_2);
}

#[test]
fn packed_len() {
    let parents = StrongParents::from_vec(rand_block_ids(5)).unwrap();

    assert_eq!(parents.packed_len(), 1 + 5 * 40);
    assert_eq!(parents.pack_to_vec().len(), 1 + 5 * 40);
}

#[test]
fn pack_unpack_valid() {
    let parents_1 = StrongParents::from_vec(rand_block_ids(8)).unwrap();
    let parents_2 = StrongParents::unpack_verified(parents_1.pack_to_vec().as_slice(), &()).unwrap();

    assert_eq!(parents_1, parents_2);
}

#[test]
fn pack_unpack_invalid_less_than_min() {
    let bytes = vec![
        0, 227, 127, 245, 158, 220, 152, 191, 107, 27, 218, 187, 247, 227, 25, 215, 141, 92, 95, 138, 21, 98, 20, 83,
        206, 92, 26, 62, 9, 221, 81, 191, 4, 96, 54, 232, 50, 83, 49, 236, 80, 189, 251, 191, 192, 122, 206, 202, 209,
        145, 50, 168, 233, 176, 12, 164, 138, 207, 22, 96, 82, 189, 64, 188, 130,
    ];

    // TODO https://github.com/iotaledger/iota-sdk/issues/647
    // assert!(matches!(
    //     Parents::unpack_verified(bytes.as_slice(), &()),
    //     Err(UnpackError::Packable(Error::InvalidParentCount(
    //         TryIntoBoundedU8Error::Invalid(0)
    //     )))
    // ));
    assert!(matches!(
        StrongParents::unpack_verified(bytes.as_slice(), &()),
        Err(UnpackError::Packable(Error::InvalidParentCount))
    ));
}

#[test]
fn pack_unpack_invalid_more_than_max() {
    let bytes = vec![
        9, 227, 127, 245, 158, 220, 152, 191, 107, 27, 218, 187, 247, 227, 25, 215, 141, 92, 95, 138, 21, 98, 20, 83,
        206, 92, 26, 62, 9, 221, 81, 191, 4, 96, 54, 232, 50, 83, 49, 236, 80, 189, 251, 191, 192, 122, 206, 202, 209,
        145, 50, 168, 233, 176, 12, 164, 138, 207, 22, 96, 82, 189, 64, 188, 130,
    ];

    // TODO https://github.com/iotaledger/iota-sdk/issues/647
    // assert!(matches!(
    //     Parents::unpack_verified(bytes.as_slice(), &()),
    //     Err(UnpackError::Packable(Error::InvalidParentCount(
    //         TryIntoBoundedU8Error::Invalid(9)
    //     )))
    // ));
    assert!(matches!(
        StrongParents::unpack_verified(bytes.as_slice(), &()),
        Err(UnpackError::Packable(Error::InvalidParentCount))
    ));
}

#[test]
fn unpack_invalid_not_sorted() {
    let mut inner = rand_block_ids(8);
    inner.reverse();
    let inner = VecPrefix::<_, u8>::try_from(inner).unwrap();

    let packed = inner.pack_to_vec();
    let parents = StrongParents::unpack_verified(packed.as_slice(), &());

    assert!(matches!(
        parents,
        Err(UnpackError::Packable(Error::ParentsNotUniqueSorted))
    ),);
}

#[test]
fn unpack_invalid_not_unique() {
    let mut inner = rand_block_ids(7);
    inner.push(*inner.last().unwrap());
    let inner = VecPrefix::<_, u8>::try_from(inner).unwrap();

    let packed = inner.pack_to_vec();
    let parents = StrongParents::unpack_verified(packed.as_slice(), &());

    assert!(matches!(
        parents,
        Err(UnpackError::Packable(Error::ParentsNotUniqueSorted))
    ),);
}
