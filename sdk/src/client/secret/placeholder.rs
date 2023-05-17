// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Implementation of [`PlaceholderSecretManager`].

/// Secret manager that is only useful to prevent accidental address generation in a wallet
/// that has an offline counterpart for address generation and signing.
#[derive(Debug)]
pub struct PlaceholderSecretManager;
