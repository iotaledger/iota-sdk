// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::slip10::Chain;
use derivative::Derivative;
use iota_sdk::client::{api::GetAddressesBuilderOptions, secret::SecretManagerDto};
use serde::{Deserialize, Serialize};

use super::OmittedDebug;

/// Each public secret manager method.
#[derive(Clone, Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
#[serde(tag = "name", content = "data", rename_all = "camelCase")]
pub enum SecretManagerMethod {
    /// Generate addresses.
    GenerateAddresses {
        /// Create secret manager from json
        #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
        #[serde(rename = "secretManager")]
        secret_manager: SecretManagerDto,
        /// Addresses generation options
        options: GetAddressesBuilderOptions,
    },
    /// Get the ledger status
    /// Expected response: [`LedgerNanoStatus`](crate::message_interface::Response::LedgerNanoStatus)
    #[cfg(feature = "ledger_nano")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
    GetLedgerNanoStatus {
        /// To use a Ledger Speculos simulator, pass `true` to `is_simulator`; `false` otherwise.
        #[serde(rename = "isSimulator")]
        is_simulator: bool,
    },
    /// Create a single Signature Unlock.
    SignatureUnlock {
        /// Secret manager
        #[serde(rename = "secretManager")]
        #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
        secret_manager: SecretManagerDto,
        /// Transaction Essence Hash
        #[serde(rename = "transactionEssenceHash")]
        transaction_essence_hash: Vec<u8>,
        /// Chain to sign the essence hash with
        chain: Chain,
    },
    /// Signs a message with an Ed25519 private key.
    SignEd25519 {
        /// Secret manager
        #[serde(rename = "secretManager")]
        secret_manager: SecretManagerDto,
        /// The message to sign, hex encoded String
        message: String,
        /// Chain to sign the essence hash with
        chain: Chain,
    },
    /// Store a mnemonic in the Stronghold vault
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    StoreMnemonic {
        /// Stronghold secret manager
        #[serde(rename = "secretManager")]
        #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
        secret_manager: SecretManagerDto,
        /// Mnemonic
        #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
        mnemonic: String,
    },
}
