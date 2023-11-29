// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(all(feature = "events", feature = "ledger_nano"))]
use {
    crate::client::api::PreparedTransactionDataDto,
    crate::client::secret::{ledger_nano::needs_blind_signing, DowncastSecretManager},
};

#[cfg(feature = "events")]
use crate::wallet::events::types::{TransactionProgressEvent, WalletEvent};
use crate::{
    client::{
        api::{
            transaction::validate_signed_transaction_payload_length, PreparedTransactionData, SignedTransactionData,
        },
        secret::SecretManage,
    },
    wallet::{operations::transaction::SignedTransactionPayload, Wallet},
};

impl Wallet {
    /// Signs a transaction.
    pub async fn sign_transaction<S: 'static + SecretManage>(
        &self,
        secret_manager: &S,
        prepared_transaction_data: &PreparedTransactionData,
    ) -> crate::wallet::Result<SignedTransactionData>
    where
        crate::client::Error: From<S::Error>,
    {
        log::debug!("[TRANSACTION] sign_transaction");
        log::debug!("[TRANSACTION] prepared_transaction_data {prepared_transaction_data:?}");
        #[cfg(feature = "events")]
        self.emit(WalletEvent::TransactionProgress(
            TransactionProgressEvent::SigningTransaction,
        ))
        .await;

        #[cfg(all(feature = "events", feature = "ledger_nano"))]
        {
            if let Ok(ledger) = secret_manager.as_ledger_nano() {
                let ledger_nano_status = ledger.get_ledger_nano_status().await;
                if let Some(buffer_size) = ledger_nano_status.buffer_size() {
                    if needs_blind_signing(prepared_transaction_data, buffer_size) {
                        self.emit(WalletEvent::TransactionProgress(
                            TransactionProgressEvent::PreparedTransactionSigningHash(
                                prepared_transaction_data.transaction.signing_hash().to_string(),
                            ),
                        ))
                        .await;
                    } else {
                        self.emit(WalletEvent::TransactionProgress(
                            TransactionProgressEvent::PreparedTransaction(Box::new(PreparedTransactionDataDto::from(
                                prepared_transaction_data,
                            ))),
                        ))
                        .await;
                    }
                }
            }
        }

        let unlocks = match secret_manager
            .transaction_unlocks(prepared_transaction_data)
            .await
            .map_err(crate::client::Error::from)
        {
            Ok(res) => res,
            Err(err) => {
                // unlock outputs so they are available for a new transaction
                self.unlock_inputs(&prepared_transaction_data.inputs_data).await?;
                return Err(err.into());
            }
        };
        let payload = SignedTransactionPayload::new(prepared_transaction_data.transaction.clone(), unlocks)?;

        log::debug!("[TRANSACTION] signed transaction: {:?}", payload);

        validate_signed_transaction_payload_length(&payload)?;

        Ok(SignedTransactionData {
            payload,
            inputs_data: prepared_transaction_data.inputs_data.clone(),
        })
    }
}
