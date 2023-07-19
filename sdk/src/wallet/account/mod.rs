// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// The module with the AccountBuilder.
pub(crate) mod builder;
/// Constants used for the account and account operations.
pub(crate) mod constants;
/// The account operations like address generation, syncing and creating transactions.
pub(crate) mod operations;
/// Types used in an account and returned from methods.
pub mod types;
/// Methods to update the account state.
pub(crate) mod update;

use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
    sync::Arc,
};

use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, RwLock};

#[cfg(feature = "participation")]
pub use self::operations::participation::{AccountParticipationOverview, ParticipationEventWithNodes};
use self::types::{
    address::{AccountAddress, AddressWithUnspentOutputs},
    Balance, OutputData, Transaction, TransactionDto,
};
pub use self::{
    operations::{
        output_claiming::OutputsToClaim,
        syncing::{
            options::{AccountSyncOptions, AliasSyncOptions, NftSyncOptions},
            SyncOptions,
        },
        transaction::{
            high_level::{
                create_alias::CreateAliasParams,
                minting::{
                    create_native_token::{
                        CreateNativeTokenParams, CreateNativeTokenTransactionDto,
                        PreparedCreateNativeTokenTransactionDto,
                    },
                    mint_nfts::MintNftParams,
                },
            },
            prepare_output::{Assets, Features, OutputParams, ReturnStrategy, StorageDeposit, Unlocks},
            RemainderValueStrategy, TransactionOptions, TransactionOptionsDto,
        },
    },
    types::OutputDataDto,
};
use super::core::WalletInner;
use crate::{
    client::{
        secret::{SecretManage, SecretManager},
        Client,
    },
    types::{
        api::core::response::OutputWithMetadataResponse,
        block::{
            output::{dto::FoundryOutputDto, AliasId, FoundryId, FoundryOutput, NftId, Output, OutputId, TokenId},
            payload::{
                transaction::{TransactionEssence, TransactionId},
                TransactionPayload,
            },
            protocol::ProtocolParameters,
        },
    },
    wallet::{account::types::InclusionState, Result},
};

/// Options to filter outputs
#[derive(Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FilterOptions {
    /// Filter all outputs where the booked milestone index is below the specified timestamp
    pub lower_bound_booked_timestamp: Option<u32>,
    /// Filter all outputs where the booked milestone index is above the specified timestamp
    pub upper_bound_booked_timestamp: Option<u32>,
    /// Filter all outputs for the provided types (Basic = 3, Alias = 4, Foundry = 5, NFT = 6).
    pub output_types: Option<Vec<u8>>,
    /// Return all alias outputs matching these IDs.
    pub alias_ids: Option<HashSet<AliasId>>,
    /// Return all foundry outputs matching these IDs.
    pub foundry_ids: Option<HashSet<FoundryId>>,
    /// Return all nft outputs matching these IDs.
    pub nft_ids: Option<HashSet<NftId>>,
}

/// Details of an account.
#[derive(Clone, Debug, Eq, PartialEq, Getters, Setters)]
#[getset(get = "pub")]
pub struct AccountDetails {
    /// The account index
    index: u32,
    /// The coin type
    coin_type: u32,
    /// The account alias.
    alias: String,
    /// Public addresses
    pub(crate) public_addresses: Vec<AccountAddress>,
    /// Internal addresses
    pub(crate) internal_addresses: Vec<AccountAddress>,
    /// Addresses with unspent outputs
    // used to improve performance for syncing and get balance because it's in most cases only a subset of all
    // addresses
    addresses_with_unspent_outputs: Vec<AddressWithUnspentOutputs>,
    /// Outputs
    // stored separated from the account for performance?
    outputs: HashMap<OutputId, OutputData>,
    /// Unspent outputs that are currently used as input for transactions
    // outputs used in transactions should be locked here so they don't get used again, which would result in a
    // conflicting transaction
    pub(crate) locked_outputs: HashSet<OutputId>,
    /// Unspent outputs
    // have unspent outputs in a separated hashmap so we don't need to iterate over all outputs we have
    unspent_outputs: HashMap<OutputId, OutputData>,
    /// Sent transactions
    // stored separated from the account for performance and only the transaction id here? where to add the network id?
    // transactions: HashSet<TransactionId>,
    transactions: HashMap<TransactionId, Transaction>,
    /// Pending transactions
    // Maybe pending transactions even additionally separated?
    pending_transactions: HashSet<TransactionId>,
    /// Transaction payloads for received outputs with inputs when not pruned before syncing, can be used to determine
    /// the sender address(es)
    incoming_transactions: HashMap<TransactionId, Transaction>,
    /// Some incoming transactions can be pruned by the node before we requested them, then this node can never return
    /// it. To avoid useless requests, these transaction ids are stored here and cleared when new client options are
    /// set, because another node might still have them.
    inaccessible_incoming_transactions: HashSet<TransactionId>,
    /// Foundries for native tokens in outputs
    native_token_foundries: HashMap<FoundryId, FoundryOutput>,
}

/// A thread guard over an account, so we can lock the account during operations.
#[derive(Debug)]
pub struct Account<S: SecretManage = SecretManager> {
    inner: Arc<AccountInner>,
    pub(crate) wallet: Arc<WalletInner<S>>,
}

impl<S: SecretManage> Clone for Account<S> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            wallet: self.wallet.clone(),
        }
    }
}

impl<S: SecretManage> Account<S> {
    pub fn get_secret_manager(&self) -> &Arc<RwLock<S>> {
        self.wallet.get_secret_manager()
    }
}

#[derive(Debug)]
pub struct AccountInner {
    details: RwLock<AccountDetails>,
    // mutex to prevent multiple sync calls at the same or almost the same time, the u128 is a timestamp
    // if the last synced time was < `MIN_SYNC_INTERVAL` second ago, we don't sync, but only calculate the balance
    // again, because sending transactions can change that
    pub(crate) last_synced: Mutex<u128>,
    pub(crate) default_sync_options: Mutex<SyncOptions>,
}

// impl Deref so we can use `account.details()` instead of `account.details.read()`
impl<S: SecretManage> Deref for Account<S> {
    type Target = AccountInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<S: 'static + SecretManage> Account<S>
where
    crate::wallet::Error: From<S::Error>,
{
    /// Create a new Account with an AccountDetails
    pub(crate) async fn new(details: AccountDetails, wallet: Arc<WalletInner<S>>) -> Result<Self> {
        #[cfg(feature = "storage")]
        let default_sync_options = wallet
            .storage_manager
            .read()
            .await
            .get_default_sync_options(*details.index())
            .await?
            .unwrap_or_default();
        #[cfg(not(feature = "storage"))]
        let default_sync_options = Default::default();

        Ok(Self {
            wallet,
            inner: Arc::new(AccountInner {
                details: RwLock::new(details),
                last_synced: Default::default(),
                default_sync_options: Mutex::new(default_sync_options),
            }),
        })
    }

    // Get the Client
    pub fn client(&self) -> &Client {
        &self.wallet.client
    }

    /// Get the [`Output`] that minted a native token by the token ID. First try to get it
    /// from the account, if it isn't in the account try to get it from the node
    pub async fn get_foundry_output(&self, native_token_id: TokenId) -> Result<Output> {
        let foundry_id = FoundryId::from(native_token_id);

        for output_data in self.details().await.outputs().values() {
            if let Output::Foundry(foundry_output) = &output_data.output {
                if foundry_output.id() == foundry_id {
                    return Ok(output_data.output.clone());
                }
            }
        }

        // Foundry was not found in the account, try to get it from the node
        let foundry_output_id = self.client().foundry_output_id(foundry_id).await?;
        let output_response = self.client().get_output(&foundry_output_id).await?;

        Ok(output_response.output().to_owned())
    }

    /// Save the account to the database, accepts the updated_account as option so we don't need to drop it before
    /// saving
    #[cfg(feature = "storage")]
    pub(crate) async fn save(&self, updated_account: Option<&AccountDetails>) -> Result<()> {
        log::debug!("[save] saving account to database");
        match updated_account {
            Some(account) => {
                let mut storage_manager = self.wallet.storage_manager.write().await;
                storage_manager.save_account(account).await?;
                drop(storage_manager);
            }
            None => {
                let account_details = self.details().await;
                let mut storage_manager = self.wallet.storage_manager.write().await;
                storage_manager.save_account(&account_details).await?;
                drop(storage_manager);
                drop(account_details);
            }
        }
        Ok(())
    }

    #[cfg(feature = "events")]
    pub(crate) async fn emit(&self, account_index: u32, wallet_event: super::events::types::WalletEvent) {
        self.wallet.emit(account_index, wallet_event).await
    }
}

impl AccountInner {
    pub async fn details(&self) -> tokio::sync::RwLockReadGuard<'_, AccountDetails> {
        self.details.read().await
    }

    pub async fn details_mut(&self) -> tokio::sync::RwLockWriteGuard<'_, AccountDetails> {
        self.details.write().await
    }

    pub async fn alias(&self) -> String {
        self.details().await.alias.clone()
    }

    /// Get the [`OutputData`] of an output stored in the account
    pub async fn get_output(&self, output_id: &OutputId) -> Option<OutputData> {
        self.details().await.outputs().get(output_id).cloned()
    }

    /// Get the [`Transaction`] of a transaction stored in the account
    pub async fn get_transaction(&self, transaction_id: &TransactionId) -> Option<Transaction> {
        self.details().await.transactions().get(transaction_id).cloned()
    }

    /// Get the transaction with inputs of an incoming transaction stored in the account
    /// List might not be complete, if the node pruned the data already
    pub async fn get_incoming_transaction(&self, transaction_id: &TransactionId) -> Option<Transaction> {
        self.details()
            .await
            .incoming_transactions()
            .get(transaction_id)
            .cloned()
    }

    /// Returns all addresses of the account
    pub async fn addresses(&self) -> Result<Vec<AccountAddress>> {
        let account_details = self.details().await;
        let mut all_addresses = account_details.public_addresses().clone();
        all_addresses.extend(account_details.internal_addresses().clone());
        Ok(all_addresses.to_vec())
    }

    /// Returns all public addresses of the account
    pub(crate) async fn public_addresses(&self) -> Vec<AccountAddress> {
        self.details().await.public_addresses().to_vec()
    }

    /// Returns only addresses of the account with balance
    pub async fn addresses_with_unspent_outputs(&self) -> Result<Vec<AddressWithUnspentOutputs>> {
        Ok(self.details().await.addresses_with_unspent_outputs().to_vec())
    }

    fn filter_outputs<'a>(
        &self,
        outputs: impl Iterator<Item = &'a OutputData>,
        filter: impl Into<Option<FilterOptions>>,
    ) -> Result<Vec<OutputData>> {
        let filter = filter.into();

        if let Some(filter) = filter {
            let mut filtered_outputs = Vec::new();

            for output in outputs {
                match &output.output {
                    Output::Alias(alias) => {
                        if let Some(alias_ids) = &filter.alias_ids {
                            let alias_id = alias.alias_id_non_null(&output.output_id);
                            if alias_ids.contains(&alias_id) {
                                filtered_outputs.push(output.clone());
                                continue;
                            }
                        }
                    }
                    Output::Foundry(foundry) => {
                        if let Some(foundry_ids) = &filter.foundry_ids {
                            let foundry_id = foundry.id();
                            if foundry_ids.contains(&foundry_id) {
                                filtered_outputs.push(output.clone());
                                continue;
                            }
                        }
                    }
                    Output::Nft(nft) => {
                        if let Some(nft_ids) = &filter.nft_ids {
                            let nft_id = nft.nft_id_non_null(&output.output_id);
                            if nft_ids.contains(&nft_id) {
                                filtered_outputs.push(output.clone());
                                continue;
                            }
                        }
                    }
                    _ => {}
                }

                if let Some(lower_bound_booked_timestamp) = filter.lower_bound_booked_timestamp {
                    if output.metadata.milestone_timestamp_booked() < lower_bound_booked_timestamp {
                        continue;
                    }
                }
                if let Some(upper_bound_booked_timestamp) = filter.upper_bound_booked_timestamp {
                    if output.metadata.milestone_timestamp_booked() > upper_bound_booked_timestamp {
                        continue;
                    }
                }

                if let Some(output_types) = &filter.output_types {
                    if !output_types.contains(&output.output.kind()) {
                        continue;
                    }
                }

                filtered_outputs.push(output.clone());
            }

            Ok(filtered_outputs)
        } else {
            Ok(outputs.cloned().collect())
        }
    }

    /// Returns outputs of the account
    pub async fn outputs(&self, filter: impl Into<Option<FilterOptions>> + Send) -> Result<Vec<OutputData>> {
        self.filter_outputs(self.details().await.outputs.values(), filter)
    }

    /// Returns unspent outputs of the account
    pub async fn unspent_outputs(&self, filter: impl Into<Option<FilterOptions>> + Send) -> Result<Vec<OutputData>> {
        self.filter_outputs(self.details().await.unspent_outputs.values(), filter)
    }

    /// Gets the unspent alias output matching the given ID.
    pub async fn unspent_alias_output(&self, alias_id: &AliasId) -> Result<Option<OutputData>> {
        self.unspent_outputs(FilterOptions {
            alias_ids: Some([*alias_id].into()),
            ..Default::default()
        })
        .await
        .map(|res| res.get(0).cloned())
    }

    /// Gets the unspent foundry output matching the given ID.
    pub async fn unspent_foundry_output(&self, foundry_id: &FoundryId) -> Result<Option<OutputData>> {
        self.unspent_outputs(FilterOptions {
            foundry_ids: Some([*foundry_id].into()),
            ..Default::default()
        })
        .await
        .map(|res| res.get(0).cloned())
    }

    /// Gets the unspent nft output matching the given ID.
    pub async fn unspent_nft_output(&self, nft_id: &NftId) -> Result<Option<OutputData>> {
        self.unspent_outputs(FilterOptions {
            nft_ids: Some([*nft_id].into()),
            ..Default::default()
        })
        .await
        .map(|res| res.get(0).cloned())
    }

    /// Returns all incoming transactions of the account
    pub async fn incoming_transactions(&self) -> Vec<Transaction> {
        self.details().await.incoming_transactions.values().cloned().collect()
    }

    /// Returns all transactions of the account
    pub async fn transactions(&self) -> Vec<Transaction> {
        self.details().await.transactions.values().cloned().collect()
    }

    /// Returns all pending transactions of the account
    pub async fn pending_transactions(&self) -> Vec<Transaction> {
        let mut transactions = Vec::new();
        let account_details = self.details().await;

        for transaction_id in &account_details.pending_transactions {
            if let Some(transaction) = account_details.transactions.get(transaction_id) {
                transactions.push(transaction.clone());
            }
        }

        transactions
    }
}

pub(crate) fn build_transaction_from_payload_and_inputs(
    tx_id: TransactionId,
    tx_payload: TransactionPayload,
    inputs: Vec<OutputWithMetadataResponse>,
) -> crate::wallet::Result<Transaction> {
    let TransactionEssence::Regular(tx_essence) = &tx_payload.essence();
    Ok(Transaction {
        payload: tx_payload.clone(),
        block_id: inputs.first().map(|i| *i.metadata.block_id()),
        inclusion_state: InclusionState::Confirmed,
        timestamp: inputs
            .first()
            .and_then(|i| i.metadata.milestone_timestamp_spent().map(|t| t as u128 * 1000))
            .unwrap_or_else(|| crate::utils::unix_timestamp_now().as_millis()),
        transaction_id: tx_id,
        network_id: tx_essence.network_id(),
        incoming: true,
        note: None,
        inputs,
    })
}

/// Dto for an Account.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountDetailsDto {
    /// The account index
    pub index: u32,
    /// The coin type
    pub coin_type: u32,
    /// The account alias.
    pub alias: String,
    /// Public addresses
    pub public_addresses: Vec<AccountAddress>,
    /// Internal addresses
    pub internal_addresses: Vec<AccountAddress>,
    /// Addresses with unspent outputs
    pub addresses_with_unspent_outputs: Vec<AddressWithUnspentOutputs>,
    /// Outputs
    pub outputs: HashMap<OutputId, OutputDataDto>,
    /// Unspent outputs that are currently used as input for transactions
    pub locked_outputs: HashSet<OutputId>,
    /// Unspent outputs
    pub unspent_outputs: HashMap<OutputId, OutputDataDto>,
    /// Sent transactions
    pub transactions: HashMap<TransactionId, TransactionDto>,
    /// Pending transactions
    pub pending_transactions: HashSet<TransactionId>,
    /// Incoming transactions
    pub incoming_transactions: HashMap<TransactionId, TransactionDto>,
    /// Foundries for native tokens in outputs
    #[serde(default)]
    pub native_token_foundries: HashMap<FoundryId, FoundryOutputDto>,
}

impl From<&AccountDetails> for AccountDetailsDto {
    fn from(value: &AccountDetails) -> Self {
        Self {
            index: *value.index(),
            coin_type: *value.coin_type(),
            alias: value.alias().clone(),
            public_addresses: value.public_addresses().clone(),
            internal_addresses: value.internal_addresses().clone(),
            addresses_with_unspent_outputs: value.addresses_with_unspent_outputs().clone(),
            outputs: value
                .outputs()
                .iter()
                .map(|(id, output)| (*id, OutputDataDto::from(output)))
                .collect(),
            locked_outputs: value.locked_outputs().clone(),
            unspent_outputs: value
                .unspent_outputs()
                .iter()
                .map(|(id, output)| (*id, OutputDataDto::from(output)))
                .collect(),
            transactions: value
                .transactions()
                .iter()
                .map(|(id, transaction)| (*id, TransactionDto::from(transaction)))
                .collect(),
            pending_transactions: value.pending_transactions().clone(),
            incoming_transactions: value
                .incoming_transactions()
                .iter()
                .map(|(id, transaction)| (*id, TransactionDto::from(transaction)))
                .collect(),
            native_token_foundries: value
                .native_token_foundries()
                .iter()
                .map(|(id, foundry)| (*id, FoundryOutputDto::from(foundry)))
                .collect(),
        }
    }
}

impl AccountDetails {
    pub fn try_from_dto(
        dto: AccountDetailsDto,
        protocol_parameters: &ProtocolParameters,
    ) -> crate::wallet::Result<Self> {
        Ok(Self {
            index: dto.index,
            coin_type: dto.coin_type,
            alias: dto.alias,
            public_addresses: dto.public_addresses,
            internal_addresses: dto.internal_addresses,
            addresses_with_unspent_outputs: dto.addresses_with_unspent_outputs,
            outputs: dto
                .outputs
                .into_iter()
                .map(|(id, o)| Ok((id, OutputData::try_from_dto(o, protocol_parameters.token_supply())?)))
                .collect::<crate::wallet::Result<_>>()?,
            locked_outputs: dto.locked_outputs,
            unspent_outputs: dto
                .unspent_outputs
                .into_iter()
                .map(|(id, o)| Ok((id, OutputData::try_from_dto(o, protocol_parameters.token_supply())?)))
                .collect::<crate::wallet::Result<_>>()?,
            transactions: dto
                .transactions
                .into_iter()
                .map(|(id, o)| Ok((id, Transaction::try_from_dto(o, protocol_parameters)?)))
                .collect::<crate::wallet::Result<_>>()?,
            pending_transactions: dto.pending_transactions,
            incoming_transactions: dto
                .incoming_transactions
                .into_iter()
                .map(|(id, o)| Ok((id, Transaction::try_from_dto(o, protocol_parameters)?)))
                .collect::<crate::wallet::Result<_>>()?,
            inaccessible_incoming_transactions: Default::default(),
            native_token_foundries: dto
                .native_token_foundries
                .into_iter()
                .map(|(id, o)| Ok((id, FoundryOutput::try_from_dto(o, protocol_parameters.token_supply())?)))
                .collect::<crate::wallet::Result<_>>()?,
        })
    }

    pub fn try_from_dto_unverified(dto: AccountDetailsDto) -> crate::wallet::Result<Self> {
        Ok(Self {
            index: dto.index,
            coin_type: dto.coin_type,
            alias: dto.alias,
            public_addresses: dto.public_addresses,
            internal_addresses: dto.internal_addresses,
            addresses_with_unspent_outputs: dto.addresses_with_unspent_outputs,
            outputs: dto
                .outputs
                .into_iter()
                .map(|(id, o)| Ok((id, OutputData::try_from_dto_unverified(o)?)))
                .collect::<crate::wallet::Result<_>>()?,
            locked_outputs: dto.locked_outputs,
            unspent_outputs: dto
                .unspent_outputs
                .into_iter()
                .map(|(id, o)| Ok((id, OutputData::try_from_dto_unverified(o)?)))
                .collect::<crate::wallet::Result<_>>()?,
            transactions: dto
                .transactions
                .into_iter()
                .map(|(id, o)| Ok((id, Transaction::try_from_dto_unverified(o)?)))
                .collect::<crate::wallet::Result<_>>()?,
            pending_transactions: dto.pending_transactions,
            incoming_transactions: dto
                .incoming_transactions
                .into_iter()
                .map(|(id, o)| Ok((id, Transaction::try_from_dto_unverified(o)?)))
                .collect::<crate::wallet::Result<_>>()?,
            inaccessible_incoming_transactions: Default::default(),
            native_token_foundries: dto
                .native_token_foundries
                .into_iter()
                .map(|(id, o)| Ok((id, FoundryOutput::try_from_dto_unverified(o)?)))
                .collect::<crate::wallet::Result<_>>()?,
        })
    }
}

#[test]
fn serialize() {
    use core::str::FromStr;

    use crate::types::block::{
        address::{Address, Ed25519Address},
        input::{Input, UtxoInput},
        output::{unlock_condition::AddressUnlockCondition, BasicOutput, InputsCommitment, Output},
        payload::{
            transaction::{RegularTransactionEssence, TransactionEssence, TransactionId},
            TransactionPayload,
        },
        protocol::ProtocolParameters,
        signature::{Ed25519Signature, Signature},
        unlock::{ReferenceUnlock, SignatureUnlock, Unlock, Unlocks},
    };

    const TRANSACTION_ID: &str = "0x24a1f46bdb6b2bf38f1c59f73cdd4ae5b418804bb231d76d06fbf246498d5883";
    const ED25519_ADDRESS: &str = "0xe594f9a895c0e0a6760dd12cffc2c3d1e1cbf7269b328091f96ce3d0dd550b75";
    const ED25519_PUBLIC_KEY: &str = "0x1da5ddd11ba3f961acab68fafee3177d039875eaa94ac5fdbff8b53f0c50bfb9";
    const ED25519_SIGNATURE: &str = "0xc6a40edf9a089f42c18f4ebccb35fe4b578d93b879e99b87f63573324a710d3456b03fb6d1fcc027e6401cbd9581f790ee3ed7a3f68e9c225fcb9f1cd7b7110d";

    let protocol_parameters = ProtocolParameters::new(
        2,
        String::from("testnet"),
        "rms",
        1500,
        15,
        crate::types::block::output::RentStructure::new(500, 10, 1),
        1_813_620_509_061_365,
    )
    .unwrap();

    let transaction_id = TransactionId::new(prefix_hex::decode(TRANSACTION_ID).unwrap());
    let input1 = Input::Utxo(UtxoInput::new(transaction_id, 0).unwrap());
    let input2 = Input::Utxo(UtxoInput::new(transaction_id, 1).unwrap());
    let bytes: [u8; 32] = prefix_hex::decode(ED25519_ADDRESS).unwrap();
    let address = Address::from(Ed25519Address::new(bytes));
    let amount = 1_000_000;
    let output = Output::Basic(
        BasicOutput::build_with_amount(amount)
            .add_unlock_condition(AddressUnlockCondition::new(address))
            .finish(protocol_parameters.token_supply())
            .unwrap(),
    );
    let essence = TransactionEssence::Regular(
        RegularTransactionEssence::builder(protocol_parameters.network_id(), InputsCommitment::from([0u8; 32]))
            .with_inputs([input1, input2])
            .add_output(output)
            .finish(&protocol_parameters)
            .unwrap(),
    );

    let pub_key_bytes = prefix_hex::decode(ED25519_PUBLIC_KEY).unwrap();
    let sig_bytes = prefix_hex::decode(ED25519_SIGNATURE).unwrap();
    let signature = Ed25519Signature::try_from_bytes(pub_key_bytes, sig_bytes).unwrap();
    let sig_unlock = Unlock::Signature(SignatureUnlock::from(Signature::from(signature)));
    let ref_unlock = Unlock::Reference(ReferenceUnlock::new(0).unwrap());
    let unlocks = Unlocks::new([sig_unlock, ref_unlock]).unwrap();

    let tx_payload = TransactionPayload::new(essence, unlocks).unwrap();

    let incoming_transaction = Transaction {
        transaction_id: TransactionId::from_str("0x131fc4cb8f315ae36ae3bf6a4e4b3486d5f17581288f1217410da3e0700d195a")
            .unwrap(),
        payload: tx_payload,
        block_id: None,
        network_id: 0,
        timestamp: 0,
        inclusion_state: InclusionState::Pending,
        incoming: false,
        note: None,
        inputs: Vec::new(),
    };

    let mut incoming_transactions = HashMap::new();
    incoming_transactions.insert(
        TransactionId::from_str("0x131fc4cb8f315ae36ae3bf6a4e4b3486d5f17581288f1217410da3e0700d195a").unwrap(),
        incoming_transaction,
    );

    let account = AccountDetails {
        index: 0,
        coin_type: 4218,
        alias: "0".to_string(),
        public_addresses: Vec::new(),
        internal_addresses: Vec::new(),
        addresses_with_unspent_outputs: Vec::new(),
        outputs: HashMap::new(),
        locked_outputs: HashSet::new(),
        unspent_outputs: HashMap::new(),
        transactions: HashMap::new(),
        pending_transactions: HashSet::new(),
        incoming_transactions,
        inaccessible_incoming_transactions: HashSet::new(),
        native_token_foundries: HashMap::new(),
    };

    let deser_account = AccountDetails::try_from_dto_unverified(
        serde_json::from_str::<AccountDetailsDto>(&serde_json::to_string(&AccountDetailsDto::from(&account)).unwrap())
            .unwrap(),
    )
    .unwrap();

    assert_eq!(account, deser_account);
}

#[cfg(test)]
impl AccountDetails {
    /// Returns a mock of this type with the following values:
    /// index: 0, coin_type: 4218, alias: "Alice", public_addresses: contains a single public account address
    /// (rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy), all other fields are set to their Rust
    /// defaults.
    #[cfg(feature = "storage")]
    pub(crate) fn mock() -> Self {
        use core::str::FromStr;
        Self {
            index: 0,
            coin_type: 4218,
            alias: "Alice".to_string(),
            public_addresses: vec![AccountAddress {
                address: crate::types::block::address::Bech32Address::from_str(
                    "rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy",
                )
                .unwrap(),
                key_index: 0,
                internal: false,
                used: false,
            }],
            internal_addresses: Vec::new(),
            addresses_with_unspent_outputs: Vec::new(),
            outputs: HashMap::new(),
            locked_outputs: HashSet::new(),
            unspent_outputs: HashMap::new(),
            transactions: HashMap::new(),
            pending_transactions: HashSet::new(),
            incoming_transactions: HashMap::new(),
            inaccessible_incoming_transactions: HashSet::new(),
            native_token_foundries: HashMap::new(),
        }
    }
}
