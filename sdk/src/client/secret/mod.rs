// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Secret manager module enabling address generation and transaction essence signing.

#[cfg(feature = "ledger_nano")]
#[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
pub mod ledger_nano;
/// Module for signing with a mnemonic or seed
pub mod mnemonic;
/// Module for the PlaceholderSecretManager
pub mod placeholder;
/// Module for signing with a Stronghold vault
#[cfg(feature = "stronghold")]
#[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
pub mod stronghold;
/// Signing related types
pub mod types;

#[cfg(feature = "stronghold")]
use std::time::Duration;
use std::{collections::HashMap, ops::Range, str::FromStr};

use async_trait::async_trait;
use crypto::{
    keys::slip10::Chain,
    signatures::secp256k1_ecdsa::{self, EvmAddress},
};
use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

#[cfg(feature = "ledger_nano")]
use self::ledger_nano::LedgerSecretManager;
#[cfg(feature = "stronghold")]
use self::stronghold::StrongholdSecretManager;
pub use self::types::{GenerateAddressOptions, LedgerNanoStatus};
use self::{mnemonic::MnemonicSecretManager, placeholder::PlaceholderSecretManager};
#[cfg(feature = "stronghold")]
use crate::client::secret::types::StrongholdDto;
use crate::{
    client::{
        api::{
            input_selection::{is_alias_transition, Error as InputSelectionError},
            transaction::validate_transaction_payload_length,
            verify_semantic, PreparedTransactionData,
        },
        Error,
    },
    types::block::{
        address::{Address, Ed25519Address},
        output::Output,
        payload::{transaction::TransactionEssence, Payload, TransactionPayload},
        semantic::ConflictReason,
        signature::{Ed25519Signature, Signature},
        unlock::{AliasUnlock, NftUnlock, ReferenceUnlock, SignatureUnlock, Unlock, Unlocks},
    },
    utils::unix_timestamp_now,
};

/// The secret manager interface.
#[async_trait]
pub trait SecretManage: Send + Sync {
    type Error;

    /// Generates addresses.
    ///
    /// For `coin_type`, see also <https://github.com/satoshilabs/slips/blob/master/slip-0044.md>.
    async fn generate_ed25519_addresses(
        &self,
        coin_type: u32,
        account_index: u32,
        address_indexes: Range<u32>,
        options: impl Into<Option<GenerateAddressOptions>> + Send,
    ) -> Result<Vec<Ed25519Address>, Self::Error>;

    async fn generate_evm_addresses(
        &self,
        coin_type: u32,
        account_index: u32,
        address_indexes: Range<u32>,
        options: impl Into<Option<GenerateAddressOptions>> + Send,
    ) -> Result<Vec<EvmAddress>, Self::Error>;

    /// Signs msg using the given [`Chain`] using Ed25519.
    async fn sign_ed25519(&self, msg: &[u8], chain: &Chain) -> Result<Ed25519Signature, Self::Error>;

    /// Signs msg using the given [`Chain`] using Secp256k1.
    async fn sign_evm(
        &self,
        msg: &[u8],
        chain: &Chain,
    ) -> Result<(secp256k1_ecdsa::PublicKey, secp256k1_ecdsa::Signature), Self::Error>;

    /// Signs `essence_hash` using the given `chain`, returning an [`Unlock`].
    async fn signature_unlock(&self, essence_hash: &[u8; 32], chain: &Chain) -> Result<Unlock, Self::Error> {
        Ok(Unlock::Signature(SignatureUnlock::new(Signature::Ed25519(
            self.sign_ed25519(essence_hash, chain).await?,
        ))))
    }
}

/// Defines a type that can sign a transaction essence.
#[async_trait]
pub trait SignTransactionEssence: SecretManage {
    /// Signs transaction essence.
    ///
    /// Secret managers usually don't implement this, as the default implementation has taken care of the placement of
    /// blocks (e.g. references between them). [`SecretManager::signature_unlock()`] will be invoked every time a
    /// necessary signing action needs to be performed.
    async fn sign_transaction_essence(
        &self,
        prepared_transaction_data: &PreparedTransactionData,
        time: Option<u32>,
    ) -> Result<Unlocks, <Self as SecretManage>::Error>;
}

/// Supported secret managers

// Boxes make this type clumsy to use.
pub enum SecretManager {
    /// Secret manager that uses [`iota_stronghold`] as the backing storage.
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    Stronghold(StrongholdSecretManager),

    /// Secret manager that uses a Ledger Nano hardware wallet or Speculos simulator.
    #[cfg(feature = "ledger_nano")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
    LedgerNano(LedgerSecretManager),

    /// Secret manager that uses a mnemonic in plain memory. It's not recommended for production use. Use
    /// LedgerNano or Stronghold instead.
    Mnemonic(MnemonicSecretManager),

    /// Secret manager that's just a placeholder, so it can be provided to an online wallet, but can't be used for
    /// signing.
    Placeholder(PlaceholderSecretManager),
}

impl std::fmt::Debug for SecretManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "stronghold")]
            Self::Stronghold(_) => f.debug_tuple("Stronghold").field(&"...").finish(),
            #[cfg(feature = "ledger_nano")]
            Self::LedgerNano(_) => f.debug_tuple("LedgerNano").field(&"...").finish(),
            Self::Mnemonic(_) => f.debug_tuple("Mnemonic").field(&"...").finish(),
            Self::Placeholder(_) => f.debug_struct("Placeholder").finish(),
        }
    }
}

impl FromStr for SecretManager {
    type Err = Error;

    fn from_str(s: &str) -> crate::client::Result<Self> {
        Self::try_from(serde_json::from_str::<SecretManagerDto>(s)?)
    }
}

/// DTO for secret manager types with required data.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SecretManagerDto {
    /// Stronghold
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    #[serde(alias = "stronghold")]
    Stronghold(StrongholdDto),
    /// Ledger Device, bool specifies if it's a simulator or not
    #[cfg(feature = "ledger_nano")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
    #[serde(alias = "ledgerNano")]
    LedgerNano(bool),
    /// Mnemonic
    #[serde(alias = "mnemonic")]
    Mnemonic(Zeroizing<String>),
    /// Hex seed
    #[serde(alias = "hexSeed")]
    HexSeed(Zeroizing<String>),
    /// Placeholder
    #[serde(alias = "placeholder")]
    Placeholder,
}

impl TryFrom<SecretManagerDto> for SecretManager {
    type Error = Error;

    fn try_from(value: SecretManagerDto) -> crate::client::Result<Self> {
        Ok(match value {
            #[cfg(feature = "stronghold")]
            SecretManagerDto::Stronghold(stronghold_dto) => {
                let mut builder = StrongholdSecretManager::builder();

                if let Some(password) = stronghold_dto.password {
                    builder = builder.password(password);
                }

                if let Some(timeout) = stronghold_dto.timeout {
                    builder = builder.timeout(Duration::from_secs(timeout));
                }

                Self::Stronghold(builder.build(&stronghold_dto.snapshot_path)?)
            }

            #[cfg(feature = "ledger_nano")]
            SecretManagerDto::LedgerNano(is_simulator) => Self::LedgerNano(LedgerSecretManager::new(is_simulator)),

            SecretManagerDto::Mnemonic(mnemonic) => Self::Mnemonic(MnemonicSecretManager::try_from_mnemonic(mnemonic)?),

            SecretManagerDto::HexSeed(hex_seed) => {
                // `SecretManagerDto` is `ZeroizeOnDrop` so it will take care of zeroizing the original.
                Self::Mnemonic(MnemonicSecretManager::try_from_hex_seed(hex_seed)?)
            }

            SecretManagerDto::Placeholder => Self::Placeholder(PlaceholderSecretManager),
        })
    }
}

impl From<&SecretManager> for SecretManagerDto {
    fn from(value: &SecretManager) -> Self {
        match value {
            #[cfg(feature = "stronghold")]
            SecretManager::Stronghold(stronghold_adapter) => Self::Stronghold(StrongholdDto {
                password: None,
                timeout: stronghold_adapter.get_timeout().map(|duration| duration.as_secs()),
                snapshot_path: stronghold_adapter
                    .snapshot_path
                    .clone()
                    .into_os_string()
                    .to_string_lossy()
                    .into(),
            }),

            #[cfg(feature = "ledger_nano")]
            SecretManager::LedgerNano(ledger_nano) => Self::LedgerNano(ledger_nano.is_simulator),

            // `MnemonicSecretManager(Seed)` doesn't have Debug or Display implemented and in the current use cases of
            // the client/wallet we also don't need to convert it in this direction with the mnemonic/seed, we only need
            // to know the type
            SecretManager::Mnemonic(_mnemonic) => Self::Mnemonic("...".to_string().into()),
            SecretManager::Placeholder(_) => Self::Placeholder,
        }
    }
}

#[async_trait]
impl SecretManage for SecretManager {
    type Error = Error;

    async fn generate_ed25519_addresses(
        &self,
        coin_type: u32,
        account_index: u32,
        address_indexes: Range<u32>,
        options: impl Into<Option<GenerateAddressOptions>> + Send,
    ) -> crate::client::Result<Vec<Ed25519Address>> {
        match self {
            #[cfg(feature = "stronghold")]
            Self::Stronghold(secret_manager) => Ok(secret_manager
                .generate_ed25519_addresses(coin_type, account_index, address_indexes, options)
                .await?),
            #[cfg(feature = "ledger_nano")]
            Self::LedgerNano(secret_manager) => Ok(secret_manager
                .generate_ed25519_addresses(coin_type, account_index, address_indexes, options)
                .await?),
            Self::Mnemonic(secret_manager) => {
                secret_manager
                    .generate_ed25519_addresses(coin_type, account_index, address_indexes, options)
                    .await
            }
            Self::Placeholder(secret_manager) => {
                secret_manager
                    .generate_ed25519_addresses(coin_type, account_index, address_indexes, options)
                    .await
            }
        }
    }

    async fn generate_evm_addresses(
        &self,
        coin_type: u32,
        account_index: u32,
        address_indexes: Range<u32>,
        options: impl Into<Option<GenerateAddressOptions>> + Send,
    ) -> Result<Vec<EvmAddress>, Self::Error> {
        match self {
            #[cfg(feature = "stronghold")]
            Self::Stronghold(secret_manager) => Ok(secret_manager
                .generate_evm_addresses(coin_type, account_index, address_indexes, options)
                .await?),
            #[cfg(feature = "ledger_nano")]
            Self::LedgerNano(secret_manager) => Ok(secret_manager
                .generate_evm_addresses(coin_type, account_index, address_indexes, options)
                .await?),
            Self::Mnemonic(secret_manager) => {
                secret_manager
                    .generate_evm_addresses(coin_type, account_index, address_indexes, options)
                    .await
            }
            Self::Placeholder(secret_manager) => {
                secret_manager
                    .generate_evm_addresses(coin_type, account_index, address_indexes, options)
                    .await
            }
        }
    }

    async fn sign_ed25519(&self, msg: &[u8], chain: &Chain) -> crate::client::Result<Ed25519Signature> {
        match self {
            #[cfg(feature = "stronghold")]
            Self::Stronghold(secret_manager) => Ok(secret_manager.sign_ed25519(msg, chain).await?),
            #[cfg(feature = "ledger_nano")]
            Self::LedgerNano(secret_manager) => Ok(secret_manager.sign_ed25519(msg, chain).await?),
            Self::Mnemonic(secret_manager) => secret_manager.sign_ed25519(msg, chain).await,
            Self::Placeholder(secret_manager) => secret_manager.sign_ed25519(msg, chain).await,
        }
    }

    async fn sign_evm(
        &self,
        msg: &[u8],
        chain: &Chain,
    ) -> Result<(secp256k1_ecdsa::PublicKey, secp256k1_ecdsa::Signature), Self::Error> {
        match self {
            #[cfg(feature = "stronghold")]
            Self::Stronghold(secret_manager) => Ok(secret_manager.sign_evm(msg, chain).await?),
            #[cfg(feature = "ledger_nano")]
            Self::LedgerNano(secret_manager) => Ok(secret_manager.sign_evm(msg, chain).await?),
            Self::Mnemonic(secret_manager) => secret_manager.sign_evm(msg, chain).await,
            Self::Placeholder(secret_manager) => secret_manager.sign_evm(msg, chain).await,
        }
    }
}

#[async_trait]
impl SignTransactionEssence for SecretManager {
    async fn sign_transaction_essence(
        &self,
        prepared_transaction_data: &PreparedTransactionData,
        time: Option<u32>,
    ) -> crate::client::Result<Unlocks> {
        match self {
            #[cfg(feature = "stronghold")]
            Self::Stronghold(_) => {
                self.default_sign_transaction_essence(prepared_transaction_data, time)
                    .await
            }
            #[cfg(feature = "ledger_nano")]
            Self::LedgerNano(secret_manager) => Ok(secret_manager
                .sign_transaction_essence(prepared_transaction_data, time)
                .await?),
            Self::Mnemonic(_) => {
                self.default_sign_transaction_essence(prepared_transaction_data, time)
                    .await
            }
            Self::Placeholder(secret_manager) => {
                secret_manager
                    .sign_transaction_essence(prepared_transaction_data, time)
                    .await
            }
        }
    }
}

impl SecretManager {
    /// Tries to create a [`SecretManager`] from a mnemonic string.
    pub fn try_from_mnemonic(mnemonic: impl Into<Zeroizing<String>>) -> crate::client::Result<Self> {
        Ok(Self::Mnemonic(MnemonicSecretManager::try_from_mnemonic(mnemonic)?))
    }

    /// Tries to create a [`SecretManager`] from a seed hex string.
    pub fn try_from_hex_seed(seed: impl Into<Zeroizing<String>>) -> crate::client::Result<Self> {
        Ok(Self::Mnemonic(MnemonicSecretManager::try_from_hex_seed(seed)?))
    }

    // Shared implementation for MnemonicSecretManager and StrongholdSecretManager
    async fn default_sign_transaction_essence(
        &self,
        prepared_transaction_data: &PreparedTransactionData,
        time: Option<u32>,
    ) -> crate::client::Result<Unlocks> {
        // The hashed_essence gets signed
        let hashed_essence = prepared_transaction_data.essence.hash();
        let mut blocks = Vec::new();
        let mut block_indexes = HashMap::<Address, usize>::new();

        // Assuming inputs_data is ordered by address type
        for (current_block_index, input) in prepared_transaction_data.inputs_data.iter().enumerate() {
            // Get the address that is required to unlock the input
            let TransactionEssence::Regular(regular) = &prepared_transaction_data.essence;
            let alias_transition = is_alias_transition(&input.output, *input.output_id(), regular.outputs(), None);
            let (input_address, _) = input.output.required_and_unlocked_address(
                time.unwrap_or_else(|| unix_timestamp_now().as_secs() as u32),
                input.output_metadata.output_id(),
                alias_transition,
            )?;

            // Check if we already added an [Unlock] for this address
            match block_indexes.get(&input_address) {
                // If we already have an [Unlock] for this address, add a [Unlock] based on the address type
                Some(block_index) => match input_address {
                    Address::Alias(_alias) => blocks.push(Unlock::Alias(AliasUnlock::new(*block_index as u16)?)),
                    Address::Ed25519(_ed25519) => {
                        blocks.push(Unlock::Reference(ReferenceUnlock::new(*block_index as u16)?));
                    }
                    Address::Nft(_nft) => blocks.push(Unlock::Nft(NftUnlock::new(*block_index as u16)?)),
                },
                None => {
                    // We can only sign ed25519 addresses and block_indexes needs to contain the alias or nft
                    // address already at this point, because the reference index needs to be lower
                    // than the current block index
                    if !input_address.is_ed25519() {
                        return Err(InputSelectionError::MissingInputWithEd25519Address)?;
                    }

                    let chain = input.chain.as_ref().ok_or(Error::MissingBip32Chain)?;

                    let block = self.signature_unlock(&hashed_essence, chain).await?;
                    blocks.push(block);

                    // Add the ed25519 address to the block_indexes, so it gets referenced if further inputs have
                    // the same address in their unlock condition
                    block_indexes.insert(input_address, current_block_index);
                }
            }

            // When we have an alias or Nft output, we will add their alias or nft address to block_indexes,
            // because they can be used to unlock outputs via [Unlock::Alias] or [Unlock::Nft],
            // that have the corresponding alias or nft address in their unlock condition
            match &input.output {
                Output::Alias(alias_output) => block_indexes.insert(
                    Address::Alias(alias_output.alias_address(input.output_id())),
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

    /// Sign a transaction
    pub async fn sign_transaction(
        &self,
        prepared_transaction_data: PreparedTransactionData,
    ) -> crate::client::Result<Payload> {
        log::debug!("[sign_transaction] {:?}", prepared_transaction_data);
        let current_time = unix_timestamp_now().as_secs() as u32;

        let unlocks = self
            .sign_transaction_essence(&prepared_transaction_data, Some(current_time))
            .await?;
        let tx_payload = TransactionPayload::new(prepared_transaction_data.essence.clone(), unlocks)?;

        validate_transaction_payload_length(&tx_payload)?;

        let conflict = verify_semantic(&prepared_transaction_data.inputs_data, &tx_payload, current_time)?;

        if conflict != ConflictReason::None {
            log::debug!("[sign_transaction] conflict: {conflict:?} for {:#?}", tx_payload);
            return Err(Error::TransactionSemantic(conflict));
        }

        Ok(Payload::from(tx_payload))
    }
}
