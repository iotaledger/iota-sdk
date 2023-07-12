// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
// TODO missing_docs, unreachable_pub
#![deny(clippy::nursery, rust_2018_idioms, warnings)]
#![allow(
    clippy::redundant_pub_crate,
    clippy::missing_const_for_fn,
    clippy::significant_drop_in_scrutinee,
    clippy::significant_drop_tightening
)]

extern crate alloc;

#[cfg(feature = "client")]
#[cfg_attr(docsrs, doc(cfg(feature = "client")))]
pub mod client;
#[cfg(feature = "pow")]
#[cfg_attr(docsrs, doc(cfg(feature = "pow")))]
pub mod pow;
pub mod types;
#[cfg(feature = "wallet")]
#[cfg_attr(docsrs, doc(cfg(feature = "wallet")))]
pub mod wallet;
// Utilities used in multiple submodules
pub mod utils;

pub use crypto;
pub use packable;
pub use primitive_types::U256;
#[cfg(feature = "url")]
pub use url::Url;

#[cfg(feature = "wallet")]
pub type Wallet = self::wallet::Wallet<client::secret::SecretManager>;
