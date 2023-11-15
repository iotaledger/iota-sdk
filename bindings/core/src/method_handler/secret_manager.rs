// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "ledger_nano")]
use iota_sdk::client::secret::ledger_nano::LedgerSecretManager;
#[cfg(feature = "stronghold")]
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::{
    client::{
        api::{GetAddressesOptions, PreparedTransactionData},
        secret::{DowncastSecretManager, SecretManage, SecretManager, SignBlock},
    },
    types::{
        block::{address::ToBech32Ext, core::UnsignedBlock, unlock::Unlock, BlockDto},
        TryFromDto,
    },
};

use crate::{method::SecretManagerMethod, response::Response, Result};

/// Call a secret manager method.
pub(crate) async fn call_secret_manager_method_internal<S: SecretManage + DowncastSecretManager>(
    secret_manager: &S,
    method: SecretManagerMethod,
) -> Result<Response>
where
    iota_sdk::client::Error: From<S::Error>,
{
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
                .await
                .map_err(iota_sdk::client::Error::from)?
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
                    bech32_hrp: _,
                    options,
                },
        } => {
            let addresses = secret_manager
                .generate_evm_addresses(coin_type, account_index, range, options)
                .await
                .map_err(iota_sdk::client::Error::from)?
                .into_iter()
                .map(|a| prefix_hex::encode(a.as_ref()))
                .collect();
            Response::GeneratedEvmAddresses(addresses)
        }
        #[cfg(feature = "ledger_nano")]
        SecretManagerMethod::GetLedgerNanoStatus => {
            if let Some(secret_manager) = secret_manager.downcast::<LedgerSecretManager>() {
                Response::LedgerNanoStatus(secret_manager.get_ledger_nano_status().await)
            } else {
                return Err(iota_sdk::client::Error::SecretManagerMismatch.into());
            }
        }
        SecretManagerMethod::SignTransaction {
            prepared_transaction_data,
        } => {
            let transaction = &secret_manager
                .sign_transaction(PreparedTransactionData::try_from_dto(prepared_transaction_data)?)
                .await
                .map_err(iota_sdk::client::Error::from)?;
            Response::SignedTransaction(transaction.into())
        }
        SecretManagerMethod::SignBlock { unsigned_block, chain } => Response::SignedBlock(BlockDto::from(
            &UnsignedBlock::try_from_dto(unsigned_block)?
                .sign_ed25519(secret_manager, chain)
                .await?,
        )),
        SecretManagerMethod::SignatureUnlock {
            transaction_signing_hash,
            chain,
        } => {
            let transaction_signing_hash: [u8; 32] = prefix_hex::decode(transaction_signing_hash)?;
            let unlock: Unlock = secret_manager
                .signature_unlock(&transaction_signing_hash, chain)
                .await
                .map_err(iota_sdk::client::Error::from)?;

            Response::SignatureUnlock(unlock)
        }
        SecretManagerMethod::SignEd25519 { message, chain } => {
            let msg: Vec<u8> = prefix_hex::decode(message)?;
            let signature = secret_manager
                .sign_ed25519(&msg, chain)
                .await
                .map_err(iota_sdk::client::Error::from)?;
            Response::Ed25519Signature(signature)
        }
        SecretManagerMethod::SignSecp256k1Ecdsa { message, chain } => {
            let msg: Vec<u8> = prefix_hex::decode(message)?;
            let (public_key, signature) = secret_manager
                .sign_secp256k1_ecdsa(&msg, chain)
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
            if let Some(secret_manager) = secret_manager.downcast::<StrongholdSecretManager>() {
                secret_manager.store_mnemonic(mnemonic).await?;
                Response::Ok
            } else if let Some(secret_manager) = secret_manager.downcast::<SecretManager>() {
                if let SecretManager::Stronghold(secret_manager) = secret_manager {
                    secret_manager.store_mnemonic(mnemonic).await?;
                    Response::Ok
                } else {
                    return Err(iota_sdk::client::Error::SecretManagerMismatch.into());
                }
            } else {
                return Err(iota_sdk::client::Error::SecretManagerMismatch.into());
            }
        }
    };
    Ok(response)
}
