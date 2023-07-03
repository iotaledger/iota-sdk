// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::wallet::storage::{constants::default_storage_path, manager::StorageKind};

#[cfg(feature = "storage")]
#[cfg_attr(docsrs, doc(cfg(feature = "storage")))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageOptions {
    pub(crate) path: PathBuf,
    pub(crate) encryption_key: Option<[u8; 32]>,
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
