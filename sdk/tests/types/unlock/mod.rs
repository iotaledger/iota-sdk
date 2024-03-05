// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod account;
mod nft;
mod reference;
mod signature;

use iota_sdk::types::block::{
    rand::signature::rand_signature,
    unlock::{AccountUnlock, AnchorUnlock, NftUnlock, ReferenceUnlock, SignatureUnlock, Unlock, UnlockError, Unlocks},
};
use packable::bounded::TryIntoBoundedU16Error;
use pretty_assertions::assert_eq;

#[test]
fn kind() {
    assert_eq!(Unlock::from(SignatureUnlock::from(rand_signature())).kind(), 0);
    assert_eq!(Unlock::from(ReferenceUnlock::new(0).unwrap()).kind(), 1);
    assert_eq!(Unlock::from(AccountUnlock::new(0).unwrap()).kind(), 2);
    assert_eq!(Unlock::from(AnchorUnlock::new(0).unwrap()).kind(), 3);
    assert_eq!(Unlock::from(NftUnlock::new(0).unwrap()).kind(), 4);
}

#[test]
fn new_invalid_first_reference() {
    assert!(matches!(
        Unlocks::new([ReferenceUnlock::new(42).unwrap().into()]),
        Err(UnlockError::Reference(0)),
    ));
}

#[test]
fn new_invalid_self_reference() {
    assert!(matches!(
        Unlocks::new([
            SignatureUnlock::from(rand_signature()).into(),
            ReferenceUnlock::new(1).unwrap().into()
        ]),
        Err(UnlockError::Reference(1)),
    ));
}

#[test]
fn new_invalid_future_reference() {
    assert!(matches!(
        Unlocks::new([
            SignatureUnlock::from(rand_signature()).into(),
            ReferenceUnlock::new(2).unwrap().into(),
            SignatureUnlock::from(rand_signature()).into(),
        ]),
        Err(UnlockError::Reference(1)),
    ));
}

#[test]
fn new_invalid_reference_reference() {
    assert!(matches!(
        Unlocks::new([
            SignatureUnlock::from(rand_signature()).into(),
            ReferenceUnlock::new(0).unwrap().into(),
            ReferenceUnlock::new(1).unwrap().into()
        ]),
        Err(UnlockError::Reference(2)),
    ));
}

#[test]
fn new_invalid_duplicate_signature() {
    let dup = rand_signature();
    assert!(matches!(
        Unlocks::new([
            SignatureUnlock::from(rand_signature()).into(),
            ReferenceUnlock::new(0).unwrap().into(),
            ReferenceUnlock::new(0).unwrap().into(),
            SignatureUnlock::from(rand_signature()).into(),
            SignatureUnlock::from(dup).into(),
            SignatureUnlock::from(dup).into(),
            SignatureUnlock::from(rand_signature()).into(),
            ReferenceUnlock::new(3).unwrap().into()
        ]),
        Err(UnlockError::DuplicateSignature(5)),
    ));
}

#[test]
fn new_invalid_too_many_blocks() {
    assert!(matches!(
        Unlocks::new(vec![ReferenceUnlock::new(0).unwrap().into(); 300]),
        Err(UnlockError::Count(TryIntoBoundedU16Error::Invalid(300))),
    ));
}

#[test]
fn new_valid() {
    assert!(
        Unlocks::new([
            SignatureUnlock::from(rand_signature()).into(),
            ReferenceUnlock::new(0).unwrap().into(),
            ReferenceUnlock::new(0).unwrap().into(),
            SignatureUnlock::from(rand_signature()).into(),
            SignatureUnlock::from(rand_signature()).into(),
            SignatureUnlock::from(rand_signature()).into(),
            ReferenceUnlock::new(3).unwrap().into(),
            ReferenceUnlock::new(4).unwrap().into(),
            ReferenceUnlock::new(3).unwrap().into(),
            ReferenceUnlock::new(4).unwrap().into(),
            ReferenceUnlock::new(5).unwrap().into(),
            SignatureUnlock::from(rand_signature()).into(),
            ReferenceUnlock::new(11).unwrap().into(),
            SignatureUnlock::from(rand_signature()).into(),
        ])
        .is_ok()
    );
}

#[test]
fn get_none() {
    assert!(
        Unlocks::new([SignatureUnlock::from(rand_signature()).into()])
            .unwrap()
            .get(42)
            .is_none()
    );
}

#[test]
fn get_signature() {
    let signature = Unlock::from(SignatureUnlock::from(rand_signature()));

    assert_eq!(Unlocks::new([signature.clone()]).unwrap().first(), Some(&signature));
}

#[test]
fn get_signature_through_reference() {
    let signature = Unlock::from(SignatureUnlock::from(rand_signature()));

    assert_eq!(
        Unlocks::new([signature.clone(), ReferenceUnlock::new(0).unwrap().into()])
            .unwrap()
            .get(1),
        Some(&signature)
    );
}

#[test]
fn invalid_account_0() {
    assert!(matches!(
        Unlocks::new([
            AccountUnlock::new(0).unwrap().into(),
            SignatureUnlock::from(rand_signature()).into(),
        ]),
        Err(UnlockError::Account(0)),
    ));
}

#[test]
fn invalid_account_index() {
    assert!(matches!(
        Unlocks::new([
            SignatureUnlock::from(rand_signature()).into(),
            AccountUnlock::new(2).unwrap().into(),
        ]),
        Err(UnlockError::Account(1)),
    ));
}

#[test]
fn invalid_nft_0() {
    assert!(matches!(
        Unlocks::new([
            NftUnlock::new(0).unwrap().into(),
            SignatureUnlock::from(rand_signature()).into(),
        ]),
        Err(UnlockError::Nft(0)),
    ));
}

#[test]
fn invalid_nft_index() {
    assert!(matches!(
        Unlocks::new([
            SignatureUnlock::from(rand_signature()).into(),
            NftUnlock::new(2).unwrap().into(),
        ]),
        Err(UnlockError::Nft(1)),
    ));
}
