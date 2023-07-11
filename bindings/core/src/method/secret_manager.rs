// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::bip44::Bip44;
use derivative::Derivative;
use iota_sdk::client::{
    api::{GetAddressesOptions, PreparedTransactionDataDto},
    constants::IOTA_COIN_TYPE,
};
use serde::{Deserialize, Serialize};

#[cfg(feature = "stronghold")]
use crate::OmittedDebug;

/// Each public secret manager method.
#[derive(Clone, Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
#[serde(tag = "name", content = "data", rename_all = "camelCase")]
pub enum SecretManagerMethod {
    /// Generate Ed25519 addresses.
    GenerateEd25519Addresses {
        /// Addresses generation options
        options: GetAddressesOptions,
    },
    /// Generate Evm addresses.
    GenerateEvmAddresses { options: GetAddressesOptions },
    /// Get the ledger status
    /// Expected response: [`LedgerNanoStatus`](crate::Response::LedgerNanoStatus)
    #[cfg(feature = "ledger_nano")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
    GetLedgerNanoStatus,
    /// Create a single Signature Unlock.
    #[serde(rename_all = "camelCase")]
    SignatureUnlock {
        /// Transaction Essence Hash
        transaction_essence_hash: String,
        /// Chain to sign the essence hash with
        #[serde(with = "Bip44Def")]
        chain: Bip44,
    },
    /// Signs a message with an Ed25519 private key.
    SignEd25519 {
        /// The message to sign, hex encoded String
        message: String,
        /// Chain to sign the essence hash with
        #[serde(with = "Bip44Def")]
        chain: Bip44,
    },
    /// Signs a message with an Secp256k1Ecdsa private key.
    SignSecp256k1Ecdsa {
        /// The message to sign, hex encoded String
        message: String,
        /// Chain to sign the message with
        #[serde(with = "Bip44Def")]
        chain: Bip44,
    },
    /// Sign a transaction
    #[serde(rename_all = "camelCase")]
    SignTransaction {
        /// Prepared transaction data
        prepared_transaction_data: PreparedTransactionDataDto,
    },
    /// Store a mnemonic in the Stronghold vault
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    StoreMnemonic {
        /// Mnemonic
        #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
        mnemonic: String,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase", remote = "Bip44")]
pub struct Bip44Def {
    coin_type: u32,
    account: u32,
    change: u32,
    address_index: u32,
}

impl Default for Bip44Def {
    fn default() -> Self {
        Self {
            coin_type: IOTA_COIN_TYPE,
            account: 0,
            change: 0,
            address_index: 0,
        }
    }
}
