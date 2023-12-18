// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Secret manager module enabling address generation and transaction signing.

/// Module for ledger nano based secret management.
#[cfg(feature = "ledger_nano")]
#[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
pub mod ledger_nano;
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

#[cfg(feature = "stronghold")]
use std::time::Duration;
use std::{collections::HashMap, fmt::Debug, ops::Range, str::FromStr};

use async_trait::async_trait;
use crypto::{
    hashes::{blake2b::Blake2b256, Digest},
    keys::{bip39::Mnemonic, bip44::Bip44},
    signatures::{
        ed25519,
        secp256k1_ecdsa::{self, EvmAddress},
    },
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use zeroize::Zeroizing;

#[cfg(feature = "ledger_nano")]
use self::ledger_nano::LedgerSecretManager;
use self::mnemonic::MnemonicSecretManager;
#[cfg(feature = "private_key_secret_manager")]
use self::private_key::PrivateKeySecretManager;
#[cfg(feature = "stronghold")]
use self::stronghold::StrongholdSecretManager;
pub use self::types::{GenerateAddressOptions, LedgerNanoStatus};
#[cfg(feature = "stronghold")]
use crate::client::secret::types::StrongholdDto;
use crate::{
    client::{
        api::{
            input_selection::Error as InputSelectionError, transaction::validate_signed_transaction_payload_length,
            verify_semantic, PreparedTransactionData,
        },
        Error,
    },
    types::block::{
        address::{Address, Ed25519Address},
        core::UnsignedBlock,
        output::Output,
        payload::SignedTransactionPayload,
        protocol::ProtocolParameters,
        signature::{Ed25519Signature, Signature},
        unlock::{AccountUnlock, NftUnlock, ReferenceUnlock, SignatureUnlock, Unlock, Unlocks},
        Block, Error as BlockError,
    },
};

/// The secret manager interface.
#[async_trait]
pub trait SecretManage: Send + Sync {
    type Error: std::error::Error + Send + Sync;

    /// Generates public keys.
    ///
    /// For `coin_type`, see also <https://github.com/satoshilabs/slips/blob/master/slip-0044.md>.
    async fn generate_ed25519_public_keys(
        &self,
        coin_type: u32,
        account_index: u32,
        address_indexes: Range<u32>,
        options: impl Into<Option<GenerateAddressOptions>> + Send,
    ) -> Result<Vec<ed25519::PublicKey>, Self::Error>;

    /// Generates addresses.
    ///
    /// For `coin_type`, see also <https://github.com/satoshilabs/slips/blob/master/slip-0044.md>.
    async fn generate_ed25519_addresses(
        &self,
        coin_type: u32,
        account_index: u32,
        address_indexes: Range<u32>,
        options: impl Into<Option<GenerateAddressOptions>> + Send,
    ) -> Result<Vec<Ed25519Address>, Self::Error> {
        Ok(self
            .generate_ed25519_public_keys(coin_type, account_index, address_indexes, options)
            .await?
            .iter()
            .map(|public_key| Ed25519Address::new(Blake2b256::digest(public_key.to_bytes()).into()))
            .collect())
    }

    async fn generate_evm_addresses(
        &self,
        coin_type: u32,
        account_index: u32,
        address_indexes: Range<u32>,
        options: impl Into<Option<GenerateAddressOptions>> + Send,
    ) -> Result<Vec<EvmAddress>, Self::Error>;

    /// Signs msg using the given [`Bip44`] using Ed25519.
    async fn sign_ed25519(&self, msg: &[u8], chain: Bip44) -> Result<Ed25519Signature, Self::Error>;

    /// Signs msg using the given [`Bip44`] using Secp256k1.
    async fn sign_secp256k1_ecdsa(
        &self,
        msg: &[u8],
        chain: Bip44,
    ) -> Result<(secp256k1_ecdsa::PublicKey, secp256k1_ecdsa::RecoverableSignature), Self::Error>;

    /// Signs `transaction_signing_hash` using the given `chain`, returning an [`Unlock`].
    async fn signature_unlock(&self, transaction_signing_hash: &[u8; 32], chain: Bip44) -> Result<Unlock, Self::Error> {
        Ok(Unlock::from(SignatureUnlock::new(Signature::from(
            self.sign_ed25519(transaction_signing_hash, chain).await?,
        ))))
    }

    async fn transaction_unlocks(
        &self,
        prepared_transaction_data: &PreparedTransactionData,
        protocol_parameters: &ProtocolParameters,
    ) -> Result<Unlocks, Self::Error>;

    async fn sign_transaction(
        &self,
        prepared_transaction_data: PreparedTransactionData,
        protocol_parameters: &ProtocolParameters,
    ) -> Result<SignedTransactionPayload, Self::Error>;
}

pub trait SecretManagerConfig: SecretManage {
    type Config: Serialize + DeserializeOwned + Debug + Send + Sync;

    fn to_config(&self) -> Option<Self::Config>;

    fn from_config(config: &Self::Config) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

/// Supported secret managers
#[non_exhaustive]
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

    /// Secret manager that uses a single private key.
    #[cfg(feature = "private_key_secret_manager")]
    #[cfg_attr(docsrs, doc(cfg(feature = "private_key_secret_manager")))]
    PrivateKey(Box<PrivateKeySecretManager>),

    /// Secret manager that's just a placeholder, so it can be provided to an online wallet, but can't be used for
    /// signing.
    Placeholder,
}

#[cfg(feature = "stronghold")]
impl From<StrongholdSecretManager> for SecretManager {
    fn from(secret_manager: StrongholdSecretManager) -> Self {
        Self::Stronghold(secret_manager)
    }
}

#[cfg(feature = "ledger_nano")]
impl From<LedgerSecretManager> for SecretManager {
    fn from(secret_manager: LedgerSecretManager) -> Self {
        Self::LedgerNano(secret_manager)
    }
}

impl From<MnemonicSecretManager> for SecretManager {
    fn from(secret_manager: MnemonicSecretManager) -> Self {
        Self::Mnemonic(secret_manager)
    }
}

#[cfg(feature = "private_key_secret_manager")]
impl From<PrivateKeySecretManager> for SecretManager {
    fn from(secret_manager: PrivateKeySecretManager) -> Self {
        Self::PrivateKey(Box::new(secret_manager))
    }
}

impl Debug for SecretManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "stronghold")]
            Self::Stronghold(_) => f.debug_tuple("Stronghold").field(&"...").finish(),
            #[cfg(feature = "ledger_nano")]
            Self::LedgerNano(_) => f.debug_tuple("LedgerNano").field(&"...").finish(),
            Self::Mnemonic(_) => f.debug_tuple("Mnemonic").field(&"...").finish(),
            #[cfg(feature = "private_key_secret_manager")]
            Self::PrivateKey(_) => f.debug_tuple("PrivateKey").field(&"...").finish(),
            Self::Placeholder => f.debug_struct("Placeholder").finish(),
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
#[non_exhaustive]
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
    /// Private Key
    #[cfg(feature = "private_key_secret_manager")]
    #[cfg_attr(docsrs, doc(cfg(feature = "private_key_secret_manager")))]
    #[serde(alias = "privateKey")]
    PrivateKey(Zeroizing<String>),
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

            SecretManagerDto::Mnemonic(mnemonic) => {
                Self::Mnemonic(MnemonicSecretManager::try_from_mnemonic(mnemonic.as_str().to_owned())?)
            }

            #[cfg(feature = "private_key_secret_manager")]
            SecretManagerDto::PrivateKey(private_key) => {
                Self::PrivateKey(Box::new(PrivateKeySecretManager::try_from_hex(private_key)?))
            }

            SecretManagerDto::HexSeed(hex_seed) => {
                // `SecretManagerDto` is `ZeroizeOnDrop` so it will take care of zeroizing the original.
                Self::Mnemonic(MnemonicSecretManager::try_from_hex_seed(hex_seed)?)
            }

            SecretManagerDto::Placeholder => Self::Placeholder,
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

            #[cfg(feature = "private_key_secret_manager")]
            SecretManager::PrivateKey(_private_key) => Self::PrivateKey("...".to_string().into()),

            SecretManager::Placeholder => Self::Placeholder,
        }
    }
}

#[async_trait]
impl SecretManage for SecretManager {
    type Error = Error;

    async fn generate_ed25519_public_keys(
        &self,
        coin_type: u32,
        account_index: u32,
        address_indexes: Range<u32>,
        options: impl Into<Option<GenerateAddressOptions>> + Send,
    ) -> Result<Vec<ed25519::PublicKey>, Self::Error> {
        match self {
            #[cfg(feature = "stronghold")]
            Self::Stronghold(secret_manager) => Ok(secret_manager
                .generate_ed25519_public_keys(coin_type, account_index, address_indexes, options)
                .await?),
            #[cfg(feature = "ledger_nano")]
            Self::LedgerNano(secret_manager) => Ok(secret_manager
                .generate_ed25519_public_keys(coin_type, account_index, address_indexes, options)
                .await?),
            Self::Mnemonic(secret_manager) => {
                secret_manager
                    .generate_ed25519_public_keys(coin_type, account_index, address_indexes, options)
                    .await
            }
            #[cfg(feature = "private_key_secret_manager")]
            Self::PrivateKey(secret_manager) => {
                secret_manager
                    .generate_ed25519_public_keys(coin_type, account_index, address_indexes, options)
                    .await
            }
            Self::Placeholder => Err(Error::PlaceholderSecretManager),
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
            #[cfg(feature = "private_key_secret_manager")]
            Self::PrivateKey(secret_manager) => {
                secret_manager
                    .generate_evm_addresses(coin_type, account_index, address_indexes, options)
                    .await
            }
            Self::Placeholder => Err(Error::PlaceholderSecretManager),
        }
    }

    async fn sign_ed25519(&self, msg: &[u8], chain: Bip44) -> crate::client::Result<Ed25519Signature> {
        match self {
            #[cfg(feature = "stronghold")]
            Self::Stronghold(secret_manager) => Ok(secret_manager.sign_ed25519(msg, chain).await?),
            #[cfg(feature = "ledger_nano")]
            Self::LedgerNano(secret_manager) => Ok(secret_manager.sign_ed25519(msg, chain).await?),
            Self::Mnemonic(secret_manager) => secret_manager.sign_ed25519(msg, chain).await,
            #[cfg(feature = "private_key_secret_manager")]
            Self::PrivateKey(secret_manager) => secret_manager.sign_ed25519(msg, chain).await,
            Self::Placeholder => Err(Error::PlaceholderSecretManager),
        }
    }

    async fn sign_secp256k1_ecdsa(
        &self,
        msg: &[u8],
        chain: Bip44,
    ) -> Result<(secp256k1_ecdsa::PublicKey, secp256k1_ecdsa::RecoverableSignature), Self::Error> {
        match self {
            #[cfg(feature = "stronghold")]
            Self::Stronghold(secret_manager) => Ok(secret_manager.sign_secp256k1_ecdsa(msg, chain).await?),
            #[cfg(feature = "ledger_nano")]
            Self::LedgerNano(secret_manager) => Ok(secret_manager.sign_secp256k1_ecdsa(msg, chain).await?),
            Self::Mnemonic(secret_manager) => secret_manager.sign_secp256k1_ecdsa(msg, chain).await,
            #[cfg(feature = "private_key_secret_manager")]
            Self::PrivateKey(secret_manager) => secret_manager.sign_secp256k1_ecdsa(msg, chain).await,
            Self::Placeholder => Err(Error::PlaceholderSecretManager),
        }
    }

    async fn transaction_unlocks(
        &self,
        prepared_transaction_data: &PreparedTransactionData,
        protocol_parameters: &ProtocolParameters,
    ) -> Result<Unlocks, Self::Error> {
        match self {
            #[cfg(feature = "stronghold")]
            Self::Stronghold(secret_manager) => Ok(secret_manager
                .transaction_unlocks(prepared_transaction_data, protocol_parameters)
                .await?),
            #[cfg(feature = "ledger_nano")]
            Self::LedgerNano(secret_manager) => Ok(secret_manager
                .transaction_unlocks(prepared_transaction_data, protocol_parameters)
                .await?),
            Self::Mnemonic(secret_manager) => {
                secret_manager
                    .transaction_unlocks(prepared_transaction_data, protocol_parameters)
                    .await
            }
            #[cfg(feature = "private_key_secret_manager")]
            Self::PrivateKey(secret_manager) => {
                secret_manager
                    .transaction_unlocks(prepared_transaction_data, protocol_parameters)
                    .await
            }
            Self::Placeholder => Err(Error::PlaceholderSecretManager),
        }
    }

    async fn sign_transaction(
        &self,
        prepared_transaction_data: PreparedTransactionData,
        protocol_parameters: &ProtocolParameters,
    ) -> Result<SignedTransactionPayload, Self::Error> {
        match self {
            #[cfg(feature = "stronghold")]
            Self::Stronghold(secret_manager) => Ok(secret_manager
                .sign_transaction(prepared_transaction_data, protocol_parameters)
                .await?),
            #[cfg(feature = "ledger_nano")]
            Self::LedgerNano(secret_manager) => Ok(secret_manager
                .sign_transaction(prepared_transaction_data, protocol_parameters)
                .await?),
            Self::Mnemonic(secret_manager) => {
                secret_manager
                    .sign_transaction(prepared_transaction_data, protocol_parameters)
                    .await
            }
            #[cfg(feature = "private_key_secret_manager")]
            Self::PrivateKey(secret_manager) => {
                secret_manager
                    .sign_transaction(prepared_transaction_data, protocol_parameters)
                    .await
            }
            Self::Placeholder => Err(Error::PlaceholderSecretManager),
        }
    }
}

pub trait DowncastSecretManager: SecretManage {
    fn downcast<T: 'static + SecretManage>(&self) -> Option<&T>;
}

impl<S: 'static + SecretManage + Send + Sync> DowncastSecretManager for S {
    fn downcast<T: 'static + SecretManage>(&self) -> Option<&T> {
        (self as &(dyn std::any::Any + Send + Sync)).downcast_ref::<T>()
    }
}

impl SecretManagerConfig for SecretManager {
    type Config = SecretManagerDto;

    fn to_config(&self) -> Option<Self::Config> {
        match self {
            #[cfg(feature = "stronghold")]
            Self::Stronghold(s) => s.to_config().map(Self::Config::Stronghold),
            #[cfg(feature = "ledger_nano")]
            Self::LedgerNano(s) => s.to_config().map(Self::Config::LedgerNano),
            Self::Mnemonic(_) => None,
            #[cfg(feature = "private_key_secret_manager")]
            Self::PrivateKey(_) => None,
            Self::Placeholder => None,
        }
    }

    fn from_config(config: &Self::Config) -> Result<Self, Self::Error> {
        Ok(match config {
            #[cfg(feature = "stronghold")]
            SecretManagerDto::Stronghold(config) => Self::Stronghold(StrongholdSecretManager::from_config(config)?),
            #[cfg(feature = "ledger_nano")]
            SecretManagerDto::LedgerNano(config) => Self::LedgerNano(LedgerSecretManager::from_config(config)?),
            SecretManagerDto::HexSeed(hex_seed) => {
                Self::Mnemonic(MnemonicSecretManager::try_from_hex_seed(hex_seed.clone())?)
            }
            SecretManagerDto::Mnemonic(mnemonic) => {
                Self::Mnemonic(MnemonicSecretManager::try_from_mnemonic(mnemonic.as_str().to_owned())?)
            }
            #[cfg(feature = "private_key_secret_manager")]
            SecretManagerDto::PrivateKey(private_key) => {
                Self::PrivateKey(Box::new(PrivateKeySecretManager::try_from_hex(private_key.to_owned())?))
            }
            SecretManagerDto::Placeholder => Self::Placeholder,
        })
    }
}

impl SecretManager {
    /// Tries to create a [`SecretManager`] from a mnemonic string.
    pub fn try_from_mnemonic(mnemonic: impl Into<Mnemonic>) -> crate::client::Result<Self> {
        Ok(Self::Mnemonic(MnemonicSecretManager::try_from_mnemonic(mnemonic)?))
    }

    /// Tries to create a [`SecretManager`] from a seed hex string.
    pub fn try_from_hex_seed(seed: impl Into<Zeroizing<String>>) -> crate::client::Result<Self> {
        Ok(Self::Mnemonic(MnemonicSecretManager::try_from_hex_seed(seed)?))
    }
}

pub(crate) async fn default_transaction_unlocks<M: SecretManage>(
    secret_manager: &M,
    prepared_transaction_data: &PreparedTransactionData,
    protocol_parameters: &ProtocolParameters,
) -> crate::client::Result<Unlocks>
where
    crate::client::Error: From<M::Error>,
{
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

                let chain = input.chain.ok_or(Error::MissingBip32Chain)?;

                let block = secret_manager
                    .signature_unlock(&transaction_signing_hash, chain)
                    .await?;
                blocks.push(block);

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

pub(crate) async fn default_sign_transaction<M: SecretManage>(
    secret_manager: &M,
    prepared_transaction_data: PreparedTransactionData,
    protocol_parameters: &ProtocolParameters,
) -> crate::client::Result<SignedTransactionPayload>
where
    crate::client::Error: From<M::Error>,
{
    log::debug!("[sign_transaction] {:?}", prepared_transaction_data);

    let unlocks = secret_manager
        .transaction_unlocks(&prepared_transaction_data, protocol_parameters)
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

#[async_trait]
pub trait SignBlock {
    async fn sign_ed25519<S: SecretManage>(self, secret_manager: &S, chain: Bip44) -> crate::client::Result<Block>
    where
        crate::client::Error: From<S::Error>;
}

#[async_trait]
impl SignBlock for UnsignedBlock {
    async fn sign_ed25519<S: SecretManage>(self, secret_manager: &S, chain: Bip44) -> crate::client::Result<Block>
    where
        crate::client::Error: From<S::Error>,
    {
        let msg = self.signing_input();
        Ok(self.finish(secret_manager.sign_ed25519(&msg, chain).await?)?)
    }
}
