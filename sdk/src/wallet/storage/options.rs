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
    pub fn new(path: PathBuf, kind: StorageKind) -> Self {
        Self {
            path,
            encryption_key: None,
            kind,
        }
    }

    /// Adds an encryption key to the [`StorageOptions`].
    pub fn with_encryption_key(mut self, encryption_key: [u8; 32]) -> Self {
        self.encryption_key = Some(Zeroizing::new(encryption_key));
        self
    }

    /// Returns the path of the [`StorageOptions`];
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the encryption key of the [`StorageOptions`];
    pub fn encryption_key(&self) -> Option<&[u8; 32]> {
        self.encryption_key.as_deref()
    }

    /// Returns the kind of the [`StorageOptions`];
    pub fn kind(&self) -> StorageKind {
        self.kind
    }
}
