// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    client::{
        api::{GetAddressesOptions, PreparedTransactionData},
        secret::{DowncastSecretManager, DynSecretManagerConfig, SecretManage},
    },
    types::{
        block::{address::ToBech32Ext, unlock::Unlock},
        TryFromDto,
    },
};
use tokio::sync::RwLock;

use crate::{method::SecretManagerMethod, response::Response, Result};

/// Call a secret manager method.
pub(crate) async fn call_secret_manager_method_internal(
    secret_manager: &RwLock<Box<dyn DynSecretManagerConfig>>,
    method: SecretManagerMethod,
) -> Result<Response> {
    let secret_manager = secret_manager.read().await;
    let response = match method {
        SecretManagerMethod::GenerateEd25519Addresses {
            options:
                GetAddressesOptions {
                    coin_type,
                    account_index,
                    range,
                    bech32_hrp,
                    options,
                },
        } => {
            let addresses = secret_manager
                .generate_ed25519_addresses(coin_type, account_index, range, options)
                .await?
                .into_iter()
                .map(|a| a.to_bech32(bech32_hrp))
                .collect();
            Response::GeneratedEd25519Addresses(addresses)
        }
        SecretManagerMethod::GenerateEvmAddresses {
            options:
                GetAddressesOptions {
                    coin_type,
                    account_index,
                    range,
                    options,
                    ..
                },
        } => {
            let addresses = secret_manager
                .generate_evm_addresses(coin_type, account_index, range, options)
                .await?
                .into_iter()
                .map(|a| prefix_hex::encode(a.as_ref()))
                .collect();
            Response::GeneratedEvmAddresses(addresses)
        }
        #[cfg(feature = "ledger_nano")]
        SecretManagerMethod::GetLedgerNanoStatus => {
            Response::LedgerNanoStatus(secret_manager.as_ledger_nano()?.get_ledger_nano_status().await)
        }
        SecretManagerMethod::SignTransaction {
            prepared_transaction_data,
        } => {
            let transaction = &secret_manager
                .sign_transaction(PreparedTransactionData::try_from_dto(prepared_transaction_data)?)
                .await?;
            Response::SignedTransaction(transaction.into())
        }
        SecretManagerMethod::SignatureUnlock {
            transaction_essence_hash,
            chain,
        } => {
            let transaction_essence_hash: [u8; 32] = prefix_hex::decode(transaction_essence_hash)?;
            let unlock: Unlock = secret_manager
                .signature_unlock(&transaction_essence_hash, chain)
                .await?;

            Response::SignatureUnlock(unlock)
        }
        SecretManagerMethod::SignEd25519 { message, chain } => {
            let msg: Vec<u8> = prefix_hex::decode(message)?;
            let signature = secret_manager.sign_ed25519(&msg, chain).await?;
            Response::Ed25519Signature(signature)
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
            secret_manager.as_stronghold()?.store_mnemonic(mnemonic).await?;
            Response::Ok
        }
    };
    Ok(response)
}
