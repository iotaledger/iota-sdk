// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    client::{
        api::{GetAddressesBuilder, PreparedTransactionData},
        secret::{SecretManage, SecretManager},
    },
    types::block::{payload::dto::PayloadDto, signature::dto::Ed25519SignatureDto, unlock::Unlock, Error},
};

use crate::{method::SecretManagerMethod, response::Response, Result};

/// Call a secret manager method.
pub(crate) async fn call_secret_manager_method_internal(
    secret_manager: &SecretManager,
    method: SecretManagerMethod,
) -> Result<Response> {
    let response = match method {
        SecretManagerMethod::GenerateAddresses { options } => {
            let addresses = GetAddressesBuilder::new(secret_manager)
                .set_options(options)?
                .finish()
                .await?
                .into_iter()
                .map(|a| a.to_string())
                .collect();
            Response::GeneratedAddresses(addresses)
        }
        #[cfg(feature = "ledger_nano")]
        SecretManagerMethod::GetLedgerNanoStatus => {
            if let SecretManager::LedgerNano(secret_manager) = secret_manager {
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
                    &prepared_transaction_data,
                )?)
                .await?;
            Response::SignedTransaction(PayloadDto::from(payload))
        }
        SecretManagerMethod::SignatureUnlock {
            transaction_essence_hash,
            chain,
        } => {
            let transaction_essence_hash: [u8; 32] = transaction_essence_hash
                .try_into()
                .map_err(|_| Error::InvalidField("expected 32 bytes for transactionEssenceHash"))?;

            let unlock: Unlock = secret_manager
                .signature_unlock(&transaction_essence_hash, &chain)
                .await?;

            Response::SignatureUnlock((&unlock).into())
        }
        SecretManagerMethod::SignEd25519 { message, chain } => {
            let msg: Vec<u8> = prefix_hex::decode(message)?;
            let signature = secret_manager.sign_ed25519(&msg, &chain).await?;
            Response::Ed25519Signature(Ed25519SignatureDto::from(&signature))
        }
        #[cfg(feature = "stronghold")]
        SecretManagerMethod::StoreMnemonic { mnemonic } => {
            if let SecretManager::Stronghold(secret_manager) = secret_manager {
                secret_manager.store_mnemonic(mnemonic).await?;
                Response::Ok
            } else {
                return Err(iota_sdk::client::Error::SecretManagerMismatch.into());
            }
        }
    };
    Ok(response)
}
