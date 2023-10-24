// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(all(feature = "events", feature = "ledger_nano"))]
use {
    crate::client::api::PreparedTransactionDataDto,
    crate::client::secret::{
        ledger_nano::{needs_blind_signing, LedgerSecretManager},
        DowncastSecretManager,
    },
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
    wallet::account::{operations::transaction::SignedTransactionPayload, Account},
};

impl<S: 'static + SecretManage> Account<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    /// Signs a transaction.
    pub async fn sign_transaction(
        &self,
        prepared_transaction_data: &PreparedTransactionData,
    ) -> crate::wallet::Result<SignedTransactionData> {
        log::debug!("[TRANSACTION] sign_transaction");
        log::debug!("[TRANSACTION] prepared_transaction_data {prepared_transaction_data:?}");
        #[cfg(feature = "events")]
        self.emit(
            self.details().await.index,
            WalletEvent::TransactionProgress(TransactionProgressEvent::SigningTransaction),
        )
        .await;

        #[cfg(all(feature = "events", feature = "ledger_nano"))]
        {
            use crate::wallet::account::SecretManager;
            let secret_manager = self.wallet.secret_manager.read().await;
            if let Some(ledger) = secret_manager.downcast::<LedgerSecretManager>().or_else(|| {
                secret_manager.downcast::<SecretManager>().and_then(|s| {
                    if let SecretManager::LedgerNano(n) = s {
                        Some(n)
                    } else {
                        None
                    }
                })
            }) {
                let ledger_nano_status = ledger.get_ledger_nano_status().await;
                if let Some(buffer_size) = ledger_nano_status.buffer_size() {
                    if needs_blind_signing(prepared_transaction_data, buffer_size) {
                        self.emit(
                            self.details().await.index,
                            WalletEvent::TransactionProgress(TransactionProgressEvent::PreparedTransactionHash(
                                prepared_transaction_data.transaction.hash().to_string(),
                            )),
                        )
                        .await;
                    } else {
                        self.emit(
                            self.details().await.index,
                            WalletEvent::TransactionProgress(TransactionProgressEvent::PreparedTransaction(Box::new(
                                PreparedTransactionDataDto::from(prepared_transaction_data),
                            ))),
                        )
                        .await;
                    }
                }
            }
        }

        let unlocks = match self
            .wallet
            .secret_manager
            .read()
            .await
            .transaction_unlocks(prepared_transaction_data)
            .await
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
