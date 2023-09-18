// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

use crate::wallet::storage::{constants::default_storage_path, StorageKind};

#[cfg(feature = "storage")]
#[cfg_attr(docsrs, doc(cfg(feature = "storage")))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageOptions {
    pub(crate) path: PathBuf,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) encryption_key: Option<Zeroizing<[u8; 32]>>,
    pub(crate) kind: StorageKind,
}

#[cfg(feature = "storage")]
impl Default for StorageOptions {
    fn default() -> Self {
        Self {
            path: default_storage_path().into(),
            encryption_key: None,
            kind: StorageKind::default(),
        }
    }
}

impl StorageOptions {
    /// Creates a new [`StorageOptions`].
    pub fn new(path: impl Into<PathBuf> + Send, kind: StorageKind) -> Self {
        Self {
            path: path.into(),
            encryption_key: None,
            kind,
        }
    }

    /// Creates a new [`StorageOptions`] from a path, with a default storage kind depending on features.
    pub fn from_path(path: impl Into<PathBuf> + Send) -> Self {
        Self::new(path, Default::default())
    }

    /// Adds an encryption key to the [`StorageOptions`].
    pub fn with_encryption_key(mut self, encryption_key: [u8; 32]) -> Self {
        self.encryption_key = Some(Zeroizing::new(encryption_key));
        self
    }

    /// Returns the path of the [`StorageOptions`].
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the encryption key of the [`StorageOptions`].
    pub fn encryption_key(&self) -> Option<&[u8; 32]> {
        self.encryption_key.as_deref()
    }

    /// Returns the kind of the [`StorageOptions`].
    pub fn kind(&self) -> StorageKind {
        self.kind
    }
}

impl From<&str> for StorageOptions {
    fn from(value: &str) -> Self {
        Self::from_path(value)
    }
}
impl From<&String> for StorageOptions {
    fn from(value: &String) -> Self {
        Self::from_path(value)
    }
}
impl From<String> for StorageOptions {
    fn from(value: String) -> Self {
        Self::from_path(value)
    }
}
impl From<&Path> for StorageOptions {
    fn from(value: &Path) -> Self {
        Self::from_path(value)
    }
}
impl From<PathBuf> for StorageOptions {
    fn from(value: PathBuf) -> Self {
        Self::from_path(value)
    }
}
impl From<&PathBuf> for StorageOptions {
    fn from(value: &PathBuf) -> Self {
        Self::from_path(value)
    }
}
