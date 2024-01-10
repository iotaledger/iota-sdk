// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Secret manager module enabling address generation and transaction signing.

/// Module for ledger nano based secret management.
#[cfg(feature = "ledger_nano")]
#[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
pub mod ledger_nano;
mod manager;
/// Module for mnemonic based secret management.
pub mod mnemonic;
/// Module for single private key based secret management.
#[cfg(feature = "private_key_secret_manager")]
#[cfg_attr(docsrs, doc(cfg(feature = "private_key_secret_manager")))]
pub mod private_key;
/// Module for stronghold based secret management.
#[cfg(feature = "stronghold")]
#[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
pub mod stronghold;
/// Signing related types
pub mod types;

use std::{collections::HashMap, fmt::Debug, ops::Range, sync::Arc};

use async_trait::async_trait;
use crypto::{keys::bip44::Bip44, signatures::ed25519};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[cfg(feature = "ledger_nano")]
use self::ledger_nano::LedgerSecretManager;
pub use self::manager::{SecretManager, SecretManagerDto};
use self::mnemonic::MnemonicSecretManager;
#[cfg(feature = "private_key_secret_manager")]
use self::private_key::PrivateKeySecretManager;
#[cfg(feature = "ledger_nano")]
pub use self::types::LedgerNanoStatus;
#[cfg(feature = "stronghold")]
use super::stronghold::StrongholdAdapter;
use crate::{
    client::{
        api::{
            input_selection::Error as InputSelectionError, transaction::validate_signed_transaction_payload_length,
            verify_semantic, PreparedTransactionData,
        },
        Error,
    },
    types::block::{
        address::{Address, AnchorAddress, Ed25519Address},
        core::UnsignedBlock,
        output::Output,
        payload::SignedTransactionPayload,
        protocol::ProtocolParameters,
        signature::{Ed25519Signature, Signature},
        unlock::{AccountUnlock, NftUnlock, ReferenceUnlock, SignatureUnlock, Unlock, Unlocks},
        Block, Error as BlockError,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyOptions {
    pub coin_type: u32,
    pub account_index: u32,
    pub internal: bool,
    pub address_index: u32,
}

impl PublicKeyOptions {
    /// Create a new public key generation options
    pub fn new(coin_type: u32) -> Self {
        Self {
            coin_type,
            account_index: 0,
            internal: false,
            address_index: 0,
        }
    }

    /// Set the account index
    pub fn with_account_index(mut self, account_index: u32) -> Self {
        self.account_index = account_index;
        self
    }

    /// Set internal flag.
    pub fn with_internal(mut self, internal: bool) -> Self {
        self.internal = internal;
        self
    }

    /// Set the address index.
    pub fn with_address_index(mut self, address_index: u32) -> Self {
        self.address_index = address_index;
        self
    }
}

impl From<Bip44> for PublicKeyOptions {
    fn from(value: Bip44) -> Self {
        Self::new(value.coin_type)
            .with_account_index(value.account)
            .with_internal(value.change != 0)
            .with_address_index(value.address_index)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MultiKeyOptions {
    pub coin_type: u32,
    pub account_index: u32,
    pub internal: bool,
    pub address_range: Range<u32>,
}

impl MultiKeyOptions {
    /// Create a new multikey generation options
    pub fn new(coin_type: u32) -> Self {
        Self {
            coin_type,
            account_index: 0,
            internal: false,
            address_range: 0..1,
        }
    }

    /// Set the account index
    pub fn with_account_index(mut self, account_index: u32) -> Self {
        self.account_index = account_index;
        self
    }

    /// Set internal flag.
    pub fn with_internal(mut self, internal: bool) -> Self {
        self.internal = internal;
        self
    }

    /// Set the address index range
    pub fn with_address_range(mut self, range: Range<u32>) -> Self {
        self.address_range = range;
        self
    }
}

#[async_trait]
pub trait Generate<K>: Send + Sync {
    type Options: 'static + Send + Sync + Serialize + Clone + Debug + DeserializeOwned + PartialEq;

    async fn generate(&self, options: &Self::Options) -> crate::client::Result<K>;
}

#[async_trait]
impl<T: Generate<K>, K> Generate<K> for Arc<T> {
    type Options = T::Options;

    async fn generate(&self, options: &Self::Options) -> crate::client::Result<K> {
        self.as_ref().generate(options).await
    }
}

#[async_trait]
pub trait Sign<S>: Send + Sync {
    type Options: 'static + Send + Sync + Serialize + Clone + Debug + DeserializeOwned + PartialEq;

    async fn sign(&self, msg: &[u8], options: &Self::Options) -> crate::client::Result<S>;
}

#[async_trait]
impl<T: Sign<S>, S> Sign<S> for Arc<T> {
    type Options = T::Options;

    async fn sign(&self, msg: &[u8], options: &Self::Options) -> crate::client::Result<S> {
        self.as_ref().sign(msg, options).await
    }
}

#[async_trait]
pub trait SignBlock: Sign<Ed25519Signature> {
    async fn sign_block(&self, unsigned_block: UnsignedBlock, options: &Self::Options) -> crate::client::Result<Block> {
        let msg = unsigned_block.signing_input();
        Ok(unsigned_block.finish(self.sign(&msg, options).await?)?)
    }
}
impl<T: Sign<Ed25519Signature>> SignBlock for T {}

#[async_trait]
pub trait SignTransaction: Sign<Ed25519Signature> {
    /// Signs `transaction_signing_hash` using the given `options`, returning a [`SignatureUnlock`].
    async fn signature_unlock(
        &self,
        transaction_signing_hash: &[u8; 32],
        options: &Self::Options,
    ) -> crate::client::Result<SignatureUnlock> {
        Ok(SignatureUnlock::new(Signature::from(
            self.sign(transaction_signing_hash, options).await?,
        )))
    }

    async fn transaction_unlocks(
        &self,
        prepared_transaction_data: &PreparedTransactionData,
        protocol_parameters: &ProtocolParameters,
        options: &Self::Options,
    ) -> crate::client::Result<Unlocks> {
        let transaction_signing_hash = prepared_transaction_data.transaction.signing_hash();
        let mut blocks = Vec::new();
        let mut block_indexes = HashMap::<Address, usize>::new();
        let slot_index = prepared_transaction_data
            .transaction
            .context_inputs()
            .iter()
            .find_map(|c| c.as_commitment_opt().map(|c| c.slot_index()));

        // Assuming inputs_data is ordered by address type
        for (current_block_index, input) in prepared_transaction_data.inputs_data.iter().enumerate() {
            // Get the address that is required to unlock the input
            let required_address = input
                .output
                .required_address(slot_index, protocol_parameters.committable_age_range())?
                .ok_or(crate::client::Error::ExpirationDeadzone)?;

            let required_address = if let Address::Restricted(restricted) = &required_address {
                restricted.address()
            } else {
                &required_address
            };

            // Check if we already added an [Unlock] for this address
            match block_indexes.get(required_address) {
                // If we already have an [Unlock] for this address, add a [Unlock] based on the address type
                Some(block_index) => match required_address {
                    Address::Ed25519(_) | Address::ImplicitAccountCreation(_) => {
                        blocks.push(Unlock::Reference(ReferenceUnlock::new(*block_index as u16)?));
                    }
                    Address::Account(_) => blocks.push(Unlock::Account(AccountUnlock::new(*block_index as u16)?)),
                    Address::Nft(_) => blocks.push(Unlock::Nft(NftUnlock::new(*block_index as u16)?)),
                    _ => Err(BlockError::UnsupportedAddressKind(required_address.kind()))?,
                },
                None => {
                    // We can only sign ed25519 addresses and block_indexes needs to contain the account or nft
                    // address already at this point, because the reference index needs to be lower
                    // than the current block index
                    match &required_address {
                        Address::Ed25519(_) | Address::ImplicitAccountCreation(_) => {}
                        _ => Err(InputSelectionError::MissingInputWithEd25519Address)?,
                    }

                    let block = self.signature_unlock(&transaction_signing_hash, options).await?;
                    blocks.push(block.into());

                    // Add the ed25519 address to the block_indexes, so it gets referenced if further inputs have
                    // the same address in their unlock condition
                    block_indexes.insert(required_address.clone(), current_block_index);
                }
            }

            // When we have an account or Nft output, we will add their account or nft address to block_indexes,
            // because they can be used to unlock outputs via [Unlock::Account] or [Unlock::Nft],
            // that have the corresponding account or nft address in their unlock condition
            match &input.output {
                Output::Account(account_output) => block_indexes.insert(
                    Address::Account(account_output.account_address(input.output_id())),
                    current_block_index,
                ),
                Output::Nft(nft_output) => block_indexes.insert(
                    Address::Nft(nft_output.nft_address(input.output_id())),
                    current_block_index,
                ),
                _ => None,
            };
        }

        Ok(Unlocks::new(blocks)?)
    }

    async fn sign_transaction(
        &self,
        prepared_transaction_data: PreparedTransactionData,
        protocol_parameters: &ProtocolParameters,
        options: &Self::Options,
    ) -> crate::client::Result<SignedTransactionPayload> {
        log::debug!("[sign_transaction] {:?}", prepared_transaction_data);

        let unlocks = self
            .transaction_unlocks(&prepared_transaction_data, protocol_parameters, options)
            .await?;

        let PreparedTransactionData {
            transaction,
            inputs_data,
            ..
        } = prepared_transaction_data;
        let tx_payload = SignedTransactionPayload::new(transaction, unlocks)?;

        validate_signed_transaction_payload_length(&tx_payload)?;

        let conflict = verify_semantic(&inputs_data, &tx_payload, protocol_parameters.clone())?;

        if let Some(conflict) = conflict {
            log::debug!("[sign_transaction] conflict: {conflict:?} for {:#?}", tx_payload);
            return Err(Error::TransactionSemantic(conflict));
        }

        Ok(tx_payload)
    }
}
impl<T: SignTransaction> SignTransaction for Arc<T> {}

/// Unifying trait for secret managers. This type must be able to, at minimum, generate
/// an address and sign transactions and blocks.
#[async_trait]
pub trait SecretManage:
    Generate<ed25519::PublicKey, Options = Self::GenerationOptions>
    + SignTransaction<Options = Self::SigningOptions>
    + SignBlock<Options = Self::SigningOptions>
{
    type GenerationOptions: 'static + Send + Sync + Serialize + Clone + Debug + DeserializeOwned + PartialEq;
    type SigningOptions: 'static + Send + Sync + Serialize + Clone + Debug + DeserializeOwned + PartialEq;
}

#[async_trait]
impl<T: Generate<ed25519::PublicKey> + SignTransaction + SignBlock> SecretManage for T {
    type GenerationOptions = <Self as Generate<ed25519::PublicKey>>::Options;
    type SigningOptions = <Self as Sign<Ed25519Signature>>::Options;
}

#[async_trait]
pub trait SecretManageExt {
    async fn generate<K>(&self, options: &Self::Options) -> crate::client::Result<K>
    where
        Self: Generate<K>,
    {
        Generate::<K>::generate(self, options).await
    }

    async fn sign<S>(&self, msg: &[u8], options: &Self::Options) -> crate::client::Result<S>
    where
        Self: Sign<S>,
    {
        Sign::<S>::sign(self, msg, options).await
    }
}
impl<T> SecretManageExt for T {}

pub trait SecretManagerConfig: SecretManage {
    type Config: Serialize + DeserializeOwned + Debug + Send + Sync;

    fn to_config(&self) -> Option<Self::Config>;

    fn from_config(config: &Self::Config) -> crate::client::Result<Self>
    where
        Self: Sized;
}

impl<T: SecretManagerConfig> SecretManagerConfig for Arc<T> {
    type Config = T::Config;

    fn to_config(&self) -> Option<Self::Config> {
        self.as_ref().to_config()
    }

    fn from_config(config: &Self::Config) -> crate::client::Result<Self> {
        Ok(Arc::new(T::from_config(config)?))
    }
}

pub trait DowncastSecretManager {
    fn is<T: 'static>(&self) -> bool;

    fn downcast_ref<T: 'static>(&self) -> Option<&T>;

    fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T>;

    #[cfg(feature = "stronghold")]
    fn as_stronghold(&self) -> crate::client::Result<&StrongholdAdapter> {
        self.downcast_ref::<StrongholdAdapter>()
            .or_else(|| {
                self.downcast_ref::<SecretManager>().and_then(|s| {
                    if let SecretManager::Stronghold(a) = s {
                        Some(a)
                    } else {
                        None
                    }
                })
            })
            .ok_or(crate::client::Error::SecretManagerMismatch)
    }

    #[cfg(feature = "stronghold")]
    fn as_stronghold_mut(&mut self) -> crate::client::Result<&mut StrongholdAdapter> {
        // Have to do this because of https://docs.rs/polonius-the-crab/latest/polonius_the_crab/#rationale-limitations-of-the-nll-borrow-checker
        if self.is::<StrongholdAdapter>() {
            Ok(self.downcast_mut::<StrongholdAdapter>().unwrap())
        } else {
            self.downcast_mut::<SecretManager>()
                .and_then(|s| {
                    if let SecretManager::Stronghold(a) = s {
                        Some(a)
                    } else {
                        None
                    }
                })
                .ok_or(crate::client::Error::SecretManagerMismatch)
        }
    }

    fn as_mnemonic(&self) -> crate::client::Result<&MnemonicSecretManager> {
        self.downcast_ref::<MnemonicSecretManager>()
            .or_else(|| {
                self.downcast_ref::<SecretManager>().and_then(|s| {
                    if let SecretManager::Mnemonic(a) = s {
                        Some(a)
                    } else {
                        None
                    }
                })
            })
            .ok_or(crate::client::Error::SecretManagerMismatch)
    }

    fn as_mnemonic_mut(&mut self) -> crate::client::Result<&mut MnemonicSecretManager> {
        // Have to do this because of https://docs.rs/polonius-the-crab/latest/polonius_the_crab/#rationale-limitations-of-the-nll-borrow-checker
        if self.is::<MnemonicSecretManager>() {
            Ok(self.downcast_mut::<MnemonicSecretManager>().unwrap())
        } else {
            self.downcast_mut::<SecretManager>()
                .and_then(|s| {
                    if let SecretManager::Mnemonic(a) = s {
                        Some(a)
                    } else {
                        None
                    }
                })
                .ok_or(crate::client::Error::SecretManagerMismatch)
        }
    }

    #[cfg(feature = "ledger_nano")]
    fn as_ledger_nano(&self) -> crate::client::Result<&LedgerSecretManager> {
        self.downcast_ref::<LedgerSecretManager>()
            .or_else(|| {
                self.downcast_ref::<SecretManager>().and_then(|s| {
                    if let SecretManager::LedgerNano(a) = s {
                        Some(a)
                    } else {
                        None
                    }
                })
            })
            .ok_or(crate::client::Error::SecretManagerMismatch)
    }

    #[cfg(feature = "ledger_nano")]
    fn as_ledger_nano_mut(&mut self) -> crate::client::Result<&mut LedgerSecretManager> {
        // Have to do this because of https://docs.rs/polonius-the-crab/latest/polonius_the_crab/#rationale-limitations-of-the-nll-borrow-checker
        if self.is::<LedgerSecretManager>() {
            Ok(self.downcast_mut::<LedgerSecretManager>().unwrap())
        } else {
            self.downcast_mut::<SecretManager>()
                .and_then(|s| {
                    if let SecretManager::LedgerNano(a) = s {
                        Some(a)
                    } else {
                        None
                    }
                })
                .ok_or(crate::client::Error::SecretManagerMismatch)
        }
    }

    #[cfg(feature = "private_key_secret_manager")]
    fn as_private_key(&self) -> crate::client::Result<&PrivateKeySecretManager> {
        self.downcast_ref::<PrivateKeySecretManager>()
            .or_else(|| {
                self.downcast_ref::<SecretManager>().and_then(|s| {
                    if let SecretManager::PrivateKey(a) = s {
                        Some(a.as_ref())
                    } else {
                        None
                    }
                })
            })
            .ok_or(crate::client::Error::SecretManagerMismatch)
    }

    #[cfg(feature = "private_key_secret_manager")]
    fn as_private_key_mut(&mut self) -> crate::client::Result<&mut PrivateKeySecretManager> {
        // Have to do this because of https://docs.rs/polonius-the-crab/latest/polonius_the_crab/#rationale-limitations-of-the-nll-borrow-checker
        if self.is::<PrivateKeySecretManager>() {
            Ok(self.downcast_mut::<PrivateKeySecretManager>().unwrap())
        } else {
            self.downcast_mut::<SecretManager>()
                .and_then(|s| {
                    if let SecretManager::PrivateKey(a) = s {
                        Some(a.as_mut())
                    } else {
                        None
                    }
                })
                .ok_or(crate::client::Error::SecretManagerMismatch)
        }
    }
}

impl<S: 'static + Send + Sync> DowncastSecretManager for S {
    fn is<T: 'static>(&self) -> bool {
        self.as_any().is::<T>()
    }

    fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }

    fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.as_any_mut().downcast_mut::<T>()
    }
}

pub trait AsAny: Send + Sync {
    fn as_any(&self) -> &(dyn std::any::Any + Send + Sync);
    fn as_any_mut(&mut self) -> &mut (dyn std::any::Any + Send + Sync);
}

impl<T: 'static + Send + Sync> AsAny for T {
    fn as_any(&self) -> &(dyn std::any::Any + Send + Sync) {
        self
    }
    fn as_any_mut(&mut self) -> &mut (dyn std::any::Any + Send + Sync) {
        self
    }
}

#[async_trait]
pub trait BlockSignExt {
    async fn sign_ed25519<S: SignBlock + ?Sized>(
        self,
        secret_manager: &S,
        options: &S::Options,
    ) -> crate::client::Result<Block>
    where
        Self: Sized;
}

#[async_trait]
impl BlockSignExt for UnsignedBlock {
    async fn sign_ed25519<S: SignBlock + ?Sized>(
        self,
        secret_manager: &S,
        options: &S::Options,
    ) -> crate::client::Result<Block>
    where
        Self: Sized,
    {
        Ok(secret_manager.sign_block(self, options).await?)
    }
}
