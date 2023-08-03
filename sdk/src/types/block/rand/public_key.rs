// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::public_key::{Ed25519PublicKey, PublicKey};

/// Generates a valid random Ed25519 public key.
pub fn rand_ed25519_public_key() -> Ed25519PublicKey {
    let key = crypto::signatures::ed25519::SecretKey::generate().unwrap();
    key.public_key().into()
}

/// Generates a valid random public key.
pub fn rand_public_key() -> PublicKey {
    rand_ed25519_public_key().into()
}

/// Generates a vector of random valid public keys of a given length.
pub fn rand_public_keys(len: usize) -> Vec<PublicKey> {
    let mut public_keys = (0..len).map(|_| rand_public_key()).collect::<Vec<_>>();
    public_keys.sort();
    public_keys
}
