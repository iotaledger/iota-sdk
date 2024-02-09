// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use iota_sdk::{
    client::secret::{
        ledger_nano::{LedgerOptions, LedgerSecretManager},
        stronghold::StrongholdSecretManager,
        Generate, PublicKeyOptions, SecretManagerConfig, SecretManagerDto, Sign, SignTransaction,
    },
    crypto::{keys::bip44::Bip44, signatures::ed25519},
    types::block::{address::Ed25519Address, signature::Ed25519Signature},
};

#[derive(Debug, strum::AsRefStr)]
pub enum SecretManager {
    Stronghold(StrongholdSecretManager),
    LedgerNano(LedgerSecretManager),
}

#[async_trait]
impl Generate<ed25519::PublicKey> for SecretManager {
    type Options = LedgerOptions<PublicKeyOptions>;

    async fn generate(&self, options: &Self::Options) -> iota_sdk::client::Result<ed25519::PublicKey> {
        match self {
            SecretManager::Stronghold(s) => Ok(s.generate(&options.options).await?),
            SecretManager::LedgerNano(l) => Ok(l.generate(options).await?),
        }
    }
}

#[async_trait]
impl Generate<Ed25519Address> for SecretManager {
    type Options = LedgerOptions<PublicKeyOptions>;

    async fn generate(&self, options: &Self::Options) -> iota_sdk::client::Result<Ed25519Address> {
        let public_key: ed25519::PublicKey = self.generate(options).await?;
        Ok(Ed25519Address::from_public_key_bytes(public_key.to_bytes()))
    }
}

#[async_trait]
impl Sign<Ed25519Signature> for SecretManager {
    type Options = Bip44;

    async fn sign(&self, msg: &[u8], options: &Self::Options) -> iota_sdk::client::Result<Ed25519Signature> {
        match self {
            SecretManager::Stronghold(s) => Ok(s.sign(msg, options).await?),
            SecretManager::LedgerNano(l) => Ok(l.sign(msg, options).await?),
        }
    }
}

impl SignTransaction for SecretManager {}

impl SecretManagerConfig for SecretManager {
    type Config = SecretManagerDto;

    fn to_config(&self) -> Option<Self::Config> {
        match self {
            Self::Stronghold(s) => s.to_config().map(Self::Config::Stronghold),
            Self::LedgerNano(s) => s.to_config().map(Self::Config::LedgerNano),
        }
    }

    fn from_config(config: &Self::Config) -> iota_sdk::client::Result<Self> {
        Ok(match config {
            SecretManagerDto::Stronghold(config) => Self::Stronghold(StrongholdSecretManager::from_config(config)?),
            SecretManagerDto::LedgerNano(config) => Self::LedgerNano(LedgerSecretManager::from_config(config)?),
            _ => panic!("unsupported secret manager"),
        })
    }
}
