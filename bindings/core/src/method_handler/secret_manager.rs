// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    client::{
        api::GetAddressesBuilder,
        secret::{SecretManage, SecretManager},
    },
    types::block::{signature::dto::Ed25519SignatureDto, unlock::Unlock, DtoError},
};

use crate::{method::SecretManagerMethod, method_handler::Result, response::Response};

/// Call a secret manager method.
pub(crate) async fn call_secret_manager_method_internal(
    secret_manager: &mut SecretManager,
    method: SecretManagerMethod,
) -> Result<Response> {
    match method {
        SecretManagerMethod::GenerateAddresses { options } => {
            let addresses = GetAddressesBuilder::new(secret_manager)
                .set_options(options)?
                .finish()
                .await?;
            Ok(Response::GeneratedAddresses(addresses))
        }
        #[cfg(feature = "ledger_nano")]
        SecretManagerMethod::GetLedgerNanoStatus => {
            if let SecretManager::LedgerNano(secret_manager) = secret_manager {
                Ok(Response::LedgerNanoStatus(
                    secret_manager.get_ledger_nano_status().await,
                ))
            } else {
                Err(iota_sdk::client::Error::SecretManagerMismatch.into())
            }
        }
        // TODO: support this from secret manager alone without client?
        // SecretManagerMethod::SignTransaction {
        //     secret_manager,
        //     prepared_transaction_data,
        // } => {
        //     let mut block_builder = client.block();

        //     let secret_manager = (&secret_manager).try_into()?;

        //     block_builder = block_builder.with_secret_manager(&secret_manager);

        //     Ok(Response::SignedTransaction(PayloadDto::from(
        //         &block_builder
        //             .sign_transaction(PreparedTransactionData::try_from_dto_unverified(
        //                 &prepared_transaction_data,
        //             )?)
        //             .await?,
        //     )))
        // }
        SecretManagerMethod::SignatureUnlock {
            transaction_essence_hash,
            chain,
        } => {
            let transaction_essence_hash: [u8; 32] = transaction_essence_hash
                .try_into()
                .map_err(|_| DtoError::InvalidField("expected 32 bytes for transactionEssenceHash"))?;

            let unlock: Unlock = secret_manager
                .signature_unlock(&transaction_essence_hash, &chain)
                .await?;

            Ok(Response::SignatureUnlock((&unlock).into()))
        }
        SecretManagerMethod::SignEd25519 { message, chain } => {
            let msg: Vec<u8> = prefix_hex::decode(message)?;
            let signature = secret_manager.sign_ed25519(&msg, &chain).await?;
            Ok(Response::Ed25519Signature(Ed25519SignatureDto::from(&signature)))
        }
        #[cfg(feature = "stronghold")]
        SecretManagerMethod::StoreMnemonic { mnemonic } => {
            if let SecretManager::Stronghold(secret_manager) = secret_manager {
                secret_manager.store_mnemonic(mnemonic).await?;
                Ok(Response::Ok)
            } else {
                Err(iota_sdk::client::Error::SecretManagerMismatch.into())
            }
        }
    }
}
