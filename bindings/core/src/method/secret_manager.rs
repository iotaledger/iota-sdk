// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derivative::Derivative;
use iota_sdk::{
    client::api::PreparedTransactionDataDto,
    types::block::{address::Hrp, protocol::ProtocolParameters, UnsignedBlockDto},
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
    /// Generate Ed25519 addresses.
    GenerateEd25519Addresses {
        /// The Bech32 human-readable part
        bech32_hrp: Hrp,
        /// Addresses generation options
        options: serde_json::Value,
    },
    /// Generate Evm addresses.
    GenerateEvmAddresses { options: serde_json::Value },
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
        /// Options used to sign the hash
        signing_options: serde_json::Value,
    },
    /// Signs a message with an Ed25519 private key.
    #[serde(rename_all = "camelCase")]
    SignEd25519 {
        /// The message to sign, hex encoded String
        message: String,
        /// Options used to sign the message
        signing_options: serde_json::Value,
    },
    /// Signs a message with an Secp256k1Ecdsa private key.
    #[serde(rename_all = "camelCase")]
    SignSecp256k1Ecdsa {
        /// The message to sign, hex encoded String
        message: String,
        /// Options used to sign the hash
        signing_options: serde_json::Value,
    },
    /// Sign a transaction
    #[serde(rename_all = "camelCase")]
    SignTransaction {
        /// Prepared transaction data
        prepared_transaction_data: PreparedTransactionDataDto,
        protocol_parameters: Box<ProtocolParameters>,
        /// Options used to sign the transaction
        signing_options: serde_json::Value,
    },
    // Sign a block.
    #[serde(rename_all = "camelCase")]
    SignBlock {
        unsigned_block: UnsignedBlockDto,
        /// Options used to sign the block
        signing_options: serde_json::Value,
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
}

#[cfg(test)]
mod test {
    use crypto::keys::bip44::Bip44;
    use iota_sdk::client::constants::{ETHER_COIN_TYPE, IOTA_COIN_TYPE};
    use pretty_assertions::assert_eq;

    #[test]
    fn bip44_deserialization() {
        let signature_unlock_method = super::SecretManagerMethod::SignatureUnlock {
            transaction_signing_hash: "txhash".to_owned(),
            signing_options: serde_json::to_value(Bip44::new(IOTA_COIN_TYPE).with_address_index(1)).unwrap(),
        };

        assert_eq!(
            serde_json::to_value(&signature_unlock_method).unwrap(),
            serde_json::json!({
                "name": "signatureUnlock",
                "data": {
                    "transactionSigningHash": "txhash",
                    "signingOptions": {
                        "coin_type": 4218,
                        "account": 0,
                        "change": 0,
                        "address_index": 1
                    }
                }
            })
        );

        let sign_ed25519_method = super::SecretManagerMethod::SignEd25519 {
            message: "0xFFFFFFFF".to_owned(),
            signing_options: serde_json::to_value(Bip44::new(ETHER_COIN_TYPE).with_change(1)).unwrap(),
        };

        assert_eq!(
            serde_json::to_value(&sign_ed25519_method).unwrap(),
            serde_json::json!({
                "name": "signEd25519",
                "data": {
                    "message": "0xFFFFFFFF",
                    "signingOptions": {
                        "coin_type": 60,
                        "account": 0,
                        "change": 1,
                        "address_index": 0
                    }
                }
            })
        );

        let sign_secp256k1_ecdsa_method = super::SecretManagerMethod::SignSecp256k1Ecdsa {
            message: "0xFFFFFFFF".to_owned(),
            signing_options: serde_json::to_value(Bip44::new(IOTA_COIN_TYPE).with_account(2).with_address_index(1))
                .unwrap(),
        };

        assert_eq!(
            serde_json::to_value(&sign_secp256k1_ecdsa_method).unwrap(),
            serde_json::json!({
                "name": "signSecp256k1Ecdsa",
                "data": {
                    "message": "0xFFFFFFFF",
                    "signingOptions": {
                        "coin_type": 4218,
                        "account": 2,
                        "change": 0,
                        "address_index": 1
                    }
                }
            })
        );
    }
}
