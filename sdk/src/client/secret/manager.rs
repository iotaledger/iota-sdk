// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use async_trait::async_trait;
use crypto::{
    keys::bip39::Mnemonic,
    signatures::{
        ed25519,
        secp256k1_ecdsa::{self, EvmAddress},
    },
};
use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

#[cfg(feature = "ledger_nano")]
use crate::client::secret::ledger_nano::{self, LedgerSecretManager};
#[cfg(feature = "private_key_secret_manager")]
use crate::client::secret::private_key::PrivateKeySecretManager;
#[cfg(feature = "stronghold")]
use crate::client::secret::{stronghold::StrongholdSecretManager, types::StrongholdDto};
use crate::{
    client::{
        secret::{
            mnemonic::MnemonicSecretManager, types::EvmSignature, Generate, SecretManagerConfig, Sign, SignTransaction,
        },
        Error,
    },
    types::block::{address::Ed25519Address, signature::Ed25519Signature},
};

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

#[async_trait]
impl Generate<ed25519::PublicKey> for SecretManager {
    type Options = serde_json::Value;

    async fn generate(&self, options: &Self::Options) -> crate::client::Result<ed25519::PublicKey> {
        match self {
            #[cfg(feature = "stronghold")]
            SecretManager::Stronghold(s) => {
                let options = <StrongholdSecretManager as Generate<ed25519::PublicKey>>::Options::deserialize(options)?;
                Ok(s.generate(&options).await?)
            }
            #[cfg(feature = "ledger_nano")]
            SecretManager::LedgerNano(l) => {
                let options = <LedgerSecretManager as Generate<ed25519::PublicKey>>::Options::deserialize(options)?;
                Ok(l.generate(&options).await?)
            }
            SecretManager::Mnemonic(m) => {
                let options = <MnemonicSecretManager as Generate<ed25519::PublicKey>>::Options::deserialize(options)?;
                Ok(m.generate(&options).await?)
            }
            #[cfg(feature = "private_key_secret_manager")]
            SecretManager::PrivateKey(p) => Ok(p.generate(&()).await?),
            SecretManager::Placeholder => Err(Error::PlaceholderSecretManager),
        }
    }
}

#[async_trait]
impl Generate<Ed25519Address> for SecretManager {
    type Options = serde_json::Value;

    async fn generate(&self, options: &Self::Options) -> crate::client::Result<Ed25519Address> {
        let public_key: ed25519::PublicKey = self.generate(options).await?;
        Ok(Ed25519Address::from_public_key_bytes(public_key.to_bytes()))
    }
}

#[async_trait]
impl Generate<Vec<ed25519::PublicKey>> for SecretManager {
    type Options = serde_json::Value;

    async fn generate(&self, options: &Self::Options) -> crate::client::Result<Vec<ed25519::PublicKey>> {
        match self {
            #[cfg(feature = "stronghold")]
            SecretManager::Stronghold(s) => {
                let options =
                    <StrongholdSecretManager as Generate<Vec<ed25519::PublicKey>>>::Options::deserialize(options)?;
                Ok(s.generate(&options).await?)
            }
            #[cfg(feature = "ledger_nano")]
            SecretManager::LedgerNano(l) => {
                let options =
                    <LedgerSecretManager as Generate<Vec<ed25519::PublicKey>>>::Options::deserialize(options)?;
                Ok(l.generate(&options).await?)
            }
            SecretManager::Mnemonic(m) => {
                let options =
                    <MnemonicSecretManager as Generate<Vec<ed25519::PublicKey>>>::Options::deserialize(options)?;
                Ok(m.generate(&options).await?)
            }
            #[cfg(feature = "private_key_secret_manager")]
            SecretManager::PrivateKey(_) => todo!(),
            SecretManager::Placeholder => Err(Error::PlaceholderSecretManager),
        }
    }
}

#[async_trait]
impl Generate<Vec<Ed25519Address>> for SecretManager {
    type Options = serde_json::Value;

    async fn generate(&self, options: &Self::Options) -> crate::client::Result<Vec<Ed25519Address>> {
        let public_keys: Vec<ed25519::PublicKey> = self.generate(options).await?;
        Ok(public_keys
            .into_iter()
            .map(|k| Ed25519Address::from_public_key_bytes(k.to_bytes()))
            .collect())
    }
}

#[async_trait]
impl Generate<secp256k1_ecdsa::PublicKey> for SecretManager {
    type Options = serde_json::Value;

    async fn generate(&self, options: &Self::Options) -> crate::client::Result<secp256k1_ecdsa::PublicKey> {
        match self {
            #[cfg(feature = "stronghold")]
            SecretManager::Stronghold(s) => {
                let options =
                    <StrongholdSecretManager as Generate<secp256k1_ecdsa::PublicKey>>::Options::deserialize(options)?;
                Ok(s.generate(&options).await?)
            }
            #[cfg(feature = "ledger_nano")]
            SecretManager::LedgerNano(_) => Err(ledger_nano::Error::UnsupportedOperation.into()),
            SecretManager::Mnemonic(m) => {
                let options =
                    <MnemonicSecretManager as Generate<secp256k1_ecdsa::PublicKey>>::Options::deserialize(options)?;
                Ok(m.generate(&options).await?)
            }
            #[cfg(feature = "private_key_secret_manager")]
            SecretManager::PrivateKey(_) => todo!(),
            SecretManager::Placeholder => Err(Error::PlaceholderSecretManager),
        }
    }
}

#[async_trait]
impl Generate<EvmAddress> for SecretManager {
    type Options = serde_json::Value;

    async fn generate(&self, options: &Self::Options) -> crate::client::Result<EvmAddress> {
        let public_key: secp256k1_ecdsa::PublicKey = self.generate(options).await?;
        Ok(public_key.evm_address())
    }
}

#[async_trait]
impl Generate<Vec<secp256k1_ecdsa::PublicKey>> for SecretManager {
    type Options = serde_json::Value;

    async fn generate(&self, options: &Self::Options) -> crate::client::Result<Vec<secp256k1_ecdsa::PublicKey>> {
        match self {
            #[cfg(feature = "stronghold")]
            SecretManager::Stronghold(s) => {
                let options =
                    <StrongholdSecretManager as Generate<Vec<secp256k1_ecdsa::PublicKey>>>::Options::deserialize(
                        options,
                    )?;
                Ok(s.generate(&options).await?)
            }
            #[cfg(feature = "ledger_nano")]
            SecretManager::LedgerNano(_) => Err(ledger_nano::Error::UnsupportedOperation.into()),
            SecretManager::Mnemonic(m) => {
                let options =
                    <MnemonicSecretManager as Generate<Vec<secp256k1_ecdsa::PublicKey>>>::Options::deserialize(
                        options,
                    )?;
                Ok(m.generate(&options).await?)
            }
            #[cfg(feature = "private_key_secret_manager")]
            SecretManager::PrivateKey(_) => todo!(),
            SecretManager::Placeholder => Err(Error::PlaceholderSecretManager),
        }
    }
}

#[async_trait]
impl Generate<Vec<EvmAddress>> for SecretManager {
    type Options = serde_json::Value;

    async fn generate(&self, options: &Self::Options) -> crate::client::Result<Vec<EvmAddress>> {
        let public_keys: Vec<secp256k1_ecdsa::PublicKey> = self.generate(options).await?;
        Ok(public_keys.into_iter().map(|k| k.evm_address()).collect())
    }
}

#[async_trait]
impl Sign<Ed25519Signature> for SecretManager {
    type Options = serde_json::Value;

    async fn sign(&self, msg: &[u8], options: &Self::Options) -> crate::client::Result<Ed25519Signature> {
        match self {
            #[cfg(feature = "stronghold")]
            SecretManager::Stronghold(s) => {
                let options = <StrongholdSecretManager as Sign<Ed25519Signature>>::Options::deserialize(options)?;
                Ok(s.sign(msg, &options).await?)
            }
            #[cfg(feature = "ledger_nano")]
            SecretManager::LedgerNano(l) => {
                let options = <LedgerSecretManager as Sign<Ed25519Signature>>::Options::deserialize(options)?;
                Ok(l.sign(msg, &options).await?)
            }
            SecretManager::Mnemonic(m) => {
                let options = <MnemonicSecretManager as Sign<Ed25519Signature>>::Options::deserialize(options)?;
                Ok(m.sign(msg, &options).await?)
            }
            #[cfg(feature = "private_key_secret_manager")]
            SecretManager::PrivateKey(p) => Ok(p.sign(msg, &()).await?),
            SecretManager::Placeholder => Err(Error::PlaceholderSecretManager),
        }
    }
}

#[async_trait]
impl Sign<EvmSignature> for SecretManager {
    type Options = serde_json::Value;

    async fn sign(&self, msg: &[u8], options: &Self::Options) -> crate::client::Result<EvmSignature> {
        match self {
            #[cfg(feature = "stronghold")]
            SecretManager::Stronghold(s) => {
                let options = <StrongholdSecretManager as Sign<EvmSignature>>::Options::deserialize(options)?;
                Ok(s.sign(msg, &options).await?)
            }
            #[cfg(feature = "ledger_nano")]
            SecretManager::LedgerNano(_) => Err(ledger_nano::Error::UnsupportedOperation.into()),
            SecretManager::Mnemonic(m) => {
                let options = <MnemonicSecretManager as Sign<EvmSignature>>::Options::deserialize(options)?;
                Ok(m.sign(msg, &options).await?)
            }
            #[cfg(feature = "private_key_secret_manager")]
            SecretManager::PrivateKey(_) => todo!(),
            SecretManager::Placeholder => Err(Error::PlaceholderSecretManager),
        }
    }
}

impl SignTransaction for SecretManager {}

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

impl core::fmt::Debug for SecretManager {
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
                    builder = builder.timeout(core::time::Duration::from_secs(timeout));
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

    fn from_config(config: &Self::Config) -> crate::client::Result<Self> {
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
