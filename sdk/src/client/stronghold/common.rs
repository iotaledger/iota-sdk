// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Commonly used constants and utilities.

use iota_stronghold::KeyProvider;

/// Stronghold vault path to secrets.
///
/// The value has been hard-coded historically.
pub(super) const SECRET_VAULT_PATH: &[u8] = b"iota-wallet-secret";

/// Stronghold record path to a seed.
///
/// The value has been hard-coded historically.
pub(super) const SEED_RECORD_PATH: &[u8] = b"iota-wallet-seed";

/// Stronghold record path to a derived SLIP-10 private key.
///
/// The value has been hard-coded historically.
pub(super) const DERIVE_OUTPUT_RECORD_PATH: &[u8] = b"iota-wallet-derived";

/// The client path for the seed.
///
/// The value has been hard-coded historically.
pub(super) const PRIVATE_DATA_CLIENT_PATH: &[u8] = b"iota_seed";

/// The path for the user-data encryption key for the Stronghold store.
pub(super) const USERDATA_STORE_KEY_RECORD_PATH: &[u8] = b"userdata-store-key";

/// Hash a password, deriving a key, for accessing Stronghold.
pub(super) fn key_provider_from_password(password: &str) -> KeyProvider {
    // PANIC: the hashed password length is guaranteed to be 32.
    KeyProvider::with_passphrase_hashed_blake2b(password.as_bytes().to_vec()).unwrap()
}
