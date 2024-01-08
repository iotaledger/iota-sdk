// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) mod builder;
pub(crate) mod operations;

use std::{
    collections::{HashMap, HashSet},
    sync::{atomic::AtomicUsize, Arc},
};

use crypto::keys::{
    bip39::{Mnemonic, MnemonicRef},
    bip44::Bip44,
};
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, RwLock};

pub use self::builder::WalletBuilder;
use super::types::{TransactionWithMetadata, TransactionWithMetadataDto};
#[cfg(feature = "events")]
use crate::wallet::events::{
    types::{WalletEvent, WalletEventType},
    EventEmitter,
};
#[cfg(feature = "storage")]
use crate::wallet::storage::{StorageManager, StorageOptions};
use crate::{
    client::{
        secret::{SecretManage, SecretManager},
        verify_mnemonic, Client,
    },
    types::{
        block::{
            address::{Address, Bech32Address, Hrp, ImplicitAccountCreationAddress},
            output::{AccountId, AnchorId, DelegationId, FoundryId, FoundryOutput, NftId, Output, OutputId, TokenId},
            payload::signed_transaction::TransactionId,
            protocol::ProtocolParameters,
        },
        TryFromDto,
    },
    wallet::{operations::syncing::SyncOptions, types::OutputData, Error, FilterOptions, Result},
};

/// The stateful wallet used to interact with an IOTA network.
#[derive(Debug)]
pub struct Wallet<S: SecretManage = SecretManager> {
    pub(crate) inner: Arc<WalletInner<S>>,
    pub(crate) data: Arc<RwLock<WalletData>>,
}

impl<S: SecretManage> Clone for Wallet<S> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            data: self.data.clone(),
        }
    }
}

impl<S: SecretManage> core::ops::Deref for Wallet<S> {
    type Target = WalletInner<S>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
{
    /// Initialises the wallet builder.
    pub fn builder() -> WalletBuilder<S> {
        WalletBuilder::<S>::new()
    }
}

/// Wallet inner.
#[derive(Debug)]
pub struct WalletInner<S: SecretManage = SecretManager> {
    // mutex to prevent multiple sync calls at the same or almost the same time, the u128 is a timestamp
    // if the last synced time was < `MIN_SYNC_INTERVAL` second ago, we don't sync, but only calculate the balance
    // again, because sending transactions can change that
    pub(crate) last_synced: Mutex<u128>,
    pub(crate) default_sync_options: Mutex<SyncOptions>,
    // 0 = not running, 1 = running, 2 = stopping
    pub(crate) background_syncing_status: AtomicUsize,
    pub(crate) client: Client,
    // TODO: make this optional?
    pub(crate) secret_manager: Arc<RwLock<S>>,
    #[cfg(feature = "events")]
    pub(crate) event_emitter: tokio::sync::RwLock<EventEmitter>,
    #[cfg(feature = "storage")]
    pub(crate) storage_options: StorageOptions,
    #[cfg(feature = "storage")]
    pub(crate) storage_manager: tokio::sync::RwLock<StorageManager>,
}

/// Wallet data.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WalletData {
    /// The wallet BIP44 path.
    pub(crate) bip_path: Option<Bip44>,
    /// The wallet address.
    pub(crate) address: Bech32Address,
    /// The wallet alias.
    pub(crate) alias: Option<String>,
    /// Outputs
    // stored separated from the wallet for performance?
    pub(crate) outputs: HashMap<OutputId, OutputData>,
    /// Unspent outputs that are currently used as input for transactions
    // outputs used in transactions should be locked here so they don't get used again, which would result in a
    // conflicting transaction
    pub(crate) locked_outputs: HashSet<OutputId>,
    /// Unspent outputs
    // have unspent outputs in a separated hashmap so we don't need to iterate over all outputs we have
    pub(crate) unspent_outputs: HashMap<OutputId, OutputData>,
    /// Sent transactions
    // stored separated from the wallet for performance and only the transaction id here? where to add the network id?
    // transactions: HashSet<TransactionId>,
    pub(crate) transactions: HashMap<TransactionId, TransactionWithMetadata>,
    /// Pending transactions
    // Maybe pending transactions even additionally separated?
    pub(crate) pending_transactions: HashSet<TransactionId>,
    /// Transaction payloads for received outputs with inputs when not pruned before syncing, can be used to determine
    /// the sender address(es)
    pub(crate) incoming_transactions: HashMap<TransactionId, TransactionWithMetadata>,
    /// Some incoming transactions can be pruned by the node before we requested them, then this node can never return
    /// it. To avoid useless requests, these transaction ids are stored here and cleared when new client options are
    /// set, because another node might still have them.
    pub(crate) inaccessible_incoming_transactions: HashSet<TransactionId>,
    /// Foundries for native tokens in outputs
    pub(crate) native_token_foundries: HashMap<FoundryId, FoundryOutput>,
}

impl WalletData {
    pub(crate) fn new(bip_path: Option<Bip44>, address: Bech32Address, alias: Option<String>) -> Self {
        Self {
            bip_path,
            address,
            alias,
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

    fn filter_outputs<'a>(
        outputs: impl Iterator<Item = &'a OutputData>,
        filter: FilterOptions,
    ) -> impl Iterator<Item = &'a OutputData> {
        outputs.filter(move |output| {
            match &output.output {
                Output::Account(account) => {
                    if let Some(account_ids) = &filter.account_ids {
                        let account_id = account.account_id_non_null(&output.output_id);
                        if account_ids.contains(&account_id) {
                            return true;
                        }
                    }
                }
                Output::Anchor(anchor) => {
                    if let Some(anchor_ids) = &filter.anchor_ids {
                        let anchor_id = anchor.anchor_id_non_null(&output.output_id);
                        if anchor_ids.contains(&anchor_id) {
                            return true;
                        }
                    }
                }
                Output::Foundry(foundry) => {
                    if let Some(foundry_ids) = &filter.foundry_ids {
                        let foundry_id = foundry.id();
                        if foundry_ids.contains(&foundry_id) {
                            return true;
                        }
                    }
                }
                Output::Nft(nft) => {
                    if let Some(nft_ids) = &filter.nft_ids {
                        let nft_id = nft.nft_id_non_null(&output.output_id);
                        if nft_ids.contains(&nft_id) {
                            return true;
                        }
                    }
                }
                Output::Delegation(delegation) => {
                    if let Some(delegation_ids) = &filter.delegation_ids {
                        let delegation_id = delegation.delegation_id_non_null(&output.output_id);
                        if delegation_ids.contains(&delegation_id) {
                            return true;
                        }
                    }
                }
                _ => {}
            }

            // TODO filter based on slot index
            // if let Some(lower_bound_booked_timestamp) = filter.lower_bound_booked_timestamp {
            //     if output.metadata.milestone_timestamp_booked() < lower_bound_booked_timestamp {
            //         continue;
            //     }
            // }
            // if let Some(upper_bound_booked_timestamp) = filter.upper_bound_booked_timestamp {
            //     if output.metadata.milestone_timestamp_booked() > upper_bound_booked_timestamp {
            //         continue;
            //     }
            // }

            if let Some(output_types) = &filter.output_types {
                if !output_types.contains(&output.output.kind()) {
                    return false;
                }
            }

            // Include the output if we're not filtering by IDs.
            if filter.account_ids.is_none()
                && filter.anchor_ids.is_none()
                && filter.foundry_ids.is_none()
                && filter.nft_ids.is_none()
                && filter.delegation_ids.is_none()
            {
                return true;
            }
            false
        })
    }

    /// Returns outputs map of the wallet.
    pub fn outputs(&self) -> &HashMap<OutputId, OutputData> {
        &self.outputs
    }

    /// Returns unspent outputs map of the wallet.
    pub fn unspent_outputs(&self) -> &HashMap<OutputId, OutputData> {
        &self.unspent_outputs
    }

    /// Returns outputs of the wallet.
    pub fn filtered_outputs(&self, filter: FilterOptions) -> impl Iterator<Item = &OutputData> {
        Self::filter_outputs(self.outputs.values(), filter)
    }

    /// Returns unspent outputs of the wallet.
    pub fn filtered_unspent_outputs(&self, filter: FilterOptions) -> impl Iterator<Item = &OutputData> {
        Self::filter_outputs(self.unspent_outputs.values(), filter)
    }

    /// Gets the unspent account output matching the given ID.
    pub fn unspent_account_output(&self, account_id: &AccountId) -> Option<&OutputData> {
        self.filtered_unspent_outputs(FilterOptions {
            account_ids: Some([*account_id].into()),
            ..Default::default()
        })
        .next()
    }

    /// Gets the unspent anchor output matching the given ID.
    pub fn unspent_anchor_output(&self, anchor_id: &AnchorId) -> Option<&OutputData> {
        self.filtered_unspent_outputs(FilterOptions {
            anchor_ids: Some([*anchor_id].into()),
            ..Default::default()
        })
        .next()
    }

    /// Gets the unspent foundry output matching the given ID.
    pub fn unspent_foundry_output(&self, foundry_id: &FoundryId) -> Option<&OutputData> {
        self.filtered_unspent_outputs(FilterOptions {
            foundry_ids: Some([*foundry_id].into()),
            ..Default::default()
        })
        .next()
    }

    /// Gets the unspent nft output matching the given ID.
    pub fn unspent_nft_output(&self, nft_id: &NftId) -> Option<&OutputData> {
        self.filtered_unspent_outputs(FilterOptions {
            nft_ids: Some([*nft_id].into()),
            ..Default::default()
        })
        .next()
    }

    /// Gets the unspent delegation output matching the given ID.
    pub fn unspent_delegation_output(&self, delegation_id: &DelegationId) -> Option<&OutputData> {
        self.filtered_unspent_outputs(FilterOptions {
            delegation_ids: Some([*delegation_id].into()),
            ..Default::default()
        })
        .next()
    }

    /// Returns implicit accounts of the wallet.
    pub fn implicit_accounts(&self) -> impl Iterator<Item = &OutputData> {
        self.unspent_outputs
            .values()
            .filter(|output_data| output_data.output.is_implicit_account())
    }

    /// Returns accounts of the wallet.
    pub fn accounts(&self) -> impl Iterator<Item = &OutputData> {
        self.unspent_outputs
            .values()
            .filter(|output_data| output_data.output.is_account())
    }

    /// Get the [`OutputData`] of an output stored in the wallet.
    pub fn get_output(&self, output_id: &OutputId) -> Option<&OutputData> {
        self.outputs.get(output_id)
    }

    /// Get the [`TransactionWithMetadata`] of a transaction stored in the wallet.
    pub fn get_transaction(&self, transaction_id: &TransactionId) -> Option<&TransactionWithMetadata> {
        self.transactions.get(transaction_id)
    }

    /// Get the transaction with inputs of an incoming transaction stored in the wallet.
    /// List might not be complete, if the node pruned the data already
    pub fn get_incoming_transaction(&self, transaction_id: &TransactionId) -> Option<&TransactionWithMetadata> {
        self.incoming_transactions.get(transaction_id)
    }

    /// Returns all incoming transactions of the wallet
    pub fn incoming_transactions(&self) -> &HashMap<TransactionId, TransactionWithMetadata> {
        &self.incoming_transactions
    }

    /// Returns all transactions of the wallet
    pub fn transactions(&self) -> &HashMap<TransactionId, TransactionWithMetadata> {
        &self.transactions
    }

    /// Returns all pending transactions of the wallet
    pub fn pending_transactions(&self) -> impl Iterator<Item = &TransactionWithMetadata> {
        self.pending_transactions
            .iter()
            .filter_map(|transaction_id| self.get_transaction(transaction_id))
    }
}

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    /// Create a new wallet.
    pub(crate) async fn new(inner: Arc<WalletInner<S>>, data: WalletData) -> Result<Self> {
        #[cfg(feature = "storage")]
        let default_sync_options = inner
            .storage_manager
            .read()
            .await
            .get_default_sync_options()
            .await?
            .unwrap_or_default();
        #[cfg(not(feature = "storage"))]
        let default_sync_options = Default::default();

        // TODO: maybe move this into a `reset` fn or smth to avoid this kinda-weird block.
        {
            let mut last_synced = inner.last_synced.lock().await;
            *last_synced = Default::default();
            let mut sync_options = inner.default_sync_options.lock().await;
            *sync_options = default_sync_options;
        }

        Ok(Self {
            inner,
            data: Arc::new(RwLock::new(data)),
        })
    }

    /// Get the [`Output`] that minted a native token by the token ID. First try to get it
    /// from the wallet, if it isn't in the wallet try to get it from the node
    pub async fn get_foundry_output(&self, native_token_id: TokenId) -> Result<Output> {
        let foundry_id = FoundryId::from(native_token_id);

        for output_data in self.data.read().await.outputs.values() {
            if let Output::Foundry(foundry_output) = &output_data.output {
                if foundry_output.id() == foundry_id {
                    return Ok(output_data.output.clone());
                }
            }
        }

        // Foundry was not found in the wallet, try to get it from the node
        let foundry_output_id = self.client().foundry_output_id(foundry_id).await?;
        let output_response = self.client().get_output(&foundry_output_id).await?;

        Ok(output_response.output)
    }

    /// Save the wallet to the database, accepts the updated wallet data as option so we don't need to drop it before
    /// saving
    #[cfg(feature = "storage")]
    pub(crate) async fn save(&self, updated_wallet: Option<&WalletData>) -> Result<()> {
        log::debug!("[save] wallet data");
        match updated_wallet {
            Some(wallet) => {
                let mut storage_manager = self.storage_manager.write().await;
                storage_manager.save_wallet_data(wallet).await?;
                drop(storage_manager);
            }
            None => {
                let wallet_data = self.data.read().await;
                let mut storage_manager = self.storage_manager.write().await;
                storage_manager.save_wallet_data(&wallet_data).await?;
                drop(storage_manager);
                drop(wallet_data);
            }
        }
        Ok(())
    }

    #[cfg(feature = "events")]
    pub(crate) async fn emit(&self, wallet_event: super::events::types::WalletEvent) {
        self.inner.emit(wallet_event).await
    }

    pub async fn data(&self) -> tokio::sync::RwLockReadGuard<'_, WalletData> {
        self.data.read().await
    }

    pub(crate) async fn data_mut(&self) -> tokio::sync::RwLockWriteGuard<'_, WalletData> {
        self.data.write().await
    }

    /// Get the alias of the wallet if one was set.
    pub async fn alias(&self) -> Option<String> {
        self.data().await.alias.clone()
    }

    /// Get the wallet address.
    pub async fn address(&self) -> Bech32Address {
        self.data().await.address.clone()
    }

    /// Returns the implicit account creation address of the wallet if it is Ed25519 based.
    pub async fn implicit_account_creation_address(&self) -> Result<Bech32Address> {
        let bech32_address = &self.data().await.address;

        if let Address::Ed25519(address) = bech32_address.inner() {
            Ok(Bech32Address::new(
                *bech32_address.hrp(),
                ImplicitAccountCreationAddress::from(*address),
            ))
        } else {
            Err(Error::NonEd25519Address)
        }
    }

    /// Get the wallet's configured Bech32 HRP.
    pub async fn bech32_hrp(&self) -> Hrp {
        self.data().await.address.hrp
    }

    /// Get the wallet's configured bip path.
    pub async fn bip_path(&self) -> Option<Bip44> {
        self.data().await.bip_path
    }
}

impl<S: SecretManage> WalletInner<S> {
    /// Get the [SecretManager]
    pub fn get_secret_manager(&self) -> &Arc<RwLock<S>> {
        &self.secret_manager
    }

    /// Listen to wallet events, empty vec will listen to all events
    #[cfg(feature = "events")]
    #[cfg_attr(docsrs, doc(cfg(feature = "events")))]
    pub async fn listen<F, I: IntoIterator<Item = WalletEventType> + Send>(&self, events: I, handler: F)
    where
        I::IntoIter: Send,
        F: Fn(&WalletEvent) + 'static + Send + Sync,
    {
        let mut emitter = self.event_emitter.write().await;
        emitter.on(events, handler);
    }

    /// Remove wallet event listeners, empty vec will remove all listeners
    #[cfg(feature = "events")]
    #[cfg_attr(docsrs, doc(cfg(feature = "events")))]
    pub async fn clear_listeners<I: IntoIterator<Item = WalletEventType> + Send>(&self, events: I)
    where
        I::IntoIter: Send,
    {
        let mut emitter = self.event_emitter.write().await;
        emitter.clear(events);
    }

    /// Generates a new random mnemonic.
    pub fn generate_mnemonic(&self) -> crate::wallet::Result<Mnemonic> {
        Ok(Client::generate_mnemonic()?)
    }

    /// Verify that a &str is a valid mnemonic.
    pub fn verify_mnemonic(&self, mnemonic: &MnemonicRef) -> crate::wallet::Result<()> {
        verify_mnemonic(mnemonic)?;
        Ok(())
    }

    #[cfg(feature = "events")]
    pub(crate) async fn emit(&self, event: crate::wallet::events::types::WalletEvent) {
        self.event_emitter.read().await.emit(event);
    }

    /// Helper function to test events.
    #[cfg(feature = "events")]
    #[cfg_attr(docsrs, doc(cfg(feature = "events")))]
    pub async fn emit_test_event(&self, event: crate::wallet::events::types::WalletEvent) {
        self.emit(event).await
    }
}

impl<S: SecretManage> Drop for Wallet<S> {
    fn drop(&mut self) {
        log::debug!("drop Wallet");
    }
}

impl<S: SecretManage> Drop for WalletInner<S> {
    fn drop(&mut self) {
        log::debug!("drop WalletInner");
    }
}

/// Dto for the wallet data.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletDataDto {
    pub bip_path: Option<Bip44>,
    pub address: Bech32Address,
    pub alias: Option<String>,
    pub outputs: HashMap<OutputId, OutputData>,
    pub locked_outputs: HashSet<OutputId>,
    pub unspent_outputs: HashMap<OutputId, OutputData>,
    pub transactions: HashMap<TransactionId, TransactionWithMetadataDto>,
    pub pending_transactions: HashSet<TransactionId>,
    pub incoming_transactions: HashMap<TransactionId, TransactionWithMetadataDto>,
    #[serde(default)]
    pub native_token_foundries: HashMap<FoundryId, FoundryOutput>,
}

impl TryFromDto<WalletDataDto> for WalletData {
    type Error = crate::wallet::Error;

    fn try_from_dto_with_params_inner(
        dto: WalletDataDto,
        params: Option<&ProtocolParameters>,
    ) -> core::result::Result<Self, Self::Error> {
        Ok(Self {
            bip_path: dto.bip_path,
            address: dto.address,
            alias: dto.alias,
            outputs: dto.outputs,
            locked_outputs: dto.locked_outputs,
            unspent_outputs: dto.unspent_outputs,
            transactions: dto
                .transactions
                .into_iter()
                .map(|(id, o)| Ok((id, TransactionWithMetadata::try_from_dto_with_params_inner(o, params)?)))
                .collect::<crate::wallet::Result<_>>()?,
            pending_transactions: dto.pending_transactions,
            incoming_transactions: dto
                .incoming_transactions
                .into_iter()
                .map(|(id, o)| Ok((id, TransactionWithMetadata::try_from_dto_with_params_inner(o, params)?)))
                .collect::<crate::wallet::Result<_>>()?,
            inaccessible_incoming_transactions: Default::default(),
            native_token_foundries: dto.native_token_foundries,
        })
    }
}

impl From<&WalletData> for WalletDataDto {
    fn from(value: &WalletData) -> Self {
        Self {
            bip_path: value.bip_path,
            address: value.address.clone(),
            alias: value.alias.clone(),
            outputs: value.outputs.clone(),
            locked_outputs: value.locked_outputs.clone(),
            unspent_outputs: value.unspent_outputs.clone(),
            transactions: value
                .transactions
                .iter()
                .map(|(id, transaction)| (*id, TransactionWithMetadataDto::from(transaction)))
                .collect(),
            pending_transactions: value.pending_transactions.clone(),
            incoming_transactions: value
                .incoming_transactions
                .iter()
                .map(|(id, transaction)| (*id, TransactionWithMetadataDto::from(transaction)))
                .collect(),
            native_token_foundries: value.native_token_foundries.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use core::str::FromStr;

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{
        types::block::{
            address::{Address, Ed25519Address},
            input::{Input, UtxoInput},
            output::{AddressUnlockCondition, BasicOutput, Output, StorageScoreParameters},
            payload::signed_transaction::{SignedTransactionPayload, Transaction, TransactionId},
            protocol::ProtocolParameters,
            rand::mana::rand_mana_allotment,
            signature::{Ed25519Signature, Signature},
            unlock::{ReferenceUnlock, SignatureUnlock, Unlock, Unlocks},
        },
        wallet::types::InclusionState,
    };

    const TRANSACTION_ID: &str = "0x24a1f46bdb6b2bf38f1c59f73cdd4ae5b418804bb231d76d06fbf246498d588300000000";
    const ED25519_ADDRESS: &str = "0xe594f9a895c0e0a6760dd12cffc2c3d1e1cbf7269b328091f96ce3d0dd550b75";
    const ED25519_PUBLIC_KEY: &str = "0x1da5ddd11ba3f961acab68fafee3177d039875eaa94ac5fdbff8b53f0c50bfb9";
    const ED25519_SIGNATURE: &str = "0xc6a40edf9a089f42c18f4ebccb35fe4b578d93b879e99b87f63573324a710d3456b03fb6d1fcc027e6401cbd9581f790ee3ed7a3f68e9c225fcb9f1cd7b7110d";

    #[test]
    fn serialize() {
        let protocol_parameters = ProtocolParameters::new(
            2,
            "testnet",
            "rms",
            StorageScoreParameters::new(500, 1, 10, 1, 1, 1),
            1_813_620_509_061_365,
            1582328545,
            10,
            20,
        )
        .unwrap();

        let transaction_id = TransactionId::new(prefix_hex::decode(TRANSACTION_ID).unwrap());
        let input1 = Input::Utxo(UtxoInput::new(transaction_id, 0));
        let input2 = Input::Utxo(UtxoInput::new(transaction_id, 1));
        let bytes: [u8; 32] = prefix_hex::decode(ED25519_ADDRESS).unwrap();
        let address = Address::from(Ed25519Address::new(bytes));
        let amount = 1_000_000;
        let output = Output::Basic(
            BasicOutput::build_with_amount(amount)
                .add_unlock_condition(AddressUnlockCondition::new(address))
                .finish()
                .unwrap(),
        );
        let transaction = Transaction::builder(protocol_parameters.network_id())
            .with_inputs([input1, input2])
            .add_output(output)
            .add_mana_allotment(rand_mana_allotment(&protocol_parameters))
            .finish_with_params(&protocol_parameters)
            .unwrap();

        let pub_key_bytes = prefix_hex::decode(ED25519_PUBLIC_KEY).unwrap();
        let sig_bytes = prefix_hex::decode(ED25519_SIGNATURE).unwrap();
        let signature = Ed25519Signature::try_from_bytes(pub_key_bytes, sig_bytes).unwrap();
        let sig_unlock = Unlock::from(SignatureUnlock::from(Signature::from(signature)));
        let ref_unlock = Unlock::from(ReferenceUnlock::new(0).unwrap());
        let unlocks = Unlocks::new([sig_unlock, ref_unlock]).unwrap();

        let tx_payload = SignedTransactionPayload::new(transaction, unlocks).unwrap();

        let incoming_transaction = TransactionWithMetadata {
            transaction_id: TransactionId::from_str(
                "0x131fc4cb8f315ae36ae3bf6a4e4b3486d5f17581288f1217410da3e0700d195a00000000",
            )
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
            TransactionId::from_str("0x131fc4cb8f315ae36ae3bf6a4e4b3486d5f17581288f1217410da3e0700d195a00000000")
                .unwrap(),
            incoming_transaction,
        );

        let wallet_data = WalletData {
            bip_path: Some(Bip44::new(4218)),
            address: crate::types::block::address::Bech32Address::from_str(
                "rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy",
            )
            .unwrap(),
            alias: Some("Alice".to_string()),
            outputs: HashMap::new(),
            locked_outputs: HashSet::new(),
            unspent_outputs: HashMap::new(),
            transactions: HashMap::new(),
            pending_transactions: HashSet::new(),
            incoming_transactions,
            inaccessible_incoming_transactions: HashSet::new(),
            native_token_foundries: HashMap::new(),
        };

        let deser_wallet_data = WalletData::try_from_dto(
            serde_json::from_str::<WalletDataDto>(&serde_json::to_string(&WalletDataDto::from(&wallet_data)).unwrap())
                .unwrap(),
        )
        .unwrap();

        assert_eq!(wallet_data, deser_wallet_data);
    }

    impl WalletData {
        /// Returns a mock of this type with the following values:
        /// index: 0, coin_type: 4218, alias: "Alice", address:
        /// rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy, all other fields are set to their Rust
        /// defaults.
        #[cfg(feature = "storage")]
        pub(crate) fn mock() -> Self {
            Self {
                bip_path: Some(Bip44::new(4218)),
                address: crate::types::block::address::Bech32Address::from_str(
                    "rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy",
                )
                .unwrap(),
                alias: Some("Alice".to_string()),
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
}
