// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::signatures::secp256k1_ecdsa::EvmAddress;
#[cfg(feature = "stronghold")]
use iota_sdk::client::secret::SecretManager;
use iota_sdk::{
    client::{
        api::PreparedTransactionData,
        secret::{types::EvmSignature, BlockSignExt, DowncastSecretManager, Generate, Sign, SignTransaction},
    },
    types::{
        block::{
            address::{Ed25519Address, ToBech32Ext},
            core::UnsignedBlock,
            BlockDto,
        },
        TryFromDto,
    },
};

use crate::{method::SecretManagerMethod, response::Response, Result};

/// Call a secret manager method.
pub(crate) async fn call_secret_manager_method_internal(
    secret_manager: &SecretManager,
    method: SecretManagerMethod,
) -> Result<Response> {
    let response = match method {
        SecretManagerMethod::GenerateEd25519Addresses { bech32_hrp, options } => {
            let options = serde_json::from_value(options)?;
            let addresses = Generate::<Vec<Ed25519Address>>::generate(secret_manager, &options)
                .await
                .map_err(iota_sdk::client::Error::from)?
                .into_iter()
                .map(|a| a.to_bech32(bech32_hrp))
                .collect();
            Response::GeneratedEd25519Addresses(addresses)
        }
        SecretManagerMethod::GenerateEvmAddresses { options } => {
            let options = serde_json::from_value(options)?;
            let addresses = Generate::<Vec<EvmAddress>>::generate(secret_manager, &options)
                .await
                .map_err(iota_sdk::client::Error::from)?
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
            protocol_parameters,
            signing_options,
        } => {
            let transaction = &secret_manager
                .sign_transaction(
                    PreparedTransactionData::try_from_dto(prepared_transaction_data)?,
                    &protocol_parameters,
                    &signing_options,
                )
                .await
                .map_err(iota_sdk::client::Error::from)?;
            Response::SignedTransaction(transaction.into())
        }
        SecretManagerMethod::SignBlock {
            unsigned_block,
            signing_options,
        } => Response::Block(BlockDto::from(
            &UnsignedBlock::try_from_dto(unsigned_block)?
                .sign_ed25519(secret_manager, &signing_options)
                .await?,
        )),
        SecretManagerMethod::SignatureUnlock {
            transaction_signing_hash,
            signing_options,
        } => {
            let transaction_signing_hash: [u8; 32] = prefix_hex::decode(transaction_signing_hash)?;
            let unlock = secret_manager
                .signature_unlock(&transaction_signing_hash, &signing_options)
                .await
                .map_err(iota_sdk::client::Error::from)?;

            Response::SignatureUnlock(unlock.into())
        }
        SecretManagerMethod::SignEd25519 {
            message,
            signing_options,
        } => {
            let msg: Vec<u8> = prefix_hex::decode(message)?;
            let signature = secret_manager
                .sign(&msg, &signing_options)
                .await
                .map_err(iota_sdk::client::Error::from)?;
            Response::Ed25519Signature(signature)
        }
        SecretManagerMethod::SignSecp256k1Ecdsa {
            message,
            signing_options,
        } => {
            let msg: Vec<u8> = prefix_hex::decode(message)?;
            let EvmSignature { public_key, signature } = secret_manager
                .sign(&msg, &signing_options)
                .await
                .map_err(iota_sdk::client::Error::from)?;
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
        #[cfg(feature = "stronghold")]
        SecretManagerMethod::SetStrongholdPassword { password } => {
            let stronghold = if let Some(secret_manager) = secret_manager.downcast::<StrongholdSecretManager>() {
                secret_manager
            } else if let Some(SecretManager::Stronghold(secret_manager)) = secret_manager.downcast::<SecretManager>() {
                secret_manager
            } else {
                return Err(iota_sdk::client::Error::SecretManagerMismatch.into());
            };
            stronghold.set_password(password).await?;
            Response::Ok
        }
    };
    Ok(response)
}
