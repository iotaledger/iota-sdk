// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    client::{
        api::PreparedTransactionData,
        secret::{SecretManage, SecretManager},
    },
    types::block::{payload::dto::PayloadDto, signature::dto::Ed25519SignatureDto, unlock::Unlock},
};
use tokio::sync::RwLock;

use crate::{method::SecretManagerMethod, response::Response, Result};

/// Call a secret manager method.
pub(crate) async fn call_secret_manager_method_internal(
    secret_manager: &RwLock<SecretManager>,
    method: SecretManagerMethod,
) -> Result<Response> {
    let secret_manager = secret_manager.read().await;
    let response = match method {
        SecretManagerMethod::GenerateEd25519Addresses { options } => {
            let addresses = secret_manager.generate_ed25519_addresses(options).await?;
            Response::GeneratedEd25519Addresses(addresses)
        }
        SecretManagerMethod::GenerateEvmAddresses { options } => {
            let addresses = secret_manager.generate_evm_addresses(options).await?;
            Response::GeneratedEvmAddresses(addresses)
        }
        #[cfg(feature = "ledger_nano")]
        SecretManagerMethod::GetLedgerNanoStatus => {
            if let SecretManager::LedgerNano(secret_manager) = &*secret_manager {
                Response::LedgerNanoStatus(secret_manager.get_ledger_nano_status().await)
            } else {
                return Err(iota_sdk::client::Error::SecretManagerMismatch.into());
            }
        }
        SecretManagerMethod::SignTransaction {
            prepared_transaction_data,
        } => {
            let payload = &secret_manager
                .sign_transaction(PreparedTransactionData::try_from_dto_unverified(
                    prepared_transaction_data,
                )?)
                .await?;
            Response::SignedTransaction(PayloadDto::from(payload))
        }
        SecretManagerMethod::SignatureUnlock {
            transaction_essence_hash,
            chain,
        } => {
            let transaction_essence_hash: [u8; 32] = prefix_hex::decode(transaction_essence_hash)?;
            let unlock: Unlock = secret_manager
                .signature_unlock(&transaction_essence_hash, chain)
                .await?;

            Response::SignatureUnlock((&unlock).into())
        }
        SecretManagerMethod::SignEd25519 { message, chain } => {
            let msg: Vec<u8> = prefix_hex::decode(message)?;
            let signature = secret_manager.sign_ed25519(&msg, chain).await?;
            Response::Ed25519Signature(Ed25519SignatureDto::from(&signature))
        }
        SecretManagerMethod::SignSecp256k1Ecdsa { message, chain } => {
            let msg: Vec<u8> = prefix_hex::decode(message)?;
            let (public_key, signature) = secret_manager.sign_secp256k1_ecdsa(&msg, chain).await?;
            Response::Secp256k1EcdsaSignature {
                public_key: prefix_hex::encode(public_key.to_bytes()),
                signature: prefix_hex::encode(signature.to_bytes()),
            }
        }
        #[cfg(feature = "stronghold")]
        SecretManagerMethod::StoreMnemonic { mnemonic } => {
            let mnemonic = crypto::keys::bip39::Mnemonic::from(mnemonic);
            if let SecretManager::Stronghold(secret_manager) = &*secret_manager {
                secret_manager.store_mnemonic(mnemonic).await?;
                Response::Ok
            } else {
                return Err(iota_sdk::client::Error::SecretManagerMismatch.into());
            }
        }
    };
    Ok(response)
}
