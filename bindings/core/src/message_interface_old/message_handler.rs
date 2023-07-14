// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "participation")]
use std::str::FromStr;
use std::{
    any::Any,
    panic::{catch_unwind, AssertUnwindSafe},
    time::Duration,
};

use backtrace::Backtrace;
use crypto::keys::bip39::Mnemonic;
use futures::{Future, FutureExt};
#[cfg(feature = "events")]
use iota_sdk::wallet::events::types::{Event, WalletEventType};
use iota_sdk::{
    client::{
        api::{PreparedTransactionData, PreparedTransactionDataDto, SignedTransactionData, SignedTransactionDataDto},
        constants::SHIMMER_TESTNET_BECH32_HRP,
        request_funds_from_faucet,
        secret::SecretManage,
        utils, Client, NodeInfoWrapper,
    },
    types::block::{
        address::{Hrp, ToBech32Ext},
        output::{
            dto::{OutputBuilderAmountDto, OutputDto},
            AliasOutput, BasicOutput, FoundryOutput, NativeToken, NftOutput, Output, Rent,
        },
        signature::Ed25519Signature,
        ConvertTo, Error,
    },
    wallet::{
        account::{
            types::{AccountIdentifier, TransactionDto},
            AccountDetailsDto, CreateNativeTokenTransactionDto, OutputDataDto, TransactionOptions,
        },
        Result, Wallet,
    },
};

use crate::message_interface_old::{account_method::AccountMethod, message::Message, response::Response};

fn panic_to_response_message(panic: Box<dyn Any>) -> Response {
    let msg = panic.downcast_ref::<String>().map_or_else(
        || {
            panic.downcast_ref::<&str>().map_or_else(
                || "Internal error".to_string(),
                |message| format!("Internal error: {message}"),
            )
        },
        |message| format!("Internal error: {message}"),
    );

    let current_backtrace = Backtrace::new();
    Response::Panic(format!("{msg}\n\n{current_backtrace:?}"))
}

fn convert_panics<F: FnOnce() -> Result<Response>>(f: F) -> Result<Response> {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(result) => result,
        Err(panic) => Ok(panic_to_response_message(panic)),
    }
}

#[cfg(not(target_family = "wasm"))]
async fn convert_async_panics<F>(f: impl FnOnce() -> F + Send) -> Result<Response>
where
    F: Future<Output = Result<Response>> + Send,
{
    AssertUnwindSafe(f())
        .catch_unwind()
        .await
        .unwrap_or_else(|panic| Ok(panic_to_response_message(panic)))
}

#[cfg(target_family = "wasm")]
#[allow(clippy::future_not_send)]
async fn convert_async_panics<F>(f: impl FnOnce() -> F) -> Result<Response>
where
    F: Future<Output = Result<Response>>,
{
    AssertUnwindSafe(f())
        .catch_unwind()
        .await
        .unwrap_or_else(|panic| Ok(panic_to_response_message(panic)))
}

/// The Wallet message handler.
pub struct WalletMessageHandler {
    wallet: Wallet,
}

impl WalletMessageHandler {
    /// Creates a new instance of the message handler with the default wallet.
    pub async fn new() -> Result<Self> {
        let instance = Self {
            wallet: Wallet::builder().finish().await?,
        };
        Ok(instance)
    }

    /// Creates a new instance of the message handler with the specified wallet.
    pub fn with_manager(wallet: Wallet) -> Self {
        Self { wallet }
    }

    /// Listen to wallet events, empty vec will listen to all events
    #[cfg(feature = "events")]
    #[cfg_attr(docsrs, doc(cfg(feature = "events")))]
    pub async fn listen<F, I: IntoIterator<Item = WalletEventType> + Send>(&self, events: I, handler: F)
    where
        I::IntoIter: Send,
        F: Fn(&Event) + 'static + Clone + Send + Sync,
    {
        self.wallet.listen(events, handler).await;
    }

    /// Send a message.
    pub async fn send_message(&self, message: Message) -> Response {
        log::debug!("Message: {:?}", message);

        let response: Result<Response> = match message {
            Message::CreateAccount { alias, bech32_hrp } => {
                convert_async_panics(|| async { self.create_account(alias, bech32_hrp).await }).await
            }
            Message::GetAccount { account_id } => {
                convert_async_panics(|| async { self.get_account(&account_id).await }).await
            }
            Message::GetAccountIndexes => {
                convert_async_panics(|| async {
                    let accounts = self.wallet.get_accounts().await?;
                    let mut account_indexes = Vec::new();
                    for account in accounts.iter() {
                        account_indexes.push(*account.details().await.index());
                    }
                    Ok(Response::AccountIndexes(account_indexes))
                })
                .await
            }
            Message::GetAccounts => convert_async_panics(|| async { self.get_accounts().await }).await,
            Message::CallAccountMethod { account_id, method } => {
                convert_async_panics(|| async { self.call_account_method(&account_id, method).await }).await
            }
            #[cfg(feature = "stronghold")]
            Message::Backup { destination, password } => {
                convert_async_panics(|| async {
                    self.wallet.backup(destination.to_path_buf(), password).await?;
                    Ok(Response::Ok(()))
                })
                .await
            }
            #[cfg(feature = "stronghold")]
            Message::ChangeStrongholdPassword {
                current_password,
                new_password,
            } => {
                convert_async_panics(|| async {
                    self.wallet
                        .change_stronghold_password(current_password, new_password)
                        .await?;
                    Ok(Response::Ok(()))
                })
                .await
            }
            #[cfg(feature = "stronghold")]
            Message::ClearStrongholdPassword => {
                convert_async_panics(|| async {
                    self.wallet.clear_stronghold_password().await?;
                    Ok(Response::Ok(()))
                })
                .await
            }
            #[cfg(feature = "stronghold")]
            Message::IsStrongholdPasswordAvailable => {
                convert_async_panics(|| async {
                    let is_available = self.wallet.is_stronghold_password_available().await?;
                    Ok(Response::Bool(is_available))
                })
                .await
            }
            Message::RecoverAccounts {
                account_start_index,
                account_gap_limit,
                address_gap_limit,
                sync_options,
            } => {
                convert_async_panics(|| async {
                    let accounts = self
                        .wallet
                        .recover_accounts(account_start_index, account_gap_limit, address_gap_limit, sync_options)
                        .await?;
                    Ok(Response::Accounts(
                        futures::future::join_all(
                            accounts
                                .into_iter()
                                .map(|account| async move { AccountDetailsDto::from(&*account.details().await) }),
                        )
                        .await,
                    ))
                })
                .await
            }
            Message::RemoveLatestAccount => {
                convert_async_panics(|| async {
                    self.wallet.remove_latest_account().await?;
                    Ok(Response::Ok(()))
                })
                .await
            }
            #[cfg(feature = "stronghold")]
            Message::RestoreBackup {
                source,
                password,
                ignore_if_coin_type_mismatch,
                ignore_if_bech32_mismatch,
            } => {
                convert_async_panics(|| async {
                    self.wallet
                        .restore_backup(
                            source.to_path_buf(),
                            password,
                            ignore_if_coin_type_mismatch,
                            ignore_if_bech32_mismatch,
                        )
                        .await?;
                    Ok(Response::Ok(()))
                })
                .await
            }
            Message::GenerateMnemonic => convert_panics(|| {
                self.wallet
                    .generate_mnemonic()
                    .map(|m| Response::GeneratedMnemonic(m.as_ref().to_owned()))
            }),
            Message::VerifyMnemonic { mnemonic } => convert_panics(|| {
                let mnemonic = Mnemonic::from(mnemonic);
                self.wallet.verify_mnemonic(&mnemonic)?;
                Ok(Response::Ok(()))
            }),
            Message::SetClientOptions { client_options } => {
                convert_async_panics(|| async {
                    self.wallet.set_client_options(*client_options).await?;
                    Ok(Response::Ok(()))
                })
                .await
            }
            #[cfg(feature = "ledger_nano")]
            Message::GetLedgerNanoStatus => {
                convert_async_panics(|| async {
                    let ledger_nano_status = self.wallet.get_ledger_nano_status().await?;
                    Ok(Response::LedgerNanoStatus(ledger_nano_status))
                })
                .await
            }
            Message::GenerateEd25519Address {
                account_index,
                address_index,
                options,
                bech32_hrp,
            } => {
                convert_async_panics(|| async {
                    let address = self
                        .wallet
                        .generate_ed25519_address(account_index, address_index, options)
                        .await?;

                    let bech32_hrp = match bech32_hrp {
                        Some(bech32_hrp) => bech32_hrp,
                        None => self.wallet.get_bech32_hrp().await?,
                    };

                    Ok(Response::Bech32Address(address.to_bech32(bech32_hrp)))
                })
                .await
            }
            Message::GetNodeInfo { url, auth } => {
                convert_async_panics(|| async {
                    match url {
                        Some(url) => {
                            let node_info = Client::get_node_info(&url, auth).await?;
                            Ok(Response::NodeInfo(NodeInfoWrapper { node_info, url }))
                        }
                        None => Ok(self.wallet.client().get_info().await.map(Response::NodeInfo)?),
                    }
                })
                .await
            }
            #[cfg(feature = "stronghold")]
            Message::SetStrongholdPassword { password } => {
                convert_async_panics(|| async {
                    self.wallet.set_stronghold_password(password).await?;
                    Ok(Response::Ok(()))
                })
                .await
            }
            #[cfg(feature = "stronghold")]
            Message::SetStrongholdPasswordClearInterval {
                interval_in_milliseconds,
            } => {
                convert_async_panics(|| async {
                    let duration = interval_in_milliseconds.map(Duration::from_millis);
                    self.wallet.set_stronghold_password_clear_interval(duration).await?;
                    Ok(Response::Ok(()))
                })
                .await
            }
            #[cfg(feature = "stronghold")]
            Message::StoreMnemonic { mnemonic } => {
                let mnemonic = mnemonic.into();
                convert_async_panics(|| async {
                    self.wallet.store_mnemonic(mnemonic).await?;
                    Ok(Response::Ok(()))
                })
                .await
            }
            Message::StartBackgroundSync {
                options,
                interval_in_milliseconds,
            } => {
                convert_async_panics(|| async {
                    let duration = interval_in_milliseconds.map(Duration::from_millis);
                    self.wallet.start_background_syncing(options, duration).await?;
                    Ok(Response::Ok(()))
                })
                .await
            }
            Message::StopBackgroundSync => {
                convert_async_panics(|| async {
                    self.wallet.stop_background_syncing().await?;
                    Ok(Response::Ok(()))
                })
                .await
            }
            #[cfg(feature = "events")]
            Message::EmitTestEvent { event } => {
                convert_async_panics(|| async {
                    self.wallet.emit_test_event(event.clone()).await;
                    Ok(Response::Ok(()))
                })
                .await
            }
            Message::Bech32ToHex { bech32_address } => {
                convert_panics(|| Ok(Response::HexAddress(utils::bech32_to_hex(bech32_address)?)))
            }
            Message::HexToBech32 { hex, bech32_hrp } => {
                convert_async_panics(|| async {
                    let bech32_hrp = match bech32_hrp {
                        Some(bech32_hrp) => bech32_hrp,
                        None => self
                            .wallet
                            .client()
                            .get_bech32_hrp()
                            .await
                            .unwrap_or(SHIMMER_TESTNET_BECH32_HRP),
                    };

                    Ok(Response::Bech32Address(utils::hex_to_bech32(&hex, bech32_hrp)?))
                })
                .await
            }
            #[cfg(feature = "events")]
            Message::ClearListeners { event_types } => {
                convert_async_panics(|| async {
                    self.wallet.clear_listeners(event_types).await;
                    Ok(Response::Ok(()))
                })
                .await
            }
            Message::UpdateNodeAuth { url, auth } => {
                convert_async_panics(|| async {
                    self.wallet.update_node_auth(url, auth).await?;
                    Ok(Response::Ok(()))
                })
                .await
            }
        };

        let response = match response {
            Ok(r) => r,
            Err(e) => Response::Error(e),
        };

        log::debug!("Response: {:?}", response);

        response
    }

    #[allow(clippy::large_stack_frames)] // Temporarily allowed as this module will soon be removed.
    async fn call_account_method(&self, account_id: &AccountIdentifier, method: AccountMethod) -> Result<Response> {
        let account = self.wallet.get_account(account_id.clone()).await?;

        match method {
            AccountMethod::BuildAliasOutput {
                amount,
                native_tokens,
                alias_id,
                state_index,
                state_metadata,
                foundry_counter,
                unlock_conditions,
                features,
                immutable_features,
            } => {
                let output = Output::from(AliasOutput::try_from_dtos(
                    if let Some(amount) = amount {
                        OutputBuilderAmountDto::Amount(amount)
                    } else {
                        OutputBuilderAmountDto::MinimumStorageDeposit(account.client().get_rent_structure().await?)
                    },
                    native_tokens,
                    &alias_id,
                    state_index,
                    state_metadata,
                    foundry_counter,
                    unlock_conditions,
                    features,
                    immutable_features,
                    account.client().get_token_supply().await?,
                )?);

                Ok(Response::Output(OutputDto::from(&output)))
            }
            AccountMethod::BuildBasicOutput {
                amount,
                native_tokens,
                unlock_conditions,
                features,
            } => {
                let output = Output::from(BasicOutput::try_from_dtos(
                    if let Some(amount) = amount {
                        OutputBuilderAmountDto::Amount(amount)
                    } else {
                        OutputBuilderAmountDto::MinimumStorageDeposit(account.client().get_rent_structure().await?)
                    },
                    native_tokens,
                    unlock_conditions,
                    features,
                    account.client().get_token_supply().await?,
                )?);

                Ok(Response::Output(OutputDto::from(&output)))
            }
            AccountMethod::BuildFoundryOutput {
                amount,
                native_tokens,
                serial_number,
                token_scheme,
                unlock_conditions,
                features,
                immutable_features,
            } => {
                let output = Output::from(FoundryOutput::try_from_dtos(
                    if let Some(amount) = amount {
                        OutputBuilderAmountDto::Amount(amount)
                    } else {
                        OutputBuilderAmountDto::MinimumStorageDeposit(account.client().get_rent_structure().await?)
                    },
                    native_tokens,
                    serial_number,
                    token_scheme,
                    unlock_conditions,
                    features,
                    immutable_features,
                    account.client().get_token_supply().await?,
                )?);

                Ok(Response::Output(OutputDto::from(&output)))
            }
            AccountMethod::BuildNftOutput {
                amount,
                native_tokens,
                nft_id,
                unlock_conditions,
                features,
                immutable_features,
            } => {
                let output = Output::from(NftOutput::try_from_dtos(
                    if let Some(amount) = amount {
                        OutputBuilderAmountDto::Amount(amount)
                    } else {
                        OutputBuilderAmountDto::MinimumStorageDeposit(account.client().get_rent_structure().await?)
                    },
                    native_tokens,
                    &nft_id,
                    unlock_conditions,
                    features,
                    immutable_features,
                    account.client().get_token_supply().await?,
                )?);

                Ok(Response::Output(OutputDto::from(&output)))
            }
            AccountMethod::BurnNativeToken {
                token_id,
                burn_amount,
                options,
            } => {
                convert_async_panics(|| async {
                    let transaction = account
                        .burn(
                            NativeToken::new(token_id, burn_amount)?,
                            options.map(TransactionOptions::try_from_dto).transpose()?,
                        )
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            AccountMethod::BurnNft { nft_id, options } => {
                convert_async_panics(|| async {
                    let transaction = account
                        .burn(nft_id, options.map(TransactionOptions::try_from_dto).transpose()?)
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            AccountMethod::ClaimableOutputs { outputs_to_claim } => {
                let output_ids = account.claimable_outputs(outputs_to_claim).await?;
                Ok(Response::OutputIds(output_ids))
            }
            AccountMethod::ConsolidateOutputs {
                force,
                output_consolidation_threshold,
            } => {
                convert_async_panics(|| async {
                    let transaction = account
                        .consolidate_outputs(force, output_consolidation_threshold)
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            AccountMethod::CreateAliasOutput { params, options } => {
                convert_async_panics(|| async {
                    let transaction = account
                        .create_alias_output(params, options.map(TransactionOptions::try_from_dto).transpose()?)
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            AccountMethod::DestroyAlias { alias_id, options } => {
                convert_async_panics(|| async {
                    let transaction = account
                        .burn(alias_id, options.map(TransactionOptions::try_from_dto).transpose()?)
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            AccountMethod::DestroyFoundry { foundry_id, options } => {
                convert_async_panics(|| async {
                    let transaction = account
                        .burn(foundry_id, options.map(TransactionOptions::try_from_dto).transpose()?)
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            AccountMethod::GenerateEd25519Addresses { amount, options } => {
                let address = account.generate_ed25519_addresses(amount, options).await?;
                Ok(Response::GeneratedEd25519Addresses(address))
            }
            AccountMethod::GenerateEvmAddresses { options } => {
                let addresses = account
                    .get_secret_manager()
                    .read()
                    .await
                    .generate_evm_addresses(options)
                    .await?;
                Ok(Response::GeneratedEvmAddresses(addresses))
            }
            AccountMethod::VerifyEd25519Signature { signature, message } => {
                let signature = Ed25519Signature::try_from(signature)?;
                let message: Vec<u8> = prefix_hex::decode(message).map_err(iota_sdk::client::Error::from)?;
                Ok(Response::Bool(signature.verify(&message)))
            }
            AccountMethod::VerifySecp256k1EcdsaSignature {
                public_key,
                signature,
                message,
            } => {
                use crypto::signatures::secp256k1_ecdsa;
                let public_key = prefix_hex::decode(public_key).map_err(|_| Error::InvalidField("publicKey"))?;
                let public_key = secp256k1_ecdsa::PublicKey::try_from_bytes(&public_key)?;
                let signature = prefix_hex::decode(signature).map_err(|_| Error::InvalidField("signature"))?;
                let signature = secp256k1_ecdsa::Signature::try_from_bytes(&signature)?;
                let message: Vec<u8> = prefix_hex::decode(message).map_err(iota_sdk::client::Error::from)?;
                Ok(Response::Bool(public_key.verify_keccak256(&signature, &message)))
            }
            AccountMethod::SignSecp256k1Ecdsa { message, chain } => {
                let msg: Vec<u8> = prefix_hex::decode(message).map_err(iota_sdk::client::Error::from)?;
                let (public_key, signature) = account
                    .get_secret_manager()
                    .read()
                    .await
                    .sign_secp256k1_ecdsa(&msg, chain)
                    .await?;
                Ok(Response::Secp256k1EcdsaSignature {
                    public_key: prefix_hex::encode(public_key.to_bytes()),
                    signature: prefix_hex::encode(signature.to_bytes()),
                })
            }
            AccountMethod::GetOutput { output_id } => {
                let output_data = account.get_output(&output_id).await;
                Ok(Response::OutputData(
                    output_data.as_ref().map(OutputDataDto::from).map(Box::new),
                ))
            }
            AccountMethod::GetFoundryOutput { token_id } => {
                let output = account.get_foundry_output(token_id).await?;
                Ok(Response::Output(OutputDto::from(&output)))
            }
            AccountMethod::GetTransaction { transaction_id } => {
                let transaction = account.get_transaction(&transaction_id).await;
                Ok(Response::Transaction(
                    transaction.as_ref().map(TransactionDto::from).map(Box::new),
                ))
            }
            AccountMethod::GetIncomingTransaction { transaction_id } => {
                let transaction = account.get_incoming_transaction(&transaction_id).await;

                transaction.map_or_else(
                    || Ok(Response::Transaction(None)),
                    |transaction| {
                        Ok(Response::Transaction(Some(Box::new(TransactionDto::from(
                            &transaction,
                        )))))
                    },
                )
            }
            AccountMethod::Addresses => {
                let addresses = account.addresses().await?;
                Ok(Response::Addresses(addresses))
            }
            AccountMethod::AddressesWithUnspentOutputs => {
                let addresses = account.addresses_with_unspent_outputs().await?;
                Ok(Response::AddressesWithUnspentOutputs(addresses))
            }
            AccountMethod::Outputs { filter_options } => {
                let outputs = account.outputs(filter_options).await?;
                Ok(Response::OutputsData(outputs.iter().map(OutputDataDto::from).collect()))
            }
            AccountMethod::UnspentOutputs { filter_options } => {
                let outputs = account.unspent_outputs(filter_options).await?;
                Ok(Response::OutputsData(outputs.iter().map(OutputDataDto::from).collect()))
            }
            AccountMethod::IncomingTransactions => {
                let transactions = account.incoming_transactions().await;
                Ok(Response::Transactions(
                    transactions.iter().map(TransactionDto::from).collect(),
                ))
            }
            AccountMethod::Transactions => {
                let transactions = account.transactions().await;
                Ok(Response::Transactions(
                    transactions.iter().map(TransactionDto::from).collect(),
                ))
            }
            AccountMethod::PendingTransactions => {
                let transactions = account.pending_transactions().await;
                Ok(Response::Transactions(
                    transactions.iter().map(TransactionDto::from).collect(),
                ))
            }
            AccountMethod::CreateNativeToken { params, options } => {
                convert_async_panics(|| async {
                    let transaction = account
                        .create_native_token(params, options.map(TransactionOptions::try_from_dto).transpose()?)
                        .await?;
                    Ok(Response::CreateNativeTokenTransaction(
                        CreateNativeTokenTransactionDto::from(&transaction),
                    ))
                })
                .await
            }
            AccountMethod::MeltNativeToken {
                token_id,
                melt_amount,
                options,
            } => {
                convert_async_panics(|| async {
                    let transaction = account
                        .melt_native_token(
                            token_id,
                            melt_amount,
                            options.map(TransactionOptions::try_from_dto).transpose()?,
                        )
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            AccountMethod::MintNativeToken {
                token_id,
                mint_amount,
                options,
            } => {
                convert_async_panics(|| async {
                    let transaction = account
                        .mint_native_token(
                            token_id,
                            mint_amount,
                            options.map(TransactionOptions::try_from_dto).transpose()?,
                        )
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            AccountMethod::MinimumRequiredStorageDeposit { output } => {
                convert_async_panics(|| async {
                    let output = Output::try_from_dto(output, account.client().get_token_supply().await?)?;
                    let rent_structure = account.client().get_rent_structure().await?;

                    let minimum_storage_deposit = output.rent_cost(&rent_structure);

                    Ok(Response::MinimumRequiredStorageDeposit(
                        minimum_storage_deposit.to_string(),
                    ))
                })
                .await
            }
            AccountMethod::MintNfts { params, options } => {
                convert_async_panics(|| async {
                    let transaction = account
                        .mint_nfts(params, options.map(TransactionOptions::try_from_dto).transpose()?)
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            AccountMethod::GetBalance => Ok(Response::Balance(account.balance().await?)),
            AccountMethod::PrepareOutput {
                params,
                transaction_options,
            } => {
                convert_async_panics(|| async {
                    let output = account
                        .prepare_output(
                            *params,
                            transaction_options.map(TransactionOptions::try_from_dto).transpose()?,
                        )
                        .await?;
                    Ok(Response::Output(OutputDto::from(&output)))
                })
                .await
            }
            AccountMethod::PrepareSend { params, options } => {
                convert_async_panics(|| async {
                    let data = account
                        .prepare_send(params, options.map(TransactionOptions::try_from_dto).transpose()?)
                        .await?;
                    Ok(Response::PreparedTransaction(PreparedTransactionDataDto::from(&data)))
                })
                .await
            }
            AccountMethod::PrepareTransaction { outputs, options } => {
                convert_async_panics(|| async {
                    let token_supply = account.client().get_token_supply().await?;
                    let data = account
                        .prepare_transaction(
                            outputs
                                .into_iter()
                                .map(|o| Ok(Output::try_from_dto(o, token_supply)?))
                                .collect::<Result<Vec<Output>>>()?,
                            options.map(TransactionOptions::try_from_dto).transpose()?,
                        )
                        .await?;
                    Ok(Response::PreparedTransaction(PreparedTransactionDataDto::from(&data)))
                })
                .await
            }
            AccountMethod::RetryTransactionUntilIncluded {
                transaction_id,
                interval,
                max_attempts,
            } => {
                convert_async_panics(|| async {
                    let block_id = account
                        .retry_transaction_until_included(&transaction_id, interval, max_attempts)
                        .await?;
                    Ok(Response::BlockId(block_id))
                })
                .await
            }
            AccountMethod::SyncAccount { options } => Ok(Response::Balance(account.sync(options).await?)),
            AccountMethod::Send { params, options } => {
                convert_async_panics(|| async {
                    let transaction = account
                        .send_with_params(params, options.map(TransactionOptions::try_from_dto).transpose()?)
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            AccountMethod::SendNativeTokens { params, options } => {
                convert_async_panics(|| async {
                    let transaction = account
                        .send_native_tokens(
                            params.clone(),
                            options.map(TransactionOptions::try_from_dto).transpose()?,
                        )
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            AccountMethod::SendNft { params, options } => {
                convert_async_panics(|| async {
                    let transaction = account
                        .send_nft(
                            params.clone(),
                            options.map(TransactionOptions::try_from_dto).transpose()?,
                        )
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            AccountMethod::SetAlias { alias } => {
                convert_async_panics(|| async {
                    account.set_alias(&alias).await?;
                    Ok(Response::Ok(()))
                })
                .await
            }
            AccountMethod::SetDefaultSyncOptions { options } => {
                convert_async_panics(|| async {
                    account.set_default_sync_options(options).await?;
                    Ok(Response::Ok(()))
                })
                .await
            }
            AccountMethod::SendOutputs { outputs, options } => {
                convert_async_panics(|| async {
                    let token_supply = account.client().get_token_supply().await?;
                    let transaction = account
                        .send_outputs(
                            outputs
                                .into_iter()
                                .map(|o| Ok(Output::try_from_dto(o, token_supply)?))
                                .collect::<iota_sdk::wallet::Result<Vec<_>>>()?,
                            options.map(TransactionOptions::try_from_dto).transpose()?,
                        )
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            AccountMethod::SignTransactionEssence {
                prepared_transaction_data,
            } => {
                convert_async_panics(|| async {
                    let signed_transaction_data = account
                        .sign_transaction_essence(&PreparedTransactionData::try_from_dto(
                            prepared_transaction_data,
                            &account.client().get_protocol_parameters().await?,
                        )?)
                        .await?;
                    Ok(Response::SignedTransactionData(SignedTransactionDataDto::from(
                        &signed_transaction_data,
                    )))
                })
                .await
            }
            AccountMethod::SubmitAndStoreTransaction {
                signed_transaction_data,
            } => {
                convert_async_panics(|| async {
                    let signed_transaction_data = SignedTransactionData::try_from_dto(
                        signed_transaction_data,
                        &account.client().get_protocol_parameters().await?,
                    )?;
                    let transaction = account
                        .submit_and_store_transaction(signed_transaction_data, None)
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            AccountMethod::ClaimOutputs { output_ids_to_claim } => {
                convert_async_panics(|| async {
                    let transaction = account.claim_outputs(output_ids_to_claim.to_vec()).await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            #[cfg(feature = "participation")]
            AccountMethod::Vote { event_id, answers } => {
                convert_async_panics(|| async {
                    let transaction = account.vote(event_id, answers).await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            #[cfg(feature = "participation")]
            AccountMethod::StopParticipating { event_id } => {
                convert_async_panics(|| async {
                    let transaction = account.stop_participating(event_id).await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            #[cfg(feature = "participation")]
            AccountMethod::GetParticipationOverview { event_ids } => {
                convert_async_panics(|| async {
                    let overview = account.get_participation_overview(event_ids).await?;
                    Ok(Response::AccountParticipationOverview(overview))
                })
                .await
            }
            #[cfg(feature = "participation")]
            AccountMethod::IncreaseVotingPower { amount } => {
                convert_async_panics(|| async {
                    let transaction = account
                        .increase_voting_power(
                            u64::from_str(&amount)
                                .map_err(|_| iota_sdk::client::Error::InvalidAmount(amount.clone()))?,
                        )
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            #[cfg(feature = "participation")]
            AccountMethod::DecreaseVotingPower { amount } => {
                convert_async_panics(|| async {
                    let transaction = account
                        .decrease_voting_power(
                            u64::from_str(&amount)
                                .map_err(|_| iota_sdk::client::Error::InvalidAmount(amount.clone()))?,
                        )
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            #[cfg(feature = "participation")]
            AccountMethod::RegisterParticipationEvents { options } => {
                convert_async_panics(|| async {
                    let events = account.register_participation_events(&options).await?;
                    Ok(Response::ParticipationEvents(events))
                })
                .await
            }
            #[cfg(feature = "participation")]
            AccountMethod::DeregisterParticipationEvent { event_id } => {
                convert_async_panics(|| async {
                    account.deregister_participation_event(&event_id).await?;
                    Ok(Response::Ok(()))
                })
                .await
            }
            #[cfg(feature = "participation")]
            AccountMethod::GetParticipationEvent { event_id } => {
                convert_async_panics(|| async {
                    let event_and_nodes = account.get_participation_event(event_id).await?;
                    Ok(Response::ParticipationEvent(event_and_nodes))
                })
                .await
            }
            #[cfg(feature = "participation")]
            AccountMethod::GetParticipationEventIds { node, event_type } => {
                convert_async_panics(|| async {
                    let event_ids = account.get_participation_event_ids(&node, event_type).await?;
                    Ok(Response::ParticipationEventIds(event_ids))
                })
                .await
            }
            #[cfg(feature = "participation")]
            AccountMethod::GetParticipationEventStatus { event_id } => {
                convert_async_panics(|| async {
                    let event_status = account.get_participation_event_status(&event_id).await?;
                    Ok(Response::ParticipationEventStatus(event_status))
                })
                .await
            }
            #[cfg(feature = "participation")]
            AccountMethod::GetParticipationEvents => {
                convert_async_panics(|| async {
                    let events = account.get_participation_events().await?;
                    Ok(Response::ParticipationEvents(events))
                })
                .await
            }
            AccountMethod::RequestFundsFromFaucet { url, address } => {
                convert_async_panics(|| async {
                    Ok(Response::Faucet(request_funds_from_faucet(&url, &address).await?))
                })
                .await
            }
        }
    }

    /// The create account message handler.
    async fn create_account(&self, alias: Option<String>, bech32_hrp: Option<impl ConvertTo<Hrp>>) -> Result<Response> {
        let mut builder = self.wallet.create_account();

        if let Some(alias) = alias {
            builder = builder.with_alias(alias);
        }

        if let Some(bech32_hrp) = bech32_hrp {
            builder = builder.with_bech32_hrp(bech32_hrp.convert()?);
        }

        match builder.finish().await {
            Ok(account) => {
                let account = account.details().await;
                Ok(Response::Account(AccountDetailsDto::from(&*account)))
            }
            Err(e) => Err(e),
        }
    }

    async fn get_account(&self, account_id: &AccountIdentifier) -> Result<Response> {
        let account = self.wallet.get_account(account_id.clone()).await?;
        let account = account.details().await;
        Ok(Response::Account(AccountDetailsDto::from(&*account)))
    }

    async fn get_accounts(&self) -> Result<Response> {
        let accounts = self.wallet.get_accounts().await?;

        Ok(Response::Accounts(
            futures::future::join_all(
                accounts
                    .into_iter()
                    .map(|account| async move { AccountDetailsDto::from(&*account.details().await) }),
            )
            .await,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{convert_async_panics, Response};

    #[tokio::test]
    async fn panic_to_response() {
        match convert_async_panics(|| async { panic!("rekt") }).await.unwrap() {
            Response::Panic(msg) => {
                assert!(msg.contains("rekt"));
            }
            response_type => panic!("Unexpected response type: {response_type:?}"),
        };
    }
}
