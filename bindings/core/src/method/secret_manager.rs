// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::bip44::Bip44;
use derivative::Derivative;
use iota_sdk::{
    client::api::{GetAddressesOptions, PreparedTransactionDataDto},
    types::block::{address::Hrp, protocol::ProtocolParameters, UnsignedBlockDto},
    utils::serde::bip44::Bip44Def,
};
use serde::{Deserialize, Serialize};

#[cfg(feature = "stronghold")]
use crate::OmittedDebug;

/// Each public secret manager method.
#[derive(Clone, Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
#[serde(tag = "name", content = "data", rename_all = "camelCase")]
#[non_exhaustive]
pub enum SecretManagerMethod {
    // TODO: Should we refactor `GetAddressesOptions` so we can use it for this method as well?
    /// Generate a single Ed25519 address.
    #[serde(rename_all = "camelCase")]
    GenerateEd25519Address {
        coin_type: u32,
        #[serde(default)]
        account_index: u32,
        #[serde(default)]
        address_index: u32,
        #[serde(default)]
        internal: bool,
        #[serde(default)]
        ledger_nano_prompt: bool,
        bech32_hrp: Hrp,
    },
    /// Generate multiple Ed25519 addresses at once.
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
        /// Transaction signing hash
        transaction_signing_hash: String,
        /// Chain used to sign the hash
        #[serde(with = "Bip44Def")]
        chain: Bip44,
    },
    /// Signs a message with an Ed25519 private key.
    SignEd25519 {
        /// The message to sign, hex encoded String
        message: String,
        /// Chain used to sign the message
        #[serde(with = "Bip44Def")]
        chain: Bip44,
    },
    /// Signs a message with an Secp256k1Ecdsa private key.
    SignSecp256k1Ecdsa {
        /// The message to sign, hex encoded String
        message: String,
        /// Chain used to sign the message
        #[serde(with = "Bip44Def")]
        chain: Bip44,
    },
    /// Sign a transaction
    #[serde(rename_all = "camelCase")]
    SignTransaction {
        /// Prepared transaction data
        prepared_transaction_data: PreparedTransactionDataDto,
        protocol_parameters: Box<ProtocolParameters>,
    },
    // Sign a block.
    #[serde(rename_all = "camelCase")]
    SignBlock {
        unsigned_block: UnsignedBlockDto,
        /// Chain used to sign the block
        #[serde(with = "Bip44Def")]
        chain: Bip44,
    },
    /// Store a mnemonic in the Stronghold vault
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    StoreMnemonic {
        /// Mnemonic
        #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
        mnemonic: String,
    },
    /// Set the stronghold password.
    /// Expected response: [`Ok`](crate::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    SetStrongholdPassword {
        #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
        password: String,
    },
    /// Change the stronghold password.
    /// Expected response: [`Ok`](crate::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    ChangeStrongholdPassword {
        #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
        password: String,
    },
    /// Clear the stronghold password.
    /// Expected response: [`Ok`](crate::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    ClearStrongholdPassword,
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    #[test]
    fn bip44_deserialization() {
        let signature_unlock_method: super::SecretManagerMethod = serde_json::from_str(
            r#"{"name": "signatureUnlock", "data": {"transactionSigningHash": "txhash", "chain": {"addressIndex": 1}}}"#,
        )
        .unwrap();

        assert_eq!(
            serde_json::to_value(&signature_unlock_method).unwrap(),
            serde_json::json!({
                "name": "signatureUnlock",
                "data": {
                    "transactionSigningHash": "txhash",
                    "chain": {
                        "coinType": 4218,
                        "account": 0,
                        "change": 0,
                        "addressIndex": 1
                    }
                }
            })
        );

        let sign_ed25519_method: super::SecretManagerMethod = serde_json::from_str(
            r#"{"name": "signEd25519", "data": {"message": "0xFFFFFFFF", "chain": {"coinType": 60, "change": 1}}}"#,
        )
        .unwrap();

        assert_eq!(
            serde_json::to_value(&sign_ed25519_method).unwrap(),
            serde_json::json!({
                "name": "signEd25519",
                "data": {
                    "message": "0xFFFFFFFF",
                    "chain": {
                        "coinType": 60,
                        "account": 0,
                        "change": 1,
                        "addressIndex": 0
                    }
                }
            })
        );

        let sign_secp256k1_ecdsa_method: super::SecretManagerMethod = serde_json::from_str(
            r#"{"name": "signSecp256k1Ecdsa", "data": {"message": "0xFFFFFFFF", "chain": {"account": 2, "addressIndex": 1}}}"#,
        )
        .unwrap();

        assert_eq!(
            serde_json::to_value(&sign_secp256k1_ecdsa_method).unwrap(),
            serde_json::json!({
                "name": "signSecp256k1Ecdsa",
                "data": {
                    "message": "0xFFFFFFFF",
                    "chain": {
                        "coinType": 4218,
                        "account": 2,
                        "change": 0,
                        "addressIndex": 1
                    }
                }
            })
        );
    }
}
