// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The IOTA Wallet Library

/// [`Account`]: crate::wallet::Account
/// The account module. Interaction with an Account happens via an [`Account`].
pub mod account;
/// The core module.
pub mod core;
#[cfg(any(feature = "stronghold", feature = "storage"))]
pub(crate) mod migration;

/// The ClientOptions to build the iota_client for interactions with the IOTA Tangle.
pub use crate::client::ClientBuilder as ClientOptions;

/// The error module.
pub mod error;
/// The event module.
#[cfg(feature = "events")]
#[cfg_attr(docsrs, doc(cfg(feature = "events")))]
pub mod events;
/// The storage module.
#[cfg(feature = "storage")]
#[cfg_attr(docsrs, doc(cfg(feature = "storage")))]
pub mod storage;
/// The module for spawning tasks on a thread
pub(crate) mod task;

pub use self::{
    account::{
        operations::transaction::high_level::{
            minting::{create_native_token::CreateNativeTokenParams, mint_nfts::MintNftParams},
            send::SendParams,
            send_native_tokens::SendNativeTokensParams,
            send_nft::SendNftParams,
        },
        Account,
    },
    core::{Wallet, WalletBuilder},
    error::Error,
};

/// The wallet Result type.
pub type Result<T> = std::result::Result<T, Error>;
