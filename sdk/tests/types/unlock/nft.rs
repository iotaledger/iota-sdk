// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::unlock::{NftUnlock, UnlockError};
use packable::{bounded::InvalidBoundedU16, PackableExt};
use pretty_assertions::assert_eq;

#[test]
fn kind() {
    assert_eq!(NftUnlock::KIND, 4);
}

#[test]
fn new_valid_min_index() {
    assert_eq!(NftUnlock::new(0).unwrap().index(), 0);
}

#[test]
fn new_valid_max_index() {
    assert_eq!(NftUnlock::new(126).unwrap().index(), 126);
}

#[test]
fn new_invalid_more_than_max_index() {
    assert!(matches!(
        NftUnlock::new(128),
        Err(UnlockError::NftIndex(InvalidBoundedU16(128)))
    ));
}

#[test]
fn try_from_valid() {
    assert_eq!(NftUnlock::try_from(0).unwrap().index(), 0);
}

#[test]
fn try_from_invalid() {
    assert!(matches!(
        NftUnlock::try_from(128),
        Err(UnlockError::NftIndex(InvalidBoundedU16(128)))
    ));
}

#[test]
fn packed_len() {
    let reference = NftUnlock::new(0).unwrap();

    assert_eq!(reference.packed_len(), 2);
    assert_eq!(reference.pack_to_vec().len(), 2);
}

#[test]
fn pack_unpack_valid() {
    let unlock_1 = NftUnlock::new(42).unwrap();
    let unlock_2 = NftUnlock::unpack_bytes_verified(unlock_1.pack_to_vec().as_slice(), &()).unwrap();

    assert_eq!(unlock_1, unlock_2);
}
