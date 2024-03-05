// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) mod builder;
pub(crate) mod operations;

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use crypto::keys::{
    bip39::{Mnemonic, MnemonicRef},
    bip44::Bip44,
};
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, RwLock};

pub use self::builder::WalletBuilder;
use self::operations::background_syncing::BackgroundSyncStatus;
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
    wallet::{operations::syncing::SyncOptions, types::OutputData, FilterOptions, WalletError},
};

/// The stateful wallet used to interact with an IOTA network.
#[derive(Debug)]
pub struct Wallet<S: SecretManage = SecretManager> {
    pub(crate) address: Arc<RwLock<Bech32Address>>,
    pub(crate) bip_path: Arc<RwLock<Option<Bip44>>>,
    pub(crate) alias: Arc<RwLock<Option<String>>>,
    pub(crate) inner: Arc<WalletInner<S>>,
    pub(crate) ledger: Arc<RwLock<WalletLedger>>,
}

impl<S: SecretManage> Clone for Wallet<S> {
    fn clone(&self) -> Self {
        Self {
            address: self.address.clone(),
            bip_path: self.bip_path.clone(),
            alias: self.alias.clone(),
            inner: self.inner.clone(),
            ledger: self.ledger.clone(),
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
    WalletError: From<S::Error>,
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
    pub(crate) background_syncing_status: (
        Arc<tokio::sync::watch::Sender<BackgroundSyncStatus>>,
        tokio::sync::watch::Receiver<BackgroundSyncStatus>,
    ),
    pub(crate) client: Client,
    // TODO: make this optional?
    pub(crate) secret_manager: Arc<RwLock<S>>,
    #[cfg(feature = "events")]
    pub(crate) event_emitter: tokio::sync::RwLock<EventEmitter>,
    #[cfg(feature = "storage")]
    pub(crate) storage_options: StorageOptions,
    #[cfg(feature = "storage")]
    pub(crate) storage_manager: StorageManager,
}

/// Wallet ledger.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WalletLedger {
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

impl WalletLedger {
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

    // Returns the first possible Account id, which can be an implicit account.
    pub fn first_account_id(&self) -> Option<AccountId> {
        self.accounts()
            .next()
            .map(|o| o.output.as_account().account_id_non_null(&o.output_id))
            .or_else(|| self.implicit_accounts().next().map(|o| AccountId::from(&o.output_id)))
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

impl<S: 'static + SecretManage> Wallet<S> {
    /// Get the [`Output`] that minted a native token by the token ID. First try to get it
    /// from the wallet, if it isn't in the wallet try to get it from the node
    pub async fn get_foundry_output(&self, native_token_id: TokenId) -> Result<Output, WalletError> {
        let foundry_id = FoundryId::from(native_token_id);

        for output_data in self.ledger.read().await.outputs.values() {
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

    #[cfg(feature = "events")]
    pub(crate) async fn emit(&self, wallet_event: super::events::types::WalletEvent) {
        self.inner.emit(wallet_event).await
    }

    /// Get the wallet address.
    pub async fn address(&self) -> Bech32Address {
        self.address.read().await.clone()
    }

    pub(crate) async fn address_mut(&self) -> tokio::sync::RwLockWriteGuard<'_, Bech32Address> {
        self.address.write().await
    }

    /// Get the wallet's Bech32 HRP.
    pub async fn bech32_hrp(&self) -> Hrp {
        self.address.read().await.hrp
    }

    /// Get the wallet's bip path.
    pub async fn bip_path(&self) -> Option<Bip44> {
        *self.bip_path.read().await
    }

    pub(crate) async fn bip_path_mut(&self) -> tokio::sync::RwLockWriteGuard<'_, Option<Bip44>> {
        self.bip_path.write().await
    }

    /// Get the alias of the wallet if one was set.
    pub async fn alias(&self) -> Option<String> {
        self.alias.read().await.clone()
    }

    pub(crate) async fn alias_mut(&self) -> tokio::sync::RwLockWriteGuard<'_, Option<String>> {
        self.alias.write().await
    }

    /// Get the wallet's ledger state.
    pub async fn ledger(&self) -> tokio::sync::RwLockReadGuard<'_, WalletLedger> {
        self.ledger.read().await
    }

    pub(crate) async fn ledger_mut(&self) -> tokio::sync::RwLockWriteGuard<'_, WalletLedger> {
        self.ledger.write().await
    }

    #[cfg(feature = "storage")]
    pub(crate) fn storage_manager(&self) -> &StorageManager {
        &self.storage_manager
    }

    /// Returns the implicit account creation address of the wallet if it is Ed25519 based.
    pub async fn implicit_account_creation_address(&self) -> Result<Bech32Address, WalletError> {
        let bech32_address = &self.address().await;

        if let Address::Ed25519(address) = bech32_address.inner() {
            Ok(Bech32Address::new(
                *bech32_address.hrp(),
                ImplicitAccountCreationAddress::from(*address),
            ))
        } else {
            Err(WalletError::NonEd25519Address)
        }
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
    pub fn generate_mnemonic(&self) -> Result<Mnemonic, WalletError> {
        Ok(Client::generate_mnemonic()?)
    }

    /// Verify that a &str is a valid mnemonic.
    pub fn verify_mnemonic(&self, mnemonic: &MnemonicRef) -> Result<(), WalletError> {
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

/// Dto for the wallet ledger.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletLedgerDto {
    pub outputs: HashMap<OutputId, OutputData>,
    pub locked_outputs: HashSet<OutputId>,
    pub unspent_outputs: HashMap<OutputId, OutputData>,
    pub transactions: HashMap<TransactionId, TransactionWithMetadataDto>,
    pub pending_transactions: HashSet<TransactionId>,
    pub incoming_transactions: HashMap<TransactionId, TransactionWithMetadataDto>,
    #[serde(default)]
    pub native_token_foundries: HashMap<FoundryId, FoundryOutput>,
}

impl TryFromDto<WalletLedgerDto> for WalletLedger {
    type Error = WalletError;

    fn try_from_dto_with_params_inner(
        dto: WalletLedgerDto,
        params: Option<&ProtocolParameters>,
    ) -> core::result::Result<Self, Self::Error> {
        Ok(Self {
            outputs: dto.outputs,
            locked_outputs: dto.locked_outputs,
            unspent_outputs: dto.unspent_outputs,
            transactions: dto
                .transactions
                .into_iter()
                .map(|(id, o)| Ok((id, TransactionWithMetadata::try_from_dto_with_params_inner(o, params)?)))
                .collect::<Result<_, WalletError>>()?,
            pending_transactions: dto.pending_transactions,
            incoming_transactions: dto
                .incoming_transactions
                .into_iter()
                .map(|(id, o)| Ok((id, TransactionWithMetadata::try_from_dto_with_params_inner(o, params)?)))
                .collect::<Result<_, WalletError>>()?,
            inaccessible_incoming_transactions: Default::default(),
            native_token_foundries: dto.native_token_foundries,
        })
    }
}

impl From<&WalletLedger> for WalletLedgerDto {
    fn from(value: &WalletLedger) -> Self {
        Self {
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

#[cfg(all(test, feature = "protocol_parameters_samples"))]
mod test {
    use core::str::FromStr;

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{
        types::block::{
            address::{Address, Ed25519Address},
            input::{Input, UtxoInput},
            output::{AddressUnlockCondition, BasicOutput, Output},
            payload::signed_transaction::{SignedTransactionPayload, Transaction, TransactionId},
            protocol::iota_mainnet_protocol_parameters,
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
        let protocol_parameters = iota_mainnet_protocol_parameters();

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
            .add_mana_allotment(rand_mana_allotment(protocol_parameters))
            .finish_with_params(protocol_parameters)
            .unwrap();

        let pub_key_bytes = prefix_hex::decode(ED25519_PUBLIC_KEY).unwrap();
        let sig_bytes = prefix_hex::decode(ED25519_SIGNATURE).unwrap();
        let signature = Ed25519Signature::from_bytes(pub_key_bytes, sig_bytes);
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

        let wallet_ledger = WalletLedger {
            outputs: HashMap::new(),
            locked_outputs: HashSet::new(),
            unspent_outputs: HashMap::new(),
            transactions: HashMap::new(),
            pending_transactions: HashSet::new(),
            incoming_transactions,
            inaccessible_incoming_transactions: HashSet::new(),
            native_token_foundries: HashMap::new(),
        };

        let deser_wallet_ledger = WalletLedger::try_from_dto(
            serde_json::from_str::<WalletLedgerDto>(
                &serde_json::to_string(&WalletLedgerDto::from(&wallet_ledger)).unwrap(),
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(wallet_ledger, deser_wallet_ledger);
    }

    impl WalletLedger {
        // TODO: use something non-empty
        #[cfg(feature = "storage")]
        pub(crate) fn test_instance() -> Self {
            Self {
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
