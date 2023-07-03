// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::{
    rand::bytes::rand_bytes_array,
    signature::{Ed25519Signature, Signature},
    unlock::{ReferenceUnlock, SignatureUnlock, Unlock, Unlocks},
    Error,
};
use packable::bounded::TryIntoBoundedU16Error;

#[test]
fn kind() {
    assert_eq!(
        Unlock::from(SignatureUnlock::from(Signature::from(
            Ed25519Signature::try_from_bytes(rand_bytes_array(), rand_bytes_array()).unwrap()
        )))
        .kind(),
        0
    );
    assert_eq!(Unlock::from(ReferenceUnlock::new(0).unwrap()).kind(), 1);
}

#[test]
fn new_invalid_first_reference() {
    assert!(matches!(
        Unlocks::new([ReferenceUnlock::new(42).unwrap().into()]),
        Err(Error::InvalidUnlockReference(0)),
    ));
}

#[test]
fn new_invalid_self_reference() {
    assert!(matches!(
        Unlocks::new([
            SignatureUnlock::from(Signature::from(
                Ed25519Signature::try_from_bytes([0; 32], [0; 64]).unwrap()
            ))
            .into(),
            ReferenceUnlock::new(1).unwrap().into()
        ]),
        Err(Error::InvalidUnlockReference(1)),
    ));
}

#[test]
fn new_invalid_future_reference() {
    assert!(matches!(
        Unlocks::new([
            SignatureUnlock::from(Signature::from(
                Ed25519Signature::try_from_bytes([0; 32], [0; 64]).unwrap()
            ))
            .into(),
            ReferenceUnlock::new(2).unwrap().into(),
            SignatureUnlock::from(Signature::from(
                Ed25519Signature::try_from_bytes([1; 32], [1; 64]).unwrap()
            ))
            .into(),
        ]),
        Err(Error::InvalidUnlockReference(1)),
    ));
}

#[test]
fn new_invalid_reference_reference() {
    assert!(matches!(
        Unlocks::new([
            SignatureUnlock::from(Signature::from(
                Ed25519Signature::try_from_bytes([0; 32], [0; 64]).unwrap()
            ))
            .into(),
            ReferenceUnlock::new(0).unwrap().into(),
            ReferenceUnlock::new(1).unwrap().into()
        ]),
        Err(Error::InvalidUnlockReference(2)),
    ));
}

#[test]
fn new_invalid_duplicate_signature() {
    assert!(matches!(
        Unlocks::new([
            SignatureUnlock::from(Signature::from(
                Ed25519Signature::try_from_bytes([0; 32], [0; 64]).unwrap()
            ))
            .into(),
            ReferenceUnlock::new(0).unwrap().into(),
            ReferenceUnlock::new(0).unwrap().into(),
            SignatureUnlock::from(Signature::from(
                Ed25519Signature::try_from_bytes([1; 32], [1; 64]).unwrap()
            ))
            .into(),
            SignatureUnlock::from(Signature::from(
                Ed25519Signature::try_from_bytes([2; 32], [2; 64]).unwrap()
            ))
            .into(),
            SignatureUnlock::from(Signature::from(
                Ed25519Signature::try_from_bytes([2; 32], [2; 64]).unwrap()
            ))
            .into(),
            SignatureUnlock::from(Signature::from(
                Ed25519Signature::try_from_bytes([3; 32], [3; 64]).unwrap()
            ))
            .into(),
            ReferenceUnlock::new(3).unwrap().into()
        ]),
        Err(Error::DuplicateSignatureUnlock(5)),
    ));
}

#[test]
fn new_invalid_too_many_blocks() {
    assert!(matches!(
        Unlocks::new(vec![ReferenceUnlock::new(0).unwrap().into(); 300]),
        Err(Error::InvalidUnlockCount(TryIntoBoundedU16Error::Invalid(300))),
    ));
}

#[test]
fn new_valid() {
    assert!(
        Unlocks::new([
            SignatureUnlock::from(Signature::from(
                Ed25519Signature::try_from_bytes([0; 32], [0; 64]).unwrap()
            ))
            .into(),
            ReferenceUnlock::new(0).unwrap().into(),
            ReferenceUnlock::new(0).unwrap().into(),
            SignatureUnlock::from(Signature::from(
                Ed25519Signature::try_from_bytes([1; 32], [1; 64]).unwrap()
            ))
            .into(),
            SignatureUnlock::from(Signature::from(
                Ed25519Signature::try_from_bytes([2; 32], [2; 64]).unwrap()
            ))
            .into(),
            SignatureUnlock::from(Signature::from(
                Ed25519Signature::try_from_bytes([3; 32], [3; 64]).unwrap()
            ))
            .into(),
            ReferenceUnlock::new(3).unwrap().into(),
            ReferenceUnlock::new(4).unwrap().into(),
            ReferenceUnlock::new(3).unwrap().into(),
            ReferenceUnlock::new(4).unwrap().into(),
            ReferenceUnlock::new(5).unwrap().into(),
            SignatureUnlock::from(Signature::from(
                Ed25519Signature::try_from_bytes([4; 32], [4; 64]).unwrap()
            ))
            .into(),
            ReferenceUnlock::new(11).unwrap().into(),
            SignatureUnlock::from(Signature::from(
                Ed25519Signature::try_from_bytes([5; 32], [5; 64]).unwrap()
            ))
            .into(),
        ])
        .is_ok()
    );
}

#[test]
fn get_none() {
    assert!(
        Unlocks::new([SignatureUnlock::from(Signature::from(
            Ed25519Signature::try_from_bytes([0; 32], [0; 64]).unwrap()
        ))
        .into()])
        .unwrap()
        .get(42)
        .is_none()
    );
}

#[test]
fn get_signature() {
    let signature = Unlock::from(SignatureUnlock::from(Signature::from(
        Ed25519Signature::try_from_bytes([0; 32], [0; 64]).unwrap(),
    )));

    assert_eq!(Unlocks::new([signature.clone()]).unwrap().get(0), Some(&signature));
}

#[test]
fn get_signature_through_reference() {
    let signature = Unlock::from(SignatureUnlock::from(Signature::from(
        Ed25519Signature::try_from_bytes([0; 32], [0; 64]).unwrap(),
    )));

    assert_eq!(
        Unlocks::new([signature.clone(), ReferenceUnlock::new(0).unwrap().into()])
            .unwrap()
            .get(1),
        Some(&signature)
    );
}
