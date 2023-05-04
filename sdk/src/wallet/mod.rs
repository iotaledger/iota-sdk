// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The IOTA Wallet Library

/// [`Account`]: crate::wallet::Account
/// The account module. Interaction with an Account happens via an [`Account`].
pub mod account;
/// The message passing interface for the library. A different way to call the wallet functions, useful for bindings to
/// other languages.
#[cfg(feature = "message_interface")]
#[cfg_attr(docsrs, doc(cfg(feature = "message_interface")))]
pub mod message_interface;
/// The wallet module.
#[allow(clippy::module_inception)]
pub mod wallet;

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

// Expose for high level functions
pub use primitive_types::U256;

pub use self::{
    account::{
        operations::transaction::high_level::{
            minting::{
                mint_native_token::MintNativeTokenParams, mint_nfts::MintNftParams,
            },
            send_amount::SendAmountParams,
            send_native_tokens::SendNativeTokensParams,
            send_nft::SendNftParams,
        },
        Account,
    },
    error::Error,
    wallet::{Wallet, WalletBuilder},
};

/// The wallet Result type.
pub type Result<T> = std::result::Result<T, Error>;
